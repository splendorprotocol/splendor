//! Accounts structs for SplendorProtocol.

use anchor_lang::prelude::*;

/// Accounts in context of initialize_vault instruction
#[account]
#[derive(Default)]
pub struct VaultInfo {

    /// Mint address of token A
    pub token_a: Pubkey,

    /// Mint address of token B
    pub token_b: Pubkey,

    // Bumps
    //pub vault_bumps: VaultBumps,
}

#[account]
#[derive(Default)]
pub struct VaultAuthority {
    pub authority: Pubkey,
}

// #[account]
// //#[derive(Default)]
// pub struct VaultBumps {
//     pub vault_token_a : u8,
//     pub vault_token_b : u8,
//     pub vault_tutoken_a : u8,
//     pub vault_tutoken_b : u8,
//     pub vault_authority : u8,
// }

// #[account]
// pub struct VaultMints<'info> {

//     /// Token A mint
//     pub token_a_mint: AccountInfo<'info>,

//     /// Token B mint
//     pub token_b_mint: AccountInfo<'info>,

//     /// tuToken A mint
//     pub tutoken_a_mint: AccountInfo<'info>,

//     /// tuToken B mint
//     pub tutoken_b_mint: AccountInfo<'info>,

// }