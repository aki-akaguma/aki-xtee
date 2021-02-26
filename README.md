# aki-xtee

*aki-xtee* is the program that copy standard input to each files and standard output.

## Features

*aki-xtee*  copy standard input to each files and standard output.

* command help

```text
aki-xtee --help
```

```
Usage:
  aki-xtee [options] [<file>...]

this is like the linux command `tee`.
copy standard input to each <file>, and to standard output.
automatic discovery file type: plain, gz, xz and zst.

Options:
  -a, --append   append to the <file>, do not overwrite [unimplemented]

  -H, --help     display this help and exit
  -V, --version  display version information and exit

Argument:
  <file>         utf-8 encoded plain text file,
                 gzip compressed file at the end with '.gz',
                 xz2 compressed file at the end with '.xz',
                 zstd compressed file at the end with '.zst'.

Examples:
  You can simple use. Just arrange the files.
    aki-xtee file1 file2.gz file3.xz file4.zst
```

* minimum support rustc 1.38.0

## Quick install

1. you can install this into cargo bin path:

```
cargo install aki-xcat
```

2. you can build debian package:

```
cargo deb
```

and install **.deb** into your local repository of debian package.

## Examples

#### Command line example 1

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

#### Command line example 2

copy input to plain text file, gzip text file, xz text file and zstd text file.
```
aki-xtee out/plain.txt out/gztext.txt.gz out/xztext.txt.xz  out/zstext.txt.zst
```

#### Library example

See [`fn execute()`] for this library examples.

[`fn execute()`]: crate::execute
