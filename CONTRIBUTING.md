# Contributing to nidx

Thank you for your interest in contributing! This document explains how to get started.

## Prerequisites

- [Rust](https://rustup.rs/) 1.85 or later
- [Python](https://www.python.org/) 3.9+ (for Python bindings)
- [Node.js](https://nodejs.org/) 22+ (for WASM bindings)
- [wasm-bindgen-cli](https://rustwasm.github.io/wasm-bindgen/) (for WASM bindings)
- [maturin](https://github.com/PyO3/maturin) (for Python bindings)

## Building

### Core library

```sh
cargo build
cargo test
```

### Python bindings

```sh
cd bindings/python
pip install maturin pytest
maturin develop
pytest tests/
```

### WASM bindings

```sh
cargo build --lib --target wasm32-unknown-unknown -p nidx-wasm
cargo test --target wasm32-unknown-unknown -p nidx-wasm
```

## Adding a new country

1. Create `src/country/<country>.rs` with its own `NidInfo`, `NidError`, `decode()`, and `is_valid()`.
2. Add `pub mod <country>;` to `src/country/mod.rs`.
3. Add `pub use country::<country>;` to `src/lib.rs`.
4. Add binding wrappers in `bindings/python/src/lib.rs` and `bindings/wasm/src/lib.rs`.
5. Add tests for the new country in the module, integration tests, and binding tests.

## Code style

- Run `cargo fmt --all` before committing.
- Run `cargo clippy --all-targets --all-features -- -D warnings` and fix any warnings.

## Pull requests

1. Fork the repository and create a feature branch.
2. Make your changes, add tests if applicable.
3. Ensure all checks pass: `cargo fmt --all -- --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test --all`.
4. Open a pull request against `main`.

## Releases

Releases are automated via GitHub Actions. Pushing a tag like `v0.2.0` triggers publication to crates.io, PyPI, and npm.
