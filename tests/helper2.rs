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
              -a, --append <file>   append to the <file>, do not overwrite

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

#[allow(dead_code)]
pub struct TestOut {
    base_dir: tempfile::TempDir,
}

#[allow(dead_code)]
impl TestOut {
    pub fn new() -> Self {
        let test_out_str = "target/test_out";
        let _ = std::fs::create_dir_all(test_out_str);
        Self {
            base_dir: tempfile::tempdir_in(test_out_str).unwrap(),
        }
    }
    pub fn base_dir(&self) -> &std::path::Path {
        self.base_dir.path()
    }
    pub fn target_path(&self, out_file_name: &str) -> std::path::PathBuf {
        self.base_dir.path().join(out_file_name)
    }
    pub fn cmp_text_file_with_fixtures(&self, out_file_name: &str) -> std::io::Result<bool> {
        cmp_text_file(
            self.target_path(out_file_name),
            format!("fixtures/{out_file_name}"),
        )
    }
    pub fn cmp_file_with_fixtures(&self, out_file_name: &str) -> std::io::Result<bool> {
        cmp_file(
            self.target_path(out_file_name),
            format!("fixtures/{out_file_name}"),
        )
    }
}

#[allow(dead_code)]
impl Default for TestOut {
    fn default() -> Self {
        Self::new()
    }
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
