use crate::central_context::central_context::CentralContext;
use crate::types::pools::Pools;
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
}
