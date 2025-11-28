use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;

/// Struct to hold data for Meteora vaults, which are used in Meteora AMMv1 and potentially Meteora
/// DLMM pools.
pub struct MeteoraVault {
  // shows up as aVault/ A Vault
  pub vault: Pubkey,
  // A Token Vault, the actual token account where the token is stored
  pub vault_token_account: Pubkey,
  // The address of the token
  pub token_address: Pubkey,
  /*
  The address of the liquidity provider token for this token. Liquidity provider tokens are global
  across all of Meteora
  */
  pub lp_token_address: Pubkey,
  /*
  The following fields are information about the vault of this token needed to calculate the
  actual amount in a liquidity pool. Data is pulled directly off of the vault program through
  IDL and filled in with token account balances.
  */
  pub last_updated_locked_profit: u64,
  pub last_report: u64,
  pub locked_profit_degradation: u64,
  pub total_amount: u64,
  // The total amount of liquidity provider tokens in circulation
  pub lp_supply: u64,
}
