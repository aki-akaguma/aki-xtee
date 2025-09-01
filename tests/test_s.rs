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
use helper2::{cmp_file, cmp_text_file};

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

mod test_0_x_options_s {
    use libaki_xtee::*;
    use runnel::medium::stringio::*;
    use runnel::*;
    use std::io::Write;
    use tempfile::tempdir;
    //
    #[test]
    fn test_x_rust_version_info() {
        let (r, sioe) = do_execute!(&["-X", "rust-version-info"]);
        assert_eq!(buff!(sioe, serr), "");
        assert!(!buff!(sioe, sout).is_empty());
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_x_option_help() {
        let (r, sioe) = do_execute!(&["-X", "help"]);
        assert_eq!(buff!(sioe, serr), "");
        assert!(buff!(sioe, sout).contains("Options:"));
        assert!(buff!(sioe, sout).contains("-X rust-version-info"));
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_x_option_rust_version_info() {
        let (r, sioe) = do_execute!(&["-X", "rust-version-info"]);
        assert_eq!(buff!(sioe, serr), "");
        assert!(buff!(sioe, sout).contains("rustc"));
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_multiple_x_options() {
        let (r, sioe) = do_execute!(&["-X", "help", "-X", "rust-version-info"]);
        assert_eq!(buff!(sioe, serr), "");
        // The first one should be executed and the program should exit.
        assert!(buff!(sioe, sout).contains("Options:"));
        assert!(!buff!(sioe, sout).contains("rustc"));
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_x_base_dir() {
        let temp_dir = tempdir().unwrap();
        let (r, sioe) = do_execute!(
            &[
                "-X",
                &format!("base_dir={}", temp_dir.path().to_str().unwrap()),
                "test_file.txt",
            ],
            "hello from base_dir\n"
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "hello from base_dir\n");
        assert!(r.is_ok());
        let content = std::fs::read_to_string(temp_dir.path().join("test_file.txt")).unwrap();
        assert_eq!(content, "hello from base_dir\n");
    }
    //
    #[test]
    fn test_x_base_dir_non_existent_dir() {
        let (r, sioe) = do_execute!(
            &["-X", "base_dir=/non/existent/dir", "test_file.txt"],
            "hello\n"
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
            assert_eq!(buff!(sioe, sout), "hello\n");
            assert!(r.is_ok());
        }
    }
    //
    #[test]
    fn test_x_base_dir_non_existent_file() {
        let temp_dir = tempdir().unwrap();
        let (r, sioe) = do_execute!(
            &[
                "-X",
                &format!("base_dir={}", temp_dir.path().to_str().unwrap()),
                "non_existent_dir/non_existent_file.txt",
            ],
            "hello\n"
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "hello\n");
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
    fn test_empty_input() {
        use std::io::Read;
        let out_file = "target/tmp/out_more_s/out.plain.txt";
        let _ = std::fs::create_dir_all("target/tmp/out_more_s");
        let (r, sioe) = do_execute!(&[out_file], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_ok());
        //
        let mut f = std::fs::File::open(out_file).unwrap();
        let mut buf = String::new();
        f.read_to_string(&mut buf).unwrap();
        assert_eq!(buf, "");
    }

    #[test]
    fn test_non_existent_output_dir() {
        let (r, sioe) = do_execute!(&["target/non_existent_dir/out.plain.txt"], "some data\n");
        //assert!(oup.stderr.contains("No such file or directory"));
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "some data\n");
        assert!(r.is_ok());
    }

    #[test]
    fn test_output_path_is_dir() {
        let _ = std::fs::create_dir_all("target/tmp/out_is_dir_s");
        let (r, sioe) = do_execute!(&["target/tmp/out_is_dir_s"], "some data\n");
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
        let dir = "target/no_write_permission_s";
        let out_file = "target/no_write_permission_s/out.plain.txt";
        let _ = std::fs::create_dir_all(dir);
        // set read-only permission
        let mut perms = std::fs::metadata(dir).unwrap().permissions();
        let perms_bak = perms.clone();
        perms.set_readonly(true);
        std::fs::set_permissions(dir, perms).unwrap();

        let (r, sioe) = do_execute!(&[out_file], "some data");

        // restore permission
        std::fs::set_permissions(dir, perms_bak).unwrap();

        assert!(buff!(sioe, serr).contains("Permission denied"));
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    //
    #[test]
    fn test_unsupported_file_extension() {
        let (r, sioe) = do_execute!(&["target/tmp/out_more2/out.unsupported"], "some data");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "some data\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_option_after_argument() {
        let (r, sioe) = do_execute!(&["target/tmp/out_more3/out.plain.txt", "-V"], "some data");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "aki-xtee 0.1.25\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_filename_with_spaces() {
        let _ = std::fs::create_dir_all("target/tmp/out_more3");
        let filename = "target/tmp/out_more3/file with spaces.txt";
        let (r, sioe) = do_execute!(&[filename], "some data");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "some data\n");
        assert!(r.is_ok());
        //
        let content = std::fs::read_to_string(filename).unwrap();
        assert_eq!(content, "some data\n");
    }
    //
    #[test]
    fn test_file_overwrite() {
        use std::io::Write;
        let _ = std::fs::create_dir_all("target/tmp/out_more4");
        let filename = "target/tmp/out_more4/overwrite.txt";
        // create a file with some initial content
        let mut f = std::fs::File::create(filename).unwrap();
        f.write_all(b"initial content").unwrap();
        //
        let (r, sioe) = do_execute!(&[filename], "new content");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "new content\n");
        assert!(r.is_ok());
        //
        let content = std::fs::read_to_string(filename).unwrap();
        assert_eq!(content, "new content\n");
    }
    //
    #[test]
    #[cfg(unix)]
    fn test_symlink() {
        use tempfile::tempdir;
        let temp_dir = tempdir().unwrap();
        let out_dir = temp_dir.path().join("out_more4");
        let _ = std::fs::create_dir_all(&out_dir);
        let filename = out_dir.join("symlink_target.txt");
        let symlink = out_dir.join("symlink.txt");
        // create a symlink to a file
        std::os::unix::fs::symlink(&filename, &symlink).unwrap();
        //
        let (r, sioe) = do_execute!(&[&symlink.display().to_string()], "symlink content");
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
        let _ = std::fs::create_dir_all("target/out_more5");
        let (r, sioe) = do_execute!(&["-"], "some data");
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
        let (r, sioe) = do_execute!(&[&long_arg], "some data");
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
        let _ = std::fs::create_dir_all("target/out_more2");
        let filename = "target/out_more2/!@#$%^&*().txt";
        //
        let (r, sioe) = do_execute!(&[filename], "special chars\n");
        #[cfg(not(windows))]
        {
            assert_eq!(buff!(sioe, serr), "");
            assert_eq!(buff!(sioe, sout), "special chars\n");
            assert!(r.is_ok());
            //
            let content = std::fs::read_to_string(filename).unwrap();
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
        let (r, sioe) = do_execute!(&["/dev/null"], "to dev null\n");
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
        let _ = std::fs::create_dir_all("target/out_more");
        let file_path = "target/out_more/append_test.txt";
        std::fs::write(file_path, "initial content\n").unwrap();
        //
        let (r, sioe) = do_execute!(&["-a", file_path], "appended content\n");
        assert_eq!(buff!(sioe, serr), "");
        assert!(r.is_ok());
        //
        let content = std::fs::read_to_string(file_path).unwrap();
        assert_eq!(content, "initial content\nappended content\n");
    }
}

#[cfg(feature = "flate2")]
mod test_3_file_gz_s {
    use libaki_xtee::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_gz() {
        let (r, sioe) = do_execute!(&["target/out_s020/out.text.gz"], "ABCDEFG\nHIJKLMN\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ABCDEFG\nHIJKLMN\n");
        assert!(r.is_ok());
        assert_file_eq!("target/out_s020/", "fixtures/", "out.text.gz");
    }
}

#[cfg(feature = "xz2")]
mod test_3_file_xz2_s {
    use libaki_xtee::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_xz() {
        let (r, sioe) = do_execute!(&["target/out_s020/out.text.xz"], "ABCDEFG\nHIJKLMN\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ABCDEFG\nHIJKLMN\n");
        assert!(r.is_ok());
        assert_file_eq!("target/out_s020/", "fixtures/", "out.text.xz");
    }
}

#[cfg(feature = "zstd")]
mod test_3_file_zstd_s {
    use libaki_xtee::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_zstd() {
        let (r, sioe) = do_execute!(&["target/out_s020/out.text.zst"], "ABCDEFG\nHIJKLMN\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ABCDEFG\nHIJKLMN\n");
        assert!(r.is_ok());
        assert_file_eq!("target/out_s020/", "fixtures/", "out.text.zst");
    }
}

#[cfg(feature = "lz4")]
mod test_3_file_lz4_s {
    use libaki_xtee::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_lz4() {
        let (r, sioe) = do_execute!(&["target/out_s020/out.text.lz4"], "ABCDEFG\nHIJKLMN\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ABCDEFG\nHIJKLMN\n");
        assert!(r.is_ok());
        assert_file_eq!("target/out_s020/", "fixtures/", "out.text.lz4");
    }
}

#[cfg(feature = "bzip2")]
mod test_3_file_bzip2_s {
    use libaki_xtee::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_bzip2() {
        let (r, sioe) = do_execute!(&["target/out_s020/out.text.bz2"], "ABCDEFG\nHIJKLMN\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ABCDEFG\nHIJKLMN\n");
        assert!(r.is_ok());
        assert_file_eq!("target/out_s020/", "fixtures/", "out.text.bz2");
    }
}

mod test_4_complex_s {
    use libaki_xtee::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_stdout_and_file_output() {
        let _ = std::fs::create_dir_all("target/out_more");
        let (r, sioe) = do_execute!(
            &["-", "target/out_more/another.plain.txt"],
            "stdout and file\n"
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "stdout and file\n");
        assert!(r.is_ok());
        //
        let content = std::fs::read_to_string("target/out_more/another.plain.txt").unwrap();
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
    use libaki_xtee::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_multiple_files_different_compression() {
        let (r, sioe) = do_execute!(
            &[
                "target/out_s021/out.plain.txt",
                "target/out_s021/out.text.gz",
                "target/out_s021/out.text.xz",
                "target/out_s021/out.text.zst",
                "target/out_s021/out.text.lz4",
                "target/out_s021/out.text.bz2",
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
        assert_file_eq!("target/out_s021/", "fixtures/", "out.text.lz4");
        assert_file_eq!("target/out_s021/", "fixtures/", "out.text.bz2");
    }
    //
    #[test]
    fn test_large_input_file() {
        use std::io::Read;
        let _ = std::fs::create_dir_all("target/out_more2");
        let mut input_data = Vec::new();
        let mut f = flate2::read::GzDecoder::new(std::fs::File::open(fixture_text10k!()).unwrap());
        f.read_to_end(&mut input_data).unwrap();
        let s = String::from_utf8_lossy(&input_data).to_string();
        //
        let (r, sioe) = do_execute!(
            &[
                "target/out_more2/out.plain.txt",
                "target/out_more2/out.text.gz",
            ],
            &s
        );
        assert_eq!(buff!(sioe, serr), "");
        assert!(r.is_ok());
        //
        // Verify plain text file
        let content = std::fs::read("target/out_more2/out.plain.txt").unwrap();
        assert_eq!(content, input_data);
        //
        // Verify gz file
        let mut gz_decoder = flate2::read::GzDecoder::new(
            std::fs::File::open("target/out_more2/out.text.gz").unwrap(),
        );
        let mut s = Vec::new();
        gz_decoder.read_to_end(&mut s).unwrap();
        assert_eq!(s, input_data);
    }
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
