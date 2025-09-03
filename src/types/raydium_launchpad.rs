use crate::central_context::central_context::CentralContext;
use crate::constants::LAMPORTS_PER_SOL;
use crate::protocol_idls::raydium::LaunchpadPoolIdl;
use crate::types::pool::Pool;
use crate::types::pool::PoolTrait;
use crate::types::pools::Pools;
use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;
use std::any::Any;
use std::sync::Arc;

/**
As of right now the only global config account for Raydium Launchlab uses the constant product pool.
We use do token a is base and token b is quote here.

*/
pub struct RaydiumLaunchpad {
  pub info: Pool,
  pub platform_config: Pubkey,
  /*
  PDAs required for swap instructions after the creator fee update.
  */
  pub platform_vault: Pubkey,
  pub creator_vault: Pubkey,
  // The initial reserves that the token pool was created with
  pub virtual_token_a_reserve: u64,
  pub virtual_token_b_reserve: u64,
  pub real_token_a_reserve: u64,
  pub real_token_b_reserve: u64,
}

impl PoolTrait for RaydiumLaunchpad {
  fn token_a_amount_units(&self) -> u64 {
    // A is the base token which is being sold as the launchpad progresses
    self.virtual_token_a_reserve - self.real_token_a_reserve
  }
  fn token_b_amount_units(&self) -> u64 {
    // B is the quote token which fills up as the launchpad progresses
    self.virtual_token_b_reserve + self.real_token_b_reserve
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

  // Look at chat convo swap fee + share fee + protocol fee which is found in various accounts
  fn total_swap_fee_lp(&self, central_context: &Arc<CentralContext>) -> u64 {
    let platform_fee = *central_context
      .raydium_launchpad_platform_fee_rates_lp
      .get(&self.platform_config)
      // Assume that if the fee rate is not found, it will be 100% and we shouldn't swap here
      .unwrap_or_else(|| &(LAMPORTS_PER_SOL as u64));
    // Platform fee + the 0.25% protocol fee
    platform_fee + 2500000
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }

  fn price_a_over_b_lp(&self) -> u128 {
    self.token_a_amount_units() as u128 * LAMPORTS_PER_SOL / self.token_b_amount_units() as u128
  }

  fn price_b_over_a_lp(&self) -> u128 {
    self.token_b_amount_units() as u128 * LAMPORTS_PER_SOL / self.token_a_amount_units() as u128
  }

  fn fetch_market_state_from_rpc(&mut self, central_context: &Arc<CentralContext>) {
    // The only values that change are real reserves, and those are stored in the pool account.
    let current_pool_state = LaunchpadPoolIdl::try_from_slice(
      central_context
        .json_rpc_client
        .get_account(&self.info.pool_address)
        .unwrap()
        .data
        .as_slice(),
    )
    .unwrap();
    self.real_token_a_reserve = current_pool_state.real_base;
    self.real_token_b_reserve = current_pool_state.real_quote;
  }
}
