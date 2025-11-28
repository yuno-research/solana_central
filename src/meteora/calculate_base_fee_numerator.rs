use crate::CentralContext;
use crate::types::meteora_dammv2_pool::MeteoraDammV2Pool;
use std::cmp;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

impl MeteoraDammV2Pool {
  /// Calculate the base fee numerator for DAMMv2 pools with fee scheduling
  ///
  /// Supports multiple fee modes:
  /// - Linear decay: Fee decreases linearly over periods
  /// - Exponential decay: Fee decreases exponentially over periods
  /// - Rate limiter: Not yet implemented, returns cliff fee
  ///
  /// The calculation is based on the activation point and elapsed time/slots since activation.
  pub fn calculate_base_fee_numerator(&self, central_context: &Arc<CentralContext>) -> u64 {
    // Decode overloaded fields based on base_fee_mode
    match self.base_fee_mode {
      0 | 1 => {
        // Fee Scheduler modes (Linear or Exponential)
        let number_of_period = self.first_factor;
        let period_frequency = u64::from_le_bytes(self.second_factor);
        let reduction_factor = self.third_factor;

        // If period frequency is 0, return cliff fee
        if period_frequency == 0 {
          return self.cliff_fee_numerator;
        }

        // Determine current point based on activation type
        let current_point = match self.activation_type {
          // Slot-based timing
          0 => *central_context.current_slot.read().unwrap(),
          // Timestamp-based timing (in seconds)
          1 => SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
          // Default to slot-based
          _ => *central_context.current_slot.read().unwrap(),
        };

        // Check if we've reached the activation point
        if current_point < self.activation_point {
          return self.cliff_fee_numerator;
        }

        let periods_passed = cmp::min(
          number_of_period as u64,
          (current_point - self.activation_point) / period_frequency,
        );

        match self.base_fee_mode {
          0 => {
            // Linear: cliff_fee_numerator - (period * reduction_factor)
            let reduction = periods_passed * reduction_factor;
            if reduction >= self.cliff_fee_numerator {
              0
            } else {
              self.cliff_fee_numerator - reduction
            }
          }
          1 => {
            // Exponential: cliff_fee_numerator * (1 - reduction_factor/BASIS_POINT_MAX)^period
            let reduction_rate = reduction_factor as f64 / 10000f64;
            let decay_factor = (1.0 - reduction_rate).powi(periods_passed as i32);
            (self.cliff_fee_numerator as f64 * decay_factor) as u64
          }
          _ => unreachable!(),
        }
      }
      2 => {
        // Rate Limiter mode - not implemented yet
        // Would need to decode: fee_increment_bps, max_limiter_duration, max_fee_bps, reference_amount
        // For now, return cliff fee
        self.cliff_fee_numerator
      }
      // Default to cliff fee for unknown modes
      _ => self.cliff_fee_numerator,
    }
  }
}
