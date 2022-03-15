use anchor_lang::prelude::*;
use anchor_lang::{ToAccountInfos};
use anchor_lang::context::{CpiContext};
use spl_token;
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;
use solana_program::{
    pubkey::PUBKEY_BYTES,

    account_info::{next_account_info, AccountInfo},
    decode_error::DecodeError,
    instruction::Instruction,
    msg,
    program_error::{PrintProgramError, ProgramError},
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
};
use crate::pyth;
use crate::constants::*;
use crate::types::ProgramResult;
use crate::instructions::initialize_vault::{VaultAuthority, VaultInfo};

// from SLP
const STALE_AFTER_SLOTS_ELAPSED: u64 = 5;


pub fn handler(
    ctx: Context<Deposit>,
    token_a_lamports: u32,
    token_b_lamports: u32,
    bumps: [u8; 6],
) -> ProgramResult {
    
    { 
        msg!("current slot is {}, {}", ctx.accounts.sys_var_clock.slot, Clock::get().unwrap().slot);
    }
    // Find lending market authority
    let (lending_market_authority_pubkey, bump_seed) = Pubkey::find_program_address(
        &[&ctx.accounts.lending_market.key.to_bytes()[..PUBKEY_BYTES]],
        &ctx.accounts.tulip_lending_program.key,
    );

    {
        let pyth_price_data = ctx.accounts.price_oracle.try_borrow_data()?;
        let pyth_price = pyth::load::<pyth::Price>(&pyth_price_data)
            .map_err(|_| ProgramError::InvalidAccountData)?;

            if pyth_price.ptype != pyth::PriceType::Price {
                msg!("Oracle price type is invalid");
                return Err(error!(LendingError::InvalidOracleConfig));
            }
        
            // if pyth_price.agg.status != pyth::PriceStatus::Trading {
            //     msg!("Oracle price status is invalid");
            //     return Err(error!(LendingError::InvalidOracleConfig));
            // }
        
            msg!("local sysvarclock slot is {}, pyth valid slot is {}. pyth status is {:?}", ctx.accounts.sys_var_clock.slot, pyth_price.valid_slot, match pyth_price.agg.status{
                pyth::PriceStatus::Trading => "trading",
                pyth::PriceStatus::Unknown => "unknown",
                pyth::PriceStatus::Auction => "auction",
                pyth::PriceStatus::Halted  =>  "halted",

            });
            let slots_elapsed = ctx.accounts.sys_var_clock
                .slot
                .checked_sub(pyth_price.valid_slot)
                .ok_or(LendingError::MathOverflow)?;
            if slots_elapsed >= STALE_AFTER_SLOTS_ELAPSED {
                msg!("Oracle price is stale");
                return Err(error!(LendingError::InvalidOracleConfig));
            }
        
            let price: u64 = pyth_price.agg.price.try_into().map_err(|_| {
                msg!("Oracle price cannot be negative");
                LendingError::InvalidOracleConfig
            }).unwrap();
    }

    msg!("token a mint {}, decimals {}", ctx.accounts.token_a_mint.key(), ctx.accounts.token_a_mint.decimals);
    msg!("lending market authority pubkey as derived is {} with seed {}", lending_market_authority_pubkey, bump_seed);
    msg!("lending market authority pubkey as passed is {}", ctx.accounts.lending_market_authority.key);
    // msg!("hererererererere in deposit program\nlending program: {}\nuser token a: {}\nvault tutoken a: {}\nreserve acct: {}\nreserve liq sup: {}\ntoken a mint: {}\n lending market: {}\nuser: {}", 
    //     ctx.accounts.tulip_lending_program.key,
    //     ctx.accounts.user_a_token_ata.key(),
    //     ctx.accounts.destination_collateral.key(),
    //     ctx.accounts.reserve_account.key,
    //     ctx.accounts.reserve_liquidity_supply.key(),
    //     ctx.accounts.token_a_mint.key(),
    //     ctx.accounts.lending_market.key,
    //     ctx.accounts.user.key,
    //     //ctx.accounts.lending_market_authority.key,
    // );

    // Deposit token a
    msg!("Constructing refresh_reserve ix");
    let ix = spl_token_lending::instruction::refresh_reserve(
        *ctx.accounts.tulip_lending_program.key,
        *ctx.accounts.reserve_account.key,
        *ctx.accounts.price_oracle.key,
    );
    
    msg!("invoking refresh_reserve ix");
    solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.tulip_lending_program.to_account_info(),
            ctx.accounts.reserve_account.to_account_info(),
            ctx.accounts.price_oracle.to_account_info(),
            ctx.accounts.sys_var_clock.to_account_info(),
        ]
    )?;

    // Deposit token a
    msg!("Constructing deposit_reserve_liquidity ix");
    let ix = spl_token_lending::instruction::deposit_reserve_liquidity(
        *ctx.accounts.tulip_lending_program.key,
        token_a_lamports as u64,
        ctx.accounts.user_a_token_ata.key(),
        //ctx.accounts.destination_collateral.key(), // we don't want to give user token
        ctx.accounts.vault_tutoken_a.key(), // we want vault to have token
        *ctx.accounts.reserve_account.key,
        ctx.accounts.reserve_liquidity_supply.key(),
        ctx.accounts.tutoken_a_mint.key(),
        *ctx.accounts.lending_market.key,
        *ctx.accounts.user.key,
    );

    msg!("Invoking deposit_reserve_liquidity ix");
    solana_program::program::invoke(
        &ix,
        //&ctx.accounts.to_account_infos(),
        &[
            ctx.accounts.lending_market_authority.to_account_info(),
            ctx.accounts.tulip_lending_program.to_account_info(),
            ctx.accounts.user_a_token_ata.to_account_info(),
            //ctx.accounts.destination_collateral.to_account_info(), // we don't want to give user token
            ctx.accounts.vault_tutoken_a.to_account_info(), // we want vault to have token
            ctx.accounts.reserve_account.to_account_info(),
            ctx.accounts.reserve_liquidity_supply.to_account_info(),
            ctx.accounts.token_a_mint.to_account_info(),
            ctx.accounts.tutoken_a_mint.to_account_info(),
            ctx.accounts.user.to_account_info(),
            ctx.accounts.lending_market.to_account_info(),
            ctx.accounts.sys_var_clock.to_account_info(),
        ],
        //&[]
        //&[&[VAULT_AUTHORITY_SEED.as_bytes(), std::str::from_utf8(&ctx.accounts.vault_info.vault_name.iter().filter(|x| **x != 0).map(|&x| x).collect::<Vec<u8>>()).unwrap().as_bytes()]]
    )?;

    // Mint our token to user's wallet
    // msg!("Creating mint ix");
    let mint_amount = 1; //TODO
    // // Construct instruction using spl_token library
    // let ix = spl_token::instruction::mint_to_checked(

    //     // token_program_id: &Pubkey, 
    //     // mint_pubkey: &Pubkey, 
    //     // account_pubkey: &Pubkey, 
    //     // owner_pubkey: &Pubkey, 
    //     // signer_pubkeys: &[&Pubkey], 
    //     // amount: u64
    //     // decimals: u8

    //     &ctx.accounts.token_program.key(),
    //     &ctx.accounts.redeemable_mint.key(),
    //     &ctx.accounts.user_redeemable_ata.key(),
    //     &ctx.accounts.vault_authority.key(),//&ctx.accounts.user.key(),
    //     &[&ctx.accounts.vault_authority.key()],
    //     mint_amount,
    //     ctx.accounts.redeemable_mint.decimals,
    // )?;

    // // The vault name is stored in the non-empty bytes in vault_info.vault_name
    // // Need to filter empty bytes and then convert to string.
    let vault_name_bytes = &ctx.accounts.vault_info.vault_name
        .iter()
        .filter(|x| **x != 0)
        .map(|&x| x)
        .collect::<Vec<u8>>()
        .clone();
    let vault_name = std::str::from_utf8(vault_name_bytes)
        .unwrap();
    msg!("unpacked vault_name as {}", vault_name);

    // // Pack seeds for pda that will sign mint to tx
    // let seeds: &[&[u8]] = &[VAULT_AUTHORITY_SEED.as_bytes(), vault_name.as_bytes()];
    // let signer_seeds = &[seeds, &[&[bumps[1]]]];
    // msg!("invoking redeemable mint_to ix");
    // solana_program::program::invoke_signed(
    //     &ix,
    //     &[
    //         //ctx.accounts.token_program.to_account_info(),
    //         ctx.accounts.redeemable_mint.to_account_info(),
    //         //ctx.accounts.program.to_account_info(),
    //         ctx.accounts.user_redeemable_ata.to_account_info(),
    //         //ctx.accounts.user.to_account_info(),
    //         ctx.accounts.vault_authority.to_account_info(),
    //     ],
    //     //&[&[TOKEN_VAULT_SEED.as_bytes()]],
    //     //signer_seeds,
    //     &[],
    // )?;

    anchor_spl::token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::MintTo {
                mint: ctx.accounts.redeemable_mint.to_account_info(),
                to: ctx.accounts.user_redeemable_ata.to_account_info(),
                authority: ctx.accounts.vault_authority.to_account_info(),
            },
            &[&[&VAULT_AUTHORITY_SEED.as_bytes(),&vault_name.as_bytes(), &[bumps[1]]]],
        ),
        mint_amount,
    )?;
    Ok(())
}

