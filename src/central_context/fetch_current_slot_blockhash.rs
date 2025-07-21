use crate::central_context::central_context::CentralContext;

/**
Use json rpc api to refresh current slot and latest blockhash. Not used in production as gRPC will
be updating these variables in central context
*/
impl CentralContext {
  pub fn fetch_current_slot_blockhash(&self) {
    let mut current_slot = self.current_slot.write().unwrap();
    *current_slot = self.json_rpc_client.get_slot().unwrap();
    let mut latest_blockhash = self.latest_blockhash.write().unwrap();
    *latest_blockhash = self.json_rpc_client.get_latest_blockhash().unwrap();
  }
}
