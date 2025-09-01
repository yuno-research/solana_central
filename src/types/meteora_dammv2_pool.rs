use crate::central_context::central_context::CentralContext;
use crate::constants::LAMPORTS_PER_SOL;
use crate::constants::METEORA_CONSTANTS;
use crate::protocol_idls::meteora::MeteoraDammv2PoolIdl;
use crate::types::pool::Pool;
use crate::types::pool::PoolTrait;
use crate::types::pools::Pools;
use borsh::BorshDeserialize;
use primitive_types::U256;
use solana_sdk::pubkey::Pubkey;
use std::any::Any;
use std::cmp;
use std::sync::Arc;

#[derive(Debug)]
pub struct MeteoraDammV2Pool {
  pub info: Pool,

  // Pool fee calculation info (disgustingly complex)
  pub cliff_fee_numerator: u64,
  pub fee_scheduler_mode: u8,
  pub number_of_period: u16,
  pub period_frequency: u64,
  pub reduction_factor: u64,
  pub protocol_fee_percent: u8,
  pub partner_fee_percent: u8,
  pub referral_fee_percent: u8,
  pub initialized: u8,
  pub variable_fee_control: u32,
  pub bin_step: u16,
  pub volatility_accumulator: u128,
  pub activation_point: u64,
  pub activation_type: u8,
  pub liquidity: u128,
  pub sqrt_price: u128,
  pub collect_fee_mode: u8,
  pub protocol_a_fee: u64,
  pub protocol_b_fee: u64,
  pub partner_a_fee: u64,
  pub partner_b_fee: u64,
  // Used for real reserve x and y calculations in xy = k
  pub sqrt_min_price: u128,
  pub sqrt_max_price: u128,
}

impl PoolTrait for MeteoraDammV2Pool {
  fn token_a_amount_units(&self) -> u64 {
    // Validate price bounds and liquidity
    if self.sqrt_price < self.sqrt_min_price
      || self.sqrt_price > self.sqrt_max_price
      || self.liquidity == 0
    {
      return 0;
    }

    let sqrt_price_diff = self.sqrt_max_price.saturating_sub(self.sqrt_price);

    // Use U256 to avoid overflow in intermediate calculations
    let liquidity_u256 = U256::from(self.liquidity);
    let sqrt_price_u256 = U256::from(self.sqrt_price);
    let sqrt_max_price_u256 = U256::from(self.sqrt_max_price);
    let price_diff_u256 = U256::from(sqrt_price_diff);

    // Calculate denominator: sqrt_price * sqrt_max_price
    let denominator = sqrt_price_u256.checked_mul(sqrt_max_price_u256);
    if denominator.is_none() || denominator.unwrap().is_zero() {
      return 0;
    }
    let denominator = denominator.unwrap();

    // Calculate numerator: liquidity * (sqrt_max_price - sqrt_price)
    let numerator = liquidity_u256.checked_mul(price_diff_u256);
    if numerator.is_none() {
      return 0;
    }
    let numerator = numerator.unwrap();

    // Calculate result: numerator / denominator
    let result = numerator.checked_div(denominator);
    if result.is_none() {
      return 0;
    }
    let result = result.unwrap();

    // Convert to u64, clamping if necessary
    if result > U256::from(u64::MAX) {
      return u64::MAX;
    }

    result.as_u64()
  }

  fn token_b_amount_units(&self) -> u64 {
    // Validate price bounds and liquidity
    if self.sqrt_price < self.sqrt_min_price
      || self.sqrt_price > self.sqrt_max_price
      || self.liquidity == 0
    {
      return 0;
    }

    let sqrt_price_diff = self.sqrt_price.saturating_sub(self.sqrt_min_price);

    // Use U256 for calculations to avoid overflow
    let liquidity_u256 = U256::from(self.liquidity);
    let price_diff_u256 = U256::from(sqrt_price_diff);

    // Calculate L * (√P_current - √P_min)
    let product = liquidity_u256.checked_mul(price_diff_u256);
    if product.is_none() {
      return 0;
    }
    let product = product.unwrap();

    // Divide by 2^128 (equivalent to right shift by 128)
    let result = product >> 128;

    // Convert to u64, clamping if necessary
    if result > U256::from(u64::MAX) {
      return u64::MAX;
    }

    result.as_u64()
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
    // Step 1: Calculate base fee numerator
    let base_fee_numerator = self.calculate_base_fee_numerator(central_context);

    // Step 2: Calculate dynamic fee numerator
    let dynamic_fee_numerator = self.calculate_dynamic_fee_numerator();
    println!(
      "[MeteoraDammV2Pool] Dynamic fee numerator: {}",
      dynamic_fee_numerator
    );

    // Step 3: Calculate total effective fee numerator
    let total_fee_numerator_uncapped = base_fee_numerator + dynamic_fee_numerator;
    println!(
      "[MeteoraDammV2Pool] Total uncapped fee numerator: {}",
      total_fee_numerator_uncapped
    );

    // We return in lamports and meteora denominator is 10^9 so we don't actually have to divide the numerator
    let total_fee_numerator_capped = cmp::min(
      total_fee_numerator_uncapped,
      METEORA_CONSTANTS.dammv2_max_fee_numerator,
    );
    total_fee_numerator_capped
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }

  fn price_a_over_b_lp(&self) -> u128 {
    if self.token_b_amount_units() == 0 {
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

  fn fetch_market_state_from_rpc(&mut self, central_context: &Arc<CentralContext>) {
    let decoded_layout: MeteoraDammv2PoolIdl = MeteoraDammv2PoolIdl::try_from_slice(
      &central_context
        .json_rpc_client
        .get_account(&self.info.pool_address)
        .unwrap()
        .data,
    )
    .unwrap();

    self.cliff_fee_numerator = decoded_layout.pool_fees.base_fee.cliff_fee_numerator;
    self.fee_scheduler_mode = decoded_layout.pool_fees.base_fee.fee_scheduler_mode;
    self.number_of_period = decoded_layout.pool_fees.base_fee.number_of_period;
    self.period_frequency = decoded_layout.pool_fees.base_fee.period_frequency;
    self.reduction_factor = decoded_layout.pool_fees.base_fee.reduction_factor;
    self.protocol_fee_percent = decoded_layout.pool_fees.protocol_fee_percent;
    self.partner_fee_percent = decoded_layout.pool_fees.partner_fee_percent;
    self.referral_fee_percent = decoded_layout.pool_fees.referral_fee_percent;
    self.initialized = decoded_layout.pool_fees.dynamic_fee.initialized;
    self.variable_fee_control = decoded_layout.pool_fees.dynamic_fee.variable_fee_control;
    self.bin_step = decoded_layout.pool_fees.dynamic_fee.bin_step;
    self.volatility_accumulator = decoded_layout.pool_fees.dynamic_fee.volatility_accumulator;
    self.activation_point = decoded_layout.activation_point;
    self.activation_type = decoded_layout.activation_type;
    self.liquidity = decoded_layout.liquidity;
    self.sqrt_price = decoded_layout.sqrt_price;
    self.collect_fee_mode = decoded_layout.collect_fee_mode;
    self.protocol_a_fee = decoded_layout.protocol_a_fee;
    self.protocol_b_fee = decoded_layout.protocol_b_fee;
    self.partner_a_fee = decoded_layout.partner_a_fee;
    self.partner_b_fee = decoded_layout.partner_b_fee;
    self.sqrt_max_price = decoded_layout.sqrt_max_price;
    self.sqrt_min_price = decoded_layout.sqrt_min_price;
  }
}
