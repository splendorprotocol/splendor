//! Accounts structs for SplendorProtocol.

use anchor_lang::prelude::*;

/// Accounts in context of initialize_vault instruction
#[account]
//#[derive(Default)]
pub struct VaultInfo {

    /// Mint address of token A
    pub token_a: Pubkey,

    /// Mint address of token B
    pub token_b: Pubkey,

    // Bumps
    pub vault_bumps: VaultBumps,
}

#[account]
//#[derive(Default)]
pub struct VaultBumps {
    pub vault_token_a : u8,
    pub vault_token_b : u8,
    pub vault_tutoken_a : u8,
    pub vault_tutoken_b : u8,
    pub vault_authority : u8,
}