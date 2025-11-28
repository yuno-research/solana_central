use crate::central_context::central_context::CentralContext;
use crate::constants::RAYDIUM_CONSTANTS;
use crate::protocol_idls::raydium::CpmmPoolConfigIdl;
use borsh::BorshDeserialize;
use solana_account_decoder::UiAccountEncoding;
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use solana_client::rpc_filter::RpcFilterType;

impl CentralContext {
  /// Load Raydium CPMM pool configurations from on-chain data
  ///
  /// Fetches all Raydium CPMM pool config accounts and populates the `raydium_cpmm_fee_rates_lp`
  /// map with fee rates. This should be called during initialization before processing pools.
  pub fn load_cpmm_pool_configs(&mut self) {
    // Fetch the Raydium CPMM pool configs and make a hash map of the pool addresses to their config
    let configs = self
      .json_rpc_client
      .get_program_accounts_with_config(
        &RAYDIUM_CONSTANTS.cpmm_program,
        RpcProgramAccountsConfig {
          // The size of the Raydium CPMM pool config account
          filters: Some(vec![RpcFilterType::DataSize(236)]),

          account_config: RpcAccountInfoConfig {
            // ask the node to send account.data as base64
            encoding: Some(UiAccountEncoding::Base64),
            // you can leave these as default if you don’t need them
            data_slice: None,
            commitment: None,
            min_context_slot: None,
          },
          // fill in the rest with defaults
          ..RpcProgramAccountsConfig::default()
        },
      )
      .unwrap();

    for (pubkey, account) in configs {
      let decoded_layout: CpmmPoolConfigIdl =
        CpmmPoolConfigIdl::try_from_slice(&account.data).unwrap();

      self
        .raydium_cpmm_fee_rates_lp
        .insert(pubkey, decoded_layout.trade_fee_rate * 1000);
    }
    println!(
      "INIT app_context: Found {} Raydium CPMM pool configs. Loaded into global app context.",
      self.raydium_cpmm_fee_rates_lp.len()
    );
  }
}
