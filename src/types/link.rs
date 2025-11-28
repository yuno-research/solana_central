use solana_sdk::pubkey::Pubkey;

/**
Check if there exists a "link" between two accounts. For there to be a link, both accounts must be on curve.
We return link as a 64 byte array with first 32 bytes being "smaller" account and last 32 bytes being "larger" account.

Accounts are compared from most to least significant bit.

In addition to storing the 2 wallets, we also store the earliest tx where they were first observed
linking, and we order by block_time, slot, index, and atomic_instruction_index.
*/

#[derive(Clone)]
pub struct Link {
  pub link: [u8; 64],
  pub block_time: u64,
  pub slot: u64,
  pub index: u64,
  pub atomic_instruction_index: u8,
}

impl Link {
  pub fn check_link(
    account1: &Pubkey,
    account2: &Pubkey,
    block_time: u64,
    slot: u64,
    index: u64,
    atomic_instruction_index: u8,
  ) -> Option<Self> {
    if !(account1.is_on_curve() && account2.is_on_curve()) {
      return None;
    }

    let mut link = [0u8; 64];
    if account1 < account2 {
      link[0..32].copy_from_slice(account1.as_ref());
      link[32..64].copy_from_slice(account2.as_ref());
    } else if account1 > account2 {
      link[0..32].copy_from_slice(account2.as_ref());
      link[32..64].copy_from_slice(account1.as_ref());
    }
    // Two identical accounts have no link
    else {
      return None;
    }
    Some(Self {
      link,
      block_time,
      slot,
      index,
      atomic_instruction_index,
    })
  }

  // Debug tostring for link
  pub fn to_string(&self) -> String {
    let wallet_1 = Pubkey::new_from_array(self.link[0..32].try_into().expect("slice with incorrect length"));
    let wallet_2 = Pubkey::new_from_array(self.link[32..64].try_into().expect("slice with incorrect length"));
    format!(
      "Wallet 1: {}, Wallet 2: {}, Block Time: {}, Slot: {}, Index: {}, Atomic Instruction Index: {}",
      wallet_1, wallet_2, self.block_time, self.slot, self.index, self.atomic_instruction_index
    )
  }
}