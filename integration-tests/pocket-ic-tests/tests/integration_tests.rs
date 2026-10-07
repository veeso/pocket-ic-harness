use std::path::Path;

use candid::{Encode, Principal};
use pocket_ic_harness::{Canister, PocketIcTestEnv};
use pocket_ic_tests::TestCanister;
use test_canister::SetCountArg;

#[pocket_ic_harness::test]
async fn test_should_query_initial_count(env: PocketIcTestEnv<TestCanister>) {
    let canister_id = env.canister_id(&TestCanister::Counter);
    let count: u64 = env
        .query(
            canister_id,
            PocketIcTestEnv::<TestCanister>::admin(),
            "get_count",
            Encode!(&()).unwrap(),
        )
        .await
        .expect("query failed");

    assert_eq!(count, 0);
}

#[pocket_ic_harness::test]
async fn test_should_increment_counter(env: PocketIcTestEnv<TestCanister>) {
    let canister_id = env.canister_id(&TestCanister::Counter);
    let admin = PocketIcTestEnv::<TestCanister>::admin();

    let count: u64 = env
        .update(canister_id, admin, "increment", Encode!(&()).unwrap())
        .await
        .expect("update failed");
    assert_eq!(count, 1);

    let count: u64 = env
        .update(canister_id, admin, "increment", Encode!(&()).unwrap())
        .await
        .expect("update failed");
    assert_eq!(count, 2);

    let count: u64 = env
        .query(canister_id, admin, "get_count", Encode!(&()).unwrap())
        .await
        .expect("query failed");
    assert_eq!(count, 2);
}

#[pocket_ic_harness::test]
async fn test_should_set_count(env: PocketIcTestEnv<TestCanister>) {
    let canister_id = env.canister_id(&TestCanister::Counter);
    let admin = PocketIcTestEnv::<TestCanister>::admin();

    let arg = SetCountArg { value: 42 };
    let count: u64 = env
        .update(canister_id, admin, "set_count", Encode!(&arg).unwrap())
        .await
        .expect("update failed");
    assert_eq!(count, 42);

    let count: u64 = env
        .query(canister_id, admin, "get_count", Encode!(&()).unwrap())
        .await
        .expect("query failed");
    assert_eq!(count, 42);
}

#[pocket_ic_harness::test]
async fn test_should_return_none_peer_when_installed_without_peer(
    env: PocketIcTestEnv<TestCanister>,
) {
    let canister_id = env.canister_id(&TestCanister::Counter);
    let peer: Option<Principal> = env
        .query(
            canister_id,
            PocketIcTestEnv::<TestCanister>::admin(),
            "get_peer",
            Encode!(&()).unwrap(),
        )
        .await
        .expect("query failed");

    assert_eq!(peer, None);
}

#[pocket_ic_harness::test]
async fn test_should_pass_peer_canister_id_in_init_arg(env: PocketIcTestEnv<TestCanister>) {
    let counter_id = env.canister_id(&TestCanister::Counter);
    let mirror_id = env.canister_id(&TestCanister::Mirror);
    assert_ne!(counter_id, mirror_id);

    let peer: Option<Principal> = env
        .query(
            mirror_id,
            PocketIcTestEnv::<TestCanister>::admin(),
            "get_peer",
            Encode!(&()).unwrap(),
        )
        .await
        .expect("query failed");

    assert_eq!(peer, Some(counter_id));
}

/// A canister enum whose `all_canisters()` forgets a variant that another
/// variant's init argument depends on.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum IncompleteCanister {
    Listed,
    Unlisted,
}

impl Canister for IncompleteCanister {
    fn as_path(&self) -> &Path {
        Path::new("../../.artifact/test_canister.wasm.gz")
    }

    fn all_canisters() -> &'static [Self] {
        &[Self::Listed]
    }

    fn init_arg(&self, env: &PocketIcTestEnv<Self>) -> Vec<u8> {
        let unlisted = env.canister_id(&IncompleteCanister::Unlisted);
        Encode!(&Some(unlisted)).unwrap()
    }
}

#[tokio::test]
#[should_panic(expected = "canister not created; add it to `Canister::all_canisters()`")]
async fn test_should_panic_when_init_arg_references_unlisted_canister() {
    PocketIcTestEnv::<IncompleteCanister>::init().await;
}
