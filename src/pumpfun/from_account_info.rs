use crate::constants::PUMP_CONSTANTS;
use crate::protocol_idls::pumpfun::PfBondingCurveIdl;
use crate::pumpfun::derive_bonding_curve::derive_bonding_curve;
use crate::types::pf_bonding_curve::PfBondingCurve;
use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;
use spl_associated_token_account::get_associated_token_address;

impl PfBondingCurve {
  /**
  This function takes in the TOKEN ADDRESS that the bonding curve is for, as well as the raw bonding
  curve data. When you call the this function, fetch the data for the derived bonding curve address
  from the token address using derive_bonding_curve.
  
  USAGE:
  ```rust
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
  ```
  */
  pub fn from_account_info(token_address: Pubkey, account_buffer: &[u8]) -> Self {
    let decoded_layout = PfBondingCurveIdl::try_from_slice(&account_buffer[..150])
      .expect("Failed to deserialize Pf Bonding Curve account");

    let (creator_vault_address, _) = Pubkey::find_program_address(
      &[b"creator-vault", decoded_layout.creator.as_array()],
      &PUMP_CONSTANTS.bonding_curve_program,
    );
    let bonding_curve_address = derive_bonding_curve(&token_address);

    Self {
      virtual_sol_reserves: decoded_layout.virtual_sol_reserves,
      virtual_token_reserves: decoded_layout.virtual_token_reserves,
      complete: decoded_layout.complete,
      token_address,
      bonding_curve_address,
      bonding_curve_associated_token_account_address: get_associated_token_address(
        &bonding_curve_address,
        &token_address,
      ),
      creator_vault_address,
    }
  }
}
