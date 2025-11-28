use solana_sdk::pubkey::Pubkey;

/**
A general independent Solana instruction type agnostic of inner or top level instructions. Designed
to conveniently work with the format used in Solana SDKs and yellowstone grpc. Does not own any of
the data just lifetimes to it.
*/
#[derive(Debug)]
pub struct Instruction<'a> {
  /// The ordered list of account keys of the tx that this instruction belongs to
  pub tx_account_keys: &'a [Pubkey],
  /// The ordered list of account indices of the accounts this instruction interacts with
  pub accounts: &'a [u8],
  /// The raw data of the instruction
  pub data: &'a [u8],
  /// The index of the program id in the tx_account_keys vector
  pub program_id_index: u8,
}
