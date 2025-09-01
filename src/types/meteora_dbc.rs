use crate::central_context::central_context::CentralContext;
use crate::types::pool::{Pool, PoolTrait};
use crate::types::pools::Pools;
use primitive_types::{U256, U512};
use solana_sdk::pubkey::Pubkey;
use std::any::Any;
use std::cmp::min;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::constants::LAMPORTS_PER_SOL;

const FEE_DENOMINATOR: u64 = 1_000_000_000;
const MAX_FEE_NUMERATOR: u64 = 990_000_000;
/// The denominator for the exponential fee reduction factor.
const EXPONENTIAL_FEE_REDUCTION_DENOMINATOR: f64 = 10000.0;
/// The denominator for scaling the dynamic fee calculation.
const DYNAMIC_FEE_SCALING_DENOMINATOR: u128 = 100_000_000_000;

/**
The base fee on meteora dbc can be either a linear fee or an exponential fee.
*/
pub enum BaseFeeMode {
  Linear = 0,
  Exponential = 1,
  RateLimiter = 2,
}

pub enum ActivationType {
  Slot,
  Time,
}

// Like in all other protocols token A is base and token B is quote.
// TODO use our own sol token account in the referral fee if we are trading with sol
pub struct MeteoraDbc {
  pub pool: Pool,
  pub config: Pubkey,
  /*
  The price in lamports of 1 token a in terms of token b (quote token). This is the same format used
  as sqrt price and we will just convert this over to lamports to maintain precision just like is
  done for the other protocols. Stored in the protocol as sqrt price
  */
  pub sqrt_price: u128,

  /*
  Fee Calculation Fields
  */
  /// The initial fee numerator at the start of the curve.
  pub cliff_fee_numerator: u64,
  /// The mode for calculating the base fee (Linear or Exponential).
  pub base_fee_mode: BaseFeeMode,
  /// The slot or timestamp when the pool becomes active and fees start decaying.
  pub activation_point: u64,
  /// Accumulator for price volatility, used in dynamic fee calculation.
  pub volatility_accumulator: u128,
  /// A control factor for adjusting the impact of volatility on the dynamic fee.
  pub variable_fee_control: u32,
  /// The method of time kept to track
  pub activation_type: ActivationType,
  pub base_fee_reduction_factor: u64,
  pub base_fee_period_frequency: u64,
  pub base_fee_number_of_periods: u16,
}

impl PoolTrait for MeteoraDbc {
  fn pool_address(&self) -> &Pubkey {
    &self.pool.pool_address
  }

  fn token_a_address(&self) -> &Pubkey {
    &self.pool.token_a_address
  }

  fn token_b_address(&self) -> &Pubkey {
    &self.pool.token_b_address
  }

  fn token_a_vault_address(&self) -> &Pubkey {
    &self.pool.token_a_vault_address
  }

  fn token_b_vault_address(&self) -> &Pubkey {
    &self.pool.token_b_vault_address
  }

  fn pool_type(&self) -> &Pools {
    &Pools::MeteoraDbc
  }

  /**
  Calculates the total trading fee numerator.
  This combines the base fee (scheduled) and the dynamic fee (volatility-based).
  Returns just the numerator as the denomiator is 10^9 whcih is lamports.
  */
  fn total_swap_fee_lp(&self, central_context: &Arc<CentralContext>) -> u64 {
    let current_point = match self.activation_type {
      // Slot-based timing
      ActivationType::Slot => *central_context.current_slot.read().unwrap(),
      // Timestamp-based timing
      ActivationType::Time => SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs(),
    };

    // Base fee numerator calculations
    let period = if current_point < self.activation_point || self.base_fee_period_frequency == 0 {
      0
    } else {
      (current_point - self.activation_point)
        .checked_div(self.base_fee_period_frequency)
        .unwrap_or(0)
        .min(self.base_fee_number_of_periods as u64)
    };

    let base_fee_numerator = match self.base_fee_mode {
      BaseFeeMode::Linear => {
        if period * self.base_fee_reduction_factor >= self.cliff_fee_numerator {
          return 0;
        }
        self.cliff_fee_numerator - (period * self.base_fee_reduction_factor)
      }
      BaseFeeMode::Exponential => {
        // Use f64 for this as it typically will not get larger than cliff fee numerator
        let result = self.cliff_fee_numerator as f64
          * (1.0 - self.base_fee_reduction_factor as f64 / EXPONENTIAL_FEE_REDUCTION_DENOMINATOR)
            .powi(period as i32);
        if result < 1.0 {
          return 0;
        }
        result.round() as u64
      }
      // we will just use cliff fee numerator for rate limiter for simplicity
      _ => self.cliff_fee_numerator,
    };

    // Dynamic fee calculations
    let dynamic_fee_numerator = ((U512::from(self.volatility_accumulator).pow(U512::from(2))
      * U512::from(self.variable_fee_control))
      / U512::from(DYNAMIC_FEE_SCALING_DENOMINATOR))
    .as_u64();

    // Cap at max fee numerator
    let total_fee_numerator = min(
      base_fee_numerator + dynamic_fee_numerator,
      MAX_FEE_NUMERATOR,
    );
    total_fee_numerator
  }

  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }

  /**
  The reciprocal of the price_b_over_a_lp.
  */
  fn price_a_over_b_lp(&self) -> u128 {
    let price = U512::from(self.sqrt_price).pow(U512::from(2));
    let two_pow_128 = U512::from(1) << 128; // Represents (2^64)^2
    (two_pow_128 * U512::from(LAMPORTS_PER_SOL) / price).as_u128()
  }

  /**
  The sqrt price of the protocol will be in terms of quote over base which for us is b over a.
  */
  fn price_b_over_a_lp(&self) -> u128 {
    let price = U512::from(self.sqrt_price).pow(U512::from(2));
    let two_pow_128 = U512::from(1) << 128; // Represents (2^64)^2
    ( price * U512::from(LAMPORTS_PER_SOL) / two_pow_128).as_u128()
  }

  fn fetch_market_state_from_rpc(&mut self, central_context: &Arc<CentralContext>) {
    
  }
}
