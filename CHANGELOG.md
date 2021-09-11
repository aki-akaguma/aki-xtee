aki-xtee TBD
===
Unreleased changes. Release notes have not yet been written.

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
