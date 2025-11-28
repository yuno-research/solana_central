use crate::central_context::central_context::CentralContext;
use crate::types::pools::Pools;
use crate::types::swap_direction::SwapDirection;
use solana_sdk::pubkey::Pubkey;
use std::any::Any;
use std::sync::Arc;
/*
Base pool info struct, this struct contains data that all other pool infos have, and each pool info
will have a value in it called "info" of this type.
*/
#[derive(Debug)]
pub struct Pool {
  pub pool_address: Pubkey,
  pub token_a_address: Pubkey,
  pub token_b_address: Pubkey,
  pub token_a_vault_address: Pubkey,
  pub token_b_vault_address: Pubkey,
  pub pool_type: Pools,
}

pub trait PoolTrait: Any + Send + Sync {
  fn pool_address(&self) -> &Pubkey;
  fn token_a_address(&self) -> &Pubkey;
  fn token_b_address(&self) -> &Pubkey;
  fn token_a_vault_address(&self) -> &Pubkey;
  fn token_b_vault_address(&self) -> &Pubkey;
  fn pool_type(&self) -> &Pools;
  /*
  10^9 lamports = 1. This is to follow the same convention to use big numbers for high
  precision like what solana does
  The total swap fee in lamports points for the pool
  */
  fn total_swap_fee_lp(&self, central_context: &Arc<CentralContext>) -> u64;

  fn as_any(&self) -> &dyn Any;
  fn as_any_mut(&mut self) -> &mut dyn Any;

  /*
  Returns the price in lamports of token A in terms of token B.
  How many A tokens it costs to buy 1 token B. For amms this is calculated as:
  A reserves * LAMPORTS_PER_SOL / B reserves
  */
  fn price_a_over_b_lp(&self) -> u128;

  // Same thing except b over a
  fn price_b_over_a_lp(&self) -> u128;

  /**
  This function is meant for debugging to just fetch current state from rpc and immediately override
  the market state in the pool. It is meant to be used for initial market state fetching and will
  not work if the pool is being updated by gRPC

  TODO add checks to not run the JSON RPC calls and add indices for slot updated at and index of TX
  updated at where relevant
  */
  fn fetch_market_state_from_rpc(&mut self, central_context: &Arc<CentralContext>);

  /*
  We do not just store token vault balances in the pool struct because some protocols like Meteora
  do not directly store real token vault balances and instead derive them from other balances such
  as lp token balances for that pool and other values. Therefore, we instead use a trait and have
  every AmmPool define the ability to calculate the real token vault balances from the data it has.
  Each protocol stores the data necessary to calculate the real token vault balances in their own
  structs and implements this trait.
  */
  // The actual real amount of token A in the pool in the units of the token
  fn token_a_amount_units(&self) -> u64;
  // The actual real amount of token B in the pool in the units of the token
  fn token_b_amount_units(&self) -> u64;

  /**
  Returns the directional fees as fractions for a given swap direction.
  Returns (fee_a_fraction, fee_b_fraction) where:
  - fee_a_fraction is the fee applied to token A (0.0 to 1.0, e.g., 0.003 = 0.3%)
  - fee_b_fraction is the fee applied to token B

  For constant product formula: (x + Δx(1 - fee_x)) × (y - Δy/(1 - fee_y)) = x

  central_context is needed to get current slot/timestamp for time-based fee calculations for
  meteora dammv2
  */
  fn directional_fees(
    &self,
    direction: SwapDirection,
    central_context: &Arc<CentralContext>,
  ) -> (f64, f64);
}
