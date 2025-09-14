const TARGET_EXE_PATH: &str = env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")));

#[macro_use]
mod helper;

#[macro_use]
mod helper_e;

mod test_0_e {
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

mod test_0_x_options_e {
    use exec_target::exec_target;
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_x_option_help() {
        let oup = exec_target(TARGET_EXE_PATH, ["-X", "help"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, x_help_msg!());
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
        let test_out = crate::helper::TestOut::new();
        let fnm = "test_file.txt";
        let target_path = test_out.target_path(fnm);
        let base_dir = test_out.base_dir();
        let base_dir_str = base_dir.to_str().unwrap();
        //
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-X", &format!("base_dir={base_dir_str}"), fnm],
            b"hello from base_dir\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "hello from base_dir\n");
        assert!(oup.status.success());
        //
        let content = std::fs::read_to_string(target_path).unwrap();
        assert_eq!(content, "hello from base_dir\n");
    }
    //
    #[test]
    fn test_x_base_dir_non_existent_dir() {
        let fnm = "test_file.txt";
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-X", "base_dir=/non/existent/dir", fnm],
            b"hello from base_dir\n" as &[u8],
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
            assert_eq!(oup.stdout, "hello from base_dir\n");
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
        let test_out = crate::helper::TestOut::new();
        let base_dir = test_out.base_dir();
        let base_dir_str = base_dir.to_str().unwrap();
        //
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            [
                "-X",
                &format!("base_dir={base_dir_str}"),
                "non_existent_dir/non_existent_file.txt",
            ],
            b"hello from base_dir\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "hello from base_dir\n");
        assert!(oup.status.success());
    }
}

mod test_1_stdout_e {
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
        let v = std::fs::read(fixture_invalid_utf8!()).unwrap();
        let oup = exec_target_with_in(TARGET_EXE_PATH, &[] as &[&str], &v);
        assert_eq!(
            oup.stderr,
            concat!(program_name!(), ": stream did not contain valid UTF-8\n",)
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
}

mod test_2_file_e {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_plain() {
        let test_out = crate::helper::TestOut::new();
        let fnm = "out.plain.txt";
        let target_path = test_out.target_path(fnm);
        let target_path_str = target_path.to_str().unwrap();
        //
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            [target_path_str],
            b"ABCDEFG\nHIJKLMN\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ABCDEFG\nHIJKLMN\n");
        assert!(oup.status.success());
        //
        assert!(test_out.cmp_text_file_with_fixtures(fnm).unwrap());
    }
    //
    #[test]
    fn test_empty_input() {
        use std::io::Read;
        //
        let test_out = crate::helper::TestOut::new();
        let fnm = "out.plain.txt";
        let target_path = test_out.target_path(fnm);
        let target_path_str = target_path.to_str().unwrap();
        //
        let oup = exec_target_with_in(TARGET_EXE_PATH, [target_path_str], b"" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "");
        assert!(oup.status.success());
        //
        let mut f = std::fs::File::open(target_path).unwrap();
        let mut buf = String::new();
        f.read_to_string(&mut buf).unwrap();
        assert_eq!(buf, "");
    }
    //
    #[test]
    fn test_non_existent_output_dir() {
        let test_out = crate::helper::TestOut::new();
        let out_dir = test_out.base_dir().to_str().unwrap();
        //
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            [&format!("{out_dir}/non_existent_dir/out.plain.txt")],
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
        let test_out = crate::helper::TestOut::new();
        let target_path = test_out.base_dir().join("out_is_dir");
        let target_path_str = target_path.to_str().unwrap();
        //
        let _ = std::fs::create_dir_all(&target_path);
        let oup = exec_target_with_in(TARGET_EXE_PATH, [target_path_str], b"some data" as &[u8]);
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
        let test_out = crate::helper::TestOut::new();
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

        let oup = exec_target_with_in(TARGET_EXE_PATH, [target_path_str], b"some data" as &[u8]);

        // restore permission
        std::fs::set_permissions(&target_path_dir, perms_bak).unwrap();

        assert!(oup.stderr.contains("Permission denied"));
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_unsupported_file_extension() {
        let test_out = crate::helper::TestOut::new();
        let fnm = "out.unsupported";
        let target_path = test_out.target_path(fnm);
        let target_path_str = target_path.to_str().unwrap();
        //
        let oup = exec_target_with_in(TARGET_EXE_PATH, [target_path_str], b"some data" as &[u8]);
        assert_eq!(oup.stdout, "some data\n");
        assert_eq!(oup.stderr, "");
        //assert!(oup.stderr.contains("unsupported file extension"));
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_option_after_argument() {
        let test_out = crate::helper::TestOut::new();
        let fnm = "out.plain.txt";
        let target_path = test_out.target_path(fnm);
        let target_path_str = target_path.to_str().unwrap();
        //
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            [target_path_str, "-V"],
            b"some data" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, version_msg!());
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_filename_with_spaces() {
        let test_out = crate::helper::TestOut::new();
        let fnm = "file with spaces.txt";
        let target_path = test_out.target_path(fnm);
        let target_path_str = target_path.to_str().unwrap();
        //
        let oup = exec_target_with_in(TARGET_EXE_PATH, [target_path_str], b"some data" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "some data\n");
        assert!(oup.status.success());
        //
        let content = std::fs::read_to_string(target_path_str).unwrap();
        assert_eq!(content, "some data\n");
    }
    //
    #[test]
    fn test_file_overwrite() {
        use std::io::Write;
        //
        let test_out = crate::helper::TestOut::new();
        let fnm = "overwrite.txt";
        let target_path = test_out.target_path(fnm);
        let target_path_str = target_path.to_str().unwrap();
        //
        // create a file with some initial content
        let mut f = std::fs::File::create(target_path_str).unwrap();
        f.write_all(b"initial content").unwrap();
        //
        let oup = exec_target_with_in(TARGET_EXE_PATH, [target_path_str], b"new content" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "new content\n");
        assert!(oup.status.success());
        //
        let content = std::fs::read_to_string(target_path_str).unwrap();
        assert_eq!(content, "new content\n");
    }
    //
    #[test]
    #[cfg(unix)]
    fn test_symlink() {
        let test_out = crate::helper::TestOut::new();
        let out_dir = test_out.base_dir();
        let _ = std::fs::create_dir_all(out_dir);
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
        let test_out = crate::helper::TestOut::new();
        let fnm = "!@#$%^&*().txt";
        let target_path = test_out.target_path(fnm);
        let target_path_str = target_path.to_str().unwrap();
        //
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            [target_path_str],
            b"special chars\n" as &[u8],
        );
        #[cfg(not(windows))]
        {
            assert_eq!(oup.stderr, "");
            assert_eq!(oup.stdout, "special chars\n");
            assert!(oup.status.success());
            //
            let content = std::fs::read_to_string(target_path_str).unwrap();
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
    fn test_append_mode() {
        let test_out = crate::helper::TestOut::new();
        let fnm = "append_test.txt";
        let target_path = test_out.target_path(fnm);
        let target_path_str = target_path.to_str().unwrap();
        //
        std::fs::write(target_path_str, "initial content\n").unwrap();
        //
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-a", target_path_str],
            b"appended content\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert!(oup.status.success());
        //
        let content = std::fs::read_to_string(target_path_str).unwrap();
        assert_eq!(content, "initial content\nappended content\n");
    }
}

#[cfg(feature = "flate2")]
mod test_3_file_gz_e {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_gz() {
        let test_out = crate::helper::TestOut::new();
        let fnm = "out.text.gz";
        let target_path = test_out.target_path(fnm);
        let target_path_str = target_path.to_str().unwrap();
        //
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            [target_path_str],
            b"ABCDEFG\nHIJKLMN\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ABCDEFG\nHIJKLMN\n");
        assert!(oup.status.success());
        //
        assert!(test_out.cmp_file_with_fixtures(fnm).unwrap());
    }
}

#[cfg(feature = "xz2")]
mod test_3_file_xz2_e {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_xz() {
        let test_out = crate::helper::TestOut::new();
        let fnm = "out.text.xz";
        let target_path = test_out.target_path(fnm);
        let target_path_str = target_path.to_str().unwrap();
        //
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            [target_path_str],
            b"ABCDEFG\nHIJKLMN\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ABCDEFG\nHIJKLMN\n");
        assert!(oup.status.success());
        //
        assert!(test_out.cmp_file_with_fixtures(fnm).unwrap());
    }
}

#[cfg(feature = "zstd")]
mod test_3_file_zstd_e {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_zstd() {
        let test_out = crate::helper::TestOut::new();
        let fnm = "out.text.zst";
        let target_path = test_out.target_path(fnm);
        let target_path_str = target_path.to_str().unwrap();
        //
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            [target_path_str],
            b"ABCDEFG\nHIJKLMN\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ABCDEFG\nHIJKLMN\n");
        assert!(oup.status.success());
        //
        assert!(test_out.cmp_file_with_fixtures(fnm).unwrap());
    }
}

#[cfg(feature = "lz4")]
mod test_2_file_lz4_e {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_lz4() {
        let test_out = crate::helper::TestOut::new();
        let fnm = "out.text.lz4";
        let target_path = test_out.target_path(fnm);
        let target_path_str = target_path.to_str().unwrap();
        //
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            [target_path_str],
            b"ABCDEFG\nHIJKLMN\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ABCDEFG\nHIJKLMN\n");
        assert!(oup.status.success());
        //
        assert!(test_out.cmp_file_with_fixtures(fnm).unwrap());
    }
}

#[cfg(feature = "bzip2")]
mod test_3_file_bzip2_e {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_bzip2() {
        let test_out = crate::helper::TestOut::new();
        let fnm = "out.text.bz2";
        let target_path = test_out.target_path(fnm);
        let target_path_str = target_path.to_str().unwrap();
        //
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            [target_path_str],
            b"ABCDEFG\nHIJKLMN\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ABCDEFG\nHIJKLMN\n");
        assert!(oup.status.success());
        //
        assert!(test_out.cmp_file_with_fixtures(fnm).unwrap());
    }
}

