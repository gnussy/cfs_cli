use reedline_repl_rs::{clap::ArgMatches, Result as ReplResult};

use crate::context::CFSContext;

pub async fn put(_args: ArgMatches, _context: &mut CFSContext) -> ReplResult<Option<String>> {
  Err(reedline_repl_rs::Error::UnknownCommand(
    "put not implemented yet".to_string(),
  ))
}
