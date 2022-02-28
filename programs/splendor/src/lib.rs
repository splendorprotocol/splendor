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
        // let name_bytes = _vault_name.as_bytes();
        // let mut name_data = [b' '; 20];
        // name_data[..name_bytes.len()].copy_from_slice(name_bytes);

        let vault_info = &mut _ctx.accounts.vault_info;
        vault_info.token_a = _ctx.accounts.token_a_mint.key();
        vault_info.token_b = _ctx.accounts.token_b_mint.key();
        // vault_info.vault_name = name_data;

        Ok(())
    }
}


