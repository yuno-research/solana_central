use crate::central_context::central_context::CentralContext;
use crate::meteora::get_meteora_vault_from_token_address::get_meteora_vault_from_token_address;
use crate::protocol_idls::meteora::MeteoraAmmPoolIdl;
use crate::types::meteora_amm_pool::MeteoraAmmPool;
use crate::types::pool::Pool;
use crate::types::pools::Pools;
use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;
use std::sync::Arc;

impl MeteoraAmmPool {
  pub fn from_account_info(
    pubkey: Pubkey,
    account_buffer: &Vec<u8>,
    central_context: Arc<CentralContext>,
  ) -> Self {
    let decoded_layout: MeteoraAmmPoolIdl =
      MeteoraAmmPoolIdl::try_from_slice(account_buffer).unwrap();
    let token_a_address: Pubkey = decoded_layout.token_a_mint;
    let token_b_address: Pubkey = decoded_layout.token_b_mint;
    let token_a_vault =
      get_meteora_vault_from_token_address(&token_a_address, central_context.clone());
    let token_b_vault =
      get_meteora_vault_from_token_address(&token_b_address, central_context.clone());
    let token_a_vault_address = token_a_vault.read().unwrap().vault;
    let token_b_vault_address = token_b_vault.read().unwrap().vault;

    Self {
      info: Pool {
        pool_address: pubkey,
        token_a_address,
        token_b_address,
        pool_type: Pools::MeteoraAmm,
        token_a_vault_address,
        token_b_vault_address,
      },
      enabled: decoded_layout.enabled,
      token_a_vault,
      token_b_vault,
      token_a_fee_address: decoded_layout.protocol_token_a_fee,
      token_b_fee_address: decoded_layout.protocol_token_b_fee,
      token_a_lp_vault_address: decoded_layout.a_vault_lp,
      token_b_lp_vault_address: decoded_layout.b_vault_lp,
      trade_fee_numerator: decoded_layout.fees.trade_fee_numerator as u128,
      trade_fee_denominator: decoded_layout.fees.trade_fee_denominator as u128,
      protocol_trade_fee_numerator: decoded_layout.fees.protocol_trade_fee_numerator as u128,
      protocol_trade_fee_denominator: decoded_layout.fees.protocol_trade_fee_denominator as u128,

      /*
      Going to leave these as dummy values for now, if we need to actually fetch them we will fetch
      them.
      */
      token_a_lp_amount: 0,
      token_b_lp_amount: 0,
    }
  }
}
