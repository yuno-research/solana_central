use crate::protocol_idls::raydium::CpmmPoolInfoIdl;
use crate::raydium::get_cpmm_fee_amount_from_config_account::get_cpmm_fee_amount_from_config_account;
use crate::types::pool::Pool;
use crate::types::pools::Pools;
use crate::types::raydium_cpmm_pool::RaydiumCpmmPool;
use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;

impl RaydiumCpmmPool {
  /// Create a Raydium CPMM pool from on-chain account data
  ///
  /// Parses the account buffer and looks up the fee rate from the config account.
  pub fn from_account_info(pubkey: Pubkey, account_buffer: &[u8]) -> Self {
    let decoded_layout: CpmmPoolInfoIdl = CpmmPoolInfoIdl::try_from_slice(account_buffer).unwrap();
    let fee_fraction_lp = get_cpmm_fee_amount_from_config_account(decoded_layout.amm_config, &pubkey);
    Self {
      info: Pool {
        pool_address: pubkey,
        token_a_address: decoded_layout.token_0_mint,
        token_b_address: decoded_layout.token_1_mint,
        pool_type: Pools::RaydiumCpmm,
        token_a_vault_address: decoded_layout.token_0_vault,
        token_b_vault_address: decoded_layout.token_1_vault,
      },
      pool_config_account: decoded_layout.amm_config,
      observation_state_account: decoded_layout.observation_key,
      // Account layout doesn't contain vault balances, so we set them to 0
      token_a_vault_amount: 0,
      token_b_vault_amount: 0,
      // Initialize accumulated fees from the decoded pool state
      protocol_fees_token_a: decoded_layout.protocol_fees_token_0,
      protocol_fees_token_b: decoded_layout.protocol_fees_token_1,
      fund_fees_token_a: decoded_layout.fund_fees_token_0,
      fund_fees_token_b: decoded_layout.fund_fees_token_1,
      creator_fees_token_a: decoded_layout.creator_fees_token_0,
      creator_fees_token_b: decoded_layout.creator_fees_token_1,
      fee_fraction_lp,
    }
  }
}
