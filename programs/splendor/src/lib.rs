use anchor_lang::prelude::*;


declare_id!("xmtn2vByiRMraod2aVHYXB9mxRJQ3Z3Y7SnvSdAy8qn");

pub mod constants;
pub mod ctx_accounts;
pub mod contexts;
pub mod address;

use contexts::*;


type ProgramResult = Result<()>;

#[program]
pub mod splendor {
    use super::*;

    pub fn initialize_vault(
        _ctx: Context<InitializeVault>,
        _vault_name: String,
        _info_bump: u8,
        _admin_bump: u8,
        _token_a_bump: u8,
        _token_b_bump: u8,
        _tutoken_a_bump: u8,
        _tutoken_b_bump: u8,
        // _token_a_mint_address: Pubkey,
        // _token_b_mint_address: Pubkey,
        // _tutoken_a_mint_address: Pubkey,
        // _tutoken_b_mint_address: Pubkey,
    ) -> ProgramResult {

        Ok(())
    }
}


