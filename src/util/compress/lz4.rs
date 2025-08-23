#[cfg(feature = "lz4")]
use lz4::Encoder as Lz4Encoder;

use super::Finish;
use std::fs::File;
use std::io::Write;

pub struct Lz4Enc(Option<Lz4Encoder<File>>);

impl Lz4Enc {
    pub fn new(file: File) -> anyhow::Result<Self> {
        //let enc = Lz4Encoder::new(file, 3)?;
        let enc = lz4::EncoderBuilder::new().level(3).build(file)?;
        Ok(Self(Some(enc)))
    }
}

impl Write for Lz4Enc {
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

impl Finish for Lz4Enc {
    fn finish(&mut self) -> anyhow::Result<()> {
        if let Some(a) = self.0.take() {
            let (_w, r) = a.finish();
            r?;
        }
        Ok(())
    }
}
