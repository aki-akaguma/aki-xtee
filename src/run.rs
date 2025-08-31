use crate::conf::CmdOptConf;
use crate::util::compress::Finish;
use crate::util::err::BrokenPipeError;
use crate::util::open_files;
use runnel::RunnelIoe;
use std::io::Write;

pub fn run(sioe: &RunnelIoe, conf: &CmdOptConf) -> anyhow::Result<()> {
    //println!("{:?}", conf);
    //
    let r = run_0(sioe, conf);
    if r.is_broken_pipe() {
        return Ok(());
    }
    r
}
fn run_0(sioe: &RunnelIoe, conf: &CmdOptConf) -> anyhow::Result<()> {
    let files: &[String] = &conf.arg_params;
    let mut file_vec = open_files(conf.base_dir(), files)?;
    //
    for line in sioe.pg_in().lines() {
        let line_s = line?;
        let line_ss = line_s.as_str();
        //
        for file in file_vec.iter_mut() {
            file.write_fmt(format_args!("{line_ss}\n"))?;
        }
        //
        sioe.pg_out().write_line(line_s)?;
    }
    //
    sioe.pg_out().flush_line()?;
    {
        for file in file_vec.iter_mut() {
            file.flush()?;
            file.finish()?;
        }
    }
    //
    Ok(())
}
