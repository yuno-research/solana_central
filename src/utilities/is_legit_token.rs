use crate::CentralContext;
use crate::constants::SOLANA_PROGRAMS;
use solana_sdk::pubkey::Pubkey;
use std::sync::Arc;

/// Check if a token is legitimate based on its Metaplex metadata
///
/// A token is considered legit if:
/// 1. It has Metaplex metadata
/// 2. The update authority matches a known legitimate launchpad (Raydium Launchpad or Pumpfun)
///
/// Results are cached in the central context for performance.
pub fn is_legit_token(token_address: &Pubkey, central_context: &Arc<CentralContext>) -> bool {
  let mut legit_tokens = central_context.legit_tokens.lock().unwrap();
  if let Some(legit_token) = legit_tokens.get(token_address) {
    return *legit_token;
  }

  // Derive metaplex pda for token
  let (metaplex_pda, _) = Pubkey::find_program_address(
    &[
      b"metadata",
      SOLANA_PROGRAMS.metaplex_program.as_ref(),
      token_address.as_ref(),
    ],
    &SOLANA_PROGRAMS.metaplex_program,
  );

  // Get account data for metaplex pda
  if let Ok(metaplex_pda_data) = central_context.json_rpc_client.get_account(&metaplex_pda) {
    let metaplex_pda_data = metaplex_pda_data.data;
    // Must have discriminator and update authority
    if metaplex_pda_data.len() < 33 {
      return false;
    }
    let update_authority = Pubkey::new_from_array(metaplex_pda_data[1..33].try_into().unwrap());
    // Cache and return result
    let legit = central_context
      .legit_update_authorities
      .contains(&update_authority);
    legit_tokens.insert(*token_address, legit);
    return legit;
  }
  /*
  Unsuccessful request for metaplex data for this token address, means its definitely not legit as
  all protocols store metadata in metaplex
  */
  false
}
