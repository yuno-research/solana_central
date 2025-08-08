/*
This is pulled from Raydium io Raydium SDK:
https://github.com/raydium-io/raydium-sdk/blob/f4b7f47e744c12a8b0119b85e16d0d8274aa5ba9/src/liquidity/layout.ts
The official Raydium IDL on their site did not contain all this info
*/
use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;

#[derive(BorshDeserialize)]
pub struct AmmV4PoolInfoIdl {
  status: u64,
  nonce: u64,
  max_order: u64,
  depth: u64,
  pub base_decimal: u64,
  pub quote_decimal: u64,
  state: u64,
  reset_flag: u64,
  min_size: u64,
  vol_max_cut_ratio: u64,
  amount_wave_ratio: u64,
  base_lot_size: u64,
  quote_lot_size: u64,
  min_price_multiplier: u64,
  max_price_multiplier: u64,
  system_decimal_value: u64,
  min_separate_numerator: u64,
  min_separate_denominator: u64,
  trade_fee_numerator: u64,
  trade_fee_denominator: u64,
  pnl_numerator: u64,
  pnl_denominator: u64,
  pub swap_fee_numerator: u64,
  pub swap_fee_denominator: u64,
  base_need_take_pnl: u64,
  quote_need_take_pnl: u64,
  quote_total_pnl: u64,
  base_total_pnl: u64,
  pool_open_time: u64,
  punish_pc_amount: u64,
  punish_coin_amount: u64,
  orderbook_to_init_time: u64,
  swap_base_in_amount: u128,
  swap_quote_out_amount: u128,
  swap_base2quote_fee: u64,
  swap_quote_in_amount: u128,
  swap_base_out_amount: u128,
  swap_quote2base_fee: u64,
  // amm vault
  pub base_vault: Pubkey,
  pub quote_vault: Pubkey,
  // mint
  pub base_mint: Pubkey,
  pub quote_mint: Pubkey,
  lp_mint: Pubkey,
  // market
  open_orders: Pubkey,
  market_id: Pubkey,
  market_program_id: Pubkey,
  target_orders: Pubkey,
  withdraw_queue: Pubkey,
  lp_vault: Pubkey,
  owner: Pubkey,
  lp_reserve: u64,
  padding: [u64; 3],
}

/*
Pulled from Raydium SDK V2. Confirmed by the IDL to be accurate minus some discriminator and buffer
value differences.
https://github.com/raydium-io/raydium-sdk-V2/blob/45d37d45460f4948d61762e8e4b149706778c217/src/raydium/cpmm/layout.ts
*/
#[derive(BorshDeserialize)]
pub struct CpmmPoolConfigIdl {
  /// Account discriminator / reserved blob
  pub discriminator: [u8; 8],
  /// Bump to identify PDA
  pub bump: u8,
  /// Status to control if new pool can be created
  pub disable_create_pool: bool,
  /// Config index
  pub index: u16,
  /// The trade fee, denominated in hundredths of a bip (10^-6)
  pub trade_fee_rate: u64,
  /// The protocol fee
  pub protocol_fee_rate: u64,
  /// The fund fee, denominated in hundredths of a bip (10^-6)
  pub fund_fee_rate: u64,
  /// Fee for creating a new pool
  pub create_pool_fee: u64,
  /// Address of the protocol fee owner
  pub protocol_owner: Pubkey,
  /// Address of the fund fee owner
  pub fund_owner: Pubkey,
  /// Sequence of 16 u64 values
  pub extra: [u64; 16],
}

