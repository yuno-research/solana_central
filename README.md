# Solana Central

Shared Rust library providing core utilities, types, protocol IDLs, and DEX pool management for all Solana-related repositories.

Rust components are extensively documented in the following rustdoc: [https://yuno-research.github.io/docs/target/doc/solana_central/index.html](https://yuno-research.github.io/docs/target/doc/solana_central/index.html)

## Overview

This repository serves as the central dependency for all Solana projects, providing common functionality including:

- **Central Context**: Shared state management for DEX liquidity pools, markets, RPC clients, and blockchain state
- **Protocol Utilities**: Helper functions for interacting with various DEX protocols and parsing on-chain account data
- **Common Types**: Standardized data structures and traits used across all repositories
- **Protocol IDLs**: Borsh-deserializable structs copied from on-chain Solana protocols for decoding raw account data

## Key Components

### Central Context

`CentralContext` manages shared state across the entire system:

- Bidirectional market graph mapping token pairs to pools
- Protocol-specific pool storage with thread-safe access
- RPC client management for on-chain data fetching
- Token legitimacy validation and caching
- Current network state (slot, blockhash)

### Protocol Support

Protocol-specific utilities are provided for:

- **Meteora**: AMMv1, DAMMv2, Dynamic Bonding Curve (DBC) pools and vaults
- **Raydium**: AMMv4, CPMM (AMMv5), Launchpad pools
- **Pumpfun**: Bonding curve derivation and utilities
- **Pumpswap**: AMM pool parsing

Each protocol module provides functions to create pool structures from raw on-chain account data using `from_account_info` methods.

### Core Types

- **Pool Types**: `MeteoraAmmPool`, `MeteoraDammV2Pool`, `MeteoraVault`, `RaydiumAmmV4Pool`, `RaydiumCpmmPool`, `RaydiumLaunchpad`, `PumpswapPool`, `PfBondingCurve`, `MeteoraDbc`
- **Traits**: `PoolTrait` for common pool operations and abstractions
- **Standardized Types**: `SwapTx`, `MarketUpdate`, `Instruction`, `Link` for cross-system communication

### Utilities

General-purpose helper functions including:

- Pool loading from RPC
- Account key extraction from transactions
- Token legitimacy checking
- Protocol-specific pool processing

## Usage

This library is included as a local path dependency in other repositories:

```toml
[dependencies]
solana_central = { path = "../solana_central" }
```

All public types and functions are re-exported at the crate root for convenient access.

## Environment Variables

- `RPC_NODE_URL`: URL of the Solana JSON RPC node to use for on-chain data fetching

## Notes

- This is a library dependency, not a standalone executable
- Protocol IDLs in the `protocol_idls` module are copied from various on-chain Solana protocols
- The `constants` module contains protocol-specific addresses and discriminators
