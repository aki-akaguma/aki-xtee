# Changelog: aki-xtee
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Document code review report `docs/reviews/2026-06-02_code_review.2.md`

### Changed
- Organize review files into `docs/reviews/`

### Fixed
- Create file in append mode (`-a`) if it does not exist
- Manage ownership in compression wrappers

### Removed
- Unimplemented `-p` (pipe-out) option


## [0.2.1] - 2026-05-20

### Changed
- Update `flood-tide` to 0.2.14
- Update `flood-tide-gen` to 0.2.2
- Patch `getrandom` to 0.3.4
- Update `runnel` to 0.4.2

### Fixed
- Typo in error message for unsupported `.xz` files
- Remove `uninlined-format-args` clippy lint
- Remove `needless_borrow` clippy lint

### Removed
- `memx-cdy` dependency


## [0.2.0] - 2025-09-15

### Added
- Support `bzip2`
- Support `lz4`
- Specifications documentation in `specs/`
- Support `-X base_dir=dir` option
- Append mode via `-a file` option
- Additional tests
- Test case for invalid UTF-8 input

### Changed
- Implement `IntoIterator` for `execute()` arguments
- Update `runnel` to 0.4.0
- Update `rust-version-info-file` to 0.2

### Fixed
- Set `rust-version` to 1.75.0


## [0.1.25] - 2024-06-19

### Added
- GitHub Actions workflows for Ubuntu, macOS, and Windows
- Test status badges to `README.tpl`
- Miri support for tests
- Tarpaulin support in `Makefile`

### Changed
- Rename `config` to `config.toml`
- Remove `cfg(has_not_matches)`
- Update `rust-version` from 1.60.0 to 1.65.0
- Refactor `Makefile`
- Update `flood-tide` to 0.2.11
- Update `flood-tide-gen` to 0.1.22
- Update `memx-cdy` to 0.1.13
- Update `runnel` to 0.3.19
- Update `exec-target` to 0.2.9
- Update `indoc` to 2.0.5
- Update `rust-version-info-file` to 0.1.10
- Update `zstd` to 0.13.1
- Rename `fixtures/text10k.txt.gz` to `fixtures/text10k.text.gz`

### Fixed
- License files (`LICENSE-APACHE`, `LICENSE-MIT`)
- Clippy lints: `redundant_static_lifetimes`, `needless_borrow`, `bool_assert_comparison`
- Clippy lints: `uninlined_format_args`, `unused_imports`, `dead_code`, `derivable_impls`
- Update `rust-version` from 1.56.0 to 1.60.0
- File comparison on Windows
- Fix `zcat` issue on macOS by using `gzcat`

### Removed
- `COPYING` file


## [0.1.24] - 2023-01-11

### Added
- Badges to `README.tpl`
- `rust-version = "1.56.0"` to `Cargo.toml`

### Changed
- Reformat `CHANGELOG.md`
- Update `anyhow` to 1.0.68
- Update `flood-tide` to 0.2.8
- Update `flood-tide-gen` to 0.1.19
- Update `memx-cdy` to 0.1.10
- Update `runnel` to 0.3.15
- Update `flate2` to 1.0.25
- Update `xz2` to 0.1.7
- Update `zstd` to 0.12.1+zstd.1.5.2

### Fixed
- Clippy lint: implement `Eq` when deriving `PartialEq`
- Clippy lint: `uninlined_format_args`


## [0.1.23] - 2022-06-18

### Fixed
- Git log entry


## [0.1.22] - 2022-06-18

### Fixed
- Issue in `README.md`


## [0.1.21] - 2022-06-18

### Changed
- Migrate to Rust 2021 edition
- Update `cfg-iif` to 0.2.3
- Update `flood-tide` to 0.2.5
- Update `linux-procfs` to 0.3.11
- Update `memx` to 0.1.21
- Update `memx-cdy` to 0.1.8
- Update `naive_opt` to 0.1.18
- Update `runnel` to 0.3.11
- Update `assert-text` to 0.2.6
- Update `exec-target` to 0.2.6
- Update `flood-tide-gen` to 0.1.16
- Update `rust-version-info-file` to 0.1.6
- Update `semver` to 1.0.10
- Update `flate2` to 1.0.24
- Update `lzma-sys` to 0.1.19
- Update `miniz_oxide` to 0.5.3
- Update `xz2` to 0.1.7


