use anchor_lang::prelude::*;
use anchor_lang::{ToAccountInfos};
use anchor_lang::context::{CpiContext};
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;
use solana_program::pubkey::PUBKEY_BYTES;
use crate::constants::*;
use crate::types::ProgramResult;
use crate::instructions::initialize_vault::{VaultAuthority, VaultInfo};

pub fn handler(
    ctx: Context<Deposit>,
    token_a_lamports: u32,
    token_b_lamports: u32,
) -> ProgramResult {
    
    // Find lending market authority
    let (lending_market_authority_pubkey, _bump_seed) = Pubkey::find_program_address(
        &[&ctx.accounts.lending_market.key.to_bytes()[..PUBKEY_BYTES]],
        &ctx.accounts.lending_program.key,
    );

    msg!("hererererererere in deposit program\nlending program: {}\nuser token a: {}\nvault tutoken a: {}\nreserve acct: {}\nreserve liq sup: {}\ntoken a mint: {}\n lending market: {}\nuser: {}", 
        ctx.accounts.lending_program.key,
        ctx.accounts.user_a_token_ata.key(),
        ctx.accounts.destination_collateral.key(),
        ctx.accounts.reserve_account.key,
        ctx.accounts.reserve_liquidity_supply.key(),
        ctx.accounts.token_a_mint.key(),
        ctx.accounts.lending_market.key,
        ctx.accounts.user.key,
        //ctx.accounts.lending_market_authority.key,
    );

    // Deposit token a
    let ix = spl_token_lending::instruction::deposit_reserve_liquidity(
        *ctx.accounts.lending_program.key,
        token_a_lamports as u64,
        ctx.accounts.user_a_token_ata.key(),
        //ctx.accounts.destination_collateral.key(), // we don't want to give user token
        ctx.accounts.vault_tutoken_a.key(), // we want vault to have token
        *ctx.accounts.reserve_account.key,
        ctx.accounts.reserve_liquidity_supply.key(),
        ctx.accounts.token_a_mint.key(),
        *ctx.accounts.lending_market.key,
        *ctx.accounts.user.key,
        //*ctx.accounts.lending_market_authority.key,
    );

    msg!("{:?}",ctx.accounts.user.to_account_info());
    msg!("{:?}",ctx.accounts.lending_market.to_account_info());
    msg!("{:?}",ctx.accounts.lending_market_authority.to_account_info());
    msg!("about to invoke");
    solana_program::program::invoke(
        &ix,
        &ctx.accounts.to_account_infos(),
        // &[
        //     ctx.accounts.lending_market_authority.to_account_info(),
        //     ctx.accounts.lending_program.to_account_info(),
        //     ctx.accounts.user_a_token_ata.to_account_info(),
        //     //ctx.accounts.destination_collateral.to_account_info(), // we don't want to give user token
        //     ctx.accounts.vault_tutoken_a.to_account_info(), // we want vault to have token
        //     ctx.accounts.reserve_account.to_account_info(),
        //     ctx.accounts.reserve_liquidity_supply.to_account_info(),
        //     ctx.accounts.token_a_mint.to_account_info(),
        //     ctx.accounts.tutoken_a_mint.to_account_info(),
        // ],
        //&[]
        //&[&[VAULT_AUTHORITY_SEED.as_bytes(), std::str::from_utf8(&ctx.accounts.vault_info.vault_name.iter().filter(|x| **x != 0).map(|&x| x).collect::<Vec<u8>>()).unwrap().as_bytes()]]
    )?;

    // Transfer token A/B from user to vault
    // TODO

    // Deposits
    // let dummy_cpi_ctx = ctx;
    // let dummy_tulip_cpi = |ctx : Context<Deposit>, a, b| {
    //     // uses cpi_ctx containing (at least) A/B, tuA/tuB mints, as well
    //     // also
    //     msg!("Running dummy tulip cpi with {:?}! Pseudo-depositing {} token A lamports and {} token B lamports", 
    //         ctx.accounts.vault_info.vault_name.iter().filter(|x| **x != 0).map(|&x| x).collect::<Vec<u8>>(), 
    //         a, 
    //         b,
    //     );
    // };
    // dummy_tulip_cpi(dummy_cpi_ctx, token_a_lamports, token_b_lamports);

    // Mint and send spAsset to user
    // TODO

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
    #[account()]//address = tutoken_a_mint_address)]
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

    #[account(mut)]
    pub destination_collateral: Box<Account<'info, TokenAccount>>,

    /// CHECK: tulip checks underneath
    #[account(mut)]
    pub reserve_account: UncheckedAccount<'info>,

    #[account(mut)]
    pub reserve_liquidity_supply: Box<Account<'info, TokenAccount>>,

    /// CHECK: tulip checks underneath
    #[account(mut)]
    pub lending_market: UncheckedAccount<'info>,

    /// CHECK: tulip checks underneath
    #[account(mut)]
    pub lending_market_authority: UncheckedAccount<'info>,

    /// Mint of user's redeemable token (e.g. spUSD)
    #[account(address=vault_info.redeemable_mint)]
    pub redeemable_mint: Box<Account<'info, Mint>>,

    /// System Programs
    pub system_program: Program<'info, System>,
    
    /// Token Program
    pub token_program: Program<'info, Token>,

    /// CHECK: Tulip checks underneath the hood
    /// Lending Program
    pub lending_program: UncheckedAccount<'info>,

    /// Clock Program
    pub sys_var_clock: Sysvar<'info, Clock>,
    /// Rent Program
    pub rent: Sysvar<'info, Rent>,

    /// Associated Token Program
    pub associated_token_program: Program<'info, AssociatedToken>,
}