mod test_4_complex_e {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_stdout_and_file_output() {
        let test_out = crate::helper::TestOut::new();
        let fnm = "another.plain.txt";
        let target_path = test_out.target_path(fnm);
        let target_path_str = target_path.to_str().unwrap();
        //
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-", target_path_str],
            b"stdout and file\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "stdout and file\n");
        assert!(oup.status.success());
        //
        let content = std::fs::read_to_string(target_path_str).unwrap();
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
mod test_4_complex_more_e {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_multiple_files_different_compression() {
        let test_out = crate::helper::TestOut::new();
        let fnm_plain = "out.plain.txt";
        let fnm_gz = "out.text.gz";
        let fnm_xz = "out.text.xz";
        let fnm_zst = "out.text.zst";
        let fnm_lz4 = "out.text.lz4";
        let fnm_bz2 = "out.text.bz2";
        //
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            [
                test_out.target_path(fnm_plain),
                test_out.target_path(fnm_gz),
                test_out.target_path(fnm_xz),
                test_out.target_path(fnm_zst),
                test_out.target_path(fnm_lz4),
                test_out.target_path(fnm_bz2),
            ],
            b"ABCDEFG\nHIJKLMN\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ABCDEFG\nHIJKLMN\n");
        assert!(oup.status.success());
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
        let test_out = crate::helper::TestOut::new();
        let fnm_plain = "out.plain.txt";
        let fnm_gz = "out.text.gz";
        let target_path_plain = test_out.target_path(fnm_plain);
        let target_path_gz = test_out.target_path(fnm_gz);
        //
        let mut input_data = Vec::new();
        let mut f = flate2::read::GzDecoder::new(std::fs::File::open(fixture_text10k!()).unwrap());
        f.read_to_end(&mut input_data).unwrap();
        //
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            [&target_path_plain, &target_path_gz],
            &input_data,
        );
        assert_eq!(oup.stderr, "");
        assert!(oup.status.success());
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

mod test_5_e {
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
