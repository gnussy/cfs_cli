use cli_prompts::{
  prompts::{AbortReason, Input},
  DisplayPrompt,
};
use reedline_repl_rs::{clap::ArgMatches, Error, Result as ReplResult};

use crate::context::CFSContext;

pub async fn rmi(args: ArgMatches, context: &mut CFSContext) -> ReplResult<Option<String>> {
  let cfs = context
    .cfs()
    .map_err(|err| reedline_repl_rs::Error::UnknownCommand(format!("Failed to get CFS: {err}")))?;

  let inode = show_input_prompt(args.get_one::<String>("inode"))
    .map_err(|_| Error::UnknownCommand("Failed to get input".to_string()))?;

  let mut cfs = cfs.lock().await;
  cfs
    .remove_dir_from_inode(context.current_inode(), inode as u32)
    .map_err(|err| {
      reedline_repl_rs::Error::UnknownCommand(format!("Failed to remove inode: {err}"))
    })?;

  Ok(Some(format!("Removed inode {}", inode.to_string())))
}

fn show_input_prompt(inode: Option<&String>) -> Result<usize, AbortReason> {
  let inode: usize = match inode.is_some() {
    true => inode.unwrap().parse::<usize>().unwrap(),
    false => Input::new("Enter the inode to remove", |inode| {
      Ok(inode.parse::<usize>().unwrap())
    })
    .default_value("2")
    .display()?,
  };

  Ok(inode)
}
