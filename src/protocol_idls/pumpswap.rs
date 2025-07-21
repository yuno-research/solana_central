use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;

/*
Anchor IDL extracted representation of the PumpSwap pool account. Only the subset of fields that we
actually need to reconstruct our high-level `PumpswapPool` type are included here. The layout order
MUST match the on-chain structure so that `BorshDeserialize` can succeed.
*/
#[derive(BorshDeserialize)]
pub struct PumpAmmPoolAccount {
  pub padding1: u64,
  pub pool_bump: u8,
  pub index: u16,
  pub creator: Pubkey,
  pub base_mint: Pubkey,
  pub quote_mint: Pubkey,
  pub lp_mint: Pubkey,
  pub pool_base_token_account: Pubkey,
  pub pool_quote_token_account: Pubkey,
  pub lp_supply: u64,
  pub coin_creator: Pubkey,
  pub padding: [u8; 57],
}

#[derive(BorshDeserialize)]
pub struct PumpAmmCreatePoolInstructionDataIdl {
  pub discriminator: [u8; 8],
  pub index: u16,
  pub base_amount_in: u64,
  pub quote_amount_in: u64,
  pub coin_creator: Pubkey,
}

#[derive(BorshDeserialize)]
pub struct PumpswapBuyEventIdl {
  pub discriminator: [u8; 16],
  pub timestamp: i64,
  pub base_amount_out: u64,
  pub max_quote_amount_in: u64,
  pub user_base_token_reserves: u64,
  pub user_quote_token_reserves: u64,
  pub pool_base_token_reserves: u64,
  pub pool_quote_token_reserves: u64,
  pub quote_amount_in: u64,
  pub lp_fee_basis_points: u64,
  pub lp_fee: u64,
  pub protocol_fee_basis_points: u64,
  pub protocol_fee: u64,
  pub quote_amount_in_with_lp_fee: u64,
  pub user_quote_amount_in: u64,
  pub pool: Pubkey,
  pub user: Pubkey,
  pub user_base_token_account: Pubkey,
  pub user_quote_token_account: Pubkey,
  pub protocol_fee_recipient: Pubkey,
  pub protocol_fee_recipient_token_account: Pubkey,
  pub coin_creator: Pubkey,
  pub coin_creator_fee_basis_points: u64,
  pub coin_creator_fee: u64,
}


#[derive(BorshDeserialize)]
pub struct PumpswapSellEventIdl {
  pub discriminator: [u8; 16],
  pub timestamp: i64,
  pub base_amount_in: u64,
  pub min_quote_amount_out: u64,
  pub user_base_token_reserves: u64,
  pub user_quote_token_reserves: u64,
  pub pool_base_token_reserves: u64,
  pub pool_quote_token_reserves: u64,
  pub quote_amount_out: u64,
  pub lp_fee_basis_points: u64,
  pub lp_fee: u64,
  pub protocol_fee_basis_points: u64,
  pub protocol_fee: u64,
  pub quote_amount_out_without_lp_fee: u64,
  pub user_quote_amount_out: u64,
  pub pool: Pubkey,
  pub user: Pubkey,
  pub user_base_token_account: Pubkey,
  pub user_quote_token_account: Pubkey,
  pub protocol_fee_recipient: Pubkey,
  pub protocol_fee_recipient_token_account: Pubkey,
  pub coin_creator: Pubkey,
  pub coin_creator_fee_basis_points: u64,
  pub coin_creator_fee: u64,
}

