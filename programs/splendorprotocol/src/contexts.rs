use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::constants::*;
use crate::ctx_accounts::*;



/// Accounts in context of initialize_vault instruction
#[derive(Accounts)]
#[instruction(vault_name: String, bumps: [u8; 6])]
pub struct InitializeVault<'info> {
    
    /// Vault admin
    #[account(mut)]
    pub vault_admin: Signer<'info>,

    /// Vault Info
    #[account(
        seeds = [VAULT_INFO_SEED.as_bytes(), vault_name.as_bytes()],
        bump,
    )]
    pub vault_info: Account<'info, VaultInfo>,

    /// Vault authority: PDA which manages everything within the vault
    /// CHECK: this is fine for now but maybe revisit later
    #[account(
        seeds = [VAULT_AUTHORITY_SEED.as_bytes(), vault_name.as_bytes()],
        bump,
    )]
    pub vault_authority: AccountInfo<'info>,

    /// Token A mint
    #[account()]
    pub token_a_mint: Account<'info, Mint>,

    /// Token B mint
    #[account()]
    pub token_b_mint: Account<'info, Mint>,

    /// tuToken A mint
    #[account()]
    pub tutoken_a_mint: Account<'info, Mint>,

    /// tuToken B mint
    #[account()]
    pub tutoken_b_mint: Account<'info, Mint>,

    /// Accounts that stores token A
    #[account(
        init, 
        token::mint = token_a_mint,
        token::authority = vault_authority,
        seeds =[VAULT_TOKENA_SEED.as_bytes(), vault_name.as_bytes()],
        bump,
        payer=vault_admin
    )]
    pub vault_token_a: Account<'info, TokenAccount>,

    /// Accounts that stores token B
    #[account(
        init, 
        token::mint = token_b_mint,
        token::authority = vault_authority,
        seeds =[VAULT_TOKENB_SEED.as_bytes(), vault_name.as_bytes()],
        bump,
        payer=vault_admin
    )]
    pub vault_token_b: Account<'info, TokenAccount>,

    /// Accounts that stores tutoken A
    #[account(
        init, 
        token::mint = tutoken_a_mint,
        token::authority = vault_authority,
        seeds = [VAULT_TOKENTUA_SEED.as_bytes(), vault_name.as_bytes()],
        bump,
        payer=vault_admin
    )]
    pub vault_tutoken_a: Account<'info, TokenAccount>,

    /// Accounts that stores tutoken B
    #[account(
        init, 
        token::mint = tutoken_b_mint,
        token::authority = vault_authority,
        seeds = [VAULT_TOKENTUB_SEED.as_bytes(), vault_name.as_bytes()],
        bump,
        payer=vault_admin
    )]
    pub vault_tutoken_b: Account<'info, TokenAccount>,

    /// Mint of user's redeemable token (e.g. spUSD)
    pub redeemable_mint: Account<'info, Mint>,

    /// System Programs
    pub system_program: Program<'info, System>,

    /// Rent
    pub rent: Sysvar<'info, Rent>,
    
    /// Token Program
    pub token_program: Program<'info, Token>,
}
