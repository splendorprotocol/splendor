use anchor_lang::prelude::*;


declare_id!("xmtn2vByiRMraod2aVHYXB9mxRJQ3Z3Y7SnvSdAy8qn");

pub mod constants;
pub mod address;
pub mod instructions;
pub mod types;

use types::ProgramResult;

// import instruction handlers
use instructions::initialize_vault::*;
use instructions::deposit::*;
use instructions::withdraw::*;
use instructions::swap::*;


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
        _bumps: [u8; 6],
        token_a_lamports: u32,
        token_b_lamports: u32,
    ) -> ProgramResult {
        instructions::deposit::handler(ctx, token_a_lamports, token_b_lamports)
    }

    pub fn withdraw(
        ctx: Context<Withdraw>,
        _bumps: [u8; 6],
        token_a_lamports: u32,
        token_b_lamports: u32,
    ) -> ProgramResult {
        instructions::withdraw::handler(ctx, token_a_lamports, token_b_lamports)
    }

    pub fn swap(
        ctx: Context<Swap>,
        _bumps: [u8; 6],
        lamports: u32,
        min_out_lamports: u32,
        is_token_a: bool,
    ) -> ProgramResult {
        instructions::swap::handler(ctx, lamports, min_out_lamports, is_token_a)
    }
}