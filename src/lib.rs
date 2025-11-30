//! # Solana Central Rust Library
//!
//! Provides utilities, types, and DEX protocol-specific implementations
//!
//! Currently supported DEXs: Meteora Dammv2, Dbc, Ammv1 (Old), Raydium Cpmm, AmmV4, Launchpad,
//! Pumpswap, Pumpfun Bonding Curve
//!
//! This library also provides:
//! - A shared context (`CentralContext`) for managing DEX liquidity pools for trading pairs, RPC
//! clients, blockchain state, etc.
//! - Protocol-specific pool parsing and raw on-chain account data decoding utilities. Every
//! protocol type has an accessible `from_account_info` method that can be used from raw on chain
//! account data.
//! - Common types and traits for abstractions with liquidity pool fees, liquidity, etc.
//!
//! Environment variables required:
//! - RPC_NODE_URL: URL of the Solana JSON RPC node to use for on-chain data fetching

mod central_context;
pub mod constants;
mod meteora;
pub mod protocol_idls;
mod pumpfun;
mod pumpswap;
mod raydium;
mod types;
mod utilities;

// Re-exports
pub use central_context::central_context::CentralContext;
pub use meteora::get_meteora_vault_from_token_address::get_meteora_vault_from_token_address;
pub use pumpfun::derive_bonding_curve::derive_bonding_curve;
pub use raydium::get_cpmm_fee_amount_from_config_account::get_cpmm_fee_amount_from_config_account;
pub use types::instruction::Instruction;
pub use types::link::Link;
pub use types::market_update::MarketUpdate;
pub use types::meteora_amm_pool::MeteoraAmmPool;
pub use types::meteora_dammv2_pool::MeteoraDammV2Pool;
pub use types::meteora_dbc::MeteoraDbc;
pub use types::meteora_vault::MeteoraVault;
pub use types::pf_bonding_curve::PfBondingCurve;
pub use types::pool::{Pool, PoolTrait};
pub use types::pools::Pools;
pub use types::pumpswap_pool::PumpswapPool;
pub use types::raydium_ammv4_pool::RaydiumAmmV4Pool;
pub use types::raydium_cpmm_pool::RaydiumCpmmPool;
pub use types::raydium_launchpad::RaydiumLaunchpad;
pub use types::swap_direction::SwapDirection;
pub use types::swap_tx::SwapTx;
pub use types::token_creation::TokenCreation;
pub use utilities::is_legit_token::is_legit_token;
pub use utilities::load_pools::load_pools;
pub use utilities::process_get_program_accounts_pool::process_get_program_accounts_pool;
