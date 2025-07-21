use crate::protocol_idls::meteora::MeteoraDammv2PoolIdl;
use crate::types::meteora_dammv2_pool::MeteoraDammV2Pool;
use crate::types::pool::Pool;
use crate::types::pools::Pools;
use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;

impl MeteoraDammV2Pool {
  pub fn from_account_info(pubkey: Pubkey, account_buffer: &[u8]) -> Self {
    let decoded_layout: MeteoraDammv2PoolIdl =
      MeteoraDammv2PoolIdl::try_from_slice(account_buffer).unwrap();

    Self {
      info: Pool {
        pool_address: pubkey,
        token_a_address: decoded_layout.token_a_mint,
        token_b_address: decoded_layout.token_b_mint,
        pool_type: Pools::MeteoraDammV2,
        token_a_vault_address: decoded_layout.token_a_vault,
        token_b_vault_address: decoded_layout.token_b_vault,
      },
      cliff_fee_numerator: decoded_layout.pool_fees.base_fee.cliff_fee_numerator,
      fee_scheduler_mode: decoded_layout.pool_fees.base_fee.fee_scheduler_mode,
      number_of_period: decoded_layout.pool_fees.base_fee.number_of_period,
      period_frequency: decoded_layout.pool_fees.base_fee.period_frequency,
      reduction_factor: decoded_layout.pool_fees.base_fee.reduction_factor,
      protocol_fee_percent: decoded_layout.pool_fees.protocol_fee_percent,
      partner_fee_percent: decoded_layout.pool_fees.partner_fee_percent,
      referral_fee_percent: decoded_layout.pool_fees.referral_fee_percent,
      initialized: decoded_layout.pool_fees.dynamic_fee.initialized,
      variable_fee_control: decoded_layout.pool_fees.dynamic_fee.variable_fee_control,
      bin_step: decoded_layout.pool_fees.dynamic_fee.bin_step,
      volatility_accumulator: decoded_layout.pool_fees.dynamic_fee.volatility_accumulator,

      activation_point: decoded_layout.activation_point,
      activation_type: decoded_layout.activation_type,
      liquidity: decoded_layout.liquidity,
      sqrt_price: decoded_layout.sqrt_price,
      collect_fee_mode: decoded_layout.collect_fee_mode,
      protocol_a_fee: decoded_layout.protocol_a_fee,
      protocol_b_fee: decoded_layout.protocol_b_fee,
      partner_a_fee: decoded_layout.partner_a_fee,
      partner_b_fee: decoded_layout.partner_b_fee,
      sqrt_max_price: decoded_layout.sqrt_max_price,
      sqrt_min_price: decoded_layout.sqrt_min_price,
    }
  }
}
