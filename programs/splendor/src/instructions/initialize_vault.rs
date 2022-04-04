use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;
use crate::constants::*;
use crate::types::ProgramResult;



pub fn handler(
    ctx: Context<InitializeVault>,
    vault_name: String,
) -> ProgramResult {

    // Initialize byte array to zeros
    let mut name_byte_array : [u8; 20] = [0; 20];

    // Load bytes into array
    for (i, byte) in vault_name.as_bytes().iter().enumerate() {

        // This method (correctly) breaks when length > 20
        name_byte_array[i] = *byte;
    }

    // Grab vault_info account
    let vault_info = &mut ctx.accounts.vault_info;

    // Initialize vault info data
    vault_info.vault_name = name_byte_array;
    vault_info.token_a_mint = ctx.accounts.token_a_mint.key();
    vault_info.token_b_mint = ctx.accounts.token_b_mint.key();
    vault_info.redeemable_mint = ctx.accounts.redeemable_mint.key();

    Ok(())
}

/// Create context of initialize_vault
/// Gather accounts required for initialize_vault instruction
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
        space = 8 + 8*20 + 32*3,
        payer = vault_admin,
        seeds = [{msg!("initializing vault_info"); VAULT_INFO_SEED}.as_bytes(), vault_name.as_bytes()],
        bump,
    )]
    pub vault_info: Box<Account<'info, VaultInfo>>,

    /// Vault authority: PDA which manages everything within the vault
    #[account(
        init,
        space = 8 + 1,
        payer = vault_admin,
        seeds = [{msg!("initializing vault_authority"); VAULT_AUTHORITY_SEED}.as_bytes(), vault_name.as_bytes()],
        bump,
    )]
    pub vault_authority: Box<Account<'info, VaultAuthority>>,

    /// Token A mint
    #[account()]//address = token_a_mint_address)]
    pub token_a_mint: Box<Account<'info, Mint>>,

    /// Token B mint
    #[account()]//address = token_b_mint_address)]
    pub token_b_mint: Box<Account<'info, Mint>>,

    /// tuToken A mint
    #[account()]//address = tutoken_a_mint_address)]
    pub tutoken_a_mint: Box<Account<'info, Mint>>,

    /// tuToken B mint
    #[account()]//address = tutoken_b_mint_address)]
    pub tutoken_b_mint: Box<Account<'info, Mint>>,

    /// Accounts that stores token A
    #[account(
        init, 
        payer = vault_admin,
        token::mint = token_a_mint,
        token::authority = vault_authority,
        seeds = [{msg!("initializing vault_token_a"); VAULT_TOKENA_SEED}.as_bytes(), vault_name.as_bytes()],
        bump,
    )]
    pub vault_token_a: Box<Account<'info, TokenAccount>>,

    /// Accounts that stores token B
    #[account(
        init, 
        payer = vault_admin,
        token::mint = token_b_mint,
        token::authority = vault_authority,
        seeds = [{msg!("initializing vault_token_b"); VAULT_TOKENB_SEED}.as_bytes(), vault_name.as_bytes()],
        bump,
    )]
    pub vault_token_b: Box<Account<'info, TokenAccount>>,

    /// Accounts that stores tutoken A
    #[account(
        init, 
        payer = vault_admin,
        token::mint = tutoken_a_mint,
        token::authority = vault_authority,
        seeds = [{msg!("initializing vault_tutoken_a"); VAULT_TUTOKENA_SEED}.as_bytes(), vault_name.as_bytes()],
        bump,
    )]
    pub vault_tutoken_a: Box<Account<'info, TokenAccount>>,

    /// Accounts that stores tutoken B
    #[account(
        init,
        payer = vault_admin,
        token::mint = tutoken_b_mint,
        token::authority = vault_authority,
        seeds = [{msg!("initializing vault_tutoken_b"); VAULT_TUTOKENB_SEED}.as_bytes(), vault_name.as_bytes()],
        bump,
    )]
    pub vault_tutoken_b: Box<Account<'info, TokenAccount>>,

    /// Mint of user's redeemable token (e.g. spUSD)
    #[account(
        init,
        payer = vault_admin,
        mint::decimals = 9,
        mint::authority = vault_authority,
        mint::freeze_authority = vault_authority,
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


// Accounts in context of initialize_vault instruction
#[account]
#[derive(Default)]
pub struct VaultInfo {
    /// Vault name 
    pub vault_name: [u8; 20],

    /// Mint address of token A
    pub token_a_mint: Pubkey,

    /// Mint address of token B
    pub token_b_mint: Pubkey,

    // Redeemable mint
    pub redeemable_mint: Pubkey,
}

#[account]
#[derive(Default)]
pub struct VaultAuthority {
    /// Authorized
    pub authorized: bool
}