const TARGET_EXE_PATH: &str = env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")));

#[macro_use]
mod helper2;
use helper2::{cmp_file, cmp_text_file};

mod test_0 {
    use exec_target::exec_target;
    //use exec_target::args_from;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_help() {
        let oup = exec_target(TARGET_EXE_PATH, ["-H"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, help_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_help_long() {
        let oup = exec_target(TARGET_EXE_PATH, ["--help"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, help_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_version() {
        let oup = exec_target(TARGET_EXE_PATH, ["-V"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, version_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_version_long() {
        let oup = exec_target(TARGET_EXE_PATH, ["--version"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, version_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_invalid_opt() {
        let oup = exec_target(TARGET_EXE_PATH, ["-z"]);
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
        assert!(!oup.status.success());
    }
}

mod test_0_x_options {
    use exec_target::exec_target;
    use exec_target::exec_target_with_in;
    use tempfile::tempdir;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_x_option_help() {
        let oup = exec_target(TARGET_EXE_PATH, ["-X", "help"]);
        assert_eq!(oup.stderr, "");
        assert!(oup.stdout.contains("Options:"));
        assert!(oup.stdout.contains("-X rust-version-info"));
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_x_option_rust_version_info() {
        let oup = exec_target(TARGET_EXE_PATH, ["-X", "rust-version-info"]);
        assert_eq!(oup.stderr, "");
        assert!(oup.stdout.contains("rustc"));
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_multiple_x_options() {
        let oup = exec_target(TARGET_EXE_PATH, ["-X", "help", "-X", "rust-version-info"]);
        assert_eq!(oup.stderr, "");
        // The first one should be executed and the program should exit.
        assert!(oup.stdout.contains("Options:"));
        assert!(!oup.stdout.contains("rustc"));
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_x_base_dir() {
        let temp_dir = tempdir().unwrap();
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            [
                "-X",
                &format!("base_dir={}", temp_dir.path().to_str().unwrap()),
                "test_file.txt",
            ],
            b"hello from base_dir\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "hello from base_dir\n");
        assert!(oup.status.success());
        let content = std::fs::read_to_string(temp_dir.path().join("test_file.txt")).unwrap();
        assert_eq!(content, "hello from base_dir\n");
    }
    //
    #[test]
    fn test_x_base_dir_non_existent_dir() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-X", "base_dir=/non/existent/dir", "test_file.txt"],
            b"hello\n" as &[u8],
        );
        #[cfg(target_os = "linux")]
        {
            assert!(oup.stderr.contains("Permission denied"));
            assert_eq!(oup.stdout, "");
            assert!(!oup.status.success());
        }
        #[cfg(target_os = "macos")]
        {
            assert!(oup.stderr.contains("Read-only file system"));
            assert_eq!(oup.stdout, "");
            assert!(!oup.status.success());
        }
        #[cfg(windows)]
        {
            assert_eq!(oup.stderr, "");
            assert_eq!(oup.stdout, "hello\n");
            assert!(oup.status.success());
            /*
            assert!(oup
                .stderr
                .contains("The system cannot find the path specified."));
            */
        }
    }
    //
    #[test]
    fn test_x_base_dir_non_existent_file() {
        let temp_dir = tempdir().unwrap();
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            [
                "-X",
                &format!("base_dir={}", temp_dir.path().to_str().unwrap()),
                "non_existent_dir/non_existent_file.txt",
            ],
            b"hello\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "hello\n");
        assert!(oup.status.success());
    }
}

mod test_1_stdout {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_non_option() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, &[] as &[&str], b"abcdefg\n" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "abcdefg\n");
        assert!(oup.status.success());
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
        assert!(!oup.status.success());
    }
}

mod test_2_file {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_plain() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["target/out020/out.plain.txt"],
            b"ABCDEFG\nHIJKLMN\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ABCDEFG\nHIJKLMN\n");
        assert!(oup.status.success());
        assert_text_file_eq!("target/out020/", "fixtures/", "out.plain.txt");
    }
    //
    #[test]
    fn test_empty_input() {
        use std::io::Read;
        let out_file = "target/tmp/out_more/out.plain.txt";
        let _ = std::fs::create_dir_all("target/tmp/out_more");
        let oup = exec_target_with_in(TARGET_EXE_PATH, [out_file], b"" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "");
        assert!(oup.status.success());
        let mut f = std::fs::File::open(out_file).unwrap();
        let mut buf = String::new();
        f.read_to_string(&mut buf).unwrap();
        assert_eq!(buf, "");
    }
    //
    #[test]
    fn test_non_existent_output_dir() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["target/non_existent_dir/out.plain.txt"],
            b"some data" as &[u8],
        );
        //assert!(oup.stderr.contains("No such file or directory"));
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "some data\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_output_path_is_dir() {
        let _ = std::fs::create_dir_all("target/tmp/out_is_dir");
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["target/tmp/out_is_dir"],
            b"some data" as &[u8],
        );
        assert_eq!(oup.stdout, "");
        // on windows, it is not `Is a directory`
        #[cfg(not(windows))]
        assert!(oup.stderr.contains("Is a directory"));
        #[cfg(windows)]
        assert!(oup.stderr.contains("Access is denied"));
        //assert!(oup.stderr.contains("Is a directory"));
        assert!(!oup.status.success());
    }
    //
    #[test]
    #[cfg(unix)] // This test is for unix-like systems
    fn test_write_permission_error() {
        let dir = "target/no_write_permission";
        let out_file = "target/no_write_permission/out.plain.txt";
        let _ = std::fs::create_dir_all(dir);
        // set read-only permission
        let mut perms = std::fs::metadata(dir).unwrap().permissions();
        let perms_bak = perms.clone();
        perms.set_readonly(true);
        std::fs::set_permissions(dir, perms).unwrap();

        let oup = exec_target_with_in(TARGET_EXE_PATH, [out_file], b"some data" as &[u8]);

        // restore permission
        std::fs::set_permissions(dir, perms_bak).unwrap();

        assert!(oup.stderr.contains("Permission denied"));
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_unsupported_file_extension() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["target/tmp/out_more2/out.unsupported"],
            b"some data" as &[u8],
        );
        assert_eq!(oup.stdout, "some data\n");
        assert_eq!(oup.stderr, "");
        //assert!(oup.stderr.contains("unsupported file extension"));
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_option_after_argument() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["target/tmp/out_more3/out.plain.txt", "-V"],
            b"some data" as &[u8],
        );
        assert_eq!(oup.stdout, "aki-xtee 0.1.25\n");
        assert_eq!(oup.stderr, "");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_filename_with_spaces() {
        let _ = std::fs::create_dir_all("target/tmp/out_more3");
        let filename = "target/tmp/out_more3/file with spaces.txt";
        let oup = exec_target_with_in(TARGET_EXE_PATH, [filename], b"some data" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "some data\n");
        assert!(oup.status.success());
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
        let oup = exec_target_with_in(TARGET_EXE_PATH, [filename], b"new content" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "new content\n");
        assert!(oup.status.success());
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

        let oup = exec_target_with_in(TARGET_EXE_PATH, [symlink], b"symlink content" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "symlink content\n");
        assert!(oup.status.success());

        let content = std::fs::read_to_string(filename).unwrap();
        assert_eq!(content, "symlink content\n");
    }
    //
    #[test]
    fn test_file_named_dash() {
        let _ = std::fs::create_dir_all("target/out_more5");
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-"], b"some data" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "some data\n");
        assert!(oup.status.success());
        // It should not create a file named "-"
        assert!(!std::path::Path::new("-").exists());
    }
    //
    #[test]
    fn test_very_long_argument() {
        let long_arg = "a".repeat(10000);
        let oup = exec_target_with_in(TARGET_EXE_PATH, [&long_arg], b"some data" as &[u8]);
        #[cfg(not(windows))]
        assert!(oup.stderr.contains("File name too long"));
        #[cfg(windows)]
        assert!(oup
            .stderr
            .contains("The filename, directory name, or volume label syntax is incorrect."));
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_binary_input() {
        let v = {
            use std::io::Read;
            let mut f = std::fs::File::open("fixtures/binary_data.bin").unwrap();
            let mut v = Vec::new();
            f.read_to_end(&mut v).unwrap();
            v
        };
        let oup = exec_target_with_in(TARGET_EXE_PATH, &[] as &[&str], &v);
        /*
        assert_eq!(
            oup.stderr,
            concat!(program_name!(), ": stream did not contain valid UTF-8\n",)
        );
        */
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "b\"\\x80\\x81\\x82\\x83\\x84\\x85\"\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_filename_with_special_chars() {
        let _ = std::fs::create_dir_all("target/out_more2");
        let filename = "target/out_more2/!@#$%^&*().txt";
        //
        let oup = exec_target_with_in(TARGET_EXE_PATH, [filename], b"special chars\n" as &[u8]);
        #[cfg(not(windows))]
        {
            assert_eq!(oup.stderr, "");
            assert_eq!(oup.stdout, "special chars\n");
            assert!(oup.status.success());
            //
            let content = std::fs::read_to_string(filename).unwrap();
            assert_eq!(content, "special chars\n");
        }
        #[cfg(windows)]
        {
            assert!(oup
                .stderr
                .contains("The filename, directory name, or volume label syntax is incorrect"));
            assert_eq!(oup.stdout, "");
            assert!(!oup.status.success());
        }
    }
    //
    #[test]
    #[cfg(unix)]
    fn test_dev_null_output() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["/dev/null"], b"to dev null\n" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "to dev null\n");
        assert!(oup.status.success());
        // verify that /dev/null is still empty
        let metadata = std::fs::metadata("/dev/null").unwrap();
        assert_eq!(metadata.len(), 0);
    }
    //
    #[test]
    #[ignore] // append is not implemented yet
    fn test_append_mode() {
        let _ = std::fs::create_dir_all("target/out_more");
        let file_path = "target/out_more/append_test.txt";
        std::fs::write(file_path, "initial content\n").unwrap();
        //
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-a", file_path],
            b"appended content\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert!(oup.status.success());
        //
        let content = std::fs::read_to_string(file_path).unwrap();
        assert_eq!(content, "initial content\nappended content\n");
    }
}

#[cfg(feature = "flate2")]
mod test_3_file_gz {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_gz() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["target/out020/out.text.gz"],
            b"ABCDEFG\nHIJKLMN\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ABCDEFG\nHIJKLMN\n");
        assert!(oup.status.success());
        assert_file_eq!("target/out020/", "fixtures/", "out.text.gz");
    }
}

#[cfg(feature = "xz2")]
mod test_3_file_xz2 {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_xz() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["target/out020/out.text.xz"],
            b"ABCDEFG\nHIJKLMN\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ABCDEFG\nHIJKLMN\n");
        assert!(oup.status.success());
        assert_file_eq!("target/out020/", "fixtures/", "out.text.xz");
    }
}

