# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

pocket-ic-harness is a Rust library providing a reusable test harness for Internet Computer canisters
using PocketIC. It provides generic utilities for integration testing so IC projects don't need to
reimplement test infrastructure.

## Workspace Structure

```
crates/
├── pocket-ic-harness/          # Main library
│   └── src/
│       ├── lib.rs              # Canister, CanisterSetup traits, re-exports
│       ├── actor.rs            # Test principals: admin(), alice(), bob()
│       ├── agent.rs            # init_new_agent() for IC agent creation
│       ├── client.rs           # PocketIcClient typed query/update wrapper
│       ├── pocket_ic.rs        # PocketIcTestEnv<S> test environment
│       └── pocket_ic/
│           └── env.rs          # PocketIC binary download and initialization
│
└── pocket-ic-harness-macro/    # Proc-macro crate
    └── src/
        └── lib.rs              # #[pocket_ic_harness::test] attribute macro
```

## Core API

- **`Canister`** trait: user-defined enum identifying canisters and their WASM paths
- **`CanisterSetup`** trait: defines how canisters are installed before each test
  - Associated type `Canister: Canister`
  - `async fn setup(env: &mut PocketIcTestEnv<Self>)`
- **`PocketIcTestEnv<S: CanisterSetup>`**: generic test environment, `init()` calls `S::setup()` automatically
- **`PocketIcClient`**: typed wrapper for query/update calls with live mode detection
- **`#[pocket_ic_harness::test]`**: proc-macro wrapping async test with setup/teardown

## Common Commands

```bash
# Code quality
just check_code              # Format check (nightly) + clippy with -D warnings
just fmt_nightly             # Format with nightly rustfmt
just clippy                  # Run clippy

# Build check
cargo check --workspace

# Publish
just publish_all             # Publish all crates in dependency order
```

## Conventions

- Uses Conventional Commits
- Always run `cargo +nightly fmt` after changing Rust code
- Design docs and plans go in `docs/superpowers/`, never in project root
