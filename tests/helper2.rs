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
