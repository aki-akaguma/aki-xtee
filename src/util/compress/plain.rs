use super::Finish;
use std::fs::File;
use std::io::{BufWriter, Write};

pub struct PlainOut {
    writer: BufWriter<File>,
    finished: bool,
}

impl PlainOut {
    pub fn new(file: File) -> anyhow::Result<Self> {
        let writer = BufWriter::new(file);
        Ok(Self {
            writer,
            finished: false,
        })
    }
}

impl Write for PlainOut {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if self.finished {
            return Err(std::io::Error::other("writer is finished"));
        }
        self.writer.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        if self.finished {
            return Err(std::io::Error::other("writer is finished"));
        }
        self.writer.flush()
    }
}

impl Finish for PlainOut {
    fn finish(&mut self) -> anyhow::Result<()> {
        if !self.finished {
            self.writer.flush()?;
            self.finished = true;
        }
        Ok(())
    }
}
