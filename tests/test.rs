const TARGET_EXE_PATH: &'static str = env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")));

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

            Examples:
              You can simple use. Just arrange the files:
                cat in-file | aki-xtee file1 file2.gz file3.xz file4.zst
            "#
            ),
            "\n",
        )
    };
}

macro_rules! try_help_msg {
    () => {
        "Try --help for help.\n"
    };
}

macro_rules! program_name {
    () => {
        "aki-xtee"
    };
}

macro_rules! version_msg {
    () => {
        concat!(program_name!(), " ", env!("CARGO_PKG_VERSION"), "\n")
    };
}

macro_rules! fixture_text10k {
    () => {
        "fixtures/text10k.txt.gz"
    };
}
macro_rules! fixture_invalid_utf8 {
    () => {
        "fixtures/invalid_utf8.txt"
    };
}

#[macro_use]
mod helper2;
//mod helper;

mod test_0 {
    use exec_target::exec_target;
    //use exec_target::args_from;
    const TARGET_EXE_PATH: &'static str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_help() {
        let oup = exec_target(TARGET_EXE_PATH, &["-H"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, help_msg!());
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_help_long() {
        let oup = exec_target(TARGET_EXE_PATH, &["--help"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, help_msg!());
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_version() {
        let oup = exec_target(TARGET_EXE_PATH, &["-V"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, version_msg!());
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_version_long() {
        let oup = exec_target(TARGET_EXE_PATH, &["--version"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, version_msg!());
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_invalid_opt() {
        let oup = exec_target(TARGET_EXE_PATH, &["-z"]);
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": ",
                "Invalid option: z\n",
                try_help_msg!()
            )
        );
        assert_eq!(oup.stdout, "");
        assert_eq!(oup.status.success(), false);
    }
}

mod test_1 {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &'static str = super::TARGET_EXE_PATH;

    //
    #[test]
    fn test_non_option() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, &[] as &[&str], b"abcdefg\n" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "abcdefg\n");
        assert_eq!(oup.status.success(), true);
    }
}

mod test_2 {
    use crate::helper2::cmp_file;
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &'static str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_plain() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["target/out020/out.plain.txt"],
            b"ABCDEFG\nHIJKLMN\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ABCDEFG\nHIJKLMN\n");
        assert_eq!(oup.status.success(), true);
        assert_file_eq!("target/out020/", "fixtures/", "out.plain.txt");
    }
    //
    #[test]
    fn test_gz() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["target/out020/out.text.gz"],
            b"ABCDEFG\nHIJKLMN\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ABCDEFG\nHIJKLMN\n");
        assert_eq!(oup.status.success(), true);
        assert_file_eq!("target/out020/", "fixtures/", "out.text.gz");
    }
    //
    #[cfg(feature = "xz2")]
    #[test]
    fn test_xz() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["target/out020/out.text.xz"],
            b"ABCDEFG\nHIJKLMN\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ABCDEFG\nHIJKLMN\n");
        assert_eq!(oup.status.success(), true);
        assert_file_eq!("target/out020/", "fixtures/", "out.text.xz");
    }
    #[cfg(feature = "zstd")]
    #[test]
    fn test_zstd() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["target/out020/out.text.zst"],
            b"ABCDEFG\nHIJKLMN\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ABCDEFG\nHIJKLMN\n");
        assert_eq!(oup.status.success(), true);
        assert_file_eq!("target/out020/", "fixtures/", "out.text.zst");
    }
    //
    #[cfg(feature = "xz2")]
    #[cfg(feature = "zstd")]
    #[test]
    fn test_plain_gz_xz() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &[
                "target/out021/out.plain.txt",
                "target/out021/out.text.gz",
                "target/out021/out.text.xz",
                "target/out021/out.text.zst",
            ],
            b"ABCDEFG\nHIJKLMN\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ABCDEFG\nHIJKLMN\n");
        assert_eq!(oup.status.success(), true);
        assert_file_eq!("target/out021/", "fixtures/", "out.plain.txt");
        assert_file_eq!("target/out021/", "fixtures/", "out.text.gz");
        assert_file_eq!("target/out021/", "fixtures/", "out.text.xz");
        assert_file_eq!("target/out021/", "fixtures/", "out.text.zst");
    }
    //
    #[test]
    fn test_invalid_utf8() {
        let v = {
            use std::io::Read;
            let mut f = std::fs::File::open(fixture_invalid_utf8!()).unwrap();
            let mut v = Vec::new();
            f.read_to_end(&mut v).unwrap();
            v
        };
        let oup = exec_target_with_in(TARGET_EXE_PATH, &[] as &[&str], &v);
        assert_eq!(
            oup.stderr,
            concat!(program_name!(), ": stream did not contain valid UTF-8\n",)
        );
        assert_eq!(oup.stdout, "");
        assert_eq!(oup.status.success(), false);
    }
}

mod test_3 {
    use exec_target::exec_target;
    const TARGET_EXE_PATH: &'static str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_output_broken_pipe() {
        let cmdstr = format!(
            "zcat \"{}\" | \"{}\" | head -n 2",
            fixture_text10k!(),
            TARGET_EXE_PATH,
        );
        let oup = exec_target("sh", &["-c", &cmdstr]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ABCDEFG\nHIJKLMN\n");
        assert_eq!(oup.status.success(), true);
    }
}
