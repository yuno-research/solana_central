use solana_sdk::pubkey::Pubkey;

/// Used to deliver high frequency market price updates without having to do read locks on the pool
/// struct that holds the current market state. It's going to be more meant to be used for
/// "pushing" market updates throughout the system.
#[repr(C)]
#[derive(Clone, Debug)]
pub struct MarketUpdate {
  pub market_address: Pubkey,
  pub token_a_address: Pubkey,
  pub token_b_address: Pubkey,
  pub price_a_b: u128,
  pub price_b_a: u128,
}
