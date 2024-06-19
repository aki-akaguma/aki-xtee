# Changelog: aki-xtee

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] *
### Added
* `.github/workflows/test-ubuntu.yml`
* `.github/workflows/test-macos.yml`
* `.github/workflows/test-windows.yml`
* test status badges into `README.tpl`
* miri supports on tests
* `tarpaulin` supports into `Makefile`

### Changed
* rename: `config` to `config.toml`
* refactored `Makefile`
* update depends: flood-tide(0.2.9), flood-tide-gen(0.1.20)
* update depends: memx-cdy(0.1.11), runnel(0.3.16)
* update depends: exec-taget(0.2.8), indoc(2.0.0), rust-version-info-file(0.1.8)
* update depends: zstd(0.12.3+zstd.1.5.2)
* rename `fixtures/text10k.txt.gz` to `fixtures/text10k.text.gz`

### Removed
* `COPYING`

### Fixed
* `LICENSE-APACHE`, `LICENSE-MIT`
* license files
* clippy: `redundant_static_lifetimes`, `needless_borrow`, `bool_assert_comparison`
* clippy: `uninlined_format_args`, `unused_imports`, `dead_code`
* rust-version: "1.56.0" to "1.60.0"
* bug: file compare on windows
* bug: on macos:
  - zcat: can't stat: fixtures/text10k.text.gz (fixtures/text10k.text.gz.Z): No such file or directory
  must be used `gzcat` on macos


## [0.1.24] (2023-01-11)
### Added
* badges into `README.tpl`
* rust-version = "1.56.0" into Cargo.toml

### Changed
* reformat `CHANGELOG.md`
* update depends: anyhow(1.0.68)
* update depends: flood-tide(0.2.8), flood-tide-gen(0.1.19)
* update depends: memx-cdy(0.1.10), runnel(0.3.15)
* update depends: flate2(1.0.25), xz2(0.1.7)
* update depends: zstd(0.12.1+zstd.1.5.2)

### Fixed
* clippy: you are deriving `PartialEq` and can implement `Eq`
* clippy: uninlined_format_args

## [0.1.23] (2022-06-18)
### Fixed
* fix git log

## [0.1.22] (2022-06-18)
### Fixed
* bug: README.md

## [0.1.21] (2022-06-18)
### Changed
* changes to edition 2021
* update depends: cfg-iif(0.2.3), flood-tide(0.2.5), linux-procfs(0.3.11)
* update depends: memx(0.1.21), memx-cdy(0.1.8), naive_opt(0.1.18), runnel(0.3.11)
* update depends: assert-text(0.2.6), exec-target(v0.2.6), flood-tide-gen(0.1.16)
* update depends: rust-version-info-file(v0.1.6)
* update depends: semver(1.0.10)
* update depends: flate2(1.0.24), lzma-sys(0.1.19), miniz_oxide(0.5.3), xz2(0.1.7)

## [0.1.20] (2022-05-22)
### Changed
* update depends: zstd(0.11.2+zstd.1.5.2)

## [0.1.19] (2022-05-22)
### Changed
* update depends: runnel(0.3.10), memx(0.1.20)
* update depends: anyhow(1.0.57), libc(0.2.126), regex(1.5.6)
* update depends: flate2(1.0.23), lz4(1.23.3), zstd(0.9.2+zstd.1.5.1)
* update depends: exec-target(v0.2.5), rust-version-info-file(v0.1.5)

## [0.1.18] (2021-11-15)
### Added
* more documents

## [0.1.17] (2021-11-15)
### Changed
* minimum support rustc 1.51.0 (2fd73fabe 2021-03-23)

## [0.1.16] (2021-11-15)
### Added
* more documents

### Changed
* minimum support rustc 1.47.0 (18bf6b4f0 2020-10-07)
* update depends: flood-tide(0.2.4), memx(0.1.18), memx-cdy(0.1.7), runnel(0.3.9)
* update depends: anyhow(1.0.45), cc(1.0.72), flate2(v1.0.22), libc(0.2.107), pkg-config(0.3.22)
* update depends: exec-target(v0.2.4), flood-tide-gen(0.1.15), rust-version-info-file(v0.1.3)

## [0.1.15] (2021-09-11)
### Changed
* update crates: flate2(1.0.21)

## [0.1.14] (2021-09-11)
### Added
* depends: indoc(1.0.3)

### Changed
* pass cargo clippy
* update depends: anyhow(1.0.43), flood-tide-gen(0.1.14), flood-tide(0.2.3), memx-cdy(0.1.6), runnel(0.3.8)
* rewite TARGET_EXE_PATH with `env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")))`
* update depends: exec-target(0.2.3)

## [0.1.13] (2021-06-24)
### Added
* `memx_cdy::memx_init(); // fast mem operation.`

### Changed
* rewite TARGET_EXE_PATH with `env!("CARGO_BIN_EXE_aki-xtree")`
* update depends: zstd(0.9.0+zstd.1.5.0)

### Fixed
* bug: `#[cfg(feature = "debian_build")]`

## [0.1.12] (2021-06-06)
### Changed
* update depends: zstd(0.8.3+zstd.1.5.0)

## [0.1.11] (2021-06-03)
### Added
* support `features = \["debian_build"\]`

### Changed
* update depends: flood-tide(0.2.2)
* update depends: regex(1.5.4)

### Fixed
* bug: command option: -X rust-version-info

## [0.1.10] (2021-04-23)
### Fixed
* bug: build.rs

## [0.1.9] (2021-04-23)
### Added
* command option: `-X`

### Changed
* update depends: flood-tide-gen(0.1.12), flood-tide(0.2.1)
* update depends: bug fix: regex(1.4.6)

## [0.1.8] (2021-04-19)
### Changed
* update depends: flood-tide-gen(0.1.10)

## [0.1.7] (2021-04-07)
### Changed
* update depends: flood-tide(0.2), zstd(0.7)
* update depends: anyhow(1.0.40), flood-tide-gen(0.1.8), runnnel(0.3.6)

## [0.1.6] (2021-03-22)
### Changed
* update depends: anyhow

## [0.1.5] (2021-03-08)
### Changed
* update crate: runnel
* update crate: rustc_version ("0.3")

## [0.1.4] (2021-03-08)
### Changed
* rename file: `xtask/src/cmd.txt` to `xtask/src/aki-xtee-cmd.txt`

## [0.1.3] (2021-03-07)
### Changed
* update crates

## [0.1.2] (2021-03-03)
### Changed
* update crate: runnel

### Fixed
* bug: github repository name: from `aki-xtree` to `aki-xtee`

## [0.1.1] (2021-03-03)
### Added
* command option: `-p, --pipe-out <num>   read from pipe <num> [unimplemented]`

### Changed
* update crate: flood-tide-gen

## [0.1.0] (2021-02-28)
* first commit

[Unreleased]: https://github.com/aki-akaguma/aki-xtee/compare/v0.1.24..HEAD
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
