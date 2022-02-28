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
    ) -> ProgramResult {

        Ok(())
    }
}


