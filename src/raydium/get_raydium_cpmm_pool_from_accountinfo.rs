use crate::protocol_idls::raydium::CpmmPoolInfoIdl;
use crate::types::pool::Pool;
use crate::types::pools::Pools;
use crate::types::raydium_cpmm_pool::RaydiumCpmmPool;
use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;

impl RaydiumCpmmPool {
  pub fn from_account_info(pubkey: Pubkey, account_buffer: &[u8]) -> Self {
    let decoded_layout: CpmmPoolInfoIdl = CpmmPoolInfoIdl::try_from_slice(account_buffer).unwrap();
    Self {
      info: Pool {
        pool_address: pubkey,
        token_a_address: decoded_layout.mint_a,
        token_b_address: decoded_layout.mint_b,
        pool_type: Pools::RaydiumAmmV4,
        token_a_vault_address: decoded_layout.vault_a,
        token_b_vault_address: decoded_layout.vault_b,
      },
      token_a_decimals: decoded_layout.mint_decimal_a,
      token_b_decimals: decoded_layout.mint_decimal_b,
      pool_config_account: decoded_layout.config_id,
      observation_state_account: decoded_layout.observation_id,
      // Account layout doesn't contain these balances, so we set them to 0
      token_a_vault_amount: 0,
      token_b_vault_amount: 0,
    }
  }
}
