use crate::types::meteora_dammv2_pool::MeteoraDammV2Pool;
use std::cmp;

impl MeteoraDammV2Pool {
  /// Calculate the dynamic fee numerator based on volatility
  ///
  /// The dynamic fee increases with volatility according to the formula:
  /// `((volatility_accumulator * bin_step)^2 * variable_fee_control + 99_999_999_999) / 100_000_000_000`
  ///
  /// Returns 0 if dynamic fees are not initialized or variable fee control is disabled.
  pub fn calculate_dynamic_fee_numerator(&self) -> u64 {
    // If dynamic fee is not initialized or variable fee control is 0, return 0
    if self.initialized == 0 || self.variable_fee_control == 0 {
      return 0;
    }

    // dynamic_fee_numerator = ((volatility_accumulator * bin_step)^2 * variable_fee_control + 99_999_999_999) / 100_000_000_000
    let volatility_bin_product = self.volatility_accumulator * self.bin_step as u128;
    let square_vfa_bin = volatility_bin_product * volatility_bin_product;

    // Handle potential overflow by using saturating operations
    let v_fee = square_vfa_bin.saturating_mul(self.variable_fee_control as u128);
    let dynamic_fee_numerator = (v_fee + 99_999_999_999) / 100_000_000_000;

    // Convert to u64, capping at max value if necessary
    cmp::min(dynamic_fee_numerator, u64::MAX as u128) as u64
  }
}
