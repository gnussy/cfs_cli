use reedline_repl_rs::{clap::ArgMatches, Result as ReplResult};

use crate::context::CFSContext;

pub async fn get(_args: ArgMatches, _context: &mut CFSContext) -> ReplResult<Option<String>> {
  Err(reedline_repl_rs::Error::UnknownCommand(
    "get not implemented yet".to_string(),
  ))
}
