mod env;

use std::collections::HashMap;
use std::io::Read as _;

use candid::{CandidType, Decode, Principal};
use pocket_ic::nonblocking::PocketIc;
use serde::de::DeserializeOwned;

use crate::Canister;
use crate::actor::{admin, alice, bob};

const DEFAULT_CYCLES: u128 = 2_000_000_000_000_000;

/// Test environment for PocketIC-based integration tests.
///
/// Generic over a user-defined [`Canister`] type that defines which
/// canisters exist and how each one is installed.
pub struct PocketIcTestEnv<C>
where
    C: Canister,
{
    pub pic: PocketIc,
    canisters: HashMap<C, Principal>,
}

impl<C> PocketIcTestEnv<C>
where
    C: Canister,
{
    /// Initialize the test environment.
    ///
    /// Sets up PocketIC with NNS, II, fiduciary, and application subnets.
    /// Downloads the PocketIC server binary if needed.
    /// Creates every canister in [`Canister::all_canisters`], then installs
    /// each one with the bytes returned by [`Canister::init_arg`], in order.
    ///
    /// # Panics
    ///
    /// Panics if a WASM file cannot be read, or if an `init_arg` looks up a
    /// canister that is not listed in `all_canisters()`.
    pub async fn init() -> Self {
        let pic = env::init_pocket_ic()
            .await
            .with_nns_subnet()
            .with_ii_subnet()
            .with_fiduciary_subnet()
            .with_application_subnet()
            .with_max_request_time_ms(Some(30_000))
            .build_async()
            .await;

        let mut env = Self {
            pic,
            canisters: HashMap::new(),
        };

        // Create every canister before installing any of them, so that init
        // arguments can reference the principals of other canisters.
        for canister in C::all_canisters() {
            let canister_id = env
                .pic
                .create_canister_with_settings(Some(admin()), None)
                .await;
            env.pic.add_cycles(canister_id, DEFAULT_CYCLES).await;
            env.canisters.insert(canister.clone(), canister_id);
        }

        for canister in C::all_canisters() {
            let init_arg = canister.init_arg(&env);
            env.install_canister(canister, init_arg).await;
        }

        env
    }

    /// Stop the PocketIC instance. Should be called after each test.
    pub async fn stop(self) {
        self.pic.drop().await
    }

    /// Toggle between live and simulated mode.
    pub async fn live(&mut self, live: bool) {
        if live {
            self.pic.make_live(None).await;
        } else {
            self.pic.stop_live().await;
        }
    }

    /// Look up the principal of a registered canister.
    ///
    /// # Panics
    ///
    /// Panics if the canister is not listed in [`Canister::all_canisters`].
    pub fn canister_id(&self, canister: &C) -> Principal {
        *self
            .canisters
            .get(canister)
            .expect("canister not created; add it to `Canister::all_canisters()`")
    }

    /// Returns the HTTP endpoint URL if in live mode.
    pub fn endpoint(&self) -> Option<url::Url> {
        self.pic.url()
    }

    /// Returns the admin test principal.
    pub fn admin() -> Principal {
        admin()
    }

    /// Returns the Alice test principal.
    pub fn alice() -> Principal {
        alice()
    }

    /// Returns the Bob test principal.
    pub fn bob() -> Principal {
        bob()
    }

    /// Performs a query call on the given canister.
    pub async fn query<R>(
        &self,
        canister: Principal,
        caller: Principal,
        method: &str,
        payload: Vec<u8>,
    ) -> anyhow::Result<R>
    where
        R: DeserializeOwned + CandidType,
    {
        let reply = match self.pic.query_call(canister, caller, method, payload).await {
            Ok(result) => result,
            Err(e) => anyhow::bail!("Error calling {method}: {e:?}"),
        };
        let ret_type = Decode!(&reply, R)?;

        Ok(ret_type)
    }

    /// Performs an update call on the given canister.
    pub async fn update<R>(
        &self,
        canister: Principal,
        caller: Principal,
        method: &str,
        payload: Vec<u8>,
    ) -> anyhow::Result<R>
    where
        R: DeserializeOwned + CandidType,
    {
        let is_live = self.pic.url().is_some();
        let reply = if is_live {
            let id = self
                .pic
                .submit_call(canister, caller, method, payload)
                .await
                .map_err(|e| anyhow::anyhow!("Error submitting call {method}: {e:?}"))?;
            self.pic.await_call_no_ticks(id).await
        } else {
            self.pic
                .update_call(canister, caller, method, payload)
                .await
        };

        let reply = match reply {
            Ok(r) => r,
            Err(r) => anyhow::bail!("{method} was rejected: {r:?}"),
        };
        let ret_type = Decode!(&reply, R)?;

        Ok(ret_type)
    }

    /// Installs the WASM of `canister` into its pre-created canister ID.
    async fn install_canister(&mut self, canister: &C, init_arg: Vec<u8>) {
        let canister_id = self.canister_id(canister);
        let wasm_bytes = Self::load_wasm(canister);

        self.pic
            .install_canister(canister_id, wasm_bytes, init_arg, Some(admin()))
            .await;
    }

    fn load_wasm(canister: &C) -> Vec<u8> {
        let path = canister.as_path();

        let mut file = std::fs::File::open(path).unwrap_or_else(|e| {
            panic!(
                "Failed to open wasm file at {path}: {e}",
                path = path.display()
            )
        });
        let mut wasm_bytes = Vec::new();
        file.read_to_end(&mut wasm_bytes)
            .expect("Failed to read wasm file");

        wasm_bytes
    }
}
