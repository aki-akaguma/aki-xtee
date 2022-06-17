TBD: aki-xtee
===
Unreleased changes. Release notes have not yet been written.

0.1.21 (2022-06-18)
=====

* changes to edition 2021
* update depends: cfg-iif(0.2.3), flood-tide(0.2.5), linux-procfs(0.3.11)
* update depends: memx(0.1.21), memx-cdy(0.1.8), naive_opt(0.1.18), runnel(0.3.11)
* update depends: assert-text(0.2.6), exec-target(v0.2.6), flood-tide-gen(0.1.16)
* update depends: rust-version-info-file(v0.1.6)
* update depends: semver(1.0.10)
* update depends: flate2(1.0.24), lzma-sys(0.1.19), miniz_oxide(0.5.3), xz2(0.1.7)

0.1.20 (2022-05-22)
=====

* update depends: zstd(0.11.2+zstd.1.5.2)

0.1.19 (2022-05-22)
=====

* update depends: runnel(0.3.10), memx(0.1.20)
* update depends: anyhow(1.0.57), libc(0.2.126), regex(1.5.6)
* update depends: flate2(1.0.23), lz4(1.23.3), zstd(0.9.2+zstd.1.5.1)
* update depends: exec-target(v0.2.5), rust-version-info-file(v0.1.5)

0.1.18 (2021-11-15)
=====

* add more documents

0.1.17 (2021-11-15)
=====

* minimum support rustc 1.51.0 (2fd73fabe 2021-03-23)

0.1.16 (2021-11-15)
=====

* minimum support rustc 1.47.0 (18bf6b4f0 2020-10-07)
* add more documents
* update depends: flood-tide(0.2.4), memx(0.1.18), memx-cdy(0.1.7), runnel(0.3.9)
* update depends: anyhow(1.0.45), cc(1.0.72), flate2(v1.0.22), libc(0.2.107), pkg-config(0.3.22)
* update depends: exec-target(v0.2.4), flood-tide-gen(0.1.15), rust-version-info-file(v0.1.3)

0.1.15 (2021-09-11)
=====

* update crates: flate2(1.0.21)

0.1.14 (2021-09-11)
=====

* pass cargo clippy
* update depends: anyhow(1.0.43), flood-tide-gen(0.1.14), flood-tide(0.2.3), memx-cdy(0.1.6), runnel(0.3.8)
* rewite TARGET_EXE_PATH with `env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")))`
* update depends: exec-target(0.2.3)
* add depends: indoc(1.0.3)

0.1.13 (2021-06-24)
=====

* add `memx_cdy::memx_init(); // fast mem operation.`
* rewite TARGET_EXE_PATH with `env!("CARGO_BIN_EXE_aki-xtree")`
* bug fix: `#[cfg(feature = "debian_build")]`
* update depends: zstd(0.9.0+zstd.1.5.0)

0.1.12 (2021-06-06)
=====

* update depends: zstd(0.8.3+zstd.1.5.0)

0.1.11 (2021-06-03)
=====

* add support features = \["debian_build"\]
* bug fix command option: -X rust-version-info
* update depends: flood-tide(0.2.2)
* update depends: regex(1.5.4)

0.1.10 (2021-04-23)
=====

* fix build.rs

0.1.9 (2021-04-23)
=====

* update depends: flood-tide-gen(0.1.12), flood-tide(0.2.1)
* add command option: -X
* update depends: bug fix: regex(1.4.6)

0.1.8 (2021-04-19)
=====

* update depends: flood-tide-gen(0.1.10)

0.1.7 (2021-04-07)
=====

* update depends: flood-tide(0.2), zstd(0.7)
* update depends: anyhow(1.0.40), flood-tide-gen(0.1.8), runnnel(0.3.6)

0.1.6 (2021-03-22)
=====

* update depends: anyhow

0.1.5 (2021-03-08)
=====

* update crate: runnel
* update crate: rustc_version ("0.3")

0.1.4 (2021-03-08)
=====

* rename file: xtask/src/cmd.txt to xtask/src/aki-xtee-cmd.txt

0.1.3 (2021-03-07)
=====

* update crates

0.1.2 (2021-03-03)
=====

* fix github repository name: from aki-xtree to aki-xtee
* update crate: runnel

0.1.1 (2021-03-03)
=====

* add option: '-p, --pipe-out <num>   read from pipe <num> [unimplemented]'
* update crate: flood-tide-gen

0.1.0 (2021-02-28)
=====
first commit
