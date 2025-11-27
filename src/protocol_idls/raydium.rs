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
Directly from the protocol source code:
https://github.com/raydium-io/raydium-cp-swap/blob/master/programs/cp-swap/src/states/pool.rs
Added in instruction discriminator for anchor accounts
*/
#[derive(BorshDeserialize)]
pub struct CpmmPoolInfoIdl {
  pub discriminator: [u8; 8],
  /// Which config the pool belongs
  pub amm_config: Pubkey,
  /// pool creator
  pub pool_creator: Pubkey,
  /// Token A
  pub token_0_vault: Pubkey,
  /// Token B
  pub token_1_vault: Pubkey,

  /// Pool tokens are issued when A or B tokens are deposited.
  /// Pool tokens can be withdrawn back to the original A or B token.
  pub lp_mint: Pubkey,
  /// Mint information for token A
  pub token_0_mint: Pubkey,
  /// Mint information for token B
  pub token_1_mint: Pubkey,

  /// token_0 program
  pub token_0_program: Pubkey,
  /// token_1 program
  pub token_1_program: Pubkey,

  /// observation account to store oracle data
  pub observation_key: Pubkey,

  pub auth_bump: u8,
  /// Bitwise representation of the state of the pool
  /// bit0, 1: disable deposit(value is 1), 0: normal
  /// bit1, 1: disable withdraw(value is 2), 0: normal
  /// bit2, 1: disable swap(value is 4), 0: normal
  pub status: u8,

  pub lp_mint_decimals: u8,
  /// mint0 and mint1 decimals
  pub mint_0_decimals: u8,
  pub mint_1_decimals: u8,

  /// True circulating supply without burns and lock ups
  pub lp_supply: u64,
  /// The amounts of token_0 and token_1 that are owed to the liquidity provider.
  pub protocol_fees_token_0: u64,
  pub protocol_fees_token_1: u64,

  pub fund_fees_token_0: u64,
  pub fund_fees_token_1: u64,

  /// The timestamp allowed for swap in the pool.
  pub open_time: u64,
  /// recent epoch
  pub recent_epoch: u64,

  /// Creator fee collect mode
  /// 0: both token_0 and token_1 can be used as trade fees. It depends on what the input token is when swapping
  /// 1: only token_0 as trade fee
  /// 2: only token_1 as trade fee
  pub creator_fee_on: u8,
  pub enable_creator_fee: bool,
  pub padding1: [u8; 6],
  pub creator_fees_token_0: u64,
  pub creator_fees_token_1: u64,
  /// padding for future updates
  pub padding: [u64; 28],
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
  pub creator_fee: u64,
  pub share_fee: u64,
  // 0 means buy, 1 means sell in this enum
  pub trade_direction: u8,
  pub pool_status: u8,
  pub exact_in: bool,
}


#[derive(BorshDeserialize)]
pub struct CpmmInitializeInstructionDataIdl {
  discriminator: [u8; 8],
  pub init_amount_0: u64,
  pub init_amount_1: u64,
  pub open_time: u64,
}

#[derive(BorshDeserialize)]
pub struct AmmV4Initialize2InstructionDataIdl {
  pub discriminator: u8,
  pub nonce: u8,
  pub open_time: u64,
  pub init_pc_amount: u64,
  pub init_coin_amount: u64,
}
