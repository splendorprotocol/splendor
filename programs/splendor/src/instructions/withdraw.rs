use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;
use crate::constants::*;
use crate::types::ProgramResult;
use crate::instructions::initialize_vault::{VaultAuthority, VaultInfo};


pub fn handler(
    ctx: Context<Withdraw>,
    token_a_lamports: u32,
    token_b_lamports: u32,
) -> ProgramResult {

    // Burn redeemable from user
    // TODO

    // withdraws
    let dummy_cpi_ctx = ctx;
    let dummy_tulip_cpi = |ctx : Context<Withdraw>, a, b| {
        // uses cpi_ctx containing (at least) A/B, tuA/tuB mints, as well
        // also
        msg!("Running dummy tulip cpi with {:?}! Pseudo-withdrawing {} token A lamports and {} token B lamports", 
            ctx.accounts.vault_info.vault_name.iter().filter(|x| **x != 0).map(|&x| x).collect::<Vec<u8>>(), 
            a, 
            b,
        );
    };
    dummy_tulip_cpi(dummy_cpi_ctx, token_a_lamports, token_b_lamports);

    // Transfer token a/b to user
     // TODO

    Ok(())
}

/// Create context of withdraw
/// Gather accounts required for withdraw instruction
#[derive(Accounts)]
#[instruction(
    bumps: [u8; 6],
)]
pub struct Withdraw<'info> {
    
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
        seeds = [VAULT_TUTOKENA_SEED.as_bytes(), std::str::from_utf8(&vault_info.vault_name.iter().filter(|x| **x != 0).map(|&x| x).collect::<Vec<u8>>()).unwrap().as_bytes()],
        bump = bumps[4],
    )]
    pub vault_tutoken_a: Box<Account<'info, TokenAccount>>,

    /// Accounts that stores tutoken B
    #[account(
        seeds = [VAULT_TUTOKENB_SEED.as_bytes(), std::str::from_utf8(&vault_info.vault_name.iter().filter(|x| **x != 0).map(|&x| x).collect::<Vec<u8>>()).unwrap().as_bytes()],
        bump = bumps[5],
    )]
    pub vault_tutoken_b: Box<Account<'info, TokenAccount>>,

    /// Mint of user's redeemable token (e.g. spUSD)
    #[account(address=vault_info.redeemable_mint)]
    pub redeemable_mint: Box<Account<'info, Mint>>,

    /// System Programs
    pub system_program: Program<'info, System>,
    
    /// Token Program
    pub token_program: Program<'info, Token>,

    /// Associated Token Program
    pub associated_token_program: Program<'info, AssociatedToken>,
}