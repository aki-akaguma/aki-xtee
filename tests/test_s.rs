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
        let (r, sioe) = do_execute!(["-H"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_help_long() {
        let (r, sioe) = do_execute!(["--help"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_version() {
        let (r, sioe) = do_execute!(["-V"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_version_long() {
        let (r, sioe) = do_execute!(["--version"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_invalid_opt() {
        let (r, sioe) = do_execute!(["-z"]);
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

mod test_0_x_options_s {
    use crate::helper2::TestOut;
    use libaki_xtee::*;
    use runnel::medium::stringio::*;
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_x_rust_version_info() {
        let (r, sioe) = do_execute!(["-X", "rust-version-info"]);
        assert_eq!(buff!(sioe, serr), "");
        assert!(!buff!(sioe, sout).is_empty());
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_x_option_help() {
        let (r, sioe) = do_execute!(["-X", "help"]);
        assert_eq!(buff!(sioe, serr), "");
        assert!(buff!(sioe, sout).contains("Options:"));
        assert!(buff!(sioe, sout).contains("-X rust-version-info"));
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_x_option_rust_version_info() {
        let (r, sioe) = do_execute!(["-X", "rust-version-info"]);
        assert_eq!(buff!(sioe, serr), "");
        assert!(buff!(sioe, sout).contains("rustc"));
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_multiple_x_options() {
        let (r, sioe) = do_execute!(["-X", "help", "-X", "rust-version-info"]);
        assert_eq!(buff!(sioe, serr), "");
        // The first one should be executed and the program should exit.
        assert!(buff!(sioe, sout).contains("Options:"));
        assert!(!buff!(sioe, sout).contains("rustc"));
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_x_base_dir() {
        let test_out = TestOut::new();
        let fnm = "test_file.txt";
        let target_path = test_out.target_path(fnm);
        let base_dir = test_out.base_dir();
        let base_dir_str = base_dir.to_str().unwrap();
        //
        let (r, sioe) = do_execute!(
            ["-X", &format!("base_dir={base_dir_str}"), fnm],
            "hello from base_dir\n"
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "hello from base_dir\n");
        assert!(r.is_ok());
        //
        let content = std::fs::read_to_string(target_path).unwrap();
        assert_eq!(content, "hello from base_dir\n");
    }
    //
    #[test]
    fn test_x_base_dir_non_existent_dir() {
        let fnm = "test_file.txt";
        let (r, sioe) = do_execute!(
            ["-X", "base_dir=/non/existent/dir", fnm],
            "hello from base_dir\n"
        );
        #[cfg(target_os = "linux")]
        {
            assert!(buff!(sioe, serr).contains("Permission denied"));
            assert_eq!(buff!(sioe, sout), "");
            assert!(r.is_err());
        }
        #[cfg(target_os = "macos")]
        {
            assert!(buff!(sioe, serr).contains("Read-only file system"));
            assert_eq!(buff!(sioe, sout), "");
            assert!(r.is_err());
        }
        #[cfg(windows)]
        {
            //assert!(buff!(sioe, serr).contains("The system cannot find the path specified."));
            assert_eq!(buff!(sioe, serr), "");
            assert_eq!(buff!(sioe, sout), "hello from base_dir\n");
            assert!(r.is_ok());
        }
    }
    //
    #[test]
    fn test_x_base_dir_non_existent_file() {
        let test_out = TestOut::new();
        let base_dir = test_out.base_dir();
        let base_dir_str = base_dir.to_str().unwrap();
        //
        let (r, sioe) = do_execute!(
            [
                "-X",
                &format!("base_dir={base_dir_str}"),
                "non_existent_dir/non_existent_file.txt",
            ],
            "hello from base_dir\n"
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "hello from base_dir\n");
        assert!(r.is_ok());
    }
}

mod test_1_stdout_s {
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
        let (r, sioe) = do_execute!(&[] as &[&str], &v);
        assert_eq!(
            buff!(sioe, serr),
            concat!(program_name!(), ": stream did not contain valid UTF-8\n",)
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    */
}

mod test_2_file_s {
    use crate::helper2::TestOut;
    use libaki_xtee::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_plain() {
        let test_out = TestOut::new();
        let fnm = "out.plain.txt";
        let target_path = test_out.target_path(fnm);
        let target_path_str = target_path.to_str().unwrap();
        //
        let (r, sioe) = do_execute!([target_path_str], "ABCDEFG\nHIJKLMN\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ABCDEFG\nHIJKLMN\n");
        assert!(r.is_ok());
        //
        assert!(test_out.cmp_text_file_with_fixtures(fnm).unwrap());
    }
    //
    #[test]
    fn test_empty_input() {
        use std::io::Read;
        //
        let test_out = TestOut::new();
        let fnm = "out.plain.txt";
        let target_path = test_out.target_path(fnm);
        let target_path_str = target_path.to_str().unwrap();
        //
        let (r, sioe) = do_execute!([target_path_str], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_ok());
        //
        let mut f = std::fs::File::open(target_path).unwrap();
        let mut buf = String::new();
        f.read_to_string(&mut buf).unwrap();
        assert_eq!(buf, "");
    }

    #[test]
    fn test_non_existent_output_dir() {
        let (r, sioe) = do_execute!(["target/non_existent_dir/out.plain.txt"], "some data\n");
        //assert!(oup.stderr.contains("No such file or directory"));
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "some data\n");
        assert!(r.is_ok());
    }

    #[test]
    fn test_output_path_is_dir() {
        let test_out = TestOut::new();
        let target_path = test_out.base_dir().join("out_is_dir");
        let target_path_str = target_path.to_str().unwrap();
        //
        let _ = std::fs::create_dir_all(&target_path);
        let (r, sioe) = do_execute!([target_path_str], "some data\n");
        #[cfg(not(windows))]
        assert!(buff!(sioe, serr).contains("Is a directory"));
        #[cfg(windows)]
        assert!(buff!(sioe, serr).contains("Access is denied"));
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }

    #[test]
    #[cfg(unix)] // This test is for unix-like systems
    fn test_write_permission_error() {
        let test_out = TestOut::new();
        let fnm = "out.plain.txt";
        let target_path_dir = test_out.base_dir().join("no_write_permission");
        let target_path = target_path_dir.join(fnm);
        let target_path_str = target_path.to_str().unwrap();
        //
        let _ = std::fs::create_dir_all(&target_path_dir);
        // set read-only permission
        let mut perms = std::fs::metadata(&target_path_dir).unwrap().permissions();
        let perms_bak = perms.clone();
        perms.set_readonly(true);
        std::fs::set_permissions(&target_path_dir, perms).unwrap();

        let (r, sioe) = do_execute!([target_path_str], "some data");

        // restore permission
        std::fs::set_permissions(&target_path_dir, perms_bak).unwrap();

        assert!(buff!(sioe, serr).contains("Permission denied"));
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    //
    #[test]
    fn test_unsupported_file_extension() {
        let test_out = TestOut::new();
        let fnm = "out.unsupported";
        let target_path = test_out.target_path(fnm);
        let target_path_str = target_path.to_str().unwrap();
        //
        let (r, sioe) = do_execute!([target_path_str], "some data");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "some data\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_option_after_argument() {
        let test_out = TestOut::new();
        let fnm = "out.plain.txt";
        let target_path = test_out.target_path(fnm);
        let target_path_str = target_path.to_str().unwrap();
        //
        let (r, sioe) = do_execute!([target_path_str, "-V"], "some data");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_filename_with_spaces() {
        let test_out = TestOut::new();
        let fnm = "file with spaces.txt";
        let target_path = test_out.target_path(fnm);
        let target_path_str = target_path.to_str().unwrap();
        //
        let (r, sioe) = do_execute!([target_path_str], "some data");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "some data\n");
        assert!(r.is_ok());
        //
        let content = std::fs::read_to_string(target_path_str).unwrap();
        assert_eq!(content, "some data\n");
    }
    //
    #[test]
    fn test_file_overwrite() {
        use std::io::Write;
        //
        let test_out = TestOut::new();
        let fnm = "overwrite.txt";
        let target_path = test_out.target_path(fnm);
        let target_path_str = target_path.to_str().unwrap();
        //
        // create a file with some initial content
        let mut f = std::fs::File::create(target_path_str).unwrap();
        f.write_all(b"initial content").unwrap();
        //
        let (r, sioe) = do_execute!([target_path_str], "new content");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "new content\n");
        assert!(r.is_ok());
        //
        let content = std::fs::read_to_string(target_path_str).unwrap();
        assert_eq!(content, "new content\n");
    }
    //
    #[test]
    #[cfg(unix)]
    fn test_symlink() {
        let test_out = TestOut::new();
        let out_dir = test_out.base_dir();
        let _ = std::fs::create_dir_all(out_dir);
        let filename = out_dir.join("symlink_target.txt");
        let symlink = out_dir.join("symlink.txt");
        // create a symlink to a file
        std::os::unix::fs::symlink(&filename, &symlink).unwrap();
        //
        let (r, sioe) = do_execute!([&symlink.display().to_string()], "symlink content");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "symlink content\n");
        assert!(r.is_ok());
        //
        let content = std::fs::read_to_string(filename).unwrap();
        assert_eq!(content, "symlink content\n");
    }
    //
    #[test]
    fn test_file_named_dash() {
        let (r, sioe) = do_execute!(["-"], "some data");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "some data\n");
        assert!(r.is_ok());
        // It should not create a file named "-"
        assert!(!std::path::Path::new("-").exists());
    }
    //
    #[test]
    fn test_very_long_argument() {
        let long_arg = "a".repeat(10000);
        let (r, sioe) = do_execute!([&long_arg], "some data");
        #[cfg(not(windows))]
        assert!(buff!(sioe, serr).contains("File name too long"));
        #[cfg(windows)]
        assert!(buff!(sioe, serr)
            .contains("The filename, directory name, or volume label syntax is incorrect."));
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    //
    /*
        #[test]
        fn test_binary_input() {
            let v = {
                use std::io::Read;
                let mut f = std::fs::File::open("fixtures/binary_data.bin").unwrap();
                let mut v = Vec::new();
                f.read_to_end(&mut v).unwrap();
                v
            };
            let (r, sioe) = do_execute!(&[], &v);
            assert_eq!(buff!(sioe, serr), "");
            assert_eq!(buff!(sioe, sout), "b\"\\x80\\x81\\x82\\x83\\x84\\x85\"\n");
            assert!(r.is_ok());
            /*
            assert_eq!(
                oup.stderr,
                concat!(program_name!(), ": stream did not contain valid UTF-8\n",)
            );
            */
        }
    */
    //
    #[test]
    fn test_filename_with_special_chars() {
        let test_out = TestOut::new();
        let fnm = "!@#$%^&*().txt";
        let target_path = test_out.target_path(fnm);
        let target_path_str = target_path.to_str().unwrap();
        //
        let (r, sioe) = do_execute!([target_path_str], "special chars\n");
        #[cfg(not(windows))]
        {
            assert_eq!(buff!(sioe, serr), "");
            assert_eq!(buff!(sioe, sout), "special chars\n");
            assert!(r.is_ok());
            //
            let content = std::fs::read_to_string(target_path_str).unwrap();
            assert_eq!(content, "special chars\n");
        }
        #[cfg(windows)]
        {
            assert!(buff!(sioe, serr)
                .contains("The filename, directory name, or volume label syntax is incorrect"));
            assert_eq!(buff!(sioe, sout), "");
            assert!(r.is_err());
        }
    }
    //
    #[test]
    #[cfg(unix)]
    fn test_dev_null_output() {
        let (r, sioe) = do_execute!(["/dev/null"], "to dev null\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "to dev null\n");
        assert!(r.is_ok());
        // verify that /dev/null is still empty
        let metadata = std::fs::metadata("/dev/null").unwrap();
        assert_eq!(metadata.len(), 0);
    }
    //
    #[test]
    fn test_append_mode() {
        let test_out = TestOut::new();
        let fnm = "append_test.txt";
        let target_path = test_out.target_path(fnm);
        let target_path_str = target_path.to_str().unwrap();
        //
        std::fs::write(target_path_str, "initial content\n").unwrap();
        //
        let (r, sioe) = do_execute!(["-a", target_path_str], "appended content\n");
        assert_eq!(buff!(sioe, serr), "");
        assert!(r.is_ok());
        //
        let content = std::fs::read_to_string(target_path_str).unwrap();
        assert_eq!(content, "initial content\nappended content\n");
    }
}

#[cfg(feature = "flate2")]
mod test_3_file_gz_s {
    use crate::helper2::TestOut;
    use libaki_xtee::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_gz() {
        let test_out = TestOut::new();
        let fnm = "out.text.gz";
        let target_path = test_out.target_path(fnm);
        let target_path_str = target_path.to_str().unwrap();
        //
        let (r, sioe) = do_execute!([target_path_str], "ABCDEFG\nHIJKLMN\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ABCDEFG\nHIJKLMN\n");
        assert!(r.is_ok());
        //
        assert!(test_out.cmp_file_with_fixtures(fnm).unwrap());
    }
}

#[cfg(feature = "xz2")]
mod test_3_file_xz2_s {
    use crate::helper2::TestOut;
    use libaki_xtee::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_xz() {
        let test_out = TestOut::new();
        let fnm = "out.text.xz";
        let target_path = test_out.target_path(fnm);
        let target_path_str = target_path.to_str().unwrap();
        //
        let (r, sioe) = do_execute!([target_path_str], "ABCDEFG\nHIJKLMN\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ABCDEFG\nHIJKLMN\n");
        assert!(r.is_ok());
        //
        assert!(test_out.cmp_file_with_fixtures(fnm).unwrap());
    }
}

#[cfg(feature = "zstd")]
mod test_3_file_zstd_s {
    use crate::helper2::TestOut;
    use libaki_xtee::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_zstd() {
        let test_out = TestOut::new();
        let fnm = "out.text.zst";
        let target_path = test_out.target_path(fnm);
        let target_path_str = target_path.to_str().unwrap();
        //
        let (r, sioe) = do_execute!([target_path_str], "ABCDEFG\nHIJKLMN\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ABCDEFG\nHIJKLMN\n");
        assert!(r.is_ok());
        //
        assert!(test_out.cmp_file_with_fixtures(fnm).unwrap());
    }
}

#[cfg(feature = "lz4")]
mod test_3_file_lz4_s {
    use crate::helper2::TestOut;
    use libaki_xtee::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_lz4() {
        let test_out = TestOut::new();
        let fnm = "out.text.lz4";
        let target_path = test_out.target_path(fnm);
        let target_path_str = target_path.to_str().unwrap();
        //
        let (r, sioe) = do_execute!([target_path_str], "ABCDEFG\nHIJKLMN\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ABCDEFG\nHIJKLMN\n");
        assert!(r.is_ok());
        //
        assert!(test_out.cmp_file_with_fixtures(fnm).unwrap());
    }
}

#[cfg(feature = "bzip2")]
mod test_3_file_bzip2_s {
    use crate::helper2::TestOut;
    use libaki_xtee::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_bzip2() {
        let test_out = TestOut::new();
        let fnm = "out.text.bz2";
        let target_path = test_out.target_path(fnm);
        let target_path_str = target_path.to_str().unwrap();
        //
        let (r, sioe) = do_execute!([target_path_str], "ABCDEFG\nHIJKLMN\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ABCDEFG\nHIJKLMN\n");
        assert!(r.is_ok());
        //
        assert!(test_out.cmp_file_with_fixtures(fnm).unwrap());
    }
}

mod test_4_complex_s {
    use crate::helper2::TestOut;
    use libaki_xtee::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_stdout_and_file_output() {
        let test_out = TestOut::new();
        let fnm = "another.plain.txt";
        let target_path = test_out.target_path(fnm);
        let target_path_str = target_path.to_str().unwrap();
        //
        let (r, sioe) = do_execute!(["-", target_path_str], "stdout and file\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "stdout and file\n");
        assert!(r.is_ok());
        //
        let content = std::fs::read_to_string(target_path_str).unwrap();
        assert_eq!(content, "stdout and file\n");
    }
    /*
    //
    #[cfg(unix)]
    #[test]
    fn test_piping_to_wc() {
        let cmd_str = format!("echo \"hello world\" | {} | wc -c", TARGET_EXE_PATH);
        let oup = exec_target("sh", &["-c", &cmd_str]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout.trim(), "12");
        assert!(oup.status.success());
    }
    //
    #[cfg(unix)]
    #[test]
    fn test_piping_to_grep() {
        let input = "hello\nworld\nhello again";
        let cmd_str = format!("echo \"{}\" | {} | grep hello", input, TARGET_EXE_PATH);
        let oup = exec_target("sh", &["-c", &cmd_str]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "hello\nhello again\n");
        assert!(oup.status.success());
    }
    */
}

#[cfg(feature = "flate2")]
#[cfg(feature = "xz2")]
#[cfg(feature = "zstd")]
#[cfg(feature = "lz4")]
#[cfg(feature = "bzip2")]
mod test_4_complex_more_s {
    use crate::helper2::TestOut;
    use libaki_xtee::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_multiple_files_different_compression() {
        let test_out = TestOut::new();
        let fnm_plain = "out.plain.txt";
        let fnm_gz = "out.text.gz";
        let fnm_xz = "out.text.xz";
        let fnm_zst = "out.text.zst";
        let fnm_lz4 = "out.text.lz4";
        let fnm_bz2 = "out.text.bz2";
        //
        let (r, sioe) = do_execute!(
            [
                test_out.target_path(fnm_plain),
                test_out.target_path(fnm_gz),
                test_out.target_path(fnm_xz),
                test_out.target_path(fnm_zst),
                test_out.target_path(fnm_lz4),
                test_out.target_path(fnm_bz2),
            ],
            "ABCDEFG\nHIJKLMN\n"
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ABCDEFG\nHIJKLMN\n");
        assert!(r.is_ok());
        //
        assert!(test_out.cmp_text_file_with_fixtures(fnm_plain).unwrap());
        assert!(test_out.cmp_file_with_fixtures(fnm_gz).unwrap());
        assert!(test_out.cmp_file_with_fixtures(fnm_xz).unwrap());
        assert!(test_out.cmp_file_with_fixtures(fnm_zst).unwrap());
        assert!(test_out.cmp_file_with_fixtures(fnm_lz4).unwrap());
        assert!(test_out.cmp_file_with_fixtures(fnm_bz2).unwrap());
    }
    //
    #[test]
    #[ignore]
    fn test_large_input_file() {
        use std::io::Read;
        let test_out = TestOut::new();
        let fnm_plain = "out.plain.txt";
        let fnm_gz = "out.text.gz";
        let target_path_plain = test_out.target_path(fnm_plain);
        let target_path_gz = test_out.target_path(fnm_gz);
        //
        let mut input_data = Vec::new();
        let mut f = flate2::read::GzDecoder::new(std::fs::File::open(fixture_text10k!()).unwrap());
        f.read_to_end(&mut input_data).unwrap();
        let s = String::from_utf8_lossy(&input_data).to_string();
        //
        let (r, sioe) = do_execute!([&target_path_plain, &target_path_gz,], &s);
        assert_eq!(buff!(sioe, serr), "");
        assert!(r.is_ok());
        //
        // Verify plain text file
        let content = std::fs::read(&target_path_plain).unwrap();
        assert_eq!(content, input_data);
        //
        // Verify gz file
        let mut gz_decoder =
            flate2::read::GzDecoder::new(std::fs::File::open(&target_path_gz).unwrap());
        let mut s = Vec::new();
        gz_decoder.read_to_end(&mut s).unwrap();
        assert_eq!(s, input_data);
    }
}

/*
mod test_5_s {
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
