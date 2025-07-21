use crate::types::pool::PoolTrait;
use crate::types::pools::Pools;
use crate::types::swap_direction::SwapDirection;
use solana_sdk::signature::Signature;
use spl_associated_token_account::solana_program::pubkey::Pubkey;
use std::collections::HashSet;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct SwapTx {
  pub market_address: Pubkey,
  pub market: Arc<RwLock<dyn PoolTrait>>,
  pub swap_direction: SwapDirection,
  // If swap is from A to B, then this is in terms of A tokens spent in
  pub amount_in: u64,
  // If swap is from A to b, then this is in terms fo B tokens recieved
  pub amount_out: u64,
  pub index: u64,
  pub slot: u64,
  pub block_time: u64,
  pub signature: Signature,
  pub token_a_address: Pubkey,
  pub token_b_address: Pubkey,
  pub pool_type: Pools,
  /*
  A hash set of all the signers of the tx that encapsulates this swap. A tx on solana can have
  multiple signers in it.
  */
  pub signers: HashSet<Pubkey>,
}
