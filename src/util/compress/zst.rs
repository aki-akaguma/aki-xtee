#[cfg(feature = "zstd")]
use zstd::stream::write::Encoder as ZstEncoder;

use super::Finish;
use std::fs::File;
use std::io::Write;

pub struct ZstEnc {
    encoder: Option<ZstEncoder<'static, File>>,
}

impl ZstEnc {
    pub fn new(file: File) -> anyhow::Result<Self> {
        let encoder = ZstEncoder::new(file, 3)?;
        Ok(Self {
            encoder: Some(encoder),
        })
    }
}

impl Write for ZstEnc {
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

impl Finish for ZstEnc {
    fn finish(&mut self) -> anyhow::Result<()> {
        if let Some(encoder) = self.encoder.take() {
            encoder.finish()?;
        }
        Ok(())
    }
}
