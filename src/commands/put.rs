use cli_prompts::{
  prompts::{AbortReason, Input},
  DisplayPrompt,
};
use reedline_repl_rs::{clap::ArgMatches, Error, Result as ReplResult};

use crate::context::CFSContext;

pub async fn put(args: ArgMatches, _context: &mut CFSContext) -> ReplResult<Option<String>> {
  let _path = show_input_prompt(args.get_one::<String>("path"))
    .map_err(|_| Error::UnknownCommand("Failed to get input".to_string()))?;

  // let file = std::fs::OpenOptions::new()
  //   .read(true)
  //   .open(path)
  //   .map_err(|_| Error::UnknownCommand("Failed to open file".to_string()))?;

  Err(reedline_repl_rs::Error::UnknownCommand(
    "put not implemented yet".to_string(),
  ))
}

fn show_input_prompt(path: Option<&String>) -> Result<String, AbortReason> {
  let path: String = match path.is_some() {
    true => path.unwrap().to_string(),
    false => Input::new("Enter the file", validate_file)
      .default_value("test.txt")
      .display()?,
  };

  Ok(path)
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
