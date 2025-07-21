use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;

#[derive(BorshDeserialize)]
pub struct PfBondingCurveIdl {
  pub discriminator: [u8; 8],
  pub virtual_token_reserves: u64,
  pub virtual_sol_reserves: u64,
  pub real_token_reserves: u64,
  pub real_sol_reserves: u64,
  pub token_total_supply: u64,
  pub complete: bool,
  pub creator: Pubkey,
  // 150 account size total, 81 bytes used up
  padding: [u8; 69],
}

#[derive(BorshDeserialize)]
pub struct PfTradeEventIdl {
  // 16 byte discriminator
  pub padding: [u8; 16],
  pub mint: Pubkey,
  pub sol_amount: u64,
  pub token_amount: u64,
  pub is_buy: bool,
  pub user: Pubkey,
  pub timestamp: i64,
  pub virtual_sol_reserves: u64,
  pub virtual_token_reserves: u64,
  pub real_sol_reserves: u64,
  pub real_token_reserves: u64,
  pub fee_recipient: Pubkey,
  pub fee_basis_points: u64,
  pub fee: u64,
  pub creator: Pubkey,
  pub creator_fee_basis_points: u64,
  pub creator_fee: u64,
}
