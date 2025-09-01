use crate::central_context::central_context::CentralContext;
use crate::constants::LAMPORTS_PER_SOL;
use crate::types::pool::Pool;
use crate::types::pool::PoolTrait;
use crate::types::pools::Pools;
use solana_sdk::pubkey::Pubkey;
use std::any::Any;
use std::sync::Arc;

pub struct RaydiumCpmmPool {
  pub info: Pool,
  pub pool_config_account: Pubkey,
  pub observation_state_account: Pubkey,
  pub token_a_vault_amount: u64,
  pub token_b_vault_amount: u64,
}

impl PoolTrait for RaydiumCpmmPool {
  fn token_a_amount_units(&self) -> u64 {
    self.token_a_vault_amount
  }
  fn token_b_amount_units(&self) -> u64 {
    self.token_b_vault_amount
  }

  fn pool_address(&self) -> &Pubkey {
    &self.info.pool_address
  }
  fn token_a_address(&self) -> &Pubkey {
    &self.info.token_a_address
  }
  fn token_b_address(&self) -> &Pubkey {
    &self.info.token_b_address
  }
  fn token_a_vault_address(&self) -> &Pubkey {
    &self.info.token_a_vault_address
  }
  fn token_b_vault_address(&self) -> &Pubkey {
    &self.info.token_b_vault_address
  }
  fn pool_type(&self) -> &Pools {
    &self.info.pool_type
  }

  fn total_swap_fee_lp(&self, central_context: &Arc<CentralContext>) -> u64 {
    *central_context
      .raydium_cpmm_fee_rates_lp
      .get(&self.pool_config_account)
      // Assume that if the fee rate is not found, it will be 100% and we shouldn't swap here
      .unwrap_or_else(|| &(LAMPORTS_PER_SOL as u64))
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }

  fn price_a_over_b_lp(&self) -> u128 {
    self.token_a_vault_amount as u128 * LAMPORTS_PER_SOL / self.token_b_vault_amount as u128
  }

  fn price_b_over_a_lp(&self) -> u128 {
    self.token_b_vault_amount as u128 * LAMPORTS_PER_SOL / self.token_a_vault_amount as u128
  }

  fn fetch_market_state_from_rpc(&mut self, central_context: &Arc<CentralContext>) {
    self.token_a_vault_amount = central_context
      .json_rpc_client
      .get_token_account_balance(&self.info.token_a_vault_address)
      .unwrap()
      .amount
      .parse()
      .unwrap();
    self.token_b_vault_amount = central_context
      .json_rpc_client
      .get_token_account_balance(&self.info.token_b_vault_address)
      .unwrap()
      .amount
      .parse()
      .unwrap();
  }
}
