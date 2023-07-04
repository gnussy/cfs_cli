use crate::context::CFSContext;
use reedline_repl_rs::{
  clap::{arg, Arg, Command},
  AsyncCallback, Error, Result,
};

pub mod get;
pub mod host_ls;
pub mod ls;
pub mod mkcfs;
pub mod put;

pub struct Commands;

type CFSCallback = AsyncCallback<CFSContext, Error>;

impl Commands {
  pub fn all() -> Result<Vec<(Command, CFSCallback)>> {
    Ok(vec![
      Commands::ls()?,
      Commands::host_ls()?,
      Commands::put()?,
      Commands::get()?,
      Commands::mkcfs()?,
    ])
  }

  pub fn ls() -> Result<(Command, CFSCallback)> {
    Ok((
      Command::new("ls")
        .arg(Arg::new("path").default_value("."))
        .about("List files in a directory in the CFS"),
      |args, context| Box::pin(ls::ls(args, context)),
    ))
  }

  pub fn host_ls() -> Result<(Command, CFSCallback)> {
    Ok((
      Command::new("!ls")
        .arg(Arg::new("path").default_value("."))
        .about("List files in a directory in the host file system"),
      |args, context| Box::pin(host_ls::ls(args, context)),
    ))
  }

  pub fn put() -> Result<(Command, CFSCallback)> {
    Ok((
      Command::new("put")
        .arg(Arg::new("path").default_value("."))
        .about("Put a file in the CFS from the host file system"),
      |args, context| Box::pin(put::put(args, context)),
    ))
  }

  pub fn get() -> Result<(Command, CFSCallback)> {
    Ok((
      Command::new("get")
        .arg(Arg::new("path").default_value("."))
        .about("Get a file from the CFS into the host file system"),
      |args, context| Box::pin(get::get(args, context)),
    ))
  }

  pub fn mkcfs() -> Result<(Command, CFSCallback)> {
    Ok((
      Command::new("mkcfs")
        .arg(arg!(-b --block_size <BLOCK_SIZE> "Block size in bytes"))
        .arg(arg!(-d --device <DEVICE> "Block size in bytes"))
        .about("Format a file as a CFS"),
      |args, context| Box::pin(mkcfs::mkcfs(args, context)),
    ))
  }
}
