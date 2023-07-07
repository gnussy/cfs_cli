use cli_prompts::{
  prompts::{AbortReason, Input},
  DisplayPrompt,
};
use reedline_repl_rs::{clap::ArgMatches, Error, Result as ReplResult};

use crate::context::CFSContext;

pub async fn cat(args: ArgMatches, context: &mut CFSContext) -> ReplResult<Option<String>> {
  let cfs = context
    .cfs()
    .map_err(|err| reedline_repl_rs::Error::UnknownCommand(format!("Failed to get CFS: {err}")))?;

  let inode = show_input_prompt(args.get_one::<String>("inode"))
    .map_err(|_| Error::UnknownCommand("Failed to get input".to_string()))?;

  let mut cfs = cfs.lock().await;
  let data = cfs.get_data_from_inode(inode).map_err(|err| {
    reedline_repl_rs::Error::UnknownCommand(format!("Failed to get data from inode: {err}"))
  })?;

  Ok(Some(string_from_u8_slice(&data)))
}

fn show_input_prompt(inode: Option<&String>) -> Result<usize, AbortReason> {
  let inode: usize = match inode.is_some() {
    true => inode.unwrap().parse::<usize>().unwrap(),
    false => Input::new("Enter the inode to cat", |inode| {
      Ok(inode.parse::<usize>().unwrap())
    })
    .default_value("2")
    .display()?,
  };

  Ok(inode)
}

// String from u8 slice
pub fn string_from_u8_slice(slice: &[u8]) -> String {
  let mut string = String::new();
  for byte in slice {
    string.push(*byte as char);
  }
  string
}
