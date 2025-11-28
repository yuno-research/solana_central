use crate::central_context::central_context::CentralContext;
use crate::constants::LAMPORTS_PER_SOL;
use crate::protocol_idls::raydium::CpmmPoolInfoIdl;
use crate::types::pool::Pool;
use crate::types::pool::PoolTrait;
use crate::types::pools::Pools;
use crate::types::swap_direction::SwapDirection;
use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;
use std::any::Any;
use std::sync::Arc;

#[derive(Debug)]
pub struct RaydiumCpmmPool {
  pub info: Pool,
  pub pool_config_account: Pubkey,
  pub fee_fraction_lp: u64,
  pub observation_state_account: Pubkey,
  // These are raw vault token account balances. Not actual liquidity reserves
  pub token_a_vault_amount: u64,
  pub token_b_vault_amount: u64,

  /*
  These are accumulated fees which are stored in the vault token accounts but are not apart of the
  liquidity of the pool. When the constant product logic is applied, they are subtracted by the
  protocol and you can see thsi in the vault_amount_without_fee function in the program source code
  here: https://github.com/raydium-io/raydium-cp-swap/blob/master/programs/cp-swap/src/states/pool.rs

  We subtract these fields as well to get our actual amount reserves for the pool.
  */
  pub protocol_fees_token_a: u64,
  pub protocol_fees_token_b: u64,
  pub fund_fees_token_a: u64,
  pub fund_fees_token_b: u64,
  pub creator_fees_token_a: u64,
  pub creator_fees_token_b: u64,
}

impl PoolTrait for RaydiumCpmmPool {
  fn token_a_amount_units(&self) -> u64 {
    self
      .token_a_vault_amount
      .saturating_sub(self.protocol_fees_token_a)
      .saturating_sub(self.fund_fees_token_a)
      .saturating_sub(self.creator_fees_token_a)
  }
  fn token_b_amount_units(&self) -> u64 {
    self
      .token_b_vault_amount
      .saturating_sub(self.protocol_fees_token_b)
      .saturating_sub(self.fund_fees_token_b)
      .saturating_sub(self.creator_fees_token_b)
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

  fn total_swap_fee_lp(&self, _central_context: &Arc<CentralContext>) -> u64 {
    self.fee_fraction_lp
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }

  fn price_a_over_b_lp(&self) -> u128 {
    // Use tradeable amounts (excluding fees) for accurate pricing
    let token_a_tradeable = self.token_a_amount_units();
    let token_b_tradeable = self.token_b_amount_units();
    token_a_tradeable as u128 * LAMPORTS_PER_SOL / token_b_tradeable as u128
  }

  fn price_b_over_a_lp(&self) -> u128 {
    // Use tradeable amounts (excluding fees) for accurate pricing
    let token_a_tradeable = self.token_a_amount_units();
    let token_b_tradeable = self.token_b_amount_units();
    token_b_tradeable as u128 * LAMPORTS_PER_SOL / token_a_tradeable as u128
  }

  fn fetch_market_state_from_rpc(&mut self, central_context: &Arc<CentralContext>) {
    // Fetch raw vault balances
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

    // Fetch pool state account to get accumulated fees
    let pool_account = central_context
      .json_rpc_client
      .get_account(&self.info.pool_address)
      .unwrap();

    let pool_state: CpmmPoolInfoIdl = CpmmPoolInfoIdl::try_from_slice(&pool_account.data).unwrap();

    // Update accumulated fees
    self.protocol_fees_token_a = pool_state.protocol_fees_token_0;
    self.protocol_fees_token_b = pool_state.protocol_fees_token_1;
    self.fund_fees_token_a = pool_state.fund_fees_token_0;
    self.fund_fees_token_b = pool_state.fund_fees_token_1;
    self.creator_fees_token_a = pool_state.creator_fees_token_0;
    self.creator_fees_token_b = pool_state.creator_fees_token_1;
  }

  fn directional_fees(&self, swap_direction: SwapDirection, _: &Arc<CentralContext>) -> (f64, f64) {
    // Fee is taken in the token that is being swapped on the way in
    if swap_direction == SwapDirection::AToB {
      (self.fee_fraction_lp as f64 / 1_000_000_000.0, 0.0)
    } else {
      (0.0, self.fee_fraction_lp as f64 / 1_000_000_000.0)
    }
  }
}
