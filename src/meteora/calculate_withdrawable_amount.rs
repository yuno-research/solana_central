use crate::constants::METEORA_CONSTANTS;
use crate::types::meteora_vault::MeteoraVault;
use std::time::{SystemTime, UNIX_EPOCH};

impl MeteoraVault {
  /// Calculate the withdrawable amount from a Meteora vault
  ///
  /// Accounts for locked profit degradation over time. The locked profit gradually
  /// becomes available based on the degradation rate and time since last report.
  pub fn calculate_withdrawable_amount(&self) -> u64 {
    let current_time: u128 = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_millis();
    let duration: u128 = current_time - self.last_report as u128;
    let locked_fund_ratio: u128 = duration * self.locked_profit_degradation as u128;
    if locked_fund_ratio > METEORA_CONSTANTS.locked_profit_degradation_denominator {
      return self.total_amount;
    }
    let locked_profit: u128 = (self.last_updated_locked_profit as u128)
      * (METEORA_CONSTANTS.locked_profit_degradation_denominator - locked_fund_ratio)
      / METEORA_CONSTANTS.locked_profit_degradation_denominator;
    self.total_amount - locked_profit as u64
  }
}
