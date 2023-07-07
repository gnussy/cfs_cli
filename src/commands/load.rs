use cli_prompts::{
  prompts::{AbortReason, Input},
  DisplayPrompt,
};
use reedline_repl_rs::{clap::ArgMatches, Error, Result as ReplResult};

use crate::context::CFSContext;

pub async fn load(args: ArgMatches, context: &mut CFSContext) -> ReplResult<Option<String>> {
  let image_path = show_input_prompt(args.get_one::<String>("image"))
    .map_err(|_| Error::UnknownCommand("Failed to get input".to_string()))?;

  let file = std::fs::OpenOptions::new()
    .read(true)
    .open(&image_path)
    .map_err(|_| Error::UnknownCommand("Failed to open file".to_string()))?;

  context
    .load_cfs(file)
    .map_err(|err| Error::UnknownCommand(format!("{err} {image_path}")))?;

  Ok(Some(format!("Loaded CFS image from {}", image_path)))
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
