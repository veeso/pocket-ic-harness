# Runs all tests (unit + integration)
[group('test')]
test_all: test integration_test

# Runs unit and doc tests
[group('test')]
test filter="":
    cargo test --workspace --exclude pocket-ic-tests --exclude test-canister {{ filter }}

# Builds test canisters and runs integration tests
[group('test')]
integration_test filter="": build_test_canister
    cargo test -p pocket-ic-tests {{ filter }}

# Builds the test canister WASM
[group('test')]
build_test_canister: _pre_build (_build_canister "test-canister" "test_canister")

[private]
_pre_build:
    mkdir -p {{ WASM_DIR }}

[private]
_build_canister crate wasm_name:
    cargo build -p {{ crate }} --target wasm32-unknown-unknown --release
    ic-wasm "target/wasm32-unknown-unknown/release/{{ wasm_name }}.wasm" -o "{{ WASM_DIR }}/{{ wasm_name }}.wasm" shrink
    gzip -k "{{ WASM_DIR }}/{{ wasm_name }}.wasm" --force
