# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.1] - 2026-02-27

### Fixed

- Fix `wasm-bindgen-cli` install in CI by adding `--locked` flag.
- Update README and CONTRIBUTING docs for 0.2.0 changes.

## [0.2.0] - 2026-02-27

### Added

- Kosovo: `validate()` to check a 10-digit Kosovo personal number using a Mod 11 checksum with weights `[4,3,2,7,6,5,4,3,2]`. Numbers starting with `9` bypass check digit validation.
- Kosovo: `is_valid()` convenience function for boolean validation.
- Python bindings for Kosovo (`kosovo.validate`, `kosovo.is_valid`).
- WASM/JS bindings for Kosovo (`Kosovo.validate`, `Kosovo.isValid`).

### Changed

- **Breaking (JS/WASM):** Replaced flat function exports (`albaniaDecode`, `albaniaIsValid`, `kosovoValidate`, `kosovoIsValid`) with namespace structs (`Albania.decode`, `Albania.isValid`, `Kosovo.validate`, `Kosovo.isValid`).
- Switched CI and release workflows from wasm-pack to wasm-bindgen-cli (pinned to 0.2.113).

## [0.1.0] - 2026-02-27

### Added

- Albania: `decode()` to validate and extract date of birth, sex, and national status from an Albanian NID.
- Albania: `is_valid()` convenience function for boolean validation.
- Python bindings via PyO3 (published to PyPI as `nidx`).
- JavaScript/WASM bindings via wasm-bindgen (published to npm as `nidx`).
- Optional `serde` feature for serialization support.

[Unreleased]: https://github.com/dedal-io/nidx/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/dedal-io/nidx/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/dedal-io/nidx/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/dedal-io/nidx/releases/tag/v0.1.0
