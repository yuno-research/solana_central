use crate::central_context::central_context::CentralContext;
use crate::types::meteora_dammv2_pool::MeteoraDammV2Pool;
use std::cmp;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

impl MeteoraDammV2Pool {
  pub fn calculate_base_fee_numerator(&self, central_context: &Arc<CentralContext>) -> u64 {
    // If period frequency is 0 or we haven't reached activation point, return cliff fee
    if self.period_frequency == 0 {
      return self.cliff_fee_numerator;
    }

    // Determine current point based on activation type
    let current_point = match self.activation_type {
      // Slot-based timing
      0 => *central_context.current_slot.read().unwrap(),
      // Timestamp-based timing
      1 => SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64,
      // Default to slot-based
      _ => *central_context.current_slot.read().unwrap(),
    };

    // Check if we've reached the activation point
    if current_point < self.activation_point {
      return self.cliff_fee_numerator;
    }

    let periods_passed = cmp::min(
      self.number_of_period as u64,
      (current_point - self.activation_point) / self.period_frequency,
    );

    match self.fee_scheduler_mode {
      0 => {
        // Linear: cliff_fee_numerator - (period * reduction_factor)
        let reduction = periods_passed * self.reduction_factor;
        if reduction >= self.cliff_fee_numerator {
          0
        } else {
          self.cliff_fee_numerator - reduction
        }
      }
      1 => {
        // Exponential: cliff_fee_numerator * (1 - reduction_factor/BASIS_POINT_MAX)^period
        let reduction_rate = self.reduction_factor as f64 / 10000f64;
        let decay_factor = (1.0 - reduction_rate).powi(periods_passed as i32);
        (self.cliff_fee_numerator as f64 * decay_factor) as u64
      }
      // Default to cliff fee for unknown modes
      _ => self.cliff_fee_numerator,
    }
  }
}
