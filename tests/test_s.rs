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
/*
macro_rules! fixture_text10k {
    () => {
        "fixtures/text10k.txt.gz"
    };
}
*/
/*
macro_rules! fixture_invalid_utf8 {
    () => {
        "fixtures/invalid_utf8.txt"
    };
}
*/

#[rustfmt::skip]
macro_rules! do_execute {
    ($args:expr) => {
        do_execute!($args, "")
    };
    ($args:expr, $sin:expr) => {{
        let sioe = RunnelIoe::new(
            Box::new(StringIn::with_str($sin)),
            #[allow(clippy::box_default)]
            Box::new(StringOut::default()),
            #[allow(clippy::box_default)]
            Box::new(StringErr::default()),
        );
        let program = env!("CARGO_PKG_NAME");
        let r = execute(&sioe, &program, $args);
        match r {
            Ok(_) => {}
            Err(ref err) => {
                let _ = sioe.pg_err().lock()
                .write_fmt(format_args!("{}: {:#}\n", program, err));
            }
        };
        (r, sioe)
    }};
}

macro_rules! buff {
    ($sioe:expr, serr) => {
        $sioe.pg_err().lock().buffer_to_string()
    };
    ($sioe:expr, sout) => {
        $sioe.pg_out().lock().buffer_to_string()
    };
}

#[macro_use]
mod helper2;

mod test_0_s {
    use libaki_xtee::*;
    use runnel::medium::stringio::*;
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_help() {
        let (r, sioe) = do_execute!(&["-H"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_help_long() {
        let (r, sioe) = do_execute!(&["--help"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_version() {
        let (r, sioe) = do_execute!(&["-V"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_version_long() {
        let (r, sioe) = do_execute!(&["--version"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_invalid_opt() {
        let (r, sioe) = do_execute!(&["-z"]);
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": ",
                "Invalid option: z\n",
                try_help_msg!()
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
}

mod test_1_s {
    use libaki_xtee::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_non_option() {
        let (r, sioe) = do_execute!(&[] as &[&str], "abcdefg\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "abcdefg\n");
        assert!(r.is_ok());
    }
}

mod test_2_s {
    use crate::helper2::{cmp_file, cmp_text_file};
    use libaki_xtee::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_plain() {
        let (r, sioe) = do_execute!(&["target/out_s020/out.plain.txt"], "ABCDEFG\nHIJKLMN\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ABCDEFG\nHIJKLMN\n");
        assert!(r.is_ok());
        assert_text_file_eq!("target/out_s020/", "fixtures/", "out.plain.txt");
    }
    //
    #[test]
    fn test_gz() {
        let (r, sioe) = do_execute!(&["target/out_s020/out.text.gz"], "ABCDEFG\nHIJKLMN\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ABCDEFG\nHIJKLMN\n");
        assert!(r.is_ok());
        assert_file_eq!("target/out_s020/", "fixtures/", "out.text.gz");
    }
    //
    #[cfg(feature = "xz2")]
    #[test]
    fn test_xz() {
        let (r, sioe) = do_execute!(&["target/out_s020/out.text.xz"], "ABCDEFG\nHIJKLMN\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ABCDEFG\nHIJKLMN\n");
        assert!(r.is_ok());
        assert_file_eq!("target/out_s020/", "fixtures/", "out.text.xz");
    }
    //
    #[cfg(feature = "zstd")]
    #[test]
    fn test_zstd() {
        let (r, sioe) = do_execute!(&["target/out_s020/out.text.zst"], "ABCDEFG\nHIJKLMN\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ABCDEFG\nHIJKLMN\n");
        assert!(r.is_ok());
        assert_file_eq!("target/out_s020/", "fixtures/", "out.text.zst");
    }
    //
    #[cfg(feature = "xz2")]
    #[cfg(feature = "zstd")]
    #[test]
    fn test_plain_gz_xz() {
        let (r, sioe) = do_execute!(
            &[
                "target/out_s021/out.plain.txt",
                "target/out_s021/out.text.gz",
                "target/out_s021/out.text.xz",
                "target/out_s021/out.text.zst",
            ],
            "ABCDEFG\nHIJKLMN\n"
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ABCDEFG\nHIJKLMN\n");
        assert!(r.is_ok());
        assert_text_file_eq!("target/out_s021/", "fixtures/", "out.plain.txt");
        assert_file_eq!("target/out_s021/", "fixtures/", "out.text.gz");
        assert_file_eq!("target/out_s021/", "fixtures/", "out.text.xz");
        assert_file_eq!("target/out_s021/", "fixtures/", "out.text.zst");
    }
    //
    /*
    #[test]
    fn test_invalid_utf8() {
        let v = {
            use std::io::Read;
            let mut f = std::fs::File::open(fixture_invalid_utf8!()).unwrap();
            let mut v = Vec::new();
            f.read_to_end(&mut v).unwrap();
            v
        };
        let (r, sioe) = do_execute!(&[], &v);
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": stream did not contain valid UTF-8\n",
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    */
}

/*
mod test_3_s {
    /*
    use libaki_xtee::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use std::io::Write;
    //
     * can NOT test
    #[test]
    fn test_output_broken_pipe() {
    }
    */
}
*/
