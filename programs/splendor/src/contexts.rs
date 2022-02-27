use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::{
    associated_token::AssociatedToken,
    mint,
};
use crate::constants::*;
use crate::address;
use crate::ctx_accounts::*;



/// Accounts in context of initialize_vault instruction
#[derive(Accounts)]
#[instruction(
    vault_name: String,
)]
pub struct InitializeVault<'info> {
    
    /// Vault admin
    #[account(mut)]
    pub vault_admin: Signer<'info>,

    /// Vault Info
    #[account(
        init,
        payer = vault_admin,
        seeds = [VAULT_INFO_SEED.as_bytes(), vault_name.as_bytes()],
        bump,
    )]
    pub vault_info: Account<'info, VaultInfo>,

    /// Vault authority: PDA which manages everything within the vault
    /// CHECK: this is fine for now but maybe revisit later
    #[account(
        init, 
        payer = vault_admin,
        seeds = [VAULT_AUTHORITY_SEED.as_bytes(), vault_name.as_bytes()],
        bump,
    )]
    pub vault_authority: Account<'info, VaultAuthority>,

    // #[account(
    //     init,
    //     payer = vault_admin,
    // )]
    // pub vault_mints: Account<'info, VaultMints>,

    /// Token A mint
    #[account()]//address = token_a_mint_address)]
    pub token_a_mint: Account<'info, Mint>,

    /// Token B mint
    #[account()]//address = token_b_mint_address)]
    pub token_b_mint: Account<'info, Mint>,

    /// tuToken A mint
    #[account()]//address = tutoken_a_mint_address)]
    pub tutoken_a_mint: Account<'info, Mint>,

    /// tuToken B mint
    #[account()]//address = tutoken_b_mint_address)]
    pub tutoken_b_mint: Account<'info, Mint>,

    /// Accounts that stores token A
    #[account(
        init, 
        associated_token::mint = token_a_mint,
        associated_token::authority = vault_authority,
        seeds = [VAULT_TOKENA_SEED.as_bytes(), vault_name.as_bytes()],
        bump,
        payer = vault_admin
    )]
    pub vault_token_a: Box<Account<'info, TokenAccount>>,

    /// Accounts that stores token B
    #[account(
        init, 
        associated_token::mint = token_b_mint,
        associated_token::authority = vault_authority,
        seeds = [VAULT_TOKENB_SEED.as_bytes(), vault_name.as_bytes()],
        bump,
        payer = vault_admin,
    )]
    pub vault_token_b: Box<Account<'info, TokenAccount>>,

    /// Accounts that stores tutoken A
    #[account(
        init, 
        associated_token::mint = tutoken_a_mint,
        associated_token::authority = vault_authority,
        seeds = [VAULT_TOKENTUA_SEED.as_bytes(), vault_name.as_bytes()],
        bump,
        payer = vault_admin
    )]
    pub vault_tutoken_a: Box<Account<'info, TokenAccount>>,

    /// Accounts that stores tutoken B
    #[account(
        init, 
        associated_token::mint = tutoken_b_mint,
        associated_token::authority = vault_authority,
        seeds = [VAULT_TOKENTUB_SEED.as_bytes(), vault_name.as_bytes()],
        bump,
        payer=vault_admin
    )]
    pub vault_tutoken_b: Box<Account<'info, TokenAccount>>,

    /// Mint of user's redeemable token (e.g. spUSD)
    #[account(
        init,
        payer = vault_admin,
        mint::decimals = 9,
        mint::authority = vault_authority,
        mint::freeze_authority = vault_authority,
        seeds = [VAULT_REDEEMABLE_MINT_SEED.as_bytes(), vault_name.as_bytes()],
        bump,
    )]
    pub redeemable_mint: Box<Account<'info, Mint>>,

    /// System Programs
    pub system_program: Program<'info, System>,

    /// Rent
    pub rent: Sysvar<'info, Rent>,
    
    /// Token Program
    pub token_program: Program<'info, Token>,

    /// Associated Token Program
    pub associated_token_program: Program<'info, AssociatedToken>,
}
