use crate::constants::PUMP_CONSTANTS;
use crate::constants::RAYDIUM_CONSTANTS;
use crate::types::meteora_vault::MeteoraVault;
use crate::types::pf_bonding_curve::PfBondingCurve;
use crate::types::pool::PoolTrait;
use crate::types::raydium_launchpad::RaydiumLaunchpad;
use solana_client;
use solana_client::rpc_client::RpcClient;
use solana_sdk::hash::Hash;
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;

pub struct CentralContext {
  /*
  RwLock on both layers of the hash map for mutability
  Arc on the vector for bidirectional reference
  RwLock on the vector for mutability and adding new markets in
  RwLock on the pool traits to modify the pools states
  */
  pub markets:
    RwLock<HashMap<Pubkey, HashMap<Pubkey, Arc<RwLock<Vec<Arc<RwLock<dyn PoolTrait>>>>>>>>,
  pub json_rpc_client: solana_client::rpc_client::RpcClient,
  pub json_rpc_client_async: solana_client::nonblocking::rpc_client::RpcClient,
  /*
  The fee rates for Raydium CPMM pools. These should be available throughout the lifetime of the
  process and available as soon as Raydium decoding starts, so they are placed in the central
  context. Stored in lamports (10^9 = 1) for high precision.
  */
  pub raydium_cpmm_fee_rates_lp: HashMap<Pubkey, u64>,

  /**
  The fee rates specifically for the platform fees for the Raydium Launchpads. Each platform will
  charge its own defined fee rate, which together with the protocol fee rate will make up the total
  fee rate
  */
  pub raydium_launchpad_platform_fee_rates_lp: HashMap<Pubkey, u64>,
  /*
  The vault cache is wrapped in a mutex to prevent race conditions that can arise when 2 potential
  meteora liquidity pools can be created and both of those pools are not in the vault. Functions
  such as get_meteora_vault_from_token_address should only be creating new vaults if they lock the
  access to the hash map cache. Then they ensure there isn't already a vault there for that token,
  and then they write a new smart pointer to the vault. This is so that we can ensure that there can
  only be one meteora vault per token and all references across all pools point to the same vault.
  */
  pub meteora_vault_cache: Mutex<HashMap<Pubkey, Arc<RwLock<MeteoraVault>>>>,
  /*
  Pf bonding curves are just one market and in practice not all will be monitored. They key will be
  the BONDING CURVE addresses of the bonding curves. The reason why this is a mutex is because there
  will be cases where we need to exclusively read the bonding curves, and add one if it does not
  exist. It is a write based on a read scenario.
  */
  pub pf_bonding_curves: Mutex<HashMap<Pubkey, Arc<RwLock<PfBondingCurve>>>>,
  /**
  Raydium Launchpads are also just one market and cannot all be monitored. The key will be the
  market address, and the value will be the pool itself
  */
  pub raydium_launchpads: Mutex<HashMap<Pubkey, Arc<RwLock<RaydiumLaunchpad>>>>,
  /*
  Legit tokens. A map of token addresses to boolean legit or not. We consider a token legit if it
  there exists a metaplex pda for it and if the update authority in the metaplex pda metadata
  account is from a legit launchpad, such as Raydium Launchpad or Pumpfun Bonding Curve. This is
  because tokens launched on these reputable launchpads will always have metaplex metadata and the
  correct update authority.

  Done as a mutex because in many cases you'll atomically lock, check if entry exists, if not
  lookup and add in an entry.
  */
  pub legit_tokens: Mutex<HashMap<Pubkey, bool>>,
  /*
  A set of all legit update authorities which are used to identify legit tokens.
  */
  pub legit_update_authorities: HashSet<Pubkey>,
  /*
  A hash map of token accounts and the pools that those token accounts are a part of. The same pool
  here can be looked up by these 3 Solana pubkey accounts that will be unique to it:
  Pool address, pool token a vault, pool token b vault.
  The insert_pool_into_central_context function makes sure to insert new pools into the main markets
  graph, as well as into this pools map.
  */
  pub pools_map: RwLock<HashMap<Pubkey, Arc<RwLock<dyn PoolTrait>>>>,
  /*
  The current slot being produced by the network
  */
  pub current_slot: RwLock<u64>,
  /*
  The most recent blockhast produced by the network. This is going to the be the blockhash of slot
  x - 1 if the current slot on the network is slot x
  */
  pub latest_blockhash: RwLock<Hash>,
}

impl CentralContext {
  pub fn new() -> Self {
    let rpc_url = env::var("RPC_NODE_URL").expect("RPC_NODE_URL must be set");
    let json_rpc_client = RpcClient::new_with_timeout(&rpc_url, Duration::from_secs(300));
    let json_rpc_client_async = solana_client::nonblocking::rpc_client::RpcClient::new_with_timeout(
      rpc_url,
      Duration::from_secs(300),
    );

    let mut legit_update_authorities = HashSet::new();
    // Raydium launchpad update authority
    legit_update_authorities.insert(RAYDIUM_CONSTANTS.launchpad_authority);
    // Pumpfun bonding curve update authority
    legit_update_authorities.insert(PUMP_CONSTANTS.bonding_curve_update_authority);

    Self {
      markets: RwLock::new(HashMap::new()),
      pf_bonding_curves: Mutex::new(HashMap::new()),
      json_rpc_client,
      json_rpc_client_async,
      raydium_cpmm_fee_rates_lp: HashMap::new(),
      raydium_launchpads: Mutex::new(HashMap::new()),
      raydium_launchpad_platform_fee_rates_lp: HashMap::new(),
      meteora_vault_cache: Mutex::new(HashMap::new()),
      current_slot: RwLock::new(0),
      pools_map: RwLock::new(HashMap::new()),
      latest_blockhash: RwLock::new(Hash::default()),
      legit_tokens: Mutex::new(HashMap::new()),
      legit_update_authorities,
    }
  }
}
