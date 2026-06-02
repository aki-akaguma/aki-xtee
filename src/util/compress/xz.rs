#[cfg(feature = "xz2")]
use xz2::write::XzEncoder;

use super::Finish;
use std::fs::File;
use std::io::Write;

pub struct XzEnc {
    encoder: Option<XzEncoder<File>>,
}

impl XzEnc {
    pub fn new(file: File) -> anyhow::Result<Self> {
        let encoder = XzEncoder::new(file, 6);
        Ok(Self {
            encoder: Some(encoder),
        })
    }
}

impl Write for XzEnc {
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

impl Finish for XzEnc {
    fn finish(&mut self) -> anyhow::Result<()> {
        if let Some(encoder) = self.encoder.take() {
            encoder.finish()?;
        }
        Ok(())
    }
}
