pub mod plain;

#[cfg(feature = "flate2")]
pub mod gz;

#[cfg(feature = "xz2")]
pub mod xz;

#[cfg(feature = "zstd")]
pub mod zst;

use std::io::Write;

pub trait Finish: Write {
    fn finish(&mut self) -> anyhow::Result<()>;
}
