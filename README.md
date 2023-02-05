# aki-xtee

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Rust Version][rustc-image]
![Apache2/MIT licensed][license-image]
[![Test ubu][test-ubuntu-image]][test-ubuntu-link]
[![Test mac][test-windows-image]][test-windows-link]
[![Test win][test-macos-image]][test-macos-link]

copy standard input to each files and standard output.

## Feature

- copy standard input to each files and standard output.
- output files are compressed by auto with filename extension.
- minimum support rustc 1.60.0 (7737e0b5c 2022-04-04)

## Command Help

```
aki-xtee --help
```

```
Usage:
  aki-xtee [options] [<file>...]

this is like the linux command `tee`.
copy standard input to each <file>, and to standard output.
automatic discovery file type: plain, gz, xz and zst.

Options:
  -a, --append <file>   append to the <file>, do not overwrite [unimplemented]
  -p, --pipe-out <num>  write to pipe <num> [unimplemented]

  -H, --help        display this help and exit
  -V, --version     display version information and exit

Argument:
  <file>         utf-8 encoded plain text file,
                 gzip compressed file at the end with '.gz',
                 xz2 compressed file at the end with '.xz',
                 zstd compressed file at the end with '.zst'.

Examples:
  You can simple use. Just arrange the files:
    cat in-file | aki-xtee file1 file2.gz file3.xz file4.zst
```

## Quick install

1. you can install this into cargo bin path:

```
cargo install aki-xtee
```

2. you can build debian package:

```
cargo deb
```

and install **.deb** into your local repository of debian package.

## Examples

### Command line example 1

output "ABCDEFGH" to standard output and plain text file.
```
echo "ABCDEFGH" | aki-xtee out/plain.txt
```
result output :
```
ABCDEFGH
```

output "ABCDEFGH" to standard output and gzip text file.
```
echo "ABCDEFGH" | aki-xtee out/gztext.txt.gz
```
result output :
```
ABCDEFGH
```

output "ABCDEFGH" to standard output and xz text file.
```
echo "ABCDEFGH" | aki-xtee out/xztext.txt.xz
```
result output :
```
ABCDEFGH
```

output "ABCDEFGH" to standard output and zstd text file.
```
echo "ABCDEFGH" | aki-xtee out/xztext.txt.zst
```
result output :
```
ABCDEFGH
```

### Command line example 2

copy input to plain text file, gzip text file, xz text file and zstd text file.
```
aki-xtee out/plain.txt out/gztext.txt.gz out/xztext.txt.xz  out/zstext.txt.zst
```

### Library example

See [`fn execute()`] for this library examples.

[`fn execute()`]: crate::execute


# Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/aki-xtee/blob/main/CHANGELOG.md)

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/aki-xtee.svg
[crate-link]: https://crates.io/crates/aki-xtee
[docs-image]: https://docs.rs/aki-xtee/badge.svg
[docs-link]: https://docs.rs/aki-xtee/
[rustc-image]: https://img.shields.io/badge/rustc-1.60+-blue.svg
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[test-ubuntu-image]: https://github.com/aki-akaguma/aki-xtee/actions/workflows/test-ubuntu.yml/badge.svg
[test-ubuntu-link]: https://github.com/aki-akaguma/aki-xtee/actions/workflows/test-ubuntu.yml
[test-macos-image]: https://github.com/aki-akaguma/aki-xtee/actions/workflows/test-macos.yml/badge.svg
[test-macos-link]: https://github.com/aki-akaguma/aki-xtee/actions/workflows/test-macos.yml
[test-windows-image]: https://github.com/aki-akaguma/aki-xtee/actions/workflows/test-windows.yml/badge.svg
[test-windows-link]: https://github.com/aki-akaguma/aki-xtee/actions/workflows/test-windows.yml
