use crate::central_context::central_context::CentralContext;
use crate::constants::SOLANA_PROGRAMS;
use crate::types::pool::PoolTrait;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

impl CentralContext {
  /**
  Used to add a newly picked up and processed pool into the centrla context so that it can be queried
  from various places like the main markets graph, and also by the token account address to pool
  hashmap
  */
  pub fn insert_pool(&self, pool: Arc<RwLock<dyn PoolTrait>>) {
    let result_unlocked = pool.read().unwrap();
    let token_a_address: &solana_sdk::pubkey::Pubkey = result_unlocked.token_a_address();
    let token_b_address = result_unlocked.token_b_address();
    // Token addresses are all 0/garbage, throw this pool away
    if *token_a_address == SOLANA_PROGRAMS.system_program
      || *token_b_address == SOLANA_PROGRAMS.system_program
    {
      return;
    }
    // Token addresses are the same, throw this pool away
    if token_a_address == token_b_address {
      return;
    }
    // println!("load_pools: processing pool {}", result_unlocked.pool_address());
    // Aquire a write lock on markets
    let mut markets = self.markets.write().unwrap();

    // Ensure both tokens exist in the markets map
    markets
      .entry(token_a_address.clone())
      .or_insert_with(|| HashMap::new());
    markets
      .entry(token_b_address.clone())
      .or_insert_with(|| HashMap::new());

    // Get references to both market maps
    let markets_a = markets.get_mut(token_a_address).unwrap();
    
    

    // Check if market pair already exists
    if let Some(existing_markets) = markets_a.get(token_b_address) {
      existing_markets.write().unwrap().push(pool.clone());
    } else {
      // Create new market pair
      let a_b_markets = Arc::new(RwLock::new(vec![pool.clone()]));
      // Insert bidirectional mapping
      markets_a.insert(token_b_address.clone(), a_b_markets.clone());
      let markets_b = markets.get_mut(token_b_address).unwrap();
      markets_b.insert(token_a_address.clone(), a_b_markets);
    }

    // Get a write lock for the token accounts and insert this pool there
    let mut token_accounts_map = self.pools_map.write().unwrap();
    token_accounts_map.insert(*result_unlocked.token_a_vault_address(), pool.clone());
    token_accounts_map.insert(*result_unlocked.token_b_vault_address(), pool.clone());
    token_accounts_map.insert(*result_unlocked.pool_address(), pool.clone());
  }
}
