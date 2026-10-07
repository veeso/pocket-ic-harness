# Changelog

All notable changes to this project are documented in this file.

## 16.0.0

Released on 2026-10-07

### Breaking changes

- collapse CanisterSetup into Canister trait (#3)

> CanisterSetup is removed. Canister requires a new init_arg() method. PocketIcTestEnv<S: CanisterSetup> becomes PocketIcTestEnv<C: Canister>. PocketIcTestEnv::install_canister is no longer public.

### Added

- bump pocket-ic to 16.0.0 (#2)
- Breaking: collapse CanisterSetup into Canister trait (#3)

> Canister now declares its own Candid init argument through init_arg(), and PocketIcTestEnv installs every canister listed in all_canisters() automatically, in order, after creating all of them. The separate CanisterSetup trait and its marker types are gone, so the test environment is generic over the canister enum directly. The README Quick Start, crate docs, and macro docs describe the new API; the README example previously omitted all_canisters() and did not compile.
>
> The test canister gains an optional peer principal init argument and a get_peer query, used by new integration tests that cover cross-canister init arguments and the panic raised when an init argument references a canister missing from all_canisters().

## 0.2.0

Released on 2026-04-01

### Breaking changes

- pre-create canisters before installation to allow referencing canister IDs in init args

> `Canister` trait now requires `Sized + Clone + 'static`
> bounds and a new `all_canisters()` method.

### Added

- Breaking: pre-create canisters before installation to allow referencing canister IDs in init args

> Add `all_canisters()` method to `Canister` trait so the test environment
> can create all canisters upfront. This lets `CanisterSetup::setup` access
> canister principals when building init arguments for other canisters.

## 0.1.0

Released on 2026-03-31

### Added

- initial pocket-ic-harness implementation

> Reusable test harness for Internet Computer canisters using PocketIC,
> extracted from the wasm-dbms project.
>
> - Canister trait for identifying canisters and their WASM binaries
> - CanisterSetup trait for automatic canister installation before tests
> - PocketIcTestEnv<S> generic test environment with canister registry
> - PocketIcClient typed wrapper for query/update calls
> - init_new_agent() utility for creating IC agents against PocketIC
> - Test actor principals: admin(), alice(), bob()
> - Automatic PocketIC server binary download and management
> - #[pocket_ic_harness::test] proc-macro for test setup/teardown

- add integration tests with test canister and just recipes

> Add integration-tests directory with a simple counter canister and
> pocket-ic-tests crate to validate the harness works end-to-end.
> Fix load_wasm to use paths as-is instead of resolving from
> CARGO_MANIFEST_DIR. Add test.just for building WASM and running tests,
> update CI workflow with just, ic-wasm, and wasm32-unknown-unknown target.

### Build

- set MSRV to 1.88.0
- add `full` feature to `syn`
