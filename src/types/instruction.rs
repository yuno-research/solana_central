use solana_sdk::pubkey::Pubkey;

/**
A general and convenient Solana instruction type processed down in convenient types.
Every solana instruction has a program_id, a public key of a program it interacts with
A vector of public keys for all of the accounts it interacts with 
And a vector uf bytes for the instruction's raw data
*/
#[derive(Debug)]
pub struct Instruction<'a> {
  // The account keys of the tx that this instruction belongs to
  pub tx_account_keys: &'a [Pubkey],
  // The ordered list of accounts where the value at each index is the index of the account in tx_account_keys
  pub accounts: &'a [u8],
  pub data: &'a [u8],
  pub program_id_index: u8,
}
