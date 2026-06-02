#[cfg(feature = "flate2")]
use flate2::write::GzEncoder;
#[cfg(feature = "flate2")]
use flate2::Compression;

use super::Finish;
use std::fs::File;
use std::io::Write;

pub struct GzEnc {
    encoder: Option<GzEncoder<File>>,
}

impl GzEnc {
    pub fn new(file: File) -> anyhow::Result<Self> {
        let encoder = GzEncoder::new(file, Compression::new(6));
        Ok(Self {
            encoder: Some(encoder),
        })
    }
}

impl Write for GzEnc {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if let Some(ref mut encoder) = self.encoder {
            encoder.write(buf)
        } else {
            Err(std::io::Error::other("encoder is finished"))
        }
    }
    fn flush(&mut self) -> std::io::Result<()> {
        if let Some(ref mut encoder) = self.encoder {
            encoder.flush()
        } else {
            Err(std::io::Error::other("encoder is finished"))
        }
    }
}

impl Finish for GzEnc {
    fn finish(&mut self) -> anyhow::Result<()> {
        if let Some(encoder) = self.encoder.take() {
            encoder.finish()?;
        }
        Ok(())
    }
}
