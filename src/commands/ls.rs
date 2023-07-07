use reedline_repl_rs::{clap::ArgMatches, Result as ReplResult};

use crate::context::CFSContext;

pub async fn ls(_args: ArgMatches, _context: &mut CFSContext) -> ReplResult<Option<String>> {
  Err(reedline_repl_rs::Error::UnknownCommand(
    "ls not implemented yet".to_string(),
  ))
}
