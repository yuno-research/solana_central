use crate::central_context::central_context::CentralContext;
use crate::constants::{LAMPORTS_PER_SOL, PUMP_CONSTANTS};
use crate::types::pool::Pool;
use crate::types::pool::PoolTrait;
use crate::types::pools::Pools;
use solana_sdk::pubkey::{Pubkey, PubkeyError};
use std::any::Any;
use std::sync::Arc;

#[derive(Debug)]
pub struct PumpswapPool {
  pub info: Pool,
  /*
  pool_creator determines canonical status by comparing against pumpPoolAuthorityPda
  coin_creator receives creator fees (original token creator from bonding curve)
  */
  pub pool_creator: Pubkey,
  pub coin_creator: Pubkey,
  /*
  There are multiple possible fee vaults for pumpswap, we will pick one at random and record which
  one we decide to use in this pool
  */
  pub fee_vault: Pubkey,
  /*
  All fees are collected in the QUOTE token for pumpswap. We will have our token A be base so the
  associated token account between the fee vault and the base token is the fee recipient
  All fees are collected in the QUOTE token (Token B) for pumpswap.
  Token A = base token (meme token), Token B = quote token (WSOL/SOL)
  */
  pub fee_vault_token_account: Pubkey,
  pub token_a_vault_amount: u64,
  pub token_b_vault_amount: u64,
  pub coin_creator_vault_authority: Pubkey,
  pub coin_creator_vault_authority_token_account: Pubkey,
}

impl PumpswapPool {
  /*
  Derives the pump pool authority PDA for a given base mint.
  This matches the TypeScript function: pumpPoolAuthorityPda(baseMint)
  Seeds: ["pool_authority", base_mint]
  */
  pub fn pump_pool_authority_pda(base_mint: &Pubkey) -> Result<Pubkey, PubkeyError> {
    let (pubkey, _) = Pubkey::find_program_address(
      &[b"pool_authority", base_mint.as_ref()],
      &PUMP_CONSTANTS.pump_swap_program,
    );
    Ok(pubkey)
  }

  /*
  Determines if this pool is canonical by checking if pool_creator matches
  the derived pump pool authority PDA for the base mint.
  */
  pub fn is_canonical_pool(&self) -> bool {
    if let Ok(pump_authority_pda) = Self::pump_pool_authority_pda(&self.info.token_a_address) {
      self.pool_creator == pump_authority_pda
    } else {
      false
    }
  }

  /*
  Calculates the market cap in lamports for canonical pools using:
  market_cap = quote_reserve * base_mint_supply / base_reserve

  For canonical pools: base_mint_supply is always 1 billion (graduated from bonding curve)
  For non-canonical pools: fetches supply from RPC
  */
  pub fn calculate_market_cap_lamports(&self) -> Option<u64> {
    // Avoid division by zero
    if self.token_a_vault_amount == 0 {
      return None;
    }

    // For canonical pools, base mint supply is always 1 billion (bonding curve graduation)
    let base_mint_supply = if self.is_canonical_pool() {
      1_000_000_000_u64 // Constant 1 billion for canonical pools
    } else {
      // For non-canonical pools, we would need to fetch from RPC
      // But for now, assume standard supply or return None
      return None; // Non-canonical pools don't use market cap for fees anyway
    };

    // Calculate market cap: quote_reserve * base_mint_supply / base_reserve
    let market_cap_lamports = (self.token_b_vault_amount as u128)
      .saturating_mul(base_mint_supply as u128)
      .saturating_div(self.token_a_vault_amount as u128) as u64;

    Some(market_cap_lamports)
  }

