use candid::Encode;
use pocket_ic_harness::PocketIcTestEnv;
use pocket_ic_tests::{TestCanister, TestSetup};
use test_canister::SetCountArg;

#[pocket_ic_harness::test]
async fn test_should_query_initial_count(env: PocketIcTestEnv<TestSetup>) {
    let canister_id = env.canister_id(&TestCanister::Counter);
    let count: u64 = env
        .query(
            canister_id,
            PocketIcTestEnv::<TestSetup>::admin(),
            "get_count",
            Encode!(&()).unwrap(),
        )
        .await
        .expect("query failed");

    assert_eq!(count, 0);
}

#[pocket_ic_harness::test]
async fn test_should_increment_counter(env: PocketIcTestEnv<TestSetup>) {
    let canister_id = env.canister_id(&TestCanister::Counter);
    let admin = PocketIcTestEnv::<TestSetup>::admin();

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
async fn test_should_set_count(env: PocketIcTestEnv<TestSetup>) {
    let canister_id = env.canister_id(&TestCanister::Counter);
    let admin = PocketIcTestEnv::<TestSetup>::admin();

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