## [0.1.20] - 2022-05-22

### Changed
- Update `zstd` to 0.11.2+zstd.1.5.2


## [0.1.19] - 2022-05-22

### Changed
- Update `runnel` to 0.3.10
- Update `memx` to 0.1.20
- Update `anyhow` to 1.0.57
- Update `libc` to 0.2.126
- Update `regex` to 1.5.6
- Update `flate2` to 1.0.23
- Update `lz4` to 1.23.3
- Update `zstd` to 0.9.2+zstd.1.5.1
- Update `exec-target` to 0.2.5
- Update `rust-version-info-file` to 0.1.5


## [0.1.18] - 2021-11-15

### Added
- Provide additional documentation


## [0.1.17] - 2021-11-15

### Changed
- Set minimum supported Rust version to 1.51.0


## [0.1.16] - 2021-11-15

### Added
- Provide additional documentation

### Changed
- Set minimum supported Rust version to 1.47.0
- Update `flood-tide` to 0.2.4
- Update `memx` to 0.1.18
- Update `memx-cdy` to 0.1.7
- Update `runnel` to 0.3.9
- Update `anyhow` to 1.0.45
- Update `cc` to 1.0.72
- Update `flate2` to 1.0.22
- Update `libc` to 0.2.107
- Update `pkg-config` to 0.3.22
- Update `exec-target` to 0.2.4
- Update `flood-tide-gen` to 0.1.15
- Update `rust-version-info-file` to 0.1.3


## [0.1.15] - 2021-09-11

### Changed
- Update `flate2` to 1.0.21


## [0.1.14] - 2021-09-11

### Added
- Add `indoc` (1.0.3) dependency

### Changed
- Address Clippy warnings
- Update `anyhow` to 1.0.43
- Update `flood-tide-gen` to 0.1.14
- Update `flood-tide` to 0.2.3
- Update `memx-cdy` to 0.1.6
- Update `runnel` to 0.3.8
- Use `env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")))` for `TARGET_EXE_PATH`
- Update `exec-target` to 0.2.3


## [0.1.13] - 2021-06-24

### Added
- Call `memx_cdy::memx_init()` for faster memory operations

### Changed
- Use `env!("CARGO_BIN_EXE_aki-xtree")` for `TARGET_EXE_PATH`
- Update `zstd` to 0.9.0+zstd.1.5.0

### Fixed
- Fix issue with `#[cfg(feature = "debian_build")]`


## [0.1.12] - 2021-06-06

### Changed
- Update `zstd` to 0.8.3+zstd.1.5.0


## [0.1.11] - 2021-06-03

### Added
- Support `debian_build` feature

### Changed
- Update `flood-tide` to 0.2.2
- Update `regex` to 1.5.4

### Fixed
- Fix issue with `-X rust-version-info` option


## [0.1.10] - 2021-04-23

### Fixed
- Fix issue in `build.rs`


## [0.1.9] - 2021-04-23

### Added
- Support `-X` command option

### Changed
- Update `flood-tide-gen` to 0.1.12
- Update `flood-tide` to 0.2.1
- Update `regex` to 1.4.6


## [0.1.8] - 2021-04-19

### Changed
- Update `flood-tide-gen` to 0.1.10


## [0.1.7] - 2021-04-07

### Changed
- Update `flood-tide` to 0.2
- Update `zstd` to 0.7
- Update `anyhow` to 1.0.40
- Update `flood-tide-gen` to 0.1.8
- Update `runnel` to 0.3.6


## [0.1.6] - 2021-03-22

### Changed
- Update `anyhow` dependency


## [0.1.5] - 2021-03-08

### Changed
- Update `runnel` dependency
- Update `rustc_version` to 0.3


