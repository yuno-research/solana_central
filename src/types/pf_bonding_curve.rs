use crate::central_context::central_context::CentralContext;
use crate::constants::LAMPORTS_PER_SOL;
use crate::constants::PUMP_CONSTANTS;
use crate::constants::TOKENS;
use crate::types::pool::PoolTrait;
use crate::types::pools::Pools;
use crate::types::swap_direction::SwapDirection;
use solana_sdk::pubkey::Pubkey;
use std::any::Any;
use std::sync::Arc;

#[derive(Debug)]
pub struct PfBondingCurve {
  pub virtual_sol_reserves: u64,
  pub virtual_token_reserves: u64,
  pub complete: bool,
  pub token_address: Pubkey,
  pub bonding_curve_address: Pubkey,
  pub bonding_curve_associated_token_account_address: Pubkey,
  // Pubkey::find_program_address(&[b"creator-vault", creator.as_array()], &PUMP_CONSTANTS.bonding_curve_program,);
  pub creator_vault_address: Pubkey,
}

impl PoolTrait for PfBondingCurve {
  fn pool_address(&self) -> &Pubkey {
    &self.bonding_curve_address
  }
  fn token_a_address(&self) -> &Pubkey {
    &self.token_address
  }
  fn token_b_address(&self) -> &Pubkey {
    &TOKENS.wsol
  }
  fn token_a_vault_address(&self) -> &Pubkey {
    &self.bonding_curve_associated_token_account_address
  }
  fn token_b_vault_address(&self) -> &Pubkey {
    &self.bonding_curve_address
  }
  fn pool_type(&self) -> &Pools {
    &Pools::PfBondingCurve
  }
  /*
  Never going to call this likely but the fee is 1.25% with 0.95% protocol fee and 0.30% creator fee
  */
  fn total_swap_fee_lp(&self, _: &Arc<CentralContext>) -> u64 {
    12500000
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }

  fn price_a_over_b_lp(&self) -> u128 {
    self.virtual_token_reserves as u128 * LAMPORTS_PER_SOL / self.virtual_sol_reserves as u128
  }

  fn price_b_over_a_lp(&self) -> u128 {
    self.virtual_sol_reserves as u128 * LAMPORTS_PER_SOL / self.virtual_token_reserves as u128
  }

  fn fetch_market_state_from_rpc(&mut self, central_context: &Arc<CentralContext>) {
    // Fetch data from json rpc before calling updater
    let data = central_context
      .json_rpc_client
      .get_account(&self.bonding_curve_address)
      .unwrap()
      .data;
    self.update_state_from_data(&data);
  }
  /**
  This will get real token reserves metric
  */
  fn token_a_amount_units(&self) -> u64 {
    self.virtual_token_reserves - PUMP_CONSTANTS.bc_init_virtual_token_reserve_diff
  }

  /**
  This will get real sol reserves metric
  */
  fn token_b_amount_units(&self) -> u64 {
    self.virtual_sol_reserves - PUMP_CONSTANTS.bc_init_virtual_sol_reserves
  }

  fn directional_fees(&self, _: SwapDirection, __: &Arc<CentralContext>) -> (f64, f64) {
    (1.0, 1.0)
  }
}
