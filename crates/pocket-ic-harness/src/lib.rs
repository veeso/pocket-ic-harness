//! A test harness for Internet Computer canisters using PocketIC.
//!
//! This crate provides reusable utilities for integration testing IC canisters:
//!
//! - [`Canister`] trait — define your canisters and their WASM paths
//! - [`CanisterSetup`] trait — define how canisters are installed before each test
//! - [`PocketIcTestEnv`] — generic test environment with canister installation and registry
//! - [`PocketIcClient`] — typed query/update calls with Candid encoding
//! - [`init_new_agent`] — create IC agents against PocketIC endpoints
//! - [`test`] — proc-macro attribute for automatic setup/teardown
//!
//! # Quick Start
//!
//! Define your canisters and setup:
//!
//! ```rust,ignore
//! use std::path::Path;
//! use candid::Encode;
//! use pocket_ic_harness::{Canister, CanisterSetup, PocketIcTestEnv};
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
//! }
//!
//! struct MySetup;
//!
//! impl CanisterSetup for MySetup {
//!     type Canister = MyCanister;
//!
//!     async fn setup(env: &mut PocketIcTestEnv<Self>) {
//!         let init_arg = Encode!(&()).unwrap();
//!         env.install_canister(MyCanister::Backend, init_arg).await;
//!     }
//! }
//! ```
//!
//! Write tests with the proc-macro — canisters are already installed:
//!
//! ```rust,ignore
//! #[pocket_ic_harness::test]
//! async fn test_my_canister(ctx: PocketIcTestEnv<MySetup>) {
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

/// Trait for identifying a canister and locating its WASM binary.
///
/// Implement this on an enum representing your project's canisters.
///
/// # Example
///
/// ```rust
/// use std::path::Path;
/// use pocket_ic_harness::Canister;
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
/// }
/// ```
pub trait Canister: Hash + Eq {
    /// Returns the path to the WASM binary for this canister.
    ///
    /// The path is relative to the crate's `CARGO_MANIFEST_DIR`.
    fn as_path(&self) -> &Path;
}

/// Trait for defining canister installation and configuration.
///
/// Implement this to specify which canisters to install and how to configure
/// them during test environment initialization. The setup is called
/// automatically by [`PocketIcTestEnv::init`].
///
/// # Example
///
/// ```rust,ignore
/// use candid::Encode;
/// use pocket_ic_harness::{CanisterSetup, PocketIcTestEnv};
///
/// struct MySetup;
///
/// impl CanisterSetup for MySetup {
///     type Canister = MyCanister;
///
///     async fn setup(env: &mut PocketIcTestEnv<Self>) {
///         let init_arg = Encode!(&()).unwrap();
///         env.install_canister(MyCanister::Backend, init_arg).await;
///     }
/// }
/// ```
pub trait CanisterSetup {
    /// The canister type used by this setup.
    type Canister: Canister;

    /// Install and configure canisters in the test environment.
    fn setup(env: &mut PocketIcTestEnv<Self>) -> impl Future<Output = ()>
    where
        Self: Sized;
}
