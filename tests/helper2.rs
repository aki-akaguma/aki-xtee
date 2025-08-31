#[allow(unused_macros)]
macro_rules! help_msg {
    () => {
        concat!(
            version_msg!(),
            "\n",
            indoc::indoc!(
                r#"
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
              -X <x-options>    x options. try -X help

            Argument:
              <file>         utf-8 encoded plain text file,
                             gzip compressed file at the end with '.gz',
                             xz2 compressed file at the end with '.xz',
                             zstd compressed file at the end with '.zst'.
                             lz4 compressed file at the end with '.lz4'.
                             bzip2 compressed file at the end with '.bz2'.

            Examples:
              You can simple use. Just arrange the files:
                cat in-file | aki-xtee file1 file2.gz file3.xz file4.zst
            "#
            ),
            "\n",
        )
    };
}

#[allow(unused_macros)]
macro_rules! try_help_msg {
    () => {
        "Try --help for help.\n"
    };
}

#[allow(unused_macros)]
macro_rules! program_name {
    () => {
        "aki-xtee"
    };
}

#[allow(unused_macros)]
macro_rules! version_msg {
    () => {
        concat!(program_name!(), " ", env!("CARGO_PKG_VERSION"), "\n")
    };
}

#[allow(unused_macros)]
macro_rules! fixture_text10k {
    () => {
        "fixtures/text10k.text.gz"
    };
}

#[allow(unused_macros)]
macro_rules! fixture_invalid_utf8 {
    () => {
        "fixtures/invalid_utf8.txt"
    };
}

#[allow(unused_macros)]
macro_rules! assert_file_eq {
    ($p1:expr, $p2:expr, $file_name:expr) => {
        assert_eq!(
            crate::cmp_file(concat!($p1, $file_name), concat!($p2, $file_name)).unwrap(),
            true
        );
    };
}

#[allow(dead_code)]
pub fn cmp_file<T1, T2>(path1: T1, path2: T2) -> std::io::Result<bool>
where
    T1: AsRef<std::path::Path>,
    T2: AsRef<std::path::Path>,
{
    use std::io::Read;
    let mut f1 = std::fs::File::open(path1)?;
    let mut f2 = std::fs::File::open(path2)?;
    let mut buf1 = Vec::new();
    let mut buf2 = Vec::new();
    f1.read_to_end(&mut buf1)?;
    f2.read_to_end(&mut buf2)?;
    Ok(buf1 == buf2)
}

#[allow(unused_macros)]
macro_rules! assert_text_file_eq {
    ($p1:expr, $p2:expr, $file_name:expr) => {
        assert_eq!(
            crate::cmp_text_file(concat!($p1, $file_name), concat!($p2, $file_name)).unwrap(),
            true
        );
    };
}

#[allow(dead_code)]
pub fn cmp_text_file<T1, T2>(path1: T1, path2: T2) -> std::io::Result<bool>
where
    T1: AsRef<std::path::Path>,
    T2: AsRef<std::path::Path>,
{
    use std::io::Read;
    let mut f1 = std::fs::File::open(path1)?;
    let mut f2 = std::fs::File::open(path2)?;
    let mut buf1 = String::new();
    let mut buf2 = String::new();
    f1.read_to_string(&mut buf1)?;
    f2.read_to_string(&mut buf2)?;
    #[cfg(windows)]
    let buf1 = buf1.replace("\r\n", "\n");
    #[cfg(windows)]
    let buf2 = buf2.replace("\r\n", "\n");
    Ok(buf1 == buf2)
}
