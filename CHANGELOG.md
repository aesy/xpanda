# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- The CLI is published to crates.io as `xpanda-cli`, so it can be installed with `cargo install xpanda-cli`.
- Prebuilt binaries for Linux ARM64 and macOS (Apple silicon).
- A landing page with a live demo that runs in the browser.

### Changed

- The CLI binary is called `xpanda` instead of `xpanda-cli` when built from source or installed with Cargo.
- Linux binaries are statically linked against musl, so they no longer depend on the system's glibc version.
- The minimum supported Rust version is lowered from 1.96.0 to 1.86.0.

## [0.2.0] - 2026-07-04

### Added

- Substring patterns: `${VAR:offset}` and `${VAR:offset:length}`.
- Prefix and suffix removal patterns: `${VAR#pattern}`, `${VAR##pattern}`, `${VAR%pattern}` and `${VAR%%pattern}`.
- Replacement patterns: `${VAR/pattern/repl}` and `${VAR//pattern/repl}`.
- Case modification patterns: `${VAR^}`, `${VAR^^}`, `${VAR,}`, `${VAR,,}`, `${VAR~}` and `${VAR~~}`.

### Changed

- Whitespace within brackets is skipped, so `${ VAR }` is the same as `${VAR}`.
- Smaller release binaries.

### Fixed

- A potential integer underflow when evaluating patterns.

## [0.1.0] - 2022-10-02

### Added

- Parameter expansion for `$VAR`, `${VAR}`, default, alternative and error patterns, `${#VAR}`, `${#}` and
  `${!VAR}` indirection.
- Positional variables, named variables, variable files and environment variables as value sources.
- Escaping of `$` to skip expansion.
- Input and output file paths for the CLI.
- A public Rust API.

[Unreleased]: https://github.com/aesy/xpanda/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/aesy/xpanda/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/aesy/xpanda/releases/tag/v0.1.0