#[cfg(feature = "zstd")]
mod test_3_file_zstd {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_zstd() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["target/out020/out.text.zst"],
            b"ABCDEFG\nHIJKLMN\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ABCDEFG\nHIJKLMN\n");
        assert!(oup.status.success());
        assert_file_eq!("target/out020/", "fixtures/", "out.text.zst");
    }
}

#[cfg(feature = "lz4")]
mod test_2_file_lz4 {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_lz4() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["target/out020/out.text.lz4"],
            b"ABCDEFG\nHIJKLMN\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ABCDEFG\nHIJKLMN\n");
        assert!(oup.status.success());
        assert_file_eq!("target/out020/", "fixtures/", "out.text.lz4");
    }
}

#[cfg(feature = "bzip2")]
mod test_3_file_bzip2 {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_bzip2() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["target/out020/out.text.bz2"],
            b"ABCDEFG\nHIJKLMN\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ABCDEFG\nHIJKLMN\n");
        assert!(oup.status.success());
        assert_file_eq!("target/out020/", "fixtures/", "out.text.bz2");
    }
}

mod test_4_complex {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_stdout_and_file_output() {
        let _ = std::fs::create_dir_all("target/out_more");
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-", "target/out_more/another.plain.txt"],
            b"stdout and file\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "stdout and file\n");
        assert!(oup.status.success());
        let content = std::fs::read_to_string("target/out_more/another.plain.txt").unwrap();
        assert_eq!(content, "stdout and file\n");
    }
    //
    #[cfg(unix)]
    #[test]
    fn test_piping_to_wc() {
        use exec_target::exec_target;
        let cmd_str = format!("echo \"hello world\" | {} | wc -c", TARGET_EXE_PATH);
        let oup = exec_target("sh", ["-c", &cmd_str]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout.trim(), "12");
        assert!(oup.status.success());
    }
    //
    #[cfg(unix)]
    #[test]
    fn test_piping_to_grep() {
        use exec_target::exec_target;
        let input = "hello\nworld\nhello again";
        let cmd_str = format!("echo \"{}\" | {} | grep hello", input, TARGET_EXE_PATH);
        let oup = exec_target("sh", ["-c", &cmd_str]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "hello\nhello again\n");
        assert!(oup.status.success());
    }
    /*
    //
    #[test]
    #[cfg(unix)]
    fn test_file_locking() {
        let _ = fs::create_dir_all("target/out_more3");
        let filename = "target/out_more3/locked_file.txt";
        let mut file = fs::File::create(filename).unwrap();
        nix::fcntl::flock(
            std::os::unix::io::AsRawFd::as_raw_fd(&file),
            nix::fcntl::FlockArg::LockEx,
        )
        .unwrap();

        let oup = exec_target_with_in(TARGET_EXE_PATH, &[filename], b"some data" as &[u8]);

        // The behavior might differ based on implementation. It might block, or fail immediately.
        // Assuming it will fail with an error.
        assert!(!oup.status.success());
        assert!(
            oup.stderr.contains("Device or resource busy")
                || oup.stderr.contains("Permission denied")
        );

        // unlock the file
        nix::fcntl::flock(
            std::os::unix::io::AsRawFd::as_raw_fd(&file),
            nix::fcntl::FlockArg::Unlock,
        )
        .unwrap();
    }
    */
}

