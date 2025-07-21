use std::str::FromStr;

use solana_sdk::pubkey::Pubkey;
use solana_transaction_status_client_types::option_serializer::OptionSerializer;
use solana_transaction_status_client_types::EncodedConfirmedTransactionWithStatusMeta;

/// Get all account keys from a transaction, including:
///   1) the “static” keys (via `static_account_keys()`), and
///   2) any lookup‐table addresses (writable + readonly)
/// so that every instruction’s account index can be looked up correctly.
///
/// Returns a Vec<Pubkey> in exactly this order:
///   1. static account keys
///   2. loaded‐addresses.writable
///   3. loaded‐addresses.readonly
///
/// Any string that fails to parse as a Pubkey is simply skipped.
pub fn get_account_keys(tx: &EncodedConfirmedTransactionWithStatusMeta) -> Vec<Pubkey> {
  let mut result: Vec<Pubkey> = Vec::new();

  // (1) STATIC KEYS
  // ----------------
  // `tx.transaction.transaction` is an `EncodedTransaction`.  Calling `.decode()` gives us
  // a `VersionedTransaction`.  We can then call `message.static_account_keys()` to get &[Pubkey].
  // 1) STATIC ACCOUNT KEYS (legacy or versioned message)
  // encoded_tx.message.accountKeys is a Vec<String> of base58 pubkey strings
  for key_str in tx
    .transaction
    .transaction
    .decode()
    .unwrap()
    .message
    .static_account_keys()
  {
    result.push(*key_str);
  }

  //
  // (2) LOADED ADDRESSES (if present)
  //
  // The field `transaction.meta` is an `Option<TransactionStatusMeta>`. We borrow it,
  // then call `.loaded_addresses.as_ref()` to get `OptionSerializer<&UiLoadedAddresses>`,
  // and finally pattern‐match on `OptionSerializer::Some(ui_loaded)`.
  if let Some(meta_ref) = &tx.transaction.meta {
    match meta_ref.loaded_addresses.as_ref() {
      OptionSerializer::Some(ui_loaded) => {
        // `ui_loaded` is a `&UiLoadedAddresses`.
        for key_str in &ui_loaded.writable {
          if let Ok(pubkey) = Pubkey::from_str(key_str) {
            result.push(pubkey);
          }
        }
        for key_str in &ui_loaded.readonly {
          if let Ok(pubkey) = Pubkey::from_str(key_str) {
            result.push(pubkey);
          }
        }
      }
      OptionSerializer::None | OptionSerializer::Skip => {
        // No loaded addresses to append.
      }
    }
  }

  result
}
