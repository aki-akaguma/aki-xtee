use crate::conf::CmdOptConf;
use crate::util::compress::Finish;
use crate::util::err::BrokenPipeError;
use crate::util::open_files;
use runnel::RunnelIoe;
use std::io::{BufRead, Write};

pub fn run(sioe: &RunnelIoe, conf: &CmdOptConf) -> anyhow::Result<()> {
    //println!("{:?}", conf);
    //
    let r = run_0(sioe, &conf.arg_params);
    if r.is_broken_pipe() {
        return Ok(());
    }
    r
}
fn run_0(sioe: &RunnelIoe, files: &[String]) -> anyhow::Result<()> {
    let mut file_vec = open_files(files)?;
    //
    for line in sioe.pin().lock().lines() {
        let line_s = line?;
        let line_ss = line_s.as_str();
        //let line_len: usize = line_ss.len();
        //
        for file in file_vec.iter_mut() {
            file.write_fmt(format_args!("{line_ss}\n"))?;
        }
        //
        #[rustfmt::skip]
        sioe.pout().lock().write_fmt(format_args!("{line_ss}\n"))?;
    }
    //
    sioe.pout().lock().flush()?;
    {
        for file in file_vec.iter_mut() {
            file.flush()?;
            file.finish()?;
        }
    }
    //
    Ok(())
}
