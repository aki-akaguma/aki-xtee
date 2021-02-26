use super::Finish;
use std::fs::File;
use std::io::{BufWriter, Write};

pub struct PlainOut(Option<BufWriter<File>>);

impl PlainOut {
    pub fn new(file: File) -> anyhow::Result<Self> {
        let enc = BufWriter::new(file);
        Ok(Self(Some(enc)))
    }
}

impl Write for PlainOut {
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

impl Finish for PlainOut {
    fn finish(&mut self) -> anyhow::Result<()> {
        if let Some(_a) = self.0.take() {
            // nothing todo
        }
        Ok(())
    }
}