/// Create context of deposit
/// Gather accounts required for deposit instruction
#[derive(Accounts)]
#[instruction(
    bumps: [u8; 6],
)]
pub struct Deposit<'info> {
    
    /// user
    #[account(mut)]
    pub user: Signer<'info>,

    /// Vault Info
    #[account(
        seeds = [VAULT_INFO_SEED.as_bytes(), {
            msg!("vault_name is {}, bump is {}", &std::str::from_utf8(&vault_info.vault_name.iter().filter(|x| **x != 0).map(|&x| x).collect::<Vec<u8>>().clone()).unwrap(), bumps[0]);
            std::str::from_utf8(&vault_info.vault_name.iter().filter(|x| **x != 0).map(|&x| x).collect::<Vec<u8>>().clone()).unwrap()}.as_bytes()],
        bump = bumps[0],
    )]
    pub vault_info: Box<Account<'info, VaultInfo>>,

    /// Vault authority: PDA which manages everything within the vault
    #[account(
        seeds = [VAULT_AUTHORITY_SEED.as_bytes(), std::str::from_utf8(&vault_info.vault_name.iter().filter(|x| **x != 0).map(|&x| x).collect::<Vec<u8>>()).unwrap().as_bytes()],
        bump = bumps[1],
    )]
    pub vault_authority: Box<Account<'info, VaultAuthority>>,

    // #[account(
    //     init,
    //     payer = vault_admin,
    // )]
    // pub vault_mints: Account<'info, VaultMints>,

    /// Token A mint
    #[account(address = vault_info.token_a_mint)]
    pub token_a_mint: Box<Account<'info, Mint>>,

    /// Token B mint
    #[account(address = vault_info.token_b_mint)]
    pub token_b_mint: Box<Account<'info, Mint>>,

    /// tuToken A mint
    #[account(mut)]//address = tutoken_a_mint_address)]
    pub tutoken_a_mint: Box<Account<'info, Mint>>,

    /// tuToken B mint
    #[account()]//address = tutoken_b_mint_address)]
    pub tutoken_b_mint: Box<Account<'info, Mint>>,

    /// Accounts that stores token A
    #[account(
        seeds = [VAULT_TOKENA_SEED.as_bytes(), std::str::from_utf8(&vault_info.vault_name.iter().filter(|x| **x != 0).map(|&x| x).collect::<Vec<u8>>()).unwrap().as_bytes()],
        bump = bumps[2],
    )]
    pub vault_token_a: Box<Account<'info, TokenAccount>>,

    /// Accounts that stores token B
    #[account(
        seeds = [VAULT_TOKENB_SEED.as_bytes(), std::str::from_utf8(&vault_info.vault_name.iter().filter(|x| **x != 0).map(|&x| x).collect::<Vec<u8>>()).unwrap().as_bytes()],
        bump = bumps[3],
    )]
    pub vault_token_b: Box<Account<'info, TokenAccount>>,

    /// Accounts that stores tutoken A
    #[account(
        mut,
        seeds = [VAULT_TUTOKENA_SEED.as_bytes(), std::str::from_utf8(&vault_info.vault_name.iter().filter(|x| **x != 0).map(|&x| x).collect::<Vec<u8>>()).unwrap().as_bytes()],
        bump = bumps[4],
    )]
    pub vault_tutoken_a: Box<Account<'info, TokenAccount>>,

    /// Accounts that stores tutoken B
    #[account(
        mut,
        seeds = [VAULT_TUTOKENB_SEED.as_bytes(), std::str::from_utf8(&vault_info.vault_name.iter().filter(|x| **x != 0).map(|&x| x).collect::<Vec<u8>>()).unwrap().as_bytes()],
        bump = bumps[5],
    )]
    pub vault_tutoken_b: Box<Account<'info, TokenAccount>>,

    /// User ata that stores token a
    #[account(
        mut,
        // constraint = user_a_token_ata.owner == user.key(),
        // constraint = user_a_token_ata.mint == vault_info.token_a_mint
        associated_token::mint = token_a_mint,
        associated_token::authority = user,
    )]
    pub user_a_token_ata: Box<Account<'info, TokenAccount>>,

    /// User ata that stores token a
    #[account(
        // constraint = user_b_token_ata.owner == user.key(),
        // constraint = user_b_token_ata.mint == vault_info.token_b_mint
        associated_token::mint = token_b_mint,
        associated_token::authority = user,
    )]
    pub user_b_token_ata: Box<Account<'info, TokenAccount>>,

    /// User ata that stores token a
    #[account(
        init,
        payer = user,
        constraint = user_redeemable_ata.owner == user.key(),
        // constraint = user_b_token_ata.mint == vault_info.token_b_mint
        associated_token::mint = redeemable_mint,
        associated_token::authority = user,
    )]
    pub user_redeemable_ata: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub destination_collateral: Box<Account<'info, TokenAccount>>,

    /// CHECK: tulip checks underneath
    #[account(mut)]
    pub reserve_account: UncheckedAccount<'info>,

    /// CHECK: no type available for oracle account
    //#[account(mut)]
    pub price_oracle: AccountInfo<'info>,

    #[account(mut)]
    pub reserve_liquidity_supply: Box<Account<'info, TokenAccount>>,

    /// CHECK: tulip checks underneath
    #[account(mut)]
    pub lending_market: UncheckedAccount<'info>,

    /// CHECK: tulip checks underneath
    //#[account(mut)]
    pub lending_market_authority: UncheckedAccount<'info>,

    /// Mint of user's redeemable token (e.g. spUSD)
    #[account(mut, address=vault_info.redeemable_mint)]
    pub redeemable_mint: Box<Account<'info, Mint>>,

    /// System Programs
    pub system_program: Program<'info, System>,
    
    /// Token Program
    pub token_program: Program<'info, Token>,

    /// CHECK: Tulip checks underneath the hood
    /// Tulip Lending Program
    pub tulip_lending_program: UncheckedAccount<'info>,

    /// Clock Program
    pub sys_var_clock: Sysvar<'info, Clock>,
    /// Rent Program
    pub rent: Sysvar<'info, Rent>,

    /// Associated Token Program
    pub associated_token_program: Program<'info, AssociatedToken>,
}




