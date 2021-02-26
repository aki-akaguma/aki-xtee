#[cfg(feature = "zstd")]
use zstd::stream::write::Encoder as ZstEncoder;

use super::Finish;
use std::fs::File;
use std::io::Write;

pub struct ZstEnc(Option<ZstEncoder<'static, File>>);

impl ZstEnc {
    pub fn new(file: File) -> anyhow::Result<Self> {
        let enc = ZstEncoder::new(file, 3)?;
        Ok(Self(Some(enc)))
    }
}

impl Write for ZstEnc {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if let Some(mut a) = self.0.take() {
            let r = a.write(buf);
            let _ = self.0.replace(a);
            r
        } else {
            Ok(0)
        }
    }
    fn flush(&mut self) -> std::io::Result<()> {
        if let Some(mut a) = self.0.take() {
            let r = a.flush();
            let _ = self.0.replace(a);
            r
        } else {
            Ok(())
        }
    }
}

impl Finish for ZstEnc {
    fn finish(&mut self) -> anyhow::Result<()> {
        if let Some(a) = self.0.take() {
            a.finish()?;
        }
        Ok(())
    }
}
