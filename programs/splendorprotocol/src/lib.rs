use anchor_lang::prelude::*;


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod constants;
pub mod ctx_accounts;
pub mod contexts;

use contexts::*;


type ProgramResult = Result<()>;

#[program]
pub mod splendor {
    use super::*;

    pub fn initialize_vault(
        _ctx: Context<InitializeVault>,
        _vault_name: String,
        _bumps: [u8; 6],
    ) -> ProgramResult {

        Ok(())
    }
}


