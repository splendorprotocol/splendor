use anchor_lang::prelude::*;


declare_id!("xmtn2vByiRMraod2aVHYXB9mxRJQ3Z3Y7SnvSdAy8qn");

pub mod constants;
pub mod address;
pub mod instructions;
pub mod types;

use types::*;
use instructions::initialize_vault::*;
use instructions::deposit::*;


#[program]
pub mod splendor {
    use super::*;

    pub fn initialize_vault(
        ctx: Context<InitializeVault>,
        vault_name: String,
    ) -> ProgramResult {
        instructions::initialize_vault::handler(ctx, vault_name)
    }

    pub fn deposit(
        ctx: Context<Deposit>,
        bumps: [u8; 6],
        token_a_lamports: u32,
        token_b_lamports: u32,
    ) -> ProgramResult {
        msg!("successfully entered ctx");
        instructions::deposit::handler(ctx, token_a_lamports, token_b_lamports)
    }
}


