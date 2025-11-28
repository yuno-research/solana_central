use crate::CentralContext;
use crate::types::meteora_vault::MeteoraVault;
use borsh::BorshDeserialize;
use solana_sdk::account::ReadableAccount;
use std::sync::Arc;
use crate::protocol_idls::meteora::{VaultIdlBig, VaultIdlSmall};

impl MeteoraVault {
  /// Update vault information from on-chain account data
  ///
  /// Fetches the vault account and updates all vault state including locked profit
  /// tracker, total amount, and LP token supply. Supports both big (10240 bytes)
  /// and small (1232 bytes) vault account formats.
  pub fn update_vault_info(&mut self, central_context: Arc<CentralContext>) {
    let vault_address = self.vault;
    let binding = central_context
      .json_rpc_client
      .get_account(&vault_address)
      .unwrap();
    let vault_data = binding.data();

    // if it is a big vault and the data length is 10240, then we need to use the big vault idl
    if vault_data.len() == 10240 {
      let decoded = VaultIdlBig::try_from_slice(&vault_data).unwrap();

      self.last_updated_locked_profit = decoded.locked_profit_tracker.last_updated_locked_profit;
      self.last_report = decoded.locked_profit_tracker.last_report;
      self.locked_profit_degradation = decoded.locked_profit_tracker.locked_profit_degradation;
      self.total_amount = decoded.total_amount;
      /*
      There are some cases where if you deterministically derive a popular token like USDC or USDT or
      WSOL then the actual address will be different. In this case, lookup the actual address.
      */
      self.lp_token_address = decoded.lp_mint;
      self.lp_supply = central_context
        .json_rpc_client
        .get_token_supply(&decoded.lp_mint)
        .unwrap()
        .amount
        .parse()
        .unwrap();
    } else if vault_data.len() == 1232 {
      let decoded = VaultIdlSmall::try_from_slice(&vault_data).unwrap();

      self.last_updated_locked_profit = decoded.locked_profit_tracker.last_updated_locked_profit;
      self.last_report = decoded.locked_profit_tracker.last_report;
      self.locked_profit_degradation = decoded.locked_profit_tracker.locked_profit_degradation;
      self.total_amount = decoded.total_amount;
      /*
      There are some cases where if you deterministically derive a popular token like USDC or USDT or
      WSOL then the actual address will be different. In this case, lookup the actual address.
      */
      self.lp_token_address = decoded.lp_mint;
      self.lp_supply = central_context
        .json_rpc_client
        .get_token_supply(&decoded.lp_mint)
        .unwrap()
        .amount
        .parse()
        .unwrap();
    } else {
      println!(
        "update_meteora_vaultinfo: Unknown vault data length for vault: {:?}",
        vault_address
      );
    }
  }
}
