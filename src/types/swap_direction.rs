/*
If token A is USDC and token B is WSOL:

A_TO_B represents a swap from token A to token B, so a buy of solana or swap from USDC to WSOL
B_TO_A represents a swap from token B to token A, ao a sell of solana or swap from WSOL to USDC

The size is 1 byte
*/
#[derive(PartialEq, Eq, Clone, Debug, Copy)]
#[repr(u8)]
pub enum SwapDirection {
  AToB = 0,
  BToA = 1,
}
