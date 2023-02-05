/*!
copy standard input to each files and standard output.

# Feature

- copy standard input to each files and standard output.
- output files are compressed by auto with filename extension.
- minimum support rustc 1.60.0 (7737e0b5c 2022-04-04)

# Command Help

```text
aki-xtee --help
```

```text
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

# Quick install

1. you can install this into cargo bin path:

```text
cargo install aki-xtee
```

2. you can build debian package:

```text
cargo deb
```

and install **.deb** into your local repository of debian package.

# Examples

## Command line example 1

output "ABCDEFGH" to standard output and plain text file.
```text
echo "ABCDEFGH" | aki-xtee out/plain.txt
```
result output :
```text
ABCDEFGH
```

output "ABCDEFGH" to standard output and gzip text file.
```text
echo "ABCDEFGH" | aki-xtee out/gztext.txt.gz
```
result output :
```text
ABCDEFGH
```

output "ABCDEFGH" to standard output and xz text file.
```text
echo "ABCDEFGH" | aki-xtee out/xztext.txt.xz
```
result output :
```text
ABCDEFGH
```

output "ABCDEFGH" to standard output and zstd text file.
```text
echo "ABCDEFGH" | aki-xtee out/xztext.txt.zst
```
result output :
```text
ABCDEFGH
```

## Command line example 2

copy input to plain text file, gzip text file, xz text file and zstd text file.
```text
aki-xtee out/plain.txt out/gztext.txt.gz out/xztext.txt.xz  out/zstext.txt.zst
```

## Library example

See [`fn execute()`] for this library examples.

[`fn execute()`]: crate::execute

*/
#[macro_use]
extern crate anyhow;

mod conf;
mod run;
mod util;

use flood_tide::HelpVersion;
use runnel::*;
use std::io::Write;

const TRY_HELP_MSG: &str = "Try --help for help.";

///
/// execute xcat
///
/// params:
///   - sioe: stream in/out/err
///   - program: program name. etc. "xcat"
///   - args: parameter arguments.
///
/// return:
///   - ok: ()
///   - err: anyhow
///
/// example:
///
/// ```
/// use runnel::RunnelIoeBuilder;
///
/// let r = libaki_xtee::execute(&RunnelIoeBuilder::new().build(),
///     "xtee", &["target/out/plain.txt", "target/out/gztext.txt.gz",
///         "target/out/xztext.txt.xz", "target/out/zstext.txt.zst"]);
/// ```
///
pub fn execute(sioe: &RunnelIoe, prog_name: &str, args: &[&str]) -> anyhow::Result<()> {
    let conf = match conf::parse_cmdopts(prog_name, args) {
        Ok(conf) => conf,
        Err(errs) => {
            for err in errs.iter().take(1) {
                if err.is_help() || err.is_version() {
                    let _r = sioe.pout().lock().write_fmt(format_args!("{err}\n"));
                    return Ok(());
                }
            }
            return Err(anyhow!("{}\n{}", errs, TRY_HELP_MSG));
        }
    };
    run::run(sioe, &conf)
}
