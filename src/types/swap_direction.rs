/// Direction of a swap operation. If on a liquidity pool/market,token A is USDC and token B is
/// WSOL: `AToB`: Swap from USDC to WSOL (buying SOL), `BToA`: Swap from WSOL to USDC (selling SOL)
#[derive(PartialEq, Eq, Clone, Debug, Copy)]
#[repr(u8)]
pub enum SwapDirection {
  /// Swap from token A to token B
  AToB = 0,
  /// Swap from token B to token A
  BToA = 1,
}
