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

/// Central context for managing DEX liquidity pools, markets, and RPC clients
///
/// This structure serves as the shared state across all pool operations, providing
/// thread-safe access to:
/// - Market graphs with bidirectional token pair mappings (markets["wsol"]["usdc"] will give you
/// the same thread safe access as if you did markets["usdc"]["wsol"])
/// - Protocol-specific pools and vaults
/// - JSON RPC clients for on-chain data fetching
/// - Token validation and legitimacy tracking
/// - Current network state (slot, blockhash)
pub struct CentralContext {
  /// Bidirectional market graph: token A -> token B -> list of pools
  ///
  /// Markets are stored bidirectionally so pools can be looked up by either
  /// token in the pair. Each pool is wrapped in Arc<RwLock<>> for shared
  /// ownership and thread-safe access.
  /// - RwLock on both layers of the hash map for mutability
  /// - Arc on the vector for bidirectional reference
  /// - RwLock on the vector for mutability and adding new markets in
  /// - RwLock on the pool traits to modify the pools states
  pub markets:
    RwLock<HashMap<Pubkey, HashMap<Pubkey, Arc<RwLock<Vec<Arc<RwLock<dyn PoolTrait>>>>>>>>,
  /// Synchronous JSON RPC client for Solana network requests
  pub json_rpc_client: solana_client::rpc_client::RpcClient,
  /// Asynchronous JSON RPC client for concurrent network requests
  pub json_rpc_client_async: solana_client::nonblocking::rpc_client::RpcClient,
  /// Fee rates for Raydium CPMM pools, keyed by config account address
  ///
  /// Stored in lamports (10^9 = 1 SOL) for high precision. Can be loaded during initialization via
  /// the load_cpmm_pool_configs function and available throughout the process lifetime.
  pub raydium_cpmm_fee_rates_lp: HashMap<Pubkey, u64>,
  /// Cache of Meteora Ammv1 vaults keyed by token address
  ///
  /// Protected by Mutex to ensure only one vault instance exists per token, preventing race
  /// conditions. Multiple pools should reference the same token vault as only one per token. Avoid
  /// creating multiple vaults per token.
  pub meteora_vault_cache: Mutex<HashMap<Pubkey, Arc<RwLock<MeteoraVault>>>>,
  /// Cache of Pumpfun bonding curves keyed by bonding curve address
  ///
  /// Protected by Mutex for thread-safe read-check-write operations when
  /// creating new bonding curve instances.
  pub pf_bonding_curves: Mutex<HashMap<Pubkey, Arc<RwLock<PfBondingCurve>>>>,
  /// Cache of Raydium launchpads keyed by launchpad/market address
  ///
  /// Protected by Mutex for thread-safe read-check-write operations when
  /// creating new bonding curve instances.
  pub raydium_launchpads: Mutex<HashMap<Pubkey, Arc<RwLock<RaydiumLaunchpad>>>>,
  /// Cache of token legitimacy flags keyed by token address
  ///
  /// A token is considered legit if it has Metaplex metadata and the update authority
  /// matches a reputable launchpad (Raydium Launchpad or Pumpfun Bonding Curve).
  /// Mutex for lookup and then store if not in cache (write based on read)
  pub legit_tokens: Mutex<HashMap<Pubkey, bool>>,
  /// Set of legitimate update authorities used to validate tokens
  ///
  /// Contains update authorities from known reputable launchpads (Raydium Launchpad and Pumpfun
  /// Bonding Curve only for now)
  pub legit_update_authorities: HashSet<Pubkey>,
  /// Map of account addresses to pools for efficient pool lookup
  ///
  /// A pool can be looked up by its pool address, token A vault address, or token B vault address.
  /// Updated automatically when pools are inserted via `insert_pool`.
  pub pools_map: RwLock<HashMap<Pubkey, Arc<RwLock<dyn PoolTrait>>>>,
  /// Current slot being produced by the Solana network
  pub current_slot: RwLock<u64>,
  /// Most recent blockhash produced by the network
  ///
  /// Typically the blockhash of slot (current_slot - 1).
  pub latest_blockhash: RwLock<Hash>,
}

impl CentralContext {
  /// Create a new `CentralContext` instance
  ///
  /// Initializes RPC clients, empty caches, and sets up legitimate update authorities
  /// from known reputable launchpads. Requires `RPC_NODE_URL` environment variable to be set.
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
      meteora_vault_cache: Mutex::new(HashMap::new()),
      current_slot: RwLock::new(0),
      pools_map: RwLock::new(HashMap::new()),
      latest_blockhash: RwLock::new(Hash::default()),
      legit_tokens: Mutex::new(HashMap::new()),
      legit_update_authorities,
    }
  }
}