## [0.1.4] - 2021-03-08

### Changed
- Rename `xtask/src/cmd.txt` to `xtask/src/aki-xtee-cmd.txt`


## [0.1.3] - 2021-03-07

### Changed
- Update dependencies


## [0.1.2] - 2021-03-03

### Changed
- Update `runnel` dependency

### Fixed
- Rename GitHub repository from `aki-xtree` to `aki-xtee`


## [0.1.1] - 2021-03-03

### Added
- Add command option: `-p, --pipe-out <num>` (unimplemented)

### Changed
- Update `flood-tide-gen` dependency


## [0.1.0] - 2021-02-28

### Added
- Perform initial release


[Unreleased]: https://github.com/aki-akaguma/aki-xtee/compare/v0.2.1..HEAD
[0.2.1]: https://github.com/aki-akaguma/aki-xtee/compare/v0.2.0..v0.2.1
[0.2.0]: https://github.com/aki-akaguma/aki-xtee/compare/v0.1.25..v0.2.0
[0.1.25]: https://github.com/aki-akaguma/aki-xtee/compare/v0.1.24..v0.1.25
[0.1.24]: https://github.com/aki-akaguma/aki-xtee/compare/v0.1.23..v0.1.24
[0.1.23]: https://github.com/aki-akaguma/aki-xtee/compare/v0.1.22..v0.1.23
[0.1.22]: https://github.com/aki-akaguma/aki-xtee/compare/v0.1.21..v0.1.22
[0.1.21]: https://github.com/aki-akaguma/aki-xtee/compare/v0.1.20..v0.1.21
[0.1.20]: https://github.com/aki-akaguma/aki-xtee/compare/v0.1.19..v0.1.20
[0.1.19]: https://github.com/aki-akaguma/aki-xtee/compare/v0.1.18..v0.1.19
[0.1.18]: https://github.com/aki-akaguma/aki-xtee/compare/v0.1.17..v0.1.18
[0.1.17]: https://github.com/aki-akaguma/aki-xtee/compare/v0.1.16..v0.1.17
[0.1.16]: https://github.com/aki-akaguma/aki-xtee/compare/v0.1.15..v0.1.16
[0.1.15]: https://github.com/aki-akaguma/aki-xtee/compare/v0.1.14..v0.1.15
[0.1.14]: https://github.com/aki-akaguma/aki-xtee/compare/v0.1.13..v0.1.14
[0.1.13]: https://github.com/aki-akaguma/aki-xtee/compare/v0.1.12..v0.1.13
[0.1.12]: https://github.com/aki-akaguma/aki-xtee/compare/v0.1.11..v0.1.12
[0.1.11]: https://github.com/aki-akaguma/aki-xtee/compare/v0.1.10..v0.1.11
[0.1.10]: https://github.com/aki-akaguma/aki-xtee/compare/v0.1.9..v0.1.10
[0.1.9]: https://github.com/aki-akaguma/aki-xtee/compare/v0.1.8..v0.1.9
[0.1.8]: https://github.com/aki-akaguma/aki-xtee/compare/v0.1.7..v0.1.8
[0.1.7]: https://github.com/aki-akaguma/aki-xtee/compare/v0.1.6..v0.1.7
[0.1.6]: https://github.com/aki-akaguma/aki-xtee/compare/v0.1.5..v0.1.6
[0.1.5]: https://github.com/aki-akaguma/aki-xtee/compare/v0.1.4..v0.1.5
[0.1.4]: https://github.com/aki-akaguma/aki-xtee/compare/v0.1.3..v0.1.4
[0.1.3]: https://github.com/aki-akaguma/aki-xtee/compare/v0.1.2..v0.1.3
[0.1.2]: https://github.com/aki-akaguma/aki-xtee/compare/v0.1.1..v0.1.2
[0.1.1]: https://github.com/aki-akaguma/aki-xtee/compare/v0.1.0..v0.1.1
[0.1.0]: https://github.com/aki-akaguma/aki-xtee/releases/tag/v0.1.0
