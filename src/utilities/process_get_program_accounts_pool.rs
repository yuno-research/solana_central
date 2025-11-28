use crate::CentralContext;
use crate::constants::{METEORA_CONSTANTS, POOLS_ACCOUNT_SIZES, PUMP_CONSTANTS, RAYDIUM_CONSTANTS};
use crate::types::meteora_amm_pool::MeteoraAmmPool;
use crate::types::meteora_dammv2_pool::MeteoraDammV2Pool;
use crate::types::pool::PoolTrait;
use crate::types::pumpswap_pool::PumpswapPool;
use crate::types::raydium_ammv4_pool::RaydiumAmmV4Pool;
use crate::types::raydium_cpmm_pool::RaydiumCpmmPool;
use solana_sdk::account::Account;
use solana_sdk::pubkey::Pubkey;
use std::sync::{Arc, RwLock};

/// Process raw account data from getProgramAccounts into pool instances. Identifies pools by their
/// program owner and account size, then parses them into the appropriate pool type (Meteora,
/// Raydium, Pumpswap, etc.). Designed to be called from multiple threads with different slice
/// ranges.
pub fn process_get_program_accounts_pool(
  raw_accounts: Arc<Vec<(Pubkey, Account)>>,
  central_context: Arc<CentralContext>,
  start: usize,
  end: usize,
) -> Vec<Arc<RwLock<dyn PoolTrait>>> {
  let raw_accounts = &raw_accounts.as_ref()[start..end];
  let mut results: Vec<Arc<RwLock<dyn PoolTrait>>> = Vec::new();
  for (pubkey, account) in raw_accounts {
    if account.owner == PUMP_CONSTANTS.pump_swap_program
      && account.data.len() == POOLS_ACCOUNT_SIZES.pump_swap
    {
      results.push(Arc::from(RwLock::from(PumpswapPool::from_account_info(
        pubkey.clone(),
        &account.data,
      ))));
    } else if account.owner == METEORA_CONSTANTS.amm_program
      && account.data.len() == POOLS_ACCOUNT_SIZES.meteora_amm
    {
      results.push(Arc::from(RwLock::from(MeteoraAmmPool::from_account_info(
        pubkey.clone(),
        &account.data,
        central_context.clone(),
      ))));
    } else if account.owner == METEORA_CONSTANTS.dammv2_program
      && account.data.len() == POOLS_ACCOUNT_SIZES.meteora_dammv2
    {
      results.push(Arc::from(RwLock::from(
        MeteoraDammV2Pool::from_account_info(pubkey.clone(), &account.data),
      )));
    } else if account.owner == RAYDIUM_CONSTANTS.amm_program
      && account.data.len() == POOLS_ACCOUNT_SIZES.raydium_ammv4
    {
      results.push(Arc::from(RwLock::from(
        RaydiumAmmV4Pool::from_account_info(pubkey.clone(), &account.data),
      )));
    } else if account.owner == RAYDIUM_CONSTANTS.cpmm_program
      && account.data.len() == POOLS_ACCOUNT_SIZES.raydium_cpmm
    {
      results.push(Arc::from(RwLock::from(RaydiumCpmmPool::from_account_info(
        pubkey.clone(),
        &account.data,
      ))));
    }
  }
  results
}
