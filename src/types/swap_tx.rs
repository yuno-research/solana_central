use crate::types::pools::Pools;
use crate::types::swap_direction::SwapDirection;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct SwapTx {
  pub pool: Pools,
  pub direction: SwapDirection,
  pub block_time: u64,
  pub slot: u64,
  pub index: u64,
  pub atomic_instruction_index: u8,
  /*
  The fraction of the fee paid in lamports. So if the fee is 0.25%, then that fraction of 0.0025 * 10^9 = 2500000
  */
  pub fee_fraction_lp: u64,

  // If swap is from A to B, then this is in terms of A tokens spent in
  pub swapped_amount_in: u64,
  // If swap is from A to b, then this is in terms fo B tokens recieved
  pub swapped_amount_received: u64,
  // Quantity of token in vault AFTER swap is completed
  pub pool_token_a_vault_amount: u64,
  pub pool_token_b_vault_amount: u64,
  pub price_a_b_lp: u128,
  pub price_b_a_lp: u128,

  pub token_a_address: Pubkey,
  pub token_b_address: Pubkey,
  pub market_address: Pubkey,
  pub signature: Signature,
  /*
  A hash set of all the signers of the tx that encapsulates this swap. A tx on solana can have
  multiple signers in it.
  */
  pub signers: HashSet<Pubkey>,
}
