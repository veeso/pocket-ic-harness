# pocket-ic-harness

[![license-mit](https://img.shields.io/crates/l/pocket-ic-harness.svg?logo=rust)](https://opensource.org/licenses/MIT)
[![repo-stars](https://img.shields.io/github/stars/veeso/pocket-ic-harness?style=flat)](https://github.com/veeso/pocket-ic-harness/stargazers)
[![downloads](https://img.shields.io/crates/d/pocket-ic-harness.svg?logo=rust)](https://crates.io/crates/pocket-ic-harness)
[![latest-version](https://img.shields.io/crates/v/pocket-ic-harness.svg?logo=rust)](https://crates.io/crates/pocket-ic-harness)
[![conventional-commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-%23FE5196?logo=conventionalcommits&logoColor=white)](https://conventionalcommits.org)

[![ci](https://github.com/veeso/pocket-ic-harness/actions/workflows/ci.yml/badge.svg)](https://github.com/veeso/pocket-ic-harness/actions)
[![docs](https://docs.rs/pocket-ic-harness/badge.svg?logo=rust)](https://docs.rs/pocket-ic-harness)

A test harness for Internet Computer canisters using PocketIC.

## Overview

pocket-ic-harness provides reusable utilities for integration testing IC canisters
with PocketIC:

- **`Canister` trait** - define your canisters, their WASM paths, and their init arguments
- **`PocketIcTestEnv<C>`** - generic test environment that creates and installs every canister
- **`PocketIcClient`** - typed query/update calls with Candid encoding
- **`init_new_agent()`** - create IC agents against PocketIC endpoints
- **`#[pocket_ic_harness::test]`** - proc-macro for automatic setup/teardown

## Quick Start

Add to your `Cargo.toml`:

```toml
[dev-dependencies]
pocket-ic-harness = "16" # pocket-ic-harness version matches pocket-ic version
```

Define your canisters:

```rust
use std::path::Path;

use candid::Encode;
use pocket_ic_harness::{Canister, PocketIcTestEnv};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum MyCanister {
    Backend,
}

impl Canister for MyCanister {
    fn as_path(&self) -> &Path {
        match self {
            MyCanister::Backend => Path::new("path/to/backend.wasm.gz"),
        }
    }

    fn all_canisters() -> &'static [Self] {
        &[Self::Backend]
    }

    fn init_arg(&self, _env: &PocketIcTestEnv<Self>) -> Vec<u8> {
        Encode!(&()).unwrap()
    }
}
```

An init argument can reference another canister, because every canister in
`all_canisters()` is created before any of them is installed:

```rust
fn init_arg(&self, env: &PocketIcTestEnv<Self>) -> Vec<u8> {
    match self {
        MyCanister::Backend => Encode!(&()).unwrap(),
        MyCanister::Frontend => {
            let backend = env.canister_id(&MyCanister::Backend);
            Encode!(&backend).unwrap()
        }
    }
}
```

Write tests with the proc-macro. Every canister is already installed:

```rust
#[pocket_ic_harness::test]
async fn test_my_canister(ctx: PocketIcTestEnv<MyCanister>) {
    let canister_id = ctx.canister_id(&MyCanister::Backend);
    // test your canister...
}
```

## License

Licensed under the MIT license. See [LICENSE](LICENSE) for details.