/// Errors that may be returned by the TokenLending program.
#[error_code]
pub enum LendingError {
    // 0
    /// Invalid instruction data passed in.
    #[msg("Failed to unpack instruction data")]
    InstructionUnpackError,
    /// The account cannot be initialized because it is already in use.
    #[msg("Account is already initialized")]
    AlreadyInitialized,
    /// Lamport balance below rent-exempt threshold.
    #[msg("Lamport balance below rent-exempt threshold")]
    NotRentExempt,
    /// The program address provided doesn't match the value generated by the program.
    #[msg("Market authority is invalid")]
    InvalidMarketAuthority,
    /// Expected a different market owner
    #[msg("Market owner is invalid")]
    InvalidMarketOwner,

    // 5
    /// The owner of the input isn't set to the program address generated by the program.
    #[msg("Input account owner is not the program address")]
    InvalidAccountOwner,
    /// The owner of the account input isn't set to the correct token program id.
    #[msg("Input token account is not owned by the correct token program id")]
    InvalidTokenOwner,
    /// Expected an SPL Token account
    #[msg("Input token account is not valid")]
    InvalidTokenAccount,
    /// Expected an SPL Token mint
    #[msg("Input token mint account is not valid")]
    InvalidTokenMint,
    /// Expected a different SPL Token program
    #[msg("Input token program account is not valid")]
    InvalidTokenProgram,

