#[cfg(feature = "bzip2")]
use bzip2::write::BzEncoder as Bzip2Encoder;

use super::Finish;
use std::fs::File;
use std::io::Write;

pub struct Bzip2Enc(Option<Bzip2Encoder<File>>);

impl Bzip2Enc {
    pub fn new(file: File) -> anyhow::Result<Self> {
        let enc = Bzip2Encoder::new(file, bzip2::Compression::new(3));
        Ok(Self(Some(enc)))
    }
}

impl Write for Bzip2Enc {
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

impl Finish for Bzip2Enc {
    fn finish(&mut self) -> anyhow::Result<()> {
        if let Some(a) = self.0.take() {
            a.finish()?;
        }
        Ok(())
    }
}
