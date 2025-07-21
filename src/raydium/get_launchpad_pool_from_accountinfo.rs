use crate::protocol_idls::raydium::LaunchpadPoolIdl;
use crate::types::pool::Pool;
use crate::types::pools::Pools;
use crate::types::raydium_launchpad::RaydiumLaunchpad;
use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;

impl RaydiumLaunchpad {
  pub fn from_account_info(pubkey: Pubkey, account_buffer: &[u8]) -> Self {
    let decoded_layout = LaunchpadPoolIdl::try_from_slice(account_buffer).unwrap();
    Self {
      info: Pool {
        pool_address: pubkey,
        token_a_address: decoded_layout.base_mint,
        token_b_address: decoded_layout.quote_mint,
        pool_type: Pools::RaydiumLaunchpad,
        token_a_vault_address: decoded_layout.base_vault,
        token_b_vault_address: decoded_layout.quote_vault,
      },
      platform_config: decoded_layout.platform_config,
      virtual_token_a_reserve: decoded_layout.virtual_base,
      virtual_token_b_reserve: decoded_layout.virtual_quote,
      real_token_a_reserve: decoded_layout.real_base,
      real_token_b_reserve: decoded_layout.real_quote,
    }
  }
}
