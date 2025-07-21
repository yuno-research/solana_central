use crate::central_context::central_context::CentralContext;
use crate::constants::RAYDIUM_CONSTANTS;
use crate::protocol_idls::raydium::LaunchpadPlatformConfigIdl;
use borsh::BorshDeserialize;
use solana_account_decoder::UiAccountEncoding;
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use solana_client::rpc_filter::RpcFilterType;

impl CentralContext {
  pub fn load_raydium_launchpad_platform_configs(&mut self) {
    // Fetch the Raydium Launchpad platform config accounts and make a hash map of the platform
    let configs = self
      .json_rpc_client
      .get_program_accounts_with_config(
        &RAYDIUM_CONSTANTS.launchpad_program,
        RpcProgramAccountsConfig {
          // The size of the Raydium CPMM pool config account
          filters: Some(vec![RpcFilterType::DataSize(944)]),
          account_config: RpcAccountInfoConfig {
            // ask the node to send account.data as base64
            encoding: Some(UiAccountEncoding::Base64),
            // you can leave these as default if you don’t need them
            data_slice: None,
            commitment: None,
            min_context_slot: None,
          },
          ..RpcProgramAccountsConfig::default()
        },
      )
      .unwrap();

    for (pubkey, account) in configs {
      let decoded_layout: LaunchpadPlatformConfigIdl =
        LaunchpadPlatformConfigIdl::try_from_slice(&account.data).unwrap();

      self
        .raydium_launchpad_platform_fee_rates_lp
        .insert(pubkey, decoded_layout.fee_rate * 1000);
    }
    println!(
      "INIT app_context: Found {} Raydium Launchpad fee configs. Loaded into global app context.",
      self.raydium_cpmm_fee_rates_lp.len()
    );
  }
}
