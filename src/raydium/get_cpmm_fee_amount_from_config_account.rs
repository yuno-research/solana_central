use solana_sdk::pubkey::Pubkey;

const ONE: Pubkey = Pubkey::from_str_const("B5u5x9S5pyaJdonf7bXUiEnBfEXsJWhNxXfLGAbRFtg2");
const TWO: Pubkey = Pubkey::from_str_const("BgxH5ifebqHDuiADWKhLjXGP5hWZeZLoCdmeWJLkRqLP");
const THREE: Pubkey = Pubkey::from_str_const("BhH6HphjBKXu2PkUc2aw3xEMdUvK14NXxE5LbNWZNZAA");
const FOUR: Pubkey = Pubkey::from_str_const("D4FPEruKEHrG5TenZ2mpDGEfu1iUvTiqBxvpU8HLBvC2");
const FIVE: Pubkey = Pubkey::from_str_const("2fGXL8uhqxJ4tpgtosHZXT4zcQap6j62z3bMDxdkMvy5");
const SIX: Pubkey = Pubkey::from_str_const("G95xxie3XbkCqtE39GgQ9Ggc7xBC8Uceve7HFDEFApkc");
const SEVEN: Pubkey = Pubkey::from_str_const("C7Cx2pMLtjybS3mDKSfsBj4zQ3PRZGkKt7RCYTTbCSx2");

pub fn get_cpmm_fee_amount_from_config_account(config_account: Pubkey, pool_account: &Pubkey) -> u64 {
  if config_account == ONE {
    return 15000000;
  } else if config_account == TWO {
    return 3000000;
  } else if config_account == THREE {
    return 5000000;
  } else if config_account == FOUR {
    return 2500000;
  } else if config_account == FIVE {
    return 20000000;
  } else if config_account == SIX {
    return 10000000;
  } else if config_account == SEVEN {
    return 4000000;
  }
  else {
    println!(
      "get_cpmm_fee_amount_from_config_account: Found a config account not defined in constants, config_account: {}, pool account: {}. Reporting a 100% swap fee rate.",
      config_account,
      pool_account
    );
    
    1000000000
  }
}
