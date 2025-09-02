use crate::protocol_idls::meteora::{DbcPoolConfig, DbcVirtualPool};
use crate::types::meteora_dbc::ActivationType;
use crate::types::meteora_dbc::{BaseFeeMode, MeteoraDbc};
use crate::types::pool::Pool;
use crate::types::pools::Pools;
use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;

impl MeteoraDbc {
  pub fn from_account_info(
    pubkey: Pubkey,
    account_buffer: &[u8],
    dbc_pool_config: &DbcPoolConfig,
  ) -> Self {
    let dbc_virtual_pool = DbcVirtualPool::try_from_slice(account_buffer).unwrap();
    Self {
      pool: Pool {
        pool_address: pubkey,
        token_a_address: dbc_virtual_pool.base_mint,
        token_b_address: dbc_pool_config.quote_mint,
        token_a_vault_address: dbc_virtual_pool.base_vault,
        token_b_vault_address: dbc_virtual_pool.quote_vault,
        pool_type: Pools::MeteoraDbc,
      },
      
      config: pubkey,

      sqrt_price: dbc_virtual_pool.sqrt_price,

      cliff_fee_numerator: dbc_pool_config.pool_fees.base_fee.cliff_fee_numerator,
      base_fee_number_of_periods: dbc_pool_config.pool_fees.base_fee.first_factor,
      base_fee_period_frequency: dbc_pool_config.pool_fees.base_fee.second_factor,
      base_fee_reduction_factor: dbc_pool_config.pool_fees.base_fee.third_factor,

      base_fee_mode: if dbc_pool_config.pool_fees.base_fee.base_fee_mode == 0 {
        BaseFeeMode::Linear
      } else if dbc_pool_config.pool_fees.base_fee.base_fee_mode == 1 {
        BaseFeeMode::Exponential
      } else {
        BaseFeeMode::Linear
      },
      activation_point: dbc_virtual_pool.activation_point,
      volatility_accumulator: dbc_virtual_pool.volatility_tracker.volatility_accumulator,
      variable_fee_control: dbc_pool_config.pool_fees.dynamic_fee.variable_fee_control,
      activation_type: if dbc_pool_config.activation_type == 0 {
        ActivationType::Slot
      } else {
        ActivationType::Time
      },
    }
  }
}
