use cli_prompts::{
  prompts::{AbortReason, Input},
  DisplayPrompt,
};
use reedline_repl_rs::{clap::ArgMatches, Error, Result as ReplResult};

use crate::context::CFSContext;

pub async fn mkcfs(args: ArgMatches, _context: &mut CFSContext) -> ReplResult<Option<String>> {
  let (block_size, device) = show_input_prompt(
    args.get_one::<String>("block_size"),
    args.get_one::<String>("device"),
  )
  .map_err(|_| Error::UnknownCommand("Failed to get input".to_string()))?;

  let file = std::fs::OpenOptions::new()
    .read(true)
    .write(true)
    .open(&device)
    .map_err(|_| Error::UnknownCommand("Failed to open device".to_string()))?;

  let mut cfs_partition = cfs::partition::CfsPartition::new(file, block_size as u64)
    .map_err(|_| Error::UnknownCommand("Failed to create partition".to_string()))?;
  cfs_partition
    .setup_root_dir()
    .map_err(|_| Error::UnknownCommand("Failed to setup root directory".to_string()))?;

  Ok(Some(format!(
    "Created CFS partition with block size {} on device {}",
    block_size, device
  )))
}

fn show_input_prompt(
  block_size: Option<&String>,
  device: Option<&String>,
) -> Result<(u16, String), AbortReason> {
  let block_size: u16 = match block_size.is_some() {
    true => block_size.unwrap().parse::<u16>().unwrap(),
    false => Input::new("Enter the block size", validate_block_size)
      .default_value("4096")
      .display()?,
  };

  let device: String = match device.is_some() {
    true => device.unwrap().to_string(),
    false => Input::new("Enter the device", validate_device)
      .default_value("/dev/sda")
      .display()?,
  };

  Ok((block_size, device))
}

fn validate_block_size(block_size: &str) -> Result<u16, String> {
  match block_size.parse::<u16>() {
    Ok(block_size) => Ok(block_size),
    Err(_) => Err("Invalid block size".to_string()),
  }
}

fn validate_device(device: &str) -> Result<String, String> {
  // Check if its a valid file
  use std::fs;

  match fs::metadata(device) {
    Ok(metadata) => {
      if metadata.is_file() {
        Ok(device.to_string())
      } else {
        Err("Invalid device".to_string())
      }
    }
    Err(_) => Err("Invalid device".to_string()),
  }
}
