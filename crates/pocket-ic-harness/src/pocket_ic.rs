mod env;

use std::collections::HashMap;
use std::io::Read as _;
use std::path::PathBuf;

use candid::{CandidType, Decode, Principal};
use pocket_ic::nonblocking::PocketIc;
use serde::de::DeserializeOwned;

use crate::CanisterSetup;
use crate::actor::{admin, alice, bob};

const DEFAULT_CYCLES: u128 = 2_000_000_000_000_000;

/// Test environment for PocketIC-based integration tests.
///
/// Generic over a user-defined [`CanisterSetup`] type that defines
/// which canisters to install and how to configure them.
pub struct PocketIcTestEnv<S>
where
    S: CanisterSetup,
{
    pub pic: PocketIc,
    canisters: HashMap<S::Canister, Principal>,
}

impl<S> PocketIcTestEnv<S>
where
    S: CanisterSetup,
{
    /// Initialize the test environment.
    ///
    /// Sets up PocketIC with NNS, II, fiduciary, and application subnets.
    /// Downloads the PocketIC server binary if needed.
    /// Calls [`CanisterSetup::setup`] to install canisters.
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

        S::setup(&mut env).await;

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

    /// Install a canister into the test environment.
    ///
    /// Creates the canister, loads and installs the WASM binary,
    /// and registers the canister principal in the internal registry.
    ///
    /// Returns the canister principal.
    pub async fn install_canister(
        &mut self,
        canister: S::Canister,
        init_arg: Vec<u8>,
    ) -> Principal {
        let canister_id = self
            .pic
            .create_canister_with_settings(Some(admin()), None)
            .await;
        self.pic.add_cycles(canister_id, DEFAULT_CYCLES).await;

        let wasm_bytes = Self::load_wasm(&canister);

        self.pic
            .install_canister(canister_id, wasm_bytes, init_arg, Some(admin()))
            .await;

        self.canisters.insert(canister, canister_id);

        canister_id
    }

    /// Look up the principal of an installed canister.
    ///
    /// # Panics
    ///
    /// Panics if the canister has not been installed.
    pub fn canister_id(&self, canister: &S::Canister) -> Principal {
        *self
            .canisters
            .get(canister)
            .expect("canister not installed")
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
            Err(e) => anyhow::bail!("Error calling {}: {:?}", method, e),
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
                .map_err(|e| anyhow::anyhow!("Error submitting call {}: {:?}", method, e))?;
            self.pic.await_call_no_ticks(id).await
        } else {
            self.pic
                .update_call(canister, caller, method, payload)
                .await
        };

        let reply = match reply {
            Ok(r) => r,
            Err(r) => anyhow::bail!("{} was rejected: {:?}", method, r),
        };
        let ret_type = Decode!(&reply, R)?;

        Ok(ret_type)
    }

    fn load_wasm(canister: &S::Canister) -> Vec<u8> {
        use crate::Canister;

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(canister.as_path());

        let mut file = std::fs::File::open(&path)
            .unwrap_or_else(|e| panic!("Failed to open wasm file at {}: {e}", path.display()));
        let mut wasm_bytes = Vec::new();
        file.read_to_end(&mut wasm_bytes)
            .expect("Failed to read wasm file");

        wasm_bytes
    }
}