#[cfg(feature = "flate2")]
#[cfg(feature = "xz2")]
#[cfg(feature = "zstd")]
#[cfg(feature = "lz4")]
#[cfg(feature = "bzip2")]
mod test_4_complex_more {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_multiple_files_different_compression() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            [
                "target/out021/out.plain.txt",
                "target/out021/out.text.gz",
                "target/out021/out.text.xz",
                "target/out021/out.text.zst",
                "target/out021/out.text.lz4",
                "target/out021/out.text.bz2",
            ],
            b"ABCDEFG\nHIJKLMN\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ABCDEFG\nHIJKLMN\n");
        assert!(oup.status.success());
        assert_text_file_eq!("target/out021/", "fixtures/", "out.plain.txt");
        assert_file_eq!("target/out021/", "fixtures/", "out.text.gz");
        assert_file_eq!("target/out021/", "fixtures/", "out.text.xz");
        assert_file_eq!("target/out021/", "fixtures/", "out.text.zst");
        assert_file_eq!("target/out021/", "fixtures/", "out.text.lz4");
        assert_file_eq!("target/out021/", "fixtures/", "out.text.bz2");
    }
    //
    #[test]
    fn test_large_input_file() {
        use std::io::Read;
        let _ = std::fs::create_dir_all("target/out_more2");
        let mut input_data = Vec::new();
        let mut f = flate2::read::GzDecoder::new(std::fs::File::open(fixture_text10k!()).unwrap());
        f.read_to_end(&mut input_data).unwrap();
        //
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            [
                "target/out_more2/out.plain.txt",
                "target/out_more2/out.text.gz",
            ],
            &input_data,
        );
        assert_eq!(oup.stderr, "");
        assert!(oup.status.success());
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

mod test_5 {
    use exec_target::exec_target;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_output_broken_pipe() {
        #[cfg(target_os = "macos")]
        let cmd = "gzcat";
        #[cfg(not(target_os = "macos"))]
        let cmd = "zcat";
        //
        let cmdstr = format!(
            "{} \"{}\" | \"{}\" | head -n 2",
            cmd,
            fixture_text10k!(),
            TARGET_EXE_PATH,
        );
        let oup = exec_target("sh", ["-c", &cmdstr]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ABCDEFG\nHIJKLMN\n");
        assert!(oup.status.success());
    }
}
