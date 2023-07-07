use reedline_repl_rs::{clap::ArgMatches, Result as ReplResult};
use tabled::{
  settings::{object::Segment, Alignment, Modify, Style},
  Table, Tabled,
};

use crate::context::CFSContext;

#[derive(Tabled, PartialEq, Eq, PartialOrd, Ord)]
struct CFSDentry {
  #[tabled(rename = "Inode index")]
  inode_index: String,
  #[tabled(rename = "File name")]
  file_name: String,
}

pub async fn ls(_args: ArgMatches, context: &mut CFSContext) -> ReplResult<Option<String>> {
  let cfs = context
    .cfs()
    .map_err(|err| reedline_repl_rs::Error::UnknownCommand(format!("Failed to get CFS: {err}")))?;

  let mut cfs = cfs.lock().await;
  let dentries = cfs
    .list_dentries_from_inode(context.current_inode())
    .map_err(|err| {
      reedline_repl_rs::Error::UnknownCommand(format!("Failed to list dentries: {err}"))
    })?;

  let mut table = Table::new(dentries.iter().map(|dentry| CFSDentry {
    inode_index: dentry.inode.to_string(),
    file_name: string_from_u8_slice(&dentry.name),
  }));

  table
    .with(Style::rounded())
    .with(Modify::new(Segment::all()).with(Alignment::left()));

  Ok(Some(format!("{}", table)))
}

// String from u8 slice
pub fn string_from_u8_slice(slice: &[u8]) -> String {
  let mut string = String::new();
  for byte in slice {
    string.push(*byte as char);
  }
  string
}