    // 10
    /// Invalid amount, must be greater than zero
    #[msg("Input amount is invalid")]
    InvalidAmount,
    /// Invalid config value
    #[msg("Input config value is invalid")]
    InvalidConfig,
    /// Invalid config value
    #[msg("Input account must be a signer")]
    InvalidSigner,
    /// Invalid account input
    #[msg("Invalid account input")]
    InvalidAccountInput,
    /// Math operation overflow
    #[msg("Math operation overflow")]
    MathOverflow,

    // 15
    /// Token initialize mint failed
    #[msg("Token initialize mint failed")]
    TokenInitializeMintFailed,
    /// Token initialize account failed
    #[msg("Token initialize account failed")]
    TokenInitializeAccountFailed,
    /// Token transfer failed
    #[msg("Token transfer failed")]
    TokenTransferFailed,
    /// Token mint to failed
    #[msg("Token mint to failed")]
    TokenMintToFailed,
    /// Token burn failed
    #[msg("Token burn failed")]
    TokenBurnFailed,

    // 20
    /// Insufficient liquidity available
    #[msg("Insufficient liquidity available")]
    InsufficientLiquidity,
    /// This reserve's collateral cannot be used for borrows
    #[msg("Input reserve has collateral disabled")]
    ReserveCollateralDisabled,
    /// Reserve state stale
    #[msg("Reserve state needs to be refreshed")]
    ReserveStale,
    /// Withdraw amount too small
    #[msg("Withdraw amount too small")]
    WithdrawTooSmall,
    /// Withdraw amount too large
    #[msg("Withdraw amount too large")]
    WithdrawTooLarge,

