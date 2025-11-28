use crate::CentralContext;
use crate::types::pools::Pools;
use crate::types::swap_direction::SwapDirection;
use solana_sdk::pubkey::Pubkey;
use std::any::Any;
use std::sync::Arc;

/// Base pool information shared by all pool types. Contains the essential addresses and
/// identifiers that every pool implementation requires. Protocol-specific pool structs will embed
/// this as an `info` field.
#[derive(Debug)]
pub struct Pool {
  pub pool_address: Pubkey,
  pub token_a_address: Pubkey,
  pub token_b_address: Pubkey,
  pub token_a_vault_address: Pubkey,
  pub token_b_vault_address: Pubkey,
  pub pool_type: Pools,
}

/// Trait for all pool types, providing a unified interface for DEX pools. All pools implement this
/// trait to provide common functionality like price queries, fee calculations, and reserve
/// lookups.
pub trait PoolTrait: Any + Send + Sync {
  /// Get the pool's on-chain address
  fn pool_address(&self) -> &Pubkey;
  /// Get the address of token A
  fn token_a_address(&self) -> &Pubkey;
  /// Get the address of token B
  fn token_b_address(&self) -> &Pubkey;
  /// Get the vault address for token A
  fn token_a_vault_address(&self) -> &Pubkey;
  /// Get the vault address for token B
  fn token_b_vault_address(&self) -> &Pubkey;
  /// Get the pool type enum variant
  fn pool_type(&self) -> &Pools;
  
  /// Get the total swap fee in lamports (10^9 lamports = 1 SOL)
  fn total_swap_fee_lp(&self, central_context: &Arc<CentralContext>) -> u64;

  /// Get a reference to the pool as `Any` for type downcasting
  fn as_any(&self) -> &dyn Any;
  /// Get a mutable reference to the pool as `Any` for type downcasting
  fn as_any_mut(&mut self) -> &mut dyn Any;

  /// Calculate the price of token A in terms of token B. Fx ticker equivalent: B/A. Returns how
  /// many units of token A are needed to buy 1 unit of token B, in lamports. For AMMs, typically
  /// calculated as: (A reserves * LAMPORTS_PER_SOL) / B reserves
  fn price_a_over_b_lp(&self) -> u128;

  /// Calculate the price of token B in terms of token A. Fx ticker equivalent: A/B. Returns how
  /// many units of token A are needed to buy 1 unit of token B, in lamports. For AMMs, typically
  /// calculated as: (B reserves * LAMPORTS_PER_SOL) / A reserves. Inverse of `price_a_over_b_lp`.
  fn price_b_over_a_lp(&self) -> u128;

  /// Fetch and update pool state from JSON RPC and immediately overrides the in-memory pool state.
  /// Should not be used in production in favor of using gRPC streams.
  fn fetch_market_state_from_rpc(&mut self, central_context: &Arc<CentralContext>);

  /// Get the actual amount of token A in the pool, in token units. This is calculated dynamically
  /// because some protocols (like Meteora) derive real token balances from LP token balances
  /// rather than storing them directly in the pool account, and other protocols require excluding
  /// collected fees from raw reserves.
  fn token_a_amount_units(&self) -> u64;
  
  /// Get the actual amount of token B in the pool, in token units. See `token_a_amount_units`
  /// for details on why this is calculated dynamically.
  fn token_b_amount_units(&self) -> u64;

  /// Get directional swap fees as fractions for a given swap direction
  ///
  /// Returns `(fee_a_fraction, fee_b_fraction)` where each value is between 0.0 and 1.0
  /// (e.g., 0.003 = 0.3%). The fees may differ based on swap direction for protocols
  /// with asymmetric fee structures.
  /// * `central_context` with updated current slot value - Needed for time-based fee calculations
  /// in Meteora DAMMv2 and DBC
  fn directional_fees(
    &self,
    direction: SwapDirection,
    central_context: &Arc<CentralContext>,
  ) -> (f64, f64);
}
