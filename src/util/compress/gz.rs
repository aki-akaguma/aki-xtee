#[cfg(feature = "flate2")]
use flate2::write::GzEncoder;
#[cfg(feature = "flate2")]
use flate2::Compression;

use super::Finish;
use std::fs::File;
use std::io::Write;

pub struct GzEnc(Option<GzEncoder<File>>);

impl GzEnc {
    pub fn new(file: File) -> anyhow::Result<Self> {
        let enc = GzEncoder::new(file, Compression::new(6));
        Ok(Self(Some(enc)))
    }
}

impl Write for GzEnc {
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

impl Finish for GzEnc {
    fn finish(&mut self) -> anyhow::Result<()> {
        if let Some(a) = self.0.take() {
            a.finish()?;
        }
        Ok(())
    }
}
