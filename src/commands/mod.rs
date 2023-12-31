use crate::context::CFSContext;
use reedline_repl_rs::{
  clap::{arg, Arg, Command},
  AsyncCallback, Error, Result,
};

pub mod cat;
pub mod get;
pub mod host_ls;
pub mod info;
pub mod load;
pub mod ls;
pub mod mkcfs;
pub mod put;
pub mod rm;
pub mod rmi;

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
      Commands::load()?,
      Commands::info()?,
      Commands::rm()?,
      Commands::rmi()?,
      Commands::cat()?,
    ])
  }

  pub fn ls() -> Result<(Command, CFSCallback)> {
    Ok((
      Command::new("ls").about("List files in a directory in the CFS"),
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
        .arg(Arg::new("path"))
        .about("Put a file in the CFS from the host file system"),
      |args, context| Box::pin(put::put(args, context)),
    ))
  }

  pub fn get() -> Result<(Command, CFSCallback)> {
    Ok((
      Command::new("get")
        .arg(Arg::new("inode"))
        .arg(Arg::new("path"))
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

  pub fn load() -> Result<(Command, CFSCallback)> {
    Ok((
      Command::new("load")
        .arg(arg!(-i --image <IMAGE> "Path to the CFS image"))
        .about("Load a CFS from a file"),
      |args, context| Box::pin(load::load(args, context)),
    ))
  }

  pub fn info() -> Result<(Command, CFSCallback)> {
    Ok((
      Command::new("info").about("Get information about the CFS"),
      |_args, context| Box::pin(info::info(context)),
    ))
  }

  pub fn rm() -> Result<(Command, CFSCallback)> {
    Ok((
      Command::new("rm")
        .arg(Arg::new("file"))
        .about("Remove a file from the CFS through its name"),
      |args, context| Box::pin(rm::rm(args, context)),
    ))
  }

  pub fn rmi() -> Result<(Command, CFSCallback)> {
    Ok((
      Command::new("rmi")
        .arg(Arg::new("inode"))
        .about("Remove a file from the CFS through its inode"),
      |args, context| Box::pin(rmi::rmi(args, context)),
    ))
  }

  pub fn cat() -> Result<(Command, CFSCallback)> {
    Ok((
      Command::new("cat")
        .arg(Arg::new("inode"))
        .about("Print the contents of a file in the CFS"),
      |args, context| Box::pin(cat::cat(args, context)),
    ))
  }
}
