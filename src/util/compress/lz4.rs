#[cfg(feature = "lz4")]
use lz4::Encoder as Lz4Encoder;

use super::Finish;
use std::fs::File;
use std::io::Write;

pub struct Lz4Enc {
    encoder: Option<Lz4Encoder<File>>,
}

impl Lz4Enc {
    pub fn new(file: File) -> anyhow::Result<Self> {
        let encoder = lz4::EncoderBuilder::new().level(3).build(file)?;
        Ok(Self {
            encoder: Some(encoder),
        })
    }
}

impl Write for Lz4Enc {
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

impl Finish for Lz4Enc {
    fn finish(&mut self) -> anyhow::Result<()> {
        if let Some(encoder) = self.encoder.take() {
            let (_w, r) = encoder.finish();
            r?;
        }
        Ok(())
    }
}
