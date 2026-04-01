# Changelog

## 0.2.0

Released on 2026-04-01

### ⚠ Breaking Changes

- pre-create canisters before installation to allow referencing canister IDs in init args
  > `Canister` trait now requires `Sized + Clone + 'static`
bounds and a new `all_canisters()` method.

### Added

- 💥 pre-create canisters before installation to allow referencing canister IDs in init args
  > Add `all_canisters()` method to `Canister` trait so the test environment
  > can create all canisters upfront. This lets `CanisterSetup::setup` access
  > canister principals when building init arguments for other canisters.
