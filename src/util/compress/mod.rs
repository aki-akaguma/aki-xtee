pub mod plain;

#[cfg(feature = "flate2")]
pub mod gz;

#[cfg(feature = "xz2")]
pub mod xz;

#[cfg(feature = "zstd")]
pub mod zst;

#[cfg(feature = "lz4")]
pub mod lz4;

#[cfg(feature = "bzip2")]
pub mod bzip2;

use std::io::Write;

pub trait Finish: Write {
    fn finish(&mut self) -> anyhow::Result<()>;
}
