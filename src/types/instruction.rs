use solana_sdk::pubkey::Pubkey;

/**
A general and convenient Solana instruction type processed down in convenient types.
Everu solana instruction has a program_id, a public key of a program it interacts with
A vector of public keys for all of the accounts it interacts with 
And a vector uf bytes for the instruction's raw data
*/
pub struct Instruction {
  pub accounts: Vec<Pubkey>,
  pub program_id: Pubkey,
  pub data: Vec<u8>,
}
