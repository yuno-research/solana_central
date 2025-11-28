use crate::CentralContext;
use crate::utilities::process_get_program_accounts_pool::process_get_program_accounts_pool;
use futures::future::join_all;
use solana_account_decoder::UiAccountEncoding;
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use solana_client::rpc_filter::RpcFilterType;
use solana_sdk::account::Account;
use solana_sdk::pubkey::Pubkey;
use std::sync::Arc;
use std::thread;

/// Load pools from multiple protocols using async getProgramAccounts JSON RPC calls. Fetches and
/// processes pool accounts for the specified protocols in parallel using multiple threads. 
/// Processed pools are inserted into the central context for later access.
pub async fn load_pools(
  protocols_to_load: &[(Pubkey, usize)],
  central_context: Arc<CentralContext>,
  threads: usize,
) {
  println!(
    "load_pools: Loading pools for the following protocols: {:?}",
    protocols_to_load
  );
  // Use async await to fetch all dex pools using get program accounts and wait for results
  let mut futures = Vec::new();
  for (protocol, size) in protocols_to_load {
    let config = RpcProgramAccountsConfig {
      filters: Some(Vec::from([RpcFilterType::DataSize(*size as u64)])),
      account_config: RpcAccountInfoConfig {
        encoding: Some(UiAccountEncoding::Base64),
        data_slice: None,
        commitment: None,
        min_context_slot: None,
      },
      with_context: None,
      sort_results: None,
    };
    futures.push(
      central_context
        .json_rpc_client_async
        .get_program_accounts_with_config(protocol, config),
    );
  }
  let results = join_all(futures).await;
  println!("load_pools: Loaded pools from rpc");

  // Pack results into big raw data array to be parsed
  let mut accounts_raw_data: Vec<(Pubkey, Account)> = Vec::new();
  for result in results {
    let accounts = result.unwrap();
    accounts_raw_data.extend(accounts);
  }

  // Compute base chunk size for each thread to go through
  let chunk_size = accounts_raw_data.len() / threads;
  let accounts_raw_data = Arc::new(accounts_raw_data);
  println!("Length of accounts_raw_data: {}", accounts_raw_data.len());

  // Create threads to process raw account data into chunks
  let mut handles = Vec::with_capacity(threads);
  for i in 0..threads {
    let accounts_raw_data = Arc::clone(&accounts_raw_data);
    let central_context = Arc::clone(&central_context);
    // Clone the Arc to bump the strong count; each thread gets its own Arc pointer.

    // Compute start/end indices for this chunk.  The last thread grabs any “leftovers.”
    let start = i * chunk_size;
    let end = if i == threads - 1 {
      accounts_raw_data.len()
    } else {
      start + chunk_size
    };

    let handle = thread::spawn(move || {
      process_get_program_accounts_pool(accounts_raw_data, central_context, start, end)
    });

    // println!("Pushed handle with start and end indices: {}, {}", start, end);
    handles.push(handle);
  }
  // println!("load_pools: handles created");

  /*
  Load processed pools into the markets cache on a single thread, write locks would make it single
  thread anyways
  */
  for handle in handles {
    let results = handle.join().unwrap();
    for result in results {
      central_context.insert_pool(result);
    }
  }
}
