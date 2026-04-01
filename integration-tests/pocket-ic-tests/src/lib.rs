use std::path::Path;

use candid::Encode;
use pocket_ic_harness::{Canister, CanisterSetup, PocketIcTestEnv};

/// Canisters available in the test environment.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum TestCanister {
    Counter,
}

impl Canister for TestCanister {
    fn as_path(&self) -> &Path {
        match self {
            TestCanister::Counter => Path::new("../../.artifact/test_canister.wasm.gz"),
        }
    }

    fn all_canisters() -> &'static [Self] {
        &[Self::Counter]
    }
}

/// Setup configuration that installs the counter canister.
pub struct TestSetup;

impl CanisterSetup for TestSetup {
    type Canister = TestCanister;

    async fn setup(env: &mut PocketIcTestEnv<Self>) {
        let init_arg = Encode!(&()).unwrap();
        env.install_canister(TestCanister::Counter, init_arg).await;
    }
}
