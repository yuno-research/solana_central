use crate::protocol_idls::pumpfun::PfBondingCurveIdl;
use crate::types::pf_bonding_curve::PfBondingCurve;
use borsh::BorshDeserialize;

impl PfBondingCurve {
  /**
  Takes the raw data for the account info and updates the bonding curve state by writing it in
  directly.
  */
  pub fn update_state_from_data(&mut self, account_buffer: &Vec<u8>) {
    // Deserialize the account buffer into a BondingCurveAccount
    let decoded_layout: PfBondingCurveIdl =
      PfBondingCurveIdl::try_from_slice(account_buffer).unwrap();
    // Update the bonding curve state
    self.sol_reserves = decoded_layout.virtual_sol_reserves;
    self.token_reserves = decoded_layout.virtual_token_reserves;
    self.complete = decoded_layout.complete;
  }
}
