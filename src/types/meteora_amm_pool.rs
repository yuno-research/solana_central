use crate::central_context::central_context::CentralContext;
use crate::constants::LAMPORTS_PER_SOL;
use crate::protocol_idls::meteora::PoolFees;
use crate::types::amm_pool::AmmPool;
use crate::types::meteora_vault::MeteoraVault;
use crate::types::pool::Pool;
use crate::types::pool::PoolTrait;
use crate::types::pools::Pools;
use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;
use std::any::Any;
use std::sync::{Arc, RwLock};
/*
This struct contains the data that is UNIQUE to a specific AMM Pool on Meteora. For shared data,
such as vaults for tokens, because they are shared across all AMM pools in the Meteora protocol,
they will be stored in shared MeteoraVault objects. References to those shared objects will be
stored in these pool info objects.

The lifetime indicator of 'a denotes that throughout the lifetime of a MeteoraAmmPool struct, there
will always be vaults for both tokens corresponding to it. Ultimately, all vaults can be found in
the vaults cache object stored in Meteora_vault.
*/
pub struct MeteoraAmmPool {
  pub info: Pool,
  // Is the pool currently enabled for trading, looks like this is a meteora specific field
  pub enabled: bool,
  pub token_a_vault: Arc<RwLock<MeteoraVault>>,
  pub token_b_vault: Arc<RwLock<MeteoraVault>>,
  // The address of the token account that receives the swap fees for token A
  pub token_a_fee_address: Pubkey,
  // The address of the token account that receives the swap fees for token B
  pub token_b_fee_address: Pubkey,
  // This is the address of the vault for the liquidity provider token for token A
  pub token_a_lp_vault_address: Pubkey,
  // This is the address of the vault for the liquidity provider token for token B
  pub token_b_lp_vault_address: Pubkey,
  // The amount of token A liquidity provider tokens in THIS pool specifically
  pub token_a_lp_amount: u128,
  pub token_b_lp_amount: u128,
  pub trade_fee_numerator: u128,
  pub trade_fee_denominator: u128,
  pub protocol_trade_fee_numerator: u128,
  pub protocol_trade_fee_denominator: u128,
}

impl AmmPool for MeteoraAmmPool {
  fn token_a_amount_units(&self) -> u64 {
    let token_a_vault_guard = self.token_a_vault.read().unwrap();
    if token_a_vault_guard.lp_supply == 0 {
      0
    } else {
      (self.token_a_lp_amount * token_a_vault_guard.calculate_withdrawable_amount() as u128
        / token_a_vault_guard.lp_supply as u128) as u64
    }
  }
  fn token_b_amount_units(&self) -> u64 {
    let token_b_vault_guard = self.token_b_vault.read().unwrap();
    if token_b_vault_guard.lp_supply == 0 {
      0
    } else {
      (self.token_b_lp_amount * token_b_vault_guard.calculate_withdrawable_amount() as u128
        / token_b_vault_guard.lp_supply as u128) as u64
    }
  }
}

impl PoolTrait for MeteoraAmmPool {
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
    (self.trade_fee_numerator * LAMPORTS_PER_SOL / self.trade_fee_denominator
      + self.protocol_trade_fee_numerator * LAMPORTS_PER_SOL / self.protocol_trade_fee_denominator)
      as u64
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }

  fn price_a_over_b_lp(&self) -> u128 {
    if self.token_a_amount_units() == 0 {
      return 0;
    }
    self.token_a_amount_units() as u128 * LAMPORTS_PER_SOL / self.token_b_amount_units() as u128
  }

  fn price_b_over_a_lp(&self) -> u128 {
    if self.token_a_amount_units() == 0 {
      return 0;
    }
    self.token_b_amount_units() as u128 * LAMPORTS_PER_SOL / self.token_a_amount_units() as u128
  }
  fn fetch_market_state_from_rpc(&mut self, context: &Arc<CentralContext>) {
    // Update vaults
    self
      .token_a_vault
      .write()
      .unwrap()
      .update_vault_info(context.clone());
    self
      .token_b_vault
      .write()
      .unwrap()
      .update_vault_info(context.clone());

    // Update this pools liquidity amounts
    self.token_a_lp_amount = context
      .json_rpc_client
      .get_token_account_balance(&self.token_a_lp_vault_address)
      .unwrap()
      .amount
      .parse()
      .unwrap();
    self.token_b_lp_amount = context
      .json_rpc_client
      .get_token_account_balance(&self.token_b_lp_vault_address)
      .unwrap()
      .amount
      .parse()
      .unwrap();
  }
}