  /*
  Returns the fee breakdown for canonical pools: (creator_fee, protocol_fee, lp_fee)
  All values in lamports per SOL (basis points * 10_000)
  */
  pub fn get_canonical_fee_breakdown(&self) -> (u64, u64, u64) {
    let market_cap_lamports = match self.calculate_market_cap_lamports() {
      Some(market_cap) => market_cap,
      None => return (3000000, 9300000, 200000), // Default to 1.250% breakdown
    };

    // Fee breakdown based on market cap (Creator, Protocol, LP)
    match market_cap_lamports {
      0..=420_000_000_000 => (3_000_000, 9_300_000, 200_000), // 0.30%, 0.93%, 0.02%
      420_000_000_001..=1_470_000_000_000 => (9_500_000, 500_000, 2_000_000), // 0.95%, 0.05%, 0.20%
      1_470_000_000_001..=2_460_000_000_000 => (9_000_000, 500_000, 2_000_000), // 0.90%, 0.05%, 0.20%
      2_460_000_000_001..=3_440_000_000_000 => (8_500_000, 500_000, 2_000_000), // 0.85%, 0.05%, 0.20%
      3_440_000_000_001..=4_420_000_000_000 => (8_000_000, 500_000, 2_000_000), // 0.80%, 0.05%, 0.20%
      4_420_000_000_001..=9_820_000_000_000 => (7_500_000, 500_000, 2_000_000), // 0.75%, 0.05%, 0.20%
      9_820_000_000_001..=14_740_000_000_000 => (7_000_000, 500_000, 2_000_000), // 0.70%, 0.05%, 0.20%
      14_740_000_000_001..=19_650_000_000_000 => (6_500_000, 500_000, 2_000_000), // 0.65%, 0.05%, 0.20%
      19_650_000_000_001..=24_560_000_000_000 => (6_000_000, 500_000, 2_000_000), // 0.60%, 0.05%, 0.20%
      24_560_000_000_001..=29_470_000_000_000 => (5_500_000, 500_000, 2_000_000), // 0.55%, 0.05%, 0.20%
      29_470_000_000_001..=34_380_000_000_000 => (5_000_000, 500_000, 2_000_000), // 0.50%, 0.05%, 0.20%
      34_380_000_000_001..=39_300_000_000_000 => (4_500_000, 500_000, 2_000_000), // 0.45%, 0.05%, 0.20%
      39_300_000_000_001..=44_210_000_000_000 => (4_000_000, 500_000, 2_000_000), // 0.40%, 0.05%, 0.20%
      44_210_000_000_001..=49_120_000_000_000 => (3_500_000, 500_000, 2_000_000), // 0.35%, 0.05%, 0.20%
      49_120_000_000_001..=54_030_000_000_000 => (3_000_000, 500_000, 2_000_000), // 0.30%, 0.05%, 0.20%
      54_030_000_000_001..=58_940_000_000_000 => (2_750_000, 500_000, 2_000_000), // 0.275%, 0.05%, 0.20%
      58_940_000_000_001..=63_860_000_000_000 => (2_500_000, 500_000, 2_000_000), // 0.25%, 0.05%, 0.20%
      63_860_000_000_001..=68_770_000_000_000 => (2_250_000, 500_000, 2_000_000), // 0.225%, 0.05%, 0.20%
      68_770_000_000_001..=73_681_000_000_000 => (2_000_000, 500_000, 2_000_000), // 0.20%, 0.05%, 0.20%
      73_681_000_000_001..=78_590_000_000_000 => (1_750_000, 500_000, 2_000_000), // 0.175%, 0.05%, 0.20%
      78_590_000_000_001..=83_500_000_000_000 => (1_500_000, 500_000, 2_000_000), // 0.15%, 0.05%, 0.20%
      83_500_000_000_001..=88_400_000_000_000 => (1_250_000, 500_000, 2_000_000), // 0.125%, 0.05%, 0.20%
      88_400_000_000_001..=93_330_000_000_000 => (1_000_000, 500_000, 2_000_000), // 0.10%, 0.05%, 0.20%
      93_330_000_000_001..=98_240_000_000_000 => (750_000, 500_000, 2_000_000), // 0.075%, 0.05%, 0.20%
      _ => (500_000, 500_000, 2_000_000), // 0.05%, 0.05%, 0.20%
    }
  }

