## Version 0.3.0 (2026-04-02)

### Breaking changes

- `Token::token()` returns `&str` instead of `String`
- `Token::new()` is now `unsafe`
- `ResultType::RecognitionPartial` and `RecognitionFinal` carry `Vec<Token>` instead of `Option<Vec<Token>>`
- `Config::new()` returns `Config` directly instead of `Result<Config, ...>`
- `ConfigBuilder::build()` returns `Config` directly instead of `Result<Config, ...>`
- `Session::feed_pcm16()` takes `&[u8]` instead of `Vec<u8>`
- `Session::free()` removed (Drop handles cleanup)

### Fixes

- Fix double-free: `Session::free()` and `Drop` both called `aas_free`
- Fix unsound `Token::new`: dereferences raw pointer without `unsafe`
- Fix `from_ne_bytes` to `from_le_bytes` in `feed_pcm16` for cross-platform correctness
- Fix use-after-free in model test
- Remove misleading `#[repr(i32)]` from data-carrying `ResultType` enum

### Improvements

- Zero-copy audio feeding on little-endian platforms via `align_to`
- Replace `impl Into` with idiomatic `impl From` (3 sites)
- Bump `aprilasr-sys` dependency to 0.1.4
- Update license SPDX to `GPL-3.0-or-later`
- Clean up tests and documentation

## Version 0.2.0 (2024-02-27)

### Features

- **Multiple Sessions Support:** Introduces a new feature allowing the creation of multiple sessions for the same `Model`. The `Session` struct has been enhanced, updating the reference handling from an `Arc` to a simple lifetime reference (`&'a Model`). This modification enables users to create and manage multiple sessions concurrently, providing improved flexibility and concurrent processing capabilities.

## Version 0.1.2 (2024-02-15)

### Changes

- Internal improvements and bug fixes.

## Version 0.1.1 (2024-02-14)

### Changes

- Internal adjustments and enhancements.

## Version 0.1.0 (2024-02-13)

### Initial Release

- The initial release of the project. Basic functionality and structure introduced.
