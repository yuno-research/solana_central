use crate::central_context::central_context::CentralContext;

impl CentralContext {
  /// Refresh current slot and latest blockhash from JSON RPC
  ///
  /// Updates the `current_slot` and `latest_blockhash` fields in the context.
  /// Note: In production, these values are typically updated via gRPC streams rather than polling.
  pub fn fetch_current_slot_blockhash(&self) {
    let mut current_slot = self.current_slot.write().unwrap();
    *current_slot = self.json_rpc_client.get_slot().unwrap();
    let mut latest_blockhash = self.latest_blockhash.write().unwrap();
    *latest_blockhash = self.json_rpc_client.get_latest_blockhash().unwrap();
  }
}