  /*
  Returns the fee breakdown: (creator_fee, protocol_fee, lp_fee)
  For canonical pools: Dynamic based on market cap
  For non-canonical pools: (0, 500_000, 2_500_000) = 0%, 0.05%, 0.25%
  */
  pub fn get_fee_breakdown(&self) -> (u64, u64, u64) {
    if self.is_canonical_pool() {
      self.get_canonical_fee_breakdown()
    } else {
      (0, 500_000, 2_500_000) // Non-canonical: 0% creator, 0.05% protocol, 0.25% LP
    }
  }
}

impl PoolTrait for PumpswapPool {
  fn token_a_amount_units(&self) -> u64 {
    self.token_a_vault_amount
  }

  fn token_b_amount_units(&self) -> u64 {
    self.token_b_vault_amount
  }

  fn pool_address(&self) -> &Pubkey {
    &self.info.pool_address
  }
  fn token_a_address(&self) -> &Pubkey {
    &self.info.token_a_address
  }
  fn token_b_address(&self) -> &Pubkey {
    &self.info.token_b_address
  }
  fn token_a_vault_address(&self) -> &Pubkey {
    &self.info.token_a_vault_address
  }
  fn token_b_vault_address(&self) -> &Pubkey {
    &self.info.token_b_vault_address
  }
  fn pool_type(&self) -> &Pools {
    &self.info.pool_type
  }
  /*
  PumpSwap uses two different fee structures:

  1. Canonical pools (migrated from pump.fun): Dynamic fees based on market cap
  2. Non-canonical pools (manually created): Fixed 0.30% total fee

  For canonical pools, fees range from 1.250% (0-420 SOL) to 0.300% (98240+ SOL)
  For non-canonical pools, fixed 0.300% total fee (no creator fees)
  */
  fn total_swap_fee_lp(&self, _central_context: &Arc<CentralContext>) -> u64 {
    // Get fee breakdown and sum them up
    let (creator_fee, protocol_fee, lp_fee) = self.get_fee_breakdown();
    creator_fee + protocol_fee + lp_fee
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }

  fn price_a_over_b_lp(&self) -> u128 {
    self.token_a_vault_amount as u128 * LAMPORTS_PER_SOL / self.token_b_vault_amount as u128
  }

  fn price_b_over_a_lp(&self) -> u128 {
    self.token_b_vault_amount as u128 * LAMPORTS_PER_SOL / self.token_a_vault_amount as u128
  }

  fn fetch_market_state_from_rpc(&mut self, central_context: &Arc<CentralContext>) {
    // Update token vaults fetch from rpc
    self.token_a_vault_amount = central_context
      .json_rpc_client
      .get_token_account_balance(&self.info.token_a_vault_address)
      .unwrap()
      .amount
      .parse()
      .unwrap();
    self.token_b_vault_amount = central_context
      .json_rpc_client
      .get_token_account_balance(&self.info.token_b_vault_address)
      .unwrap()
      .amount
      .parse()
      .unwrap();
  }
}

pub const PUMP_SWAP_FEE_VAULTS: [Pubkey; 8] = [
  Pubkey::from_str_const("62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV"),
  Pubkey::from_str_const("7VtfL8fvgNfhz17qKRMjzQEXgbdpnHHHQRh54R9jP2RJ"),
  Pubkey::from_str_const("7hTckgnGnLQR6sdH7YkqFTAA7VwTfYFaZ6EhEsU3saCX"),
  Pubkey::from_str_const("9rPYyANsfQZw3DnDmKE3YCQF5E8oD89UXoHn9JFEhJUz"),
  Pubkey::from_str_const("AVmoTthdrX6tKt4nDjco2D775W2YK3sDhxPcMmzUAmTY"),
  Pubkey::from_str_const("FWsW1xNtWscwNmKv6wVsU1iTzRN6wmmk3MjxRP5tT7hz"),
  Pubkey::from_str_const("G5UZAVbAf46s7cKWoyKu8kYTip9DGTpbLZ2qa9Aq69dP"),
  Pubkey::from_str_const("JCRGumoE9Qi5BBgULTgdgTLjSgkCMSbF62ZZfGs84JeU"),
];
