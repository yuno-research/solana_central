use crate::constants::PUMP_CONSTANTS;
use solana_sdk::pubkey::Pubkey;

pub fn derive_bonding_curve(token_address: &Pubkey) -> Pubkey {
  let (pda, _) = Pubkey::find_program_address(
    &[b"bonding-curve", token_address.as_ref()],
    &PUMP_CONSTANTS.bonding_curve_program,
  );
  pda
}
