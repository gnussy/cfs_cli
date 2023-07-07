use std::path::Path;

use owo_colors::OwoColorize;
use reedline_repl_rs::{clap::ArgMatches, Result as ReplResult};
use tabled::{
  settings::{locator::ByColumnName, object::Segment, Alignment, Disable, Modify, Style},
  Table, Tabled,
};

use crate::context::CFSContext;

#[derive(Tabled, PartialEq, Eq, PartialOrd, Ord)]
struct Entry {
  sort_key: u8, // not visible in the table
  #[tabled(rename = "File Type")]
  file_type: String,
  #[tabled(rename = "File Name")]
  name: String,
  #[tabled(rename = "File Size")]
  size: String,
}

pub async fn ls(args: ArgMatches, _context: &mut CFSContext) -> ReplResult<Option<String>> {
  let fallback = String::from(".");
  let path = args.get_one::<String>("path").unwrap_or(&fallback);
  let path = Path::new(&path);

  let output = print_dir_contents(path).await.unwrap();

  Ok(Some(output))
}

async fn print_dir_contents(path: &Path) -> std::io::Result<String> {
  let mut entries = Vec::new();

  if path.is_dir() {
    let mut dir = tokio::fs::read_dir(path).await?;

    while let Some(entry_result) = dir.next_entry().await? {
      let path = entry_result.path();

      let metadata = tokio::fs::metadata(&path).await?;
      let file_type = metadata.file_type();
      let file_size = metadata.len();
      let file_name = path.file_name().unwrap().to_str().unwrap().to_string();

      let is_hidden = file_name.starts_with('.');

      let (sort_key, file_type_str) = if file_type.is_dir() {
        (2, "\u{1F4C1}".to_string()) // Folder icon
      } else if file_type.is_symlink() {
        (1, "\u{1F517}".blue().to_string()) // Link icon
      } else if is_hidden {
        (0, "\u{F719}".white().to_string()) // Hidden file icon
      } else {
        (1, "\u{F719}".white().to_string()) // File icon
      };

      let file_size_str = if file_type.is_file() {
        format!("{}", file_size)
      } else {
        "".to_string()
      };

      entries.push(Entry {
        sort_key,
        file_type: file_type_str,
        name: file_name,
        size: file_size_str,
      });
    }
  }

  entries.sort();

  let mut table = Table::new(&entries);
  table
    .with(Style::rounded())
    .with(Disable::column(ByColumnName::new("sort_key")))
    .with(Modify::new(Segment::all()).with(Alignment::left()));

  Ok(format!("{}", table))
}
