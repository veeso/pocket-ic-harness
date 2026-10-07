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
│       ├── lib.rs              # Canister trait, re-exports
│       ├── actor.rs            # Test principals: admin(), alice(), bob()
│       ├── agent.rs            # init_new_agent() for IC agent creation
│       ├── client.rs           # PocketIcClient typed query/update wrapper
│       ├── pocket_ic.rs        # PocketIcTestEnv<C> test environment
│       └── pocket_ic/
│           └── env.rs          # PocketIC binary download and initialization
│
└── pocket-ic-harness-macro/    # Proc-macro crate
    └── src/
        └── lib.rs              # #[pocket_ic_harness::test] attribute macro

integration-tests/
├── test-canister/              # Simple counter canister for testing (publish = false)
│   └── src/lib.rs              # init (peer), get_peer, get_count (query), increment, set_count (update)
│
└── pocket-ic-tests/            # Integration tests using pocket-ic-harness (publish = false)
    ├── src/lib.rs              # TestCanister enum (Counter, Mirror) with init args
    └── tests/
        └── integration_tests.rs
```

## Core API

- **`Canister`** trait: user-defined enum identifying canisters, their WASM paths, and init arguments
  - `fn as_path(&self) -> &Path`
  - `fn all_canisters() -> &'static [Self]`
  - `fn init_arg(&self, env: &PocketIcTestEnv<Self>) -> Vec<u8>`
- **`PocketIcTestEnv<C: Canister>`**: generic test environment, `init()` creates every canister
  in `all_canisters()`, then installs each one with its `init_arg()`
- **`PocketIcClient`**: typed wrapper for query/update calls with live mode detection
- **`#[pocket_ic_harness::test]`**: proc-macro wrapping async test with setup/teardown

## Common Commands

```bash
# Code quality
just check                   # fmt_check + clippy -D warnings + doc + deny + test
just fmt                     # Format with dprint (Rust via nightly rustfmt, MD, TOML, YAML)
just fmt_check               # Check formatting
just clippy                  # Run clippy
just doc                     # Build docs, deny warnings
just deny                    # cargo-deny: advisories, licenses, bans, sources
just scan_secrets            # trufflehog scan
just setup_githooks          # Enable .githooks/pre-commit

# Changelog
just changelog <version>     # Generate CHANGELOG.md with git-cliff
just changelog_preview <version>

# Build check
cargo check --workspace

# Testing
just test_all                # Run unit tests + integration tests
just test                    # Run unit and doc tests (excludes integration tests)
just integration_test        # Build test canister WASM and run integration tests
just build_test_canister     # Build test canister WASM to .artifact/

# Publish
just publish_all             # Publish all crates in dependency order
```

### Integration Test Prerequisites

Integration tests require:

- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- `ic-wasm`: install from https://github.com/dfinity/ic-wasm/releases
- `gzip`: typically pre-installed on most systems

## Conventions

- Uses Conventional Commits
- Always run `just fmt` after changing Rust code
- Design docs and plans go in `docs/superpowers/`, never in project root
