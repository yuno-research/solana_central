use crate::constants::RAYDIUM_CONSTANTS;
use crate::protocol_idls::raydium::LaunchpadPoolIdl;
use crate::types::pool::Pool;
use crate::types::pools::Pools;
use crate::types::raydium_launchpad::RaydiumLaunchpad;
use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;

impl RaydiumLaunchpad {
  /// Create a Raydium launchpad pool from on-chain account data
  ///
  /// Parses the launchpad pool account and derives platform and creator vault PDAs.
  pub fn from_account_info(pubkey: Pubkey, account_buffer: &[u8]) -> Self {
    let decoded_layout = LaunchpadPoolIdl::try_from_slice(account_buffer).unwrap();
    let platform_config = decoded_layout.platform_config;
    let token_b_address = decoded_layout.quote_mint;

    let (platform_vault, _) = Pubkey::find_program_address(
      &[platform_config.as_array(), token_b_address.as_array()],
      &RAYDIUM_CONSTANTS.launchpad_program,
    );
    let (creator_vault, _) = Pubkey::find_program_address(
      &[
        decoded_layout.creator.as_array(),
        token_b_address.as_array(),
      ],
      &RAYDIUM_CONSTANTS.launchpad_program,
    );

    Self {
      info: Pool {
        pool_address: pubkey,
        token_a_address: decoded_layout.base_mint,
        token_b_address,
        pool_type: Pools::RaydiumLaunchpad,
        token_a_vault_address: decoded_layout.base_vault,
        token_b_vault_address: decoded_layout.quote_vault,
      },
      platform_config,
      platform_vault,
      creator_vault,
      virtual_token_a_reserve: decoded_layout.virtual_base,
      virtual_token_b_reserve: decoded_layout.virtual_quote,
      real_token_a_reserve: decoded_layout.real_base,
      real_token_b_reserve: decoded_layout.real_quote,
    }
  }
}
