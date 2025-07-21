use crate::central_context::central_context::CentralContext;
use crate::constants::LAMPORTS_PER_SOL;
use crate::types::amm_pool::AmmPool;
use crate::types::pool::Pool;
use crate::types::pool::PoolTrait;
use crate::types::pools::Pools;
use solana_sdk::pubkey::Pubkey;
use std::any::Any;
use std::sync::Arc;

#[derive(Debug)]
pub struct PumpswapPool {
  pub info: Pool,
  /*
  There are multiple possible fee vaults for pumpswap, we will pick one at random and record which
  one we decide to use in this pool
  */
  pub fee_vault: Pubkey,
  /*
  All fees are collected in the QUOTE token for pumpswap. We will have our token A be base so the
  associated token account between the fee vault and the base token is the fee recipient
  */
  pub fee_vault_token_account: Pubkey,
  pub token_a_vault_amount: u64,
  pub token_b_vault_amount: u64,
  pub coin_creator_vault_authority: Pubkey,
  pub coin_creator_vault_authority_token_account: Pubkey,
}

impl AmmPool for PumpswapPool {
  fn token_a_amount_units(&self) -> u64 {
    self.token_a_vault_amount
  }
  fn token_b_amount_units(&self) -> u64 {
    self.token_b_vault_amount
  }
}

impl PoolTrait for PumpswapPool {
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
  /*
  Going to hard code this for now as it is in the one global config account that every pumpswap
  pool uses, however if it is ever changed we will need to build infra to listen for changes and to
  update it
  0.25% = 0.0025 * LAMPORTS_PER_SOL = 2500000
  */
  fn total_swap_fee_lp(&self, _: &Arc<CentralContext>) -> u64 {
    2500000
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
    // Update token vaults fetch from rpc
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

pub const PUMP_SWAP_FEE_VAULTS: [Pubkey; 8] = [
  Pubkey::from_str_const("62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV"),
  Pubkey::from_str_const("7VtfL8fvgNfhz17qKRMjzQEXgbdpnHHHQRh54R9jP2RJ"),
  Pubkey::from_str_const("7hTckgnGnLQR6sdH7YkqFTAA7VwTfYFaZ6EhEsU3saCX"),
  Pubkey::from_str_const("9rPYyANsfQZw3DnDmKE3YCQF5E8oD89UXoHn9JFEhJUz"),
  Pubkey::from_str_const("AVmoTthdrX6tKt4nDjco2D775W2YK3sDhxPcMmzUAmTY"),
  Pubkey::from_str_const("FWsW1xNtWscwNmKv6wVsU1iTzRN6wmmk3MjxRP5tT7hz"),
  Pubkey::from_str_const("G5UZAVbAf46s7cKWoyKu8kYTip9DGTpbLZ2qa9Aq69dP"),
  Pubkey::from_str_const("JCRGumoE9Qi5BBgULTgdgTLjSgkCMSbF62ZZfGs84JeU"),
];
