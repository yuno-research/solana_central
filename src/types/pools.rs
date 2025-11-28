/// Enumeration of all supported DEX pool types
#[repr(u8)]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Pools {
  MeteoraAmm = 0,
  RaydiumCpmm = 1,
  RaydiumAmmV4 = 2,
  RaydiumClmm = 3,
  MeteoraDlmm = 4,
  OrcaWhirlpool = 5,
  /*
  The program id for this is: 5quBtoiQqxF9Jv6KYKctB59NT3gtJD2Y65kdnB1Uev3h. Look up correct vault
  authorities and whatever else is needed for this pool
  */
  RaydiumStableSwap = 6,
  PumpswapAmm = 7,
  SaberStableSwap = 8,
  AldrinAmm = 9,
  MeteoraDammV2 = 10,
  PfBondingCurve = 11,
  RaydiumLaunchpad = 12,
  MeteoraDbc = 13,
}
