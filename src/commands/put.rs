use cli_prompts::{
  prompts::{AbortReason, Input},
  DisplayPrompt,
};
use reedline_repl_rs::{clap::ArgMatches, Error, Result as ReplResult};

use crate::context::CFSContext;

pub async fn put(args: ArgMatches, context: &mut CFSContext) -> ReplResult<Option<String>> {
  let cfs = context
    .cfs()
    .map_err(|err| reedline_repl_rs::Error::UnknownCommand(format!("Failed to get CFS: {err}")))?;

  let (path, name) = show_input_prompt(args.get_one::<String>("path"))
    .map_err(|_| Error::UnknownCommand("Failed to get input".to_string()))?;

  let mut file = std::fs::OpenOptions::new()
    .read(true)
    .open(path)
    .unwrap();

  let mut cfs = cfs.lock().await;
  cfs
    .add_file_to_inode(context.current_inode(), &name, &mut file)
    .map_err(|err| Error::UnknownCommand(format!("Failed to add file: {err}")))?;

  Ok(Some(format!("Added file {} successfully", name)))
}

fn show_input_prompt(path: Option<&String>) -> Result<(String, String), AbortReason> {
  let path: String = match path.is_some() {
    true => path.unwrap().to_string(),
    false => Input::new("Enter the file", validate_file)
      .default_value("test.txt")
      .display()?,
  };

  let name: String = Input::new("Enter the name", |name| Ok(name.to_string()))
    .default_value("test.txt")
    .display()?;

  Ok((path, name))
}

/// Check if its a valid file
fn validate_file(path: &str) -> Result<String, String> {
  use std::fs;

  match fs::metadata(path) {
    Ok(metadata) => {
      if metadata.is_file() {
        Ok(path.to_string())
      } else {
        Err("Invalid file".to_string())
      }
    }
    Err(_) => Err("Invalid file".to_string()),
  }
}
