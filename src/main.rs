mod central_context;
mod constants;
mod protocol_idls;
mod pumpswap;
mod raydium;
mod types;
mod utilities;

use crate::central_context::central_context::CentralContext;
use crate::constants::PUMP_CONSTANTS;
use crate::constants::{METEORA_CONSTANTS, POOLS_ACCOUNT_SIZES, RAYDIUM_CONSTANTS};
use crate::pumpfun::derive_bonding_curve::derive_bonding_curve;
use crate::types::pf_bonding_curve::PfBondingCurve;
use crate::types::pool::PoolTrait;
use bs58;
use dotenv::dotenv;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use std::sync::Arc;

mod pumpfun;

mod meteora;

#[tokio::main]
async fn main() {
  dotenv().ok();
  let mut context: CentralContext = CentralContext::new();
  context.load_cpmm_pool_configs();
  let context = Arc::new(context);
  let protocols_to_load = [
    (
      PUMP_CONSTANTS.pump_swap_program,
      POOLS_ACCOUNT_SIZES.pump_swap,
    ),
    (
      METEORA_CONSTANTS.amm_program,
      POOLS_ACCOUNT_SIZES.meteora_amm,
    ),
    (
      METEORA_CONSTANTS.dammv2_program,
      POOLS_ACCOUNT_SIZES.meteora_dammv2,
    ),
    (
      RAYDIUM_CONSTANTS.amm_program,
      POOLS_ACCOUNT_SIZES.raydium_ammv4,
    ),
    (
      RAYDIUM_CONSTANTS.cpmm_program,
      POOLS_ACCOUNT_SIZES.raydium_cpmm,
    ),
  ];
  let token_address = Pubkey::from_str_const("ERHiB4WJQX1WQXc88hXXcie3uCStYH5Uzz3MPJW4rwKe");
  let bonding_curve_address = derive_bonding_curve(&token_address);
  let pf_bonding_curve = PfBondingCurve::from_account_info(
    token_address,
    &context
      .json_rpc_client
      .get_account(&bonding_curve_address)
      .unwrap()
      .data,
  );
  println!("{:?}", pf_bonding_curve);
  // load_pools(&protocols_to_load, Arc::from(context.clone()), 20).await;
}
