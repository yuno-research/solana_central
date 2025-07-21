use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct LiquidityDistributionConfig {
  pub sqrt_price: u128,
  pub liquidity: u128,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct BaseFeeConfig {
  pub cliff_fee_numerator: u64,
  pub second_factor: u64,
  pub third_factor: u64,
  pub first_factor: u16,
  pub base_fee_mode: u8,
  pub padding_0: [u8; 5],
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct DynamicFeeConfig {
  pub initialized: u8,
  pub padding: [u8; 7],
  pub max_volatility_accumulator: u32,
  pub variable_fee_control: u32,
  pub bin_step: u16,
  pub filter_period: u16,
  pub decay_period: u16,
  pub reduction_factor: u16,
  pub padding2: [u8; 8],
  pub bin_step_u128: u128,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct PoolFeesConfig {
  pub base_fee: BaseFeeConfig,
  pub dynamic_fee: DynamicFeeConfig,
  pub padding_0: [u64; 5],
  pub padding_1: [u8; 6],
  pub protocol_fee_percent: u8,
  pub referral_fee_percent: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct LockedVestingConfig {
  pub amount_per_period: u64,
  pub cliff_duration_from_migration_time: u64,
  pub frequency: u64,
  pub number_of_period: u64,
  pub cliff_unlock_amount: u64,
  pub _padding: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct VolatilityTracker {
  pub last_update_timestamp: u64,
  pub padding: [u8; 8],           // Add padding for u128 alignment
  pub sqrt_price_reference: u128, // reference sqrt price
  pub volatility_accumulator: u128,
  pub volatility_reference: u128, // decayed volatility accumulator
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct SwapParameters {
  pub amount_in: u64,
  pub minimum_amount_out: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct SwapResult {
  pub actual_input_amount: u64,
  pub output_amount: u64,
  pub next_sqrt_price: u128,
  pub trading_fee: u64,
  pub protocol_fee: u64,
  pub referral_fee: u64,
}

// For big vaults like this one of length 10240: mPWBpKzzchEjitz7x4Q2d7cbQ3fHibF2BHWbWk8YGnH
#[derive(BorshDeserialize)]
pub struct VaultIdlBig {
  pub discriminator: [u8; 8],
  pub enabled: u8,
  pub bumps: VaultBumps,
  pub total_amount: u64,
  pub token_vault: Pubkey,
  pub fee_vault: Pubkey,
  pub token_mint: Pubkey,
  pub lp_mint: Pubkey,
  pub strategies: [Pubkey; 30],
  pub base: Pubkey,
  pub admin: Pubkey,
  pub operator: Pubkey,
  pub locked_profit_tracker: LockedProfitTracker,
  garbage: [u8; 9013],
}

// For small vaults like this one of length 1232: 12Q6qfukBF7KwbwxRvLnxhEnPdZPb7vjd8bPckCChf8
#[derive(BorshDeserialize)]
pub struct VaultIdlSmall {
  pub discriminator: [u8; 8],
  pub enabled: u8,
  pub bumps: VaultBumps,
  pub total_amount: u64,
  pub token_vault: Pubkey,
  pub fee_vault: Pubkey,
  pub token_mint: Pubkey,
  pub lp_mint: Pubkey,
  pub strategies: [Pubkey; 30],
  pub base: Pubkey,
  pub admin: Pubkey,
  pub operator: Pubkey,
  pub locked_profit_tracker: LockedProfitTracker,
  garbage: [u8; 5],
}

#[derive(BorshDeserialize)]
pub struct LockedProfitTracker {
  pub last_updated_locked_profit: u64,
  pub last_report: u64,
  pub locked_profit_degradation: u64,
}

#[derive(BorshDeserialize)]
pub struct VaultBumps {
  pub vault_bump: u8,
  pub token_vault_bump: u8,
}

/*
https://github.com/MeteoraAg/cp-amm/blob/main/programs/cp-amm/src/state/pool.rs
Pulled directly of of Meteora DammV2 program src. All supporting types included below:
*/
#[derive(BorshDeserialize, Debug)]
pub struct MeteoraDammv2PoolIdl {
  pub discriminator: [u8; 8],
  /// Pool fee
  pub pool_fees: PoolFees,
  /// token a mint
  pub token_a_mint: Pubkey,
  /// token b mint
  pub token_b_mint: Pubkey,
  /// token a vault
  pub token_a_vault: Pubkey,
  /// token b vault
  pub token_b_vault: Pubkey,
  /// Whitelisted vault to be able to buy pool before activation_point
  pub whitelisted_vault: Pubkey,
  /// partner
  pub partner: Pubkey,
  /// liquidity share
  pub liquidity: u128,
  /// padding, previous reserve amount, be careful to use that field
  pub _padding: u128,
  /// protocol a fee
  pub protocol_a_fee: u64,
  /// protocol b fee
  pub protocol_b_fee: u64,
  /// partner a fee
  pub partner_a_fee: u64,
  /// partner b fee
  pub partner_b_fee: u64,
  /// min price
  pub sqrt_min_price: u128,
  /// max price
  pub sqrt_max_price: u128,
  /// current price
  pub sqrt_price: u128,
  /// Activation point, can be slot or timestamp
  pub activation_point: u64,
  /// Activation type, 0 means by slot, 1 means by timestamp
  pub activation_type: u8,
  /// pool status, 0: enable, 1 disable
  pub pool_status: u8,
  /// token a flag
  pub token_a_flag: u8,
  /// token b flag
  pub token_b_flag: u8,
  /// 0 is collect fee in both token, 1 only collect fee in token a, 2 only collect fee in token b
  pub collect_fee_mode: u8,
  /// pool type
  pub pool_type: u8,
  /// padding
  pub _padding_0: [u8; 2],
  /// cumulative
  pub fee_a_per_liquidity: [u8; 32], // U256
  /// cumulative
  pub fee_b_per_liquidity: [u8; 32], // U256
  pub permanent_lock_liquidity: u128,
  /// metrics
  pub metrics: PoolMetrics,
  /// Padding for further use
  _padding_1: [u64; 10],
  /// Farming reward information
  pub reward_infos: [RewardInfo; NUM_REWARDS],
}

const NUM_REWARDS: usize = 2;
#[derive(BorshDeserialize, Debug)]
pub struct BaseFeeStruct {
  pub cliff_fee_numerator: u64,
  pub fee_scheduler_mode: u8,
  pub padding_0: [u8; 5],
  pub number_of_period: u16,
  pub period_frequency: u64,
  pub reduction_factor: u64,
  pub padding_1: u64,
}

#[derive(BorshDeserialize, Debug)]
pub struct DynamicFees {
  pub initialized: u8, // 0, ignore for dynamic fee
  padding: [u8; 7],
  pub max_volatility_accumulator: u32,
  pub variable_fee_control: u32,
  pub bin_step: u16,
  pub filter_period: u16,
  pub decay_period: u16,
  pub reduction_factor: u16,
  pub last_update_timestamp: u64,
  pub bin_step_u128: u128,
  pub sqrt_price_reference: u128, // reference sqrt price
  pub volatility_accumulator: u128,
  pub volatility_reference: u128, // decayed volatility accumulator
}

#[derive(BorshDeserialize, Debug)]
pub struct PoolFees {
  /// Trade fees are extra token amounts that are held inside the token
  /// accounts during a trade, making the value of liquidity tokens rise.
  /// Trade fee numerator
  pub base_fee: BaseFeeStruct,

  /// Protocol trading fees are extra token amounts that are held inside the token
  /// accounts during a trade, with the equivalent in pool tokens minted to
  /// the protocol of the program.
  /// Protocol trade fee numerator
  pub protocol_fee_percent: u8,
  /// partner fee
  pub partner_fee_percent: u8,
  /// referral fee
  pub referral_fee_percent: u8,
  /// padding
  pub padding_0: [u8; 5],

  /// dynamic fee
  pub dynamic_fee: DynamicFees,

  /// padding
  padding_1: [u64; 2],
}

#[derive(BorshDeserialize, Debug)]
pub struct PoolMetrics {
  pub total_protocol_base_fee: u64,
  pub total_protocol_quote_fee: u64,
  pub total_trading_base_fee: u64,
  pub total_trading_quote_fee: u64,
}

#[derive(BorshDeserialize, Debug)]
struct RewardInfo {
  /// Indicates if the reward has been initialized
  initialized: u8,
  /// reward token flag
  reward_token_flag: u8,
  /// padding
  _padding_0: [u8; 6],
  /// Padding to ensure `reward_rate: u128` is 16-byte aligned
  _padding_1: [u8; 8], // 8 bytes
  /// Reward token mint.
  mint: Pubkey,
  /// Reward vault token account.
  vault: Pubkey,
  /// Authority account that allows to fund rewards
  funder: Pubkey,
  /// reward duration
  reward_duration: u64,
  /// reward duration end
  reward_duration_end: u64,
  /// reward rate
  reward_rate: u128,
  /// Reward per token stored
  reward_per_token_stored: [u8; 32], // U256
  /// The last time reward states were updated.
  last_update_time: u64,
  /// Accumulated seconds when the farm distributed rewards but the bin was empty.
  /// These rewards will be carried over to the next reward time window.
  cumulative_seconds_with_empty_liquidity_reward: u64,
}

/*
This layout is pulled from this guy's sdk but using his sdk creates manu cargo conficts so just
copied over the chunks needed manually
https://github.com/regolith-labs/meteora-pools-sdk/tree/master/src/types
*/
#[derive(BorshDeserialize)]
pub struct AmmPoolFees {
  /// Trade fees are extra token amounts that are held inside the token
  /// accounts during a trade, making the value of liquidity tokens rise.
  /// Trade fee numerator
  pub trade_fee_numerator: u64,
  /// Trade fee denominator
  pub trade_fee_denominator: u64,
  /// Protocol trading fees are extra token amounts that are held inside the token
  /// accounts during a trade, with the equivalent in pool tokens minted to
  /// the protocol of the program.
  /// Protocol trade fee numerator
  pub protocol_trade_fee_numerator: u64,
  /// Protocol trade fee denominator
  pub protocol_trade_fee_denominator: u64,
}

#[derive(BorshDeserialize)]
struct Padding {
  /// Padding 0
  pub padding0: [u8; 6],
  /// Padding 1
  pub padding1: [u64; 21],
  /// Padding 2
  pub padding2: [u64; 21],
}

#[derive(BorshDeserialize)]
enum CurveType {
  ConstantProduct,
  Stable {
    /// Amplification coefficient
    amp: u64,
    /// Multiplier for the pool token. Used to normalized token with different decimal into the same precision.
    token_multiplier: TokenMultiplier,
    /// Depeg pool information. Contains functions to allow token amount to be repeg using stake / interest bearing token virtual price
    depeg: Depeg,
    /// The last amp updated timestamp. Used to prevent update_curve_info called infinitely many times within a short period
    last_amp_updated_timestamp: u64,
  },
}

#[derive(BorshDeserialize)]
struct TokenMultiplier {
  /// Multiplier for token A of the pool.
  pub token_a_multiplier: u64,
  /// Multiplier for token B of the pool.
  pub token_b_multiplier: u64,
  /// Record the highest token decimal in the pool. For example, Token A is 6 decimal, token B is 9 decimal. This will save value of 9.
  pub precision_factor: u8,
}

#[derive(BorshDeserialize)]
struct Depeg {
  /// The virtual price of staking / interest bearing token
  pub base_virtual_price: u64,
  /// The last time base_virtual_price is updated
  pub base_cache_updated: u64,
  /// Type of the depeg pool
  pub depeg_type: DepegType,
}

#[derive(BorshDeserialize)]
enum DepegType {
  None,
  Marinade,
  Lido,
  SplStake,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct DbcPoolConfig {
  /// Discriminator: [26, 108, 14, 123, 116, 230, 129, 43]
  pub discriminator: [u8; 8],
  pub quote_mint: Pubkey,
  pub fee_claimer: Pubkey,
  pub leftover_receiver: Pubkey,
  pub pool_fees: PoolFeesConfig,
  pub collect_fee_mode: u8,
  pub migration_option: u8,
  pub activation_type: u8,
  pub token_decimal: u8,
  pub version: u8,
  pub token_type: u8,
  pub quote_token_flag: u8,
  pub partner_locked_lp_percentage: u8,
  pub partner_lp_percentage: u8,
  pub creator_locked_lp_percentage: u8,
  pub creator_lp_percentage: u8,
  pub migration_fee_option: u8,
  pub fixed_token_supply_flag: u8,
  pub creator_trading_fee_percentage: u8,
  pub token_update_authority: u8,
  pub migration_fee_percentage: u8,
  pub creator_migration_fee_percentage: u8,
  pub _padding_1: [u8; 7],
  pub swap_base_amount: u64,
  pub migration_quote_threshold: u64,
  pub migration_base_threshold: u64,
  pub migration_sqrt_price: u128,
  pub locked_vesting_config: LockedVestingConfig,
  pub pre_migration_token_supply: u64,
  pub post_migration_token_supply: u64,
  pub _padding_2: [u128; 2],
  pub sqrt_start_price: u128,
  pub curve: [LiquidityDistributionConfig; 20],
}

#[derive(BorshDeserialize, Debug)]
pub struct DbcVirtualPool {
  pub discriminator: [u8; 8],
  pub volatility_tracker: VolatilityTracker,
  pub config: Pubkey,
  pub creator: Pubkey,
  pub base_mint: Pubkey,
  pub base_vault: Pubkey,
  pub quote_vault: Pubkey,
  pub base_reserve: u64,
  pub quote_reserve: u64,
  pub protocol_base_fee: u64,
  pub protocol_quote_fee: u64,
  pub partner_base_fee: u64,
  pub partner_quote_fee: u64,
  pub sqrt_price: u128,
  pub activation_point: u64,
  pub pool_type: u8,
  pub is_migrated: u8,
  pub is_partner_withdraw_surplus: u8,
  pub is_protocol_withdraw_surplus: u8,
  pub migration_progress: u8,
  pub is_withdraw_leftover: u8,
  pub is_creator_withdraw_surplus: u8,
  pub migration_fee_withdraw_status: u8,
  pub metrics: PoolMetrics,
  pub finish_curve_timestamp: u64,
  pub creator_base_fee: u64,
  pub creator_quote_fee: u64,
  pub _padding_1: [u64; 7],
}

#[derive(BorshDeserialize)]
pub struct MeteoraAmmPoolIdl {
  pub discriminator: [u8; 8],
  /// LP token mint of the pool
  pub lp_mint: Pubkey,
  /// Token A mint of the pool. Eg: USDT
  pub token_a_mint: Pubkey,
  /// Token B mint of the pool. Eg: USDC
  pub token_b_mint: Pubkey,
  /// Vault account for token A. Token A of the pool will be deposit / withdraw from this vault account.
  pub a_vault: Pubkey,
  /// Vault account for token B. Token B of the pool will be deposit / withdraw from this vault account.
  pub b_vault: Pubkey,
  /// LP token account of vault A. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
  pub a_vault_lp: Pubkey,
  /// LP token account of vault B. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
  pub b_vault_lp: Pubkey,
  /// "A" vault lp bump. Used to create signer seeds.
  pub a_vault_lp_bump: u8,
  /// Flag to determine whether the pool is enabled, or disabled.
  pub enabled: bool,
  /// Protocol fee token account for token A. Used to receive trading fee.
  pub protocol_token_a_fee: Pubkey,
  /// Protocol fee token account for token B. Used to receive trading fee.
  pub protocol_token_b_fee: Pubkey,
  /// Fee last updated timestamp
  pub fee_last_updated_at: u64,
  pub padding0: [u8; 24],
  /// Store the fee charges setting.
  pub fees: AmmPoolFees,
  pub garbage: [u8; 590],
}
