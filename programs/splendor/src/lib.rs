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
        
        // Initialize byte array to zeros
        let mut name_byte_array : [u8; 20] = [0; 20];

        // Load bytes into array
        for (i, byte) in _vault_name.as_bytes().iter().enumerate() {

            // This method (correctly) breaks when length > 20
            name_byte_array[i] = *byte;
        }
        
        // Initialize vault info
        let vault_info = &mut _ctx.accounts.vault_info;
        vault_info.token_a = _ctx.accounts.token_a_mint.key();
        vault_info.token_b = _ctx.accounts.token_b_mint.key();
        vault_info.vault_name = name_byte_array;

        Ok(())
    }
}


