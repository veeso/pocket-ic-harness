//! A test harness for Internet Computer canisters using PocketIC.
//!
//! This crate provides reusable utilities for integration testing IC canisters:
//!
//! - [`Canister`] trait — define your canisters, their WASM paths, and their init arguments
//! - [`PocketIcTestEnv`] — generic test environment that creates and installs every canister
//! - [`PocketIcClient`] — typed query/update calls with Candid encoding
//! - [`init_new_agent`] — create IC agents against PocketIC endpoints
//! - [`test`] — proc-macro attribute for automatic setup/teardown
//!
//! # Quick Start
//!
//! Define your canisters:
//!
//! ```rust,ignore
//! use std::path::Path;
//! use candid::Encode;
//! use pocket_ic_harness::{Canister, PocketIcTestEnv};
//!
//! #[derive(Debug, Clone, Hash, PartialEq, Eq)]
//! enum MyCanister {
//!     Backend,
//! }
//!
//! impl Canister for MyCanister {
//!     fn as_path(&self) -> &Path {
//!         match self {
//!             MyCanister::Backend => Path::new("path/to/backend.wasm.gz"),
//!         }
//!     }
//!
//!     fn all_canisters() -> &'static [Self] {
//!         &[Self::Backend]
//!     }
//!
//!     fn init_arg(&self, _env: &PocketIcTestEnv<Self>) -> Vec<u8> {
//!         Encode!(&()).unwrap()
//!     }
//! }
//! ```
//!
//! Write tests with the proc-macro — every canister in `all_canisters()` is
//! already created and installed with its `init_arg()`:
//!
//! ```rust,ignore
//! #[pocket_ic_harness::test]
//! async fn test_my_canister(ctx: PocketIcTestEnv<MyCanister>) {
//!     let canister_id = ctx.canister_id(&MyCanister::Backend);
//!     // test your canister...
//! }
//! ```

mod actor;
mod agent;
mod client;
mod pocket_ic;

use std::hash::Hash;
use std::path::Path;

pub use pocket_ic_harness_macro::test;

pub use self::actor::{admin, alice, bob};
pub use self::agent::init_new_agent;
pub use self::client::PocketIcClient;
pub use self::pocket_ic::PocketIcTestEnv;

/// Trait describing a project's canisters: WASM location, full list, and init arguments.
///
/// Implement this on an enum with one variant per canister. The test
/// environment creates every canister returned by
/// [`all_canisters`](Self::all_canisters) before installing any of them, so
/// an [`init_arg`](Self::init_arg) can reference the principal of another
/// canister through [`PocketIcTestEnv::canister_id`].
///
/// # Examples
///
/// ```rust
/// use std::path::Path;
///
/// use candid::Encode;
/// use pocket_ic_harness::{Canister, PocketIcTestEnv};
///
/// #[derive(Debug, Clone, Hash, PartialEq, Eq)]
/// enum MyCanister {
///     Backend,
///     Frontend,
/// }
///
/// impl Canister for MyCanister {
///     fn as_path(&self) -> &Path {
///         match self {
///             MyCanister::Backend => Path::new("artifacts/backend.wasm.gz"),
///             MyCanister::Frontend => Path::new("artifacts/frontend.wasm.gz"),
///         }
///     }
///
///     fn all_canisters() -> &'static [Self] {
///         &[Self::Backend, Self::Frontend]
///     }
///
///     fn init_arg(&self, env: &PocketIcTestEnv<Self>) -> Vec<u8> {
///         match self {
///             MyCanister::Backend => Encode!(&()).unwrap(),
///             MyCanister::Frontend => {
///                 let backend = env.canister_id(&MyCanister::Backend);
///                 Encode!(&backend).unwrap()
///             }
///         }
///     }
/// }
/// ```
pub trait Canister: Hash + Eq + Sized + Clone + 'static {
    /// Returns the path to the WASM binary for this canister.
    ///
    /// The path is used as-is when loading the WASM file.
    fn as_path(&self) -> &Path;

    /// Returns a static list of all canisters of this type.
    ///
    /// The test environment creates all canisters before installing any of them,
    /// then installs them in this order.
    fn all_canisters() -> &'static [Self];

    /// Returns the Candid-encoded init argument for this canister.
    ///
    /// Called by [`PocketIcTestEnv::init`] once every canister in
    /// [`all_canisters`](Self::all_canisters) has been created, so the
    /// principal of any listed canister is available through
    /// [`PocketIcTestEnv::canister_id`].
    fn init_arg(&self, env: &PocketIcTestEnv<Self>) -> Vec<u8>;
}
