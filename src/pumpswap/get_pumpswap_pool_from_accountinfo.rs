use crate::constants::PUMP_CONSTANTS;
use crate::protocol_idls::pumpswap::PumpAmmPoolAccount;
use crate::types::pool::Pool;
use crate::types::pools::Pools;
use crate::types::pumpswap_pool::{PUMP_SWAP_FEE_VAULTS, PumpswapPool};
use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;
use spl_associated_token_account::get_associated_token_address;

impl PumpswapPool {
  pub fn from_account_info(pubkey: Pubkey, account_buffer: &[u8]) -> Self {
    let decoded_layout: PumpAmmPoolAccount = PumpAmmPoolAccount::try_from_slice(account_buffer)
      .expect("Failed to deserialize PumpSwap pool account");

    // Randomly pick one of the known fee vaults – same logic as the TS code.

    /*
    Compute the associated token account that will receive protocol fees. On pumpswap, all fees are
    collected in the quote token, so we use the quote mint.
    */
    let fee_vault_token_account =
      get_associated_token_address(&PUMP_SWAP_FEE_VAULTS[0], &decoded_layout.quote_mint);

    let (coin_creator_vault_authority, _) = Pubkey::find_program_address(
      &[b"creator_vault", decoded_layout.coin_creator.as_array()],
      &PUMP_CONSTANTS.pump_swap_program,
    );
    let coin_creator_vault_authority_token_account =
      get_associated_token_address(&coin_creator_vault_authority, &decoded_layout.quote_mint);

    Self {
      info: Pool {
        pool_address: pubkey,
        token_a_address: decoded_layout.base_mint,
        token_b_address: decoded_layout.quote_mint,
        pool_type: Pools::PumpswapAmm,
        token_a_vault_address: decoded_layout.pool_base_token_account,
        token_b_vault_address: decoded_layout.pool_quote_token_account,
      },
      pool_creator: decoded_layout.creator,
      coin_creator: decoded_layout.coin_creator,
      fee_vault: PUMP_SWAP_FEE_VAULTS[0],
      fee_vault_token_account,
      // These are fetched lazily later.
      token_a_vault_amount: 0,
      token_b_vault_amount: 0,
      coin_creator_vault_authority,
      coin_creator_vault_authority_token_account,
    }
  }
}
