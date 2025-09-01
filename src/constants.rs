use solana_sdk::pubkey::Pubkey;

pub const LAMPORTS_PER_SOL: u128 = 1_000_000_000;

pub struct Tokens {
  pub wsol: Pubkey,
  pub usdc: Pubkey,
  pub usdt: Pubkey,
}

pub const TOKENS: Tokens = Tokens {
  wsol: Pubkey::from_str_const("So11111111111111111111111111111111111111112"),
  usdc: Pubkey::from_str_const("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
  usdt: Pubkey::from_str_const("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB"),
};

// Constants for the Pumpfun program
pub struct PumpConstants {
  pub bonding_curve_global_config: Pubkey,
  pub bonding_curve_program: Pubkey,
  pub bonding_curve_event_authority: Pubkey,
  pub pump_swap_migrator_wallet: Pubkey,
  pub pump_swap_program: Pubkey,
  pub pump_swap_global_config: Pubkey,
  pub pump_swap_event_authority: Pubkey,
  /*
  Both the pumpswap and pf bonding curve programs use the same buy and sell instruction
  discriminators.
  */
  pub buy_instruction_discriminator: [u8; 8],
  pub sell_instruction_discriminator: [u8; 8],
  pub create_pool_instruction_discriminator: [u8; 8],
  pub bonding_curve_event_discriminator: [u8; 16],
  pub pumpswap_swap_event_discriminator: [u8; 8],
  pub pumpswap_buy_swap_event_discriminator: [u8; 8],
}

pub const PUMP_CONSTANTS: PumpConstants = PumpConstants {
  bonding_curve_global_config: Pubkey::from_str_const(
    "4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf",
  ),
  bonding_curve_program: Pubkey::from_str_const("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"),
  bonding_curve_event_authority: Pubkey::from_str_const(
    "Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1",
  ),
  pump_swap_migrator_wallet: Pubkey::from_str_const("39azUYFWPz3VHgKCf3VChUwbpURdCHRxjWVowf5jUJjg"),
  pump_swap_program: Pubkey::from_str_const("pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA"),
  pump_swap_global_config: Pubkey::from_str_const("ADyA8hdefvWN2dbGGWFotbzWxrAvLW83WG6QCVXvJKqw"),
  pump_swap_event_authority: Pubkey::from_str_const("GS4CU59F31iL7aR2Q8zVS8DRrcRnXX1yjQ66TqNVQnaR"),
  buy_instruction_discriminator: [102, 6, 61, 18, 1, 218, 235, 234],
  sell_instruction_discriminator: [51, 230, 133, 164, 1, 127, 131, 173],
  create_pool_instruction_discriminator: [233, 146, 209, 142, 207, 104, 64, 188],
  bonding_curve_event_discriminator: [
    228, 69, 165, 46, 81, 203, 154, 29, 189, 219, 127, 211, 78, 230, 97, 238,
  ],
  pumpswap_swap_event_discriminator: [228, 69, 165, 46, 81, 203, 154, 29],
  pumpswap_buy_swap_event_discriminator: [103, 244, 82, 31, 44, 245, 119, 119],
};

// Constants for the Raydium program
pub struct RaydiumConstants {
  pub amm_authority: Pubkey,
  pub market_program_id: Pubkey,
  pub amm_program: Pubkey,
  pub cpmm_program: Pubkey,
  pub cpmm_authority: Pubkey,
  pub launchpad_global_config: Pubkey,
  pub launchpad_event_authority: Pubkey,
  pub launchpad_program: Pubkey,
  pub launchpad_authority: Pubkey,
  pub cpmm_swap_discriminators: [[u8; 8]; 2],
  pub launchpad_swap_discriminators: [[u8; 8]; 4],
  pub cpmm_create_pool_instruction_discriminator: [u8; 8],
  pub ammv4_create_pool_instruction_discriminator: u8,
  pub ammv4_swap_discriminators: [u8; 2],
}

pub const RAYDIUM_CONSTANTS: RaydiumConstants = RaydiumConstants {
  amm_authority: Pubkey::from_str_const("5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1"),
  market_program_id: Pubkey::from_str_const("srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX"),
  amm_program: Pubkey::from_str_const("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8"),
  cpmm_program: Pubkey::from_str_const("CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C"),
  cpmm_authority: Pubkey::from_str_const("GpMZbSM2GgvTKHJirzeGfMFoaZ8UR2X7F4v8vHTvxFbL"),
  // This is the only global config in use for now constant product curves 0.25% protocol fee
  launchpad_global_config: Pubkey::from_str_const("6s1xP3hpbAfFoNtUNF8mfHsjr2Bd97JxFJRWLbL6aHuX"),
  launchpad_event_authority: Pubkey::from_str_const("2DPAtwB8L12vrMRExbLuyGnC7n2J5LNoZQSejeQGpwkr"),
  launchpad_program: Pubkey::from_str_const("LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj"),
  launchpad_authority: Pubkey::from_str_const("WLHv2UAZm6z4KyaaELi5pjdbJh6RESMva1Rnn8pJVVh"),
  /*
  Cpmm has two swap discriminators, swap exact in and swap exact out. Direction is determined by
  the order of input and output vault/mint accounts.
  */
  cpmm_swap_discriminators: [
    // Swap exact in
    [143, 190, 90, 218, 196, 30, 51, 222],
    // Swap exact out
    [55, 217, 98, 86, 163, 74, 180, 173],
  ],
  // Ammv4 also uses swap exact in and swap exact out instructions, but the discriminator is 1 byte
  ammv4_swap_discriminators: [9, 11],
  cpmm_create_pool_instruction_discriminator: [175, 175, 109, 31, 13, 152, 155, 237],
  ammv4_create_pool_instruction_discriminator: 1,
  // The set of all discriminators for all possible launchpad swap instructions
  launchpad_swap_discriminators: [
    // Buy exact in
    [250, 234, 13, 123, 213, 156, 19, 236],
    // Sell exact in
    [149, 39, 222, 155, 211, 124, 152, 26],
    // Buy exact out
    [24, 211, 116, 40, 105, 3, 153, 56],
    // Sell exact out
    [95, 200, 71, 34, 8, 9, 11, 166],
  ],
};

/// Constants for the Meteora program
pub struct MeteoraConstants {
  pub amm_program: Pubkey,
  pub vault_program: Pubkey,
  pub vault_base_key: Pubkey,
  pub locked_profit_degradation_denominator: u128,
  pub dammv2_program: Pubkey,
  pub dammv2_pool_authority: Pubkey,
  pub dammv2_event_authority: Pubkey,
  pub dammv2_max_fee_numerator: u64,
  pub dammv2_max_fee_denominator: u128,
  pub dbc_program: Pubkey,
  pub dbc_pool_authority: Pubkey,
  pub dbc_event_authority: Pubkey,
  pub dbc_swap_discriminator: [u8; 8],

}

pub const METEORA_CONSTANTS: MeteoraConstants = MeteoraConstants {
  amm_program: Pubkey::from_str_const("Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB"),
  vault_program: Pubkey::from_str_const("24Uqj9JCLxUeoC3hGfh5W3s9FM9uCHDS2SG3LYwBpyTi"),
  vault_base_key: Pubkey::from_str_const("HWzXGcGHy4tcpYfaRDCyLNzXqBTv3E6BttpCH2vJxArv"),
  locked_profit_degradation_denominator: 1_000_000_000_000,
  dammv2_program: Pubkey::from_str_const("cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG"),
  dammv2_pool_authority: Pubkey::from_str_const("HLnpSz9h2S4hiLQ43rnSD9XkcUThA7B8hQMKmDaiTLcC"),
  dammv2_event_authority: Pubkey::from_str_const("3rmHSu74h1ZcmAisVcWerTCiRDQbUrBKmcwptYGjHfet"),
  dammv2_max_fee_numerator: 500_000_000,
  dammv2_max_fee_denominator: 1_000_000_000,
  dbc_program: Pubkey::from_str_const("dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN"),
  dbc_pool_authority: Pubkey::from_str_const("FhVo3mqL8PW5pH5U2CN4XE33DokiyZnUwuGpH2hmHLuM"),
  dbc_event_authority: Pubkey::from_str_const("8Ks12pbrD6PXxfty1hVQiE9sc289zgU1zHkvXhrSdriF"),
  dbc_swap_discriminator: [248, 198, 158, 145, 225, 117, 135, 200]
};
pub enum TokenProgramsEnum {
  TokenStandard,
  Token2022,
}

pub struct SolanaPrograms {
  pub token_program: Pubkey,
  pub token_2022_program: Pubkey,
  pub system_program: Pubkey,
  pub associated_token_program: Pubkey,
  pub compute_budget_program: Pubkey,
}

pub const SOLANA_PROGRAMS: SolanaPrograms = SolanaPrograms {
  token_program: Pubkey::from_str_const("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
  token_2022_program: Pubkey::from_str_const("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
  system_program: Pubkey::from_str_const("11111111111111111111111111111111"),
  associated_token_program: Pubkey::from_str_const("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
  compute_budget_program: Pubkey::from_str_const("ComputeBudget111111111111111111111111111111"),
};

pub struct PoolsAccountSizes {
  pub pump_swap: usize,
  pub meteora_amm: usize,
  pub meteora_dammv2: usize,
  pub raydium_ammv4: usize,
  pub raydium_cpmm: usize,
}

pub const POOLS_ACCOUNT_SIZES: PoolsAccountSizes = PoolsAccountSizes {
  pump_swap: 300,
  meteora_amm: 952,
  meteora_dammv2: 1112,
  raydium_ammv4: 752,
  raydium_cpmm: 637,
};
