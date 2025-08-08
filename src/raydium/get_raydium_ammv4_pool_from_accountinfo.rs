use crate::protocol_idls::raydium::AmmV4PoolInfoIdl;
use crate::types::pool::Pool;
use crate::types::pools::Pools;
use crate::types::raydium_ammv4_pool::RaydiumAmmV4Pool;
use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;

impl RaydiumAmmV4Pool {
  pub fn from_account_info(pubkey: Pubkey, account_buffer: &[u8]) -> Self {
    let decoded_layout: AmmV4PoolInfoIdl =
      AmmV4PoolInfoIdl::try_from_slice(account_buffer).unwrap();
    Self {
      info: Pool {
        pool_address: pubkey,
        token_a_address: decoded_layout.base_mint,
        token_b_address: decoded_layout.quote_mint,
        pool_type: Pools::RaydiumAmmV4,
        token_a_vault_address: decoded_layout.base_vault,
        token_b_vault_address: decoded_layout.quote_vault,
      },
      swap_fee_numerator: decoded_layout.swap_fee_numerator,
      swap_fee_denominator: decoded_layout.swap_fee_denominator,
      // Account layout doesn't contain these balances, so we set them to 0
      token_a_vault_amount: 0,
      token_b_vault_amount: 0,
    }
  }
}
