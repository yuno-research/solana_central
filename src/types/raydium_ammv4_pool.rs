use crate::central_context::central_context::CentralContext;
use crate::constants::LAMPORTS_PER_SOL;
use crate::types::amm_pool::AmmPool;
use crate::types::pool::Pool;
use crate::types::pool::PoolTrait;
use crate::types::pools::Pools;
use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;
use std::any::Any;
use std::sync::Arc;
/*
The only things you actually need to trade on a Raydium AmmV4 at least are these things:

id: The liquidity pool program address,

baseVault + quoteVault

The rest of the fields can be constant blasted as random values and the program will not care
because Serum and Openbook are no longer in use for these AMMs.
*/
#[derive(Debug)]
pub struct RaydiumAmmV4Pool {
  pub info: Pool,
  pub token_a_vault_amount: u64,
  pub token_b_vault_amount: u64,
  pub swap_fee_numerator: u64,
  pub swap_fee_denominator: u64,
}

impl AmmPool for RaydiumAmmV4Pool {
  fn token_a_amount_units(&self) -> u64 {
    self.token_a_vault_amount
  }
  fn token_b_amount_units(&self) -> u64 {
    self.token_b_vault_amount
  }
}

impl PoolTrait for RaydiumAmmV4Pool {
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

  fn total_swap_fee_lp(&self, _: &Arc<CentralContext>) -> u64 {
    self.swap_fee_numerator * LAMPORTS_PER_SOL as u64 / self.swap_fee_denominator
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