/*
Pulled from Raydium SDK V2. Confirmed by the IDL to be accurate minus some discriminator and buffer
value differences.
https://github.com/raydium-io/raydium-sdk-V2/blob/45d37d45460f4948d61762e8e4b149706778c217/src/raydium/cpmm/layout.ts
*/
#[derive(BorshDeserialize)]
pub struct CpmmPoolInfoIdl {
  /// Account discriminator / reserved blob
  pub discriminator: [u8; 8],
  /// Config account this pool belongs to
  pub config_id: Pubkey,
  /// Address that created the pool
  pub pool_creator: Pubkey,
  /// Token A vault
  pub vault_a: Pubkey,
  /// Token B vault
  pub vault_b: Pubkey,
  /// LP token mint
  pub mint_lp: Pubkey,
  /// Mint for token A
  pub mint_a: Pubkey,
  /// Mint for token B
  pub mint_b: Pubkey,
  /// Program address handling mint A actions
  pub mint_program_a: Pubkey,
  /// Program address handling mint B actions
  pub mint_program_b: Pubkey,
  /// Oracle observation account
  pub observation_id: Pubkey,
  /// PDA bump for this pool
  pub bump: u8,
  /// Pool status bits
  pub status: u8,
  /// Decimals for LP token
  pub lp_decimals: u8,
  /// Decimals for token A
  pub mint_decimal_a: u8,
  /// Decimals for token B
  pub mint_decimal_b: u8,
  /// Total LP token supply
  pub lp_amount: u64,
  /// Protocol fees collected in token A
  pub protocol_fees_mint_a: u64,
  /// Protocol fees collected in token B
  pub protocol_fees_mint_b: u64,
  /// Fund fees collected in token A
  pub fund_fees_mint_a: u64,
  /// Fund fees collected in token B
  pub fund_fees_mint_b: u64,
  /// Pool open timestamp
  pub open_time: u64,
  /// Reserved for future use
  pub extra: [u64; 32],
}

#[derive(BorshDeserialize)]
pub struct LaunchpadPlatformConfigIdl {
  pub discriminator: [u8; 8],
  pub epoch: u64,
  pub platform_fee_wallet: Pubkey,
  pub platform_nft_wallet: Pubkey,
  pub platform_scale: u64,
  pub creator_scale: u64,
  pub burn_scale: u64,
  pub fee_rate: u64,
  // We don't need this info not decoding it
  garbage: [u8; 832],
}

#[derive(BorshDeserialize)]
pub struct LaunchpadPoolIdl {
  discriminator: [u8; 8],
  pub epoch: u64,
  pub auth_bump: u8,
  pub status: u8,
  pub base_decimals: u8,
  pub quote_decimals: u8,
  pub migrate_type: u8,
  pub supply: u64,
  pub total_base_sell: u64,
  pub virtual_base: u64,
  pub virtual_quote: u64,
  pub real_base: u64,
  pub real_quote: u64,
  pub total_quote_fund_raising: u64,
  pub quote_protocol_fee: u64,
  // This platform fee isn't a percent its the sum of all fees collected or something
  pub platform_fee: u64,
  pub migrate_fee: u64,
  pub vesting_schedule: [u64; 5],
  pub global_config: Pubkey,
  pub platform_config: Pubkey,
  pub base_mint: Pubkey,
  pub quote_mint: Pubkey,
  pub base_vault: Pubkey,
  pub quote_vault: Pubkey,
  pub creator: Pubkey,
  padding: [u8; 64],
}

#[derive(BorshDeserialize)]
pub struct LaunchpadTradeEventIdl {
  discriminator: [u8; 16],
  pub pool_state: Pubkey,
  pub total_base_sell: u64,
  pub virtual_base: u64,
  pub virtual_quote: u64,
  pub real_base_before: u64,
  pub real_quote_before: u64,
  pub real_base_after: u64,
  pub real_quote_after: u64,
  pub amount_in: u64,
  pub amount_out: u64,
  pub protocol_fee: u64,
  pub platform_fee: u64,
  pub share_fee: u64,
  // 0 means buy, 1 means sell in this enum
  pub trade_direction: u8,
  pub pool_status: u8,
}


#[derive(BorshDeserialize)]
pub struct CpmmInitializeInstructionDataIdl {
  discriminator: [u8; 8],
  pub init_amount_0: u64,
  pub init_amount_1: u64,
  pub open_time: u64,
}

