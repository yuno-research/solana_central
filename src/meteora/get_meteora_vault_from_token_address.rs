use crate::central_context::central_context::CentralContext;
use crate::constants::METEORA_CONSTANTS;
use crate::types::meteora_vault::MeteoraVault;
use solana_sdk::pubkey::Pubkey;
use std::sync::{Arc, RwLock};

pub fn get_meteora_vault_from_token_address(
  token_address: &Pubkey,
  central_context: Arc<CentralContext>,
) -> Arc<RwLock<MeteoraVault>> {
  // 1) If it’s already in the cache, just clone-and-return the Arc<Mutex<...>>
  let mut cache = central_context.meteora_vault_cache.lock().unwrap();
  if let Some(vault_arc) = cache.get(token_address) {
    return Arc::clone(vault_arc);
  }

  // 2) Otherwise derive all the PDAs
  let (vault, _) = Pubkey::find_program_address(
    &[
      b"vault",
      token_address.as_ref(),
      METEORA_CONSTANTS.vault_base_key.as_ref(),
    ],
    &METEORA_CONSTANTS.vault_program,
  );

  let (vault_token_account, _) = Pubkey::find_program_address(
    &[b"token_vault", vault.as_ref()],
    &METEORA_CONSTANTS.vault_program,
  );

  let (lp_token_address, _) = Pubkey::find_program_address(
    &[b"lp_mint", vault.as_ref()],
    &METEORA_CONSTANTS.vault_program,
  );

  // 3) Build the in-memory struct and wrap in Arc<Mutex<...>>
  let new_vault = MeteoraVault {
    vault,
    vault_token_account,
    token_address: *token_address,
    lp_token_address,
    // Data to be fetched later
    last_updated_locked_profit: 0,
    last_report: 0,
    locked_profit_degradation: 0,
    total_amount: 0,
    lp_supply: 0,
  };
  let vault_arc = Arc::new(RwLock::new(new_vault));

  // 4) Insert into cache and return
  cache.insert(*token_address, Arc::clone(&vault_arc));

  vault_arc
}
