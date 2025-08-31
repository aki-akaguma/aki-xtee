use crate::util::compress::plain::PlainOut;

#[cfg(feature = "flate2")]
use crate::util::compress::gz::GzEnc;

#[cfg(feature = "xz2")]
use crate::util::compress::xz::XzEnc;

#[cfg(feature = "zstd")]
use crate::util::compress::zst::ZstEnc;

#[cfg(feature = "lz4")]
use crate::util::compress::lz4::Lz4Enc;

#[cfg(feature = "bzip2")]
use crate::util::compress::bzip2::Bzip2Enc;

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

pub fn open_files(base_dir: String, paths: &[String]) -> anyhow::Result<Vec<NameWrite>> {
    let mut vec = Vec::new();
    //
    for path_string in paths {
        let path_string = if base_dir.is_empty() {
            path_string.clone()
        } else {
            format!("{base_dir}/{path_string}")
        };
        let path = std::path::Path::new(&path_string);
        if let Some(parent) = path.parent() {
            if !parent.is_dir() {
                std::fs::create_dir_all(parent)?;
            }
        }
        //
        let file =
            File::create(path).with_context(|| format!("can not create file: {path_string}"))?;
        let w: Box<dyn Finish> = if path_string.ends_with(".gz") {
            #[cfg(feature = "flate2")]
            {
                let enc = GzEnc::new(file)?;
                Box::new(enc)
            }
            #[cfg(not(feature = "flate2"))]
            {
                bail!("not support '.gz' by compile option");
            }
        } else if path_string.ends_with(".xz") {
            #[cfg(feature = "xz2")]
            {
                let enc = XzEnc::new(file)?;
                Box::new(enc)
            }
            #[cfg(not(feature = "xz2"))]
            {
                bail!("not support '.xy' by compile option");
            }
        } else if path_string.ends_with(".zst") {
            #[cfg(feature = "zstd")]
            {
                let enc = ZstEnc::new(file)?;
                Box::new(enc)
            }
            #[cfg(not(feature = "zstd"))]
            {
                bail!("not support '.zst' by compile option");
            }
        } else if path_string.ends_with(".lz4") {
            #[cfg(feature = "lz4")]
            {
                let enc = Lz4Enc::new(file)?;
                Box::new(enc)
            }
            #[cfg(not(feature = "lz4"))]
            {
                bail!("not support '.lz4' by compile option");
            }
        } else if path_string.ends_with(".bz2") {
            #[cfg(feature = "bzip2")]
            {
                let enc = Bzip2Enc::new(file)?;
                Box::new(enc)
            }
            #[cfg(not(feature = "bzip2"))]
            {
                bail!("not support '.bz2' by compile option");
            }
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
