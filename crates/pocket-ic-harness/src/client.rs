use candid::{CandidType, Decode, Principal};
use pocket_ic::RejectResponse;
use pocket_ic::nonblocking::PocketIc;

/// A typed client for making query and update calls to a canister via PocketIC.
///
/// Automatically detects live vs simulated mode and uses the appropriate
/// call mechanism.
pub struct PocketIcClient<'a> {
    caller: Principal,
    canister: Principal,
    pocket_ic: &'a PocketIc,
}

impl<'a> PocketIcClient<'a> {
    /// Creates a new PocketIC client for the given canister and caller.
    pub fn new(canister: Principal, caller: Principal, pocket_ic: &'a PocketIc) -> Self {
        Self {
            caller,
            canister,
            pocket_ic,
        }
    }

    /// Performs a query call and decodes the response with Candid.
    pub async fn query<R>(&self, method: &str, payload: Vec<u8>) -> Result<R, RejectResponse>
    where
        R: for<'de> candid::Deserialize<'de> + CandidType,
    {
        let reply = self
            .pocket_ic
            .query_call(self.canister, self.caller, method, payload)
            .await?;
        let ret_type: R = Decode!(&reply, R).expect("Failed to decode reply");

        Ok(ret_type)
    }

    /// Performs an update call and decodes the response with Candid.
    ///
    /// In live mode, uses `submit_call` + `await_call_no_ticks`.
    /// In simulated mode, uses `update_call` directly.
    pub async fn update<R>(&self, method: &str, payload: Vec<u8>) -> Result<R, RejectResponse>
    where
        R: for<'de> candid::Deserialize<'de> + CandidType,
    {
        let is_live = self.pocket_ic.url().is_some();
        let reply = if is_live {
            let id = self
                .pocket_ic
                .submit_call(self.canister, self.caller, method, payload)
                .await?;
            self.pocket_ic.await_call_no_ticks(id).await
        } else {
            self.pocket_ic
                .update_call(self.canister, self.caller, method, payload)
                .await
        }?;

        let ret_type = candid::Decode!(&reply, R).expect("Failed to decode reply");

        Ok(ret_type)
    }
}
