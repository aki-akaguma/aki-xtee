#[allow(unused_macros)]
macro_rules! assert_file_eq {
    ($p1:expr, $p2:expr, $file_name:expr) => {
        assert_eq!(
            cmp_file(concat!($p1, $file_name), concat!($p2, $file_name)).unwrap(),
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
            cmp_text_file(concat!($p1, $file_name), concat!($p2, $file_name)).unwrap(),
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

#[allow(unused_macros)]
macro_rules! assert_binary_file_eq {
    ($p1:expr, $p2:expr, $file_name:expr) => {
        assert_eq!(
            cmp_binary_file(concat!($p1, $file_name), concat!($p2, $file_name)).unwrap(),
            true
        );
    };
}

#[allow(dead_code)]
pub fn cmp_binary_file<T1, T2>(path1: T1, path2: T2) -> std::io::Result<bool>
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
    if buf1 == buf2 {
        Ok(true)
    } else {
        let hex1 = hex::encode(&buf1);
        let hex2 = hex::encode(&buf2);
        assert_text::assert_text_eq!(&hex1, &hex2);
        Ok(false)
    }
}
