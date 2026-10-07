use std::path::Path;

use candid::Encode;
use pocket_ic_harness::{Canister, PocketIcTestEnv};

/// Canisters available in the test environment.
///
/// `Mirror` is a second install of the counter WASM whose init argument
/// carries the principal of `Counter`, listed before `Counter` on purpose to
/// prove that installation order does not matter for init arguments.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum TestCanister {
    Counter,
    Mirror,
}

impl Canister for TestCanister {
    fn as_path(&self) -> &Path {
        match self {
            TestCanister::Counter | TestCanister::Mirror => {
                Path::new("../../.artifact/test_canister.wasm.gz")
            }
        }
    }

    fn all_canisters() -> &'static [Self] {
        &[Self::Mirror, Self::Counter]
    }

    fn init_arg(&self, env: &PocketIcTestEnv<Self>) -> Vec<u8> {
        match self {
            TestCanister::Counter => Encode!(&()).unwrap(),
            TestCanister::Mirror => {
                let counter = env.canister_id(&TestCanister::Counter);
                Encode!(&Some(counter)).unwrap()
            }
        }
    }
}
