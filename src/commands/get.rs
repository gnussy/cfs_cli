use std::io::Write;

use cli_prompts::{
  prompts::{AbortReason, Input},
  DisplayPrompt,
};
use reedline_repl_rs::{clap::ArgMatches, Error, Result as ReplResult};

use crate::context::CFSContext;

pub async fn get(args: ArgMatches, context: &mut CFSContext) -> ReplResult<Option<String>> {
  let cfs = context
    .cfs()
    .map_err(|err| reedline_repl_rs::Error::UnknownCommand(format!("Failed to get CFS: {err}")))?;

  let (path, inode) = show_input_prompt(
    args.get_one::<String>("path"),
    args.get_one::<String>("inode"),
  )
  .map_err(|_| Error::UnknownCommand("Failed to get input".to_string()))?;

  let mut cfs = cfs.lock().await;
  let data = cfs.get_data_from_inode(inode).map_err(|err| {
    reedline_repl_rs::Error::UnknownCommand(format!("Failed to get data from inode: {err}"))
  })?;

  // Save data to file
  let mut file = std::fs::File::create(&path).unwrap();
  file.write_all(&data).unwrap();

  Ok(Some(format!(
    "Saved inode {} to file {}",
    inode.to_string(),
    path
  )))
}

fn show_input_prompt(
  path: Option<&String>,
  inode: Option<&String>,
) -> Result<(String, usize), AbortReason> {
  let path: String = match path.is_some() {
    true => path.unwrap().to_string(),
    false => Input::new("Enter the file", |path| Ok(path.to_string()))
      .default_value("test.txt")
      .display()?,
  };

  let inode: usize = match inode.is_some() {
    true => inode.unwrap().parse::<usize>().unwrap(),
    false => Input::new("Enter the inode to save", |inode| {
      Ok(inode.parse::<usize>().unwrap())
    })
    .default_value("2")
    .display()?,
  };

  Ok((path, inode))
}
