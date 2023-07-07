use cli_prompts::{
  prompts::{AbortReason, Input},
  DisplayPrompt,
};
use reedline_repl_rs::{clap::ArgMatches, Error, Result as ReplResult};

use crate::context::CFSContext;

pub async fn rm(args: ArgMatches, context: &mut CFSContext) -> ReplResult<Option<String>> {
  let cfs = context
    .cfs()
    .map_err(|err| reedline_repl_rs::Error::UnknownCommand(format!("Failed to get CFS: {err}")))?;

  let file_name = show_input_prompt(args.get_one::<String>("file"))
    .map_err(|_| Error::UnknownCommand("Failed to get input".to_string()))?;

  let mut cfs = cfs.lock().await;
  let dentries = cfs
    .list_dentries_from_inode(context.current_inode())
    .map_err(|err| {
      reedline_repl_rs::Error::UnknownCommand(format!("Failed to list dentries: {err}"))
    })?;

  // Find the inode index of the file
  let inode_index = dentries
    .iter()
    .find(|dentry| {
      let file_name_from_dentry = string_from_u8_slice(&dentry.name);
      println!("file_name_from_dentry: [{}]", file_name_from_dentry);
      println!("file_name: [{}]", file_name);
      file_name_from_dentry == file_name
    })
    .map(|dentry| dentry.inode)
    .ok_or_else(|| {
      reedline_repl_rs::Error::UnknownCommand(format!("File {} not found", file_name))
    })?;

  cfs.remove_inode(inode_index as usize).map_err(|err| {
    reedline_repl_rs::Error::UnknownCommand(format!("Failed to remove inode: {err}"))
  })?;

  Ok(Some(format!("Removed file {}", file_name)))
}
fn show_input_prompt(path: Option<&String>) -> Result<String, AbortReason> {
  let path: String = match path.is_some() {
    true => path.unwrap().to_string(),
    false => Input::new("Enter the file", |path| Ok(path.to_string()))
      .default_value("test.txt")
      .display()?,
  };

  Ok(path)
}

pub fn string_from_u8_slice(slice: &[u8]) -> String {
  let mut string = String::new();
  for byte in slice {
    string.push(*byte as char);
  }
  string
}
