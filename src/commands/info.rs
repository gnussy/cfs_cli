use reedline_repl_rs::{clap::ArgMatches, Result as ReplResult};
use tabled::{
  settings::{object::Segment, Alignment, Modify, Style},
  Table, Tabled,
};

use crate::context::CFSContext;

#[derive(Tabled, PartialEq, Eq, PartialOrd, Ord)]
// bam offset, iam offset, inode list offset, data block offset
struct CFSReport {
  #[tabled(rename = "BAM Offset")]
  bam_offset: String,
  #[tabled(rename = "IAM Offset")]
  iam_offset: String,
  #[tabled(rename = "Inode List Offset")]
  inode_list_offset: String,
  #[tabled(rename = "Data Block Offset")]
  data_block_offset: String,
}

pub async fn info(context: &mut CFSContext) -> ReplResult<Option<String>> {
  let cfs = context
    .get_cfs()
    .map_err(|err| reedline_repl_rs::Error::UnknownCommand(format!("Failed to get CFS: {err}")))?;

  let cfs = cfs.lock().await;
  let (bam_offset, iam_offset, inode_list_offset, data_block_offset) = cfs.info();

  let table = CFSReport {
    bam_offset,
    iam_offset,
    inode_list_offset,
    data_block_offset,
  };

  let mut table = Table::new(&[table]);
  table
    .with(Style::rounded())
    .with(Modify::new(Segment::all()).with(Alignment::left()));

  Ok(Some(format!("{}", table)))
}