    // 25
    /// Borrow amount too small
    #[msg("Borrow amount too small to receive liquidity after fees")]
    BorrowTooSmall,
    /// Borrow amount too large
    #[msg("Borrow amount too large for deposited collateral")]
    BorrowTooLarge,
    /// Repay amount too small
    #[msg("Repay amount too small to transfer liquidity")]
    RepayTooSmall,
    /// Liquidation amount too small
    #[msg("Liquidation amount too small to receive collateral")]
    LiquidationTooSmall,
    /// Cannot liquidate healthy obligations
    #[msg("Cannot liquidate healthy obligations")]
    ObligationHealthy,

    // 30
    /// Obligation state stale
    #[msg("Obligation state needs to be refreshed")]
    ObligationStale,
    /// Obligation reserve limit exceeded
    #[msg("Obligation reserve limit exceeded")]
    ObligationReserveLimit,
    /// Expected a different obligation owner
    #[msg("Obligation owner is invalid")]
    InvalidObligationOwner,
    /// Obligation deposits are empty
    #[msg("Obligation deposits are empty")]
    ObligationDepositsEmpty,
    /// Obligation borrows are empty
    #[msg("Obligation borrows are empty")]
    ObligationBorrowsEmpty,

    // 35
    /// Obligation deposits have zero value
    #[msg("Obligation deposits have zero value")]
    ObligationDepositsZero,
    /// Obligation borrows have zero value
    #[msg("Obligation borrows have zero value")]
    ObligationBorrowsZero,
    /// Invalid obligation collateral
    #[msg("Invalid obligation collateral")]
    InvalidObligationCollateral,
    /// Invalid obligation liquidity
    #[msg("Invalid obligation liquidity")]
    InvalidObligationLiquidity,
    /// Obligation collateral is empty
    #[msg("Obligation collateral is empty")]
    ObligationCollateralEmpty,

    // 40
    /// Obligation liquidity is empty
    #[msg("Obligation liquidity is empty")]
    ObligationLiquidityEmpty,
    /// Negative interest rate
    #[msg("Interest rate is negative")]
    NegativeInterestRate,
    /// Oracle config is invalid
    #[msg("Input oracle config is invalid")]
    InvalidOracleConfig,
    /// Expected a different flash loan receiver program
    #[msg("Input flash loan receiver program account is not valid")]
    InvalidFlashLoanReceiverProgram,
    /// Not enough liquidity after flash loan
    #[msg("Not enough liquidity after flash loan")]
    NotEnoughLiquidityAfterFlashLoan,
    // 45
}

impl From<LendingError> for ProgramError {
    fn from(e: LendingError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for LendingError {
    fn type_of() -> &'static str {
        "Lending Error"
    }
}