use crate::protocol_idls::meteora::MeteoraDammv2PoolIdl;
use crate::types::meteora_dammv2_pool::MeteoraDammV2Pool;
use crate::types::pool::Pool;
use crate::types::pools::Pools;
use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;

impl MeteoraDammV2Pool {
  /// Create a Meteora DAMMv2 pool from on-chain account data
  ///
  /// Parses the account buffer using the Meteora DAMMv2 pool IDL structure.
  /// DAMMv2 pools support dynamic fees based on volatility and time-based schedules.
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
      base_fee_mode: decoded_layout.pool_fees.base_fee.base_fee_mode,
      first_factor: decoded_layout.pool_fees.base_fee.first_factor,
      second_factor: decoded_layout.pool_fees.base_fee.second_factor,
      third_factor: decoded_layout.pool_fees.base_fee.third_factor,
      protocol_fee_percent: decoded_layout.pool_fees.protocol_fee_percent,
      referral_fee_percent: decoded_layout.pool_fees.referral_fee_percent,
      initialized: decoded_layout.pool_fees.dynamic_fee.initialized,
      variable_fee_control: decoded_layout.pool_fees.dynamic_fee.variable_fee_control,
      bin_step: decoded_layout.pool_fees.dynamic_fee.bin_step,
      last_update_timestamp: decoded_layout.pool_fees.dynamic_fee.last_update_timestamp,
      sqrt_price_reference: decoded_layout.pool_fees.dynamic_fee.sqrt_price_reference,
      volatility_accumulator: decoded_layout.pool_fees.dynamic_fee.volatility_accumulator,
      volatility_reference: decoded_layout.pool_fees.dynamic_fee.volatility_reference,
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
      version: decoded_layout.version,
    }
  }
}
