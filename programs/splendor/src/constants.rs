
use anchor_lang::constant;

#[constant]
pub const VAULT_INFO_SEED: &str = "vault-info";
#[constant]
pub const VAULT_AUTHORITY_SEED: &str = "vault-authority";
#[constant]
pub const VAULT_TOKENA_SEED: &str = "vault-token_a-seed";
#[constant]
pub const VAULT_TOKENB_SEED: &str = "vault-token_b-seed";
#[constant]
pub const VAULT_TUTOKENA_SEED: &str = "vault-tutoken_a-seed";
#[constant]
pub const VAULT_TUTOKENB_SEED: &str = "vault-tutoken_b-seed";
#[constant]
pub const VAULT_REDEEMABLE_MINT_SEED: &str = "vault-redeemable-mint";
#[constant]
pub const TOKEN_A_DECIMALS: u8 = 6; // USDC and USDT are both 6 decimals
#[constant]
pub const TOKEN_B_DECIMALS: u8 = 6; // USDC and USDT are both 6 decimals
//TODO: UPDATE DECIMALS
#[constant]
pub const TUTOKEN_A_DECIMALS: u8 = 6; // USDC and USDT are both 6 decimals
#[constant]
pub const TUTOKEN_B_DECIMALS: u8 = 6; // USDC and USDT are both 6 decimals
