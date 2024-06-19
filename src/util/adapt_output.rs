use crate::util::compress::gz::GzEnc;
use crate::util::compress::plain::PlainOut;
use crate::util::compress::xz::XzEnc;
use crate::util::compress::zst::ZstEnc;
use crate::util::compress::Finish;
use anyhow::Context;
use std::fs::File;
use std::io::Write;

#[allow(dead_code)]
pub struct NameWrite {
    pub name: String,
    pub write: Box<dyn Finish>,
}

impl Finish for NameWrite {
    fn finish(&mut self) -> anyhow::Result<()> {
        self.write.finish()
    }
}

impl Write for NameWrite {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.write.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.write.flush()
    }
}

pub fn open_files(paths: &[String]) -> anyhow::Result<Vec<NameWrite>> {
    let mut vec = Vec::new();
    //
    for path_string in paths {
        let path = std::path::Path::new(path_string);
        if let Some(parent) = path.parent() {
            if !parent.is_dir() {
                std::fs::create_dir_all(parent)?;
            }
        }
        //
        let file =
            File::create(path).with_context(|| format!("can not create file: {path_string}"))?;
        let w: Box<dyn Finish> = if path_string.ends_with(".gz") {
            let enc = GzEnc::new(file)?;
            Box::new(enc)
        } else if path_string.ends_with(".xz") {
            let enc = XzEnc::new(file)?;
            Box::new(enc)
        } else if path_string.ends_with(".zst") {
            let enc = ZstEnc::new(file)?;
            Box::new(enc)
        } else {
            let enc = PlainOut::new(file)?;
            Box::new(enc)
        };
        vec.push(NameWrite {
            name: path_string.clone(),
            write: w,
        });
    }
    //
    Ok(vec)
}
