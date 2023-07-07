use cfs_cli::{commands::Commands, context::CFSContext, prompt::CFSPrompt};
use reedline_repl_rs::{Repl, Result};

// [ ] ls
// [✅] !ls
// [ ] cp --from
// [ ] cp --to
// [ ] rm
// [✅] mkcfs
#[tokio::main]
async fn main() -> Result<()> {
  let mut repl = Repl::new(CFSContext::default())
    .with_name("Complex File System")
    .with_version("0.1.0")
    .with_banner("Welcome to the Complex File System CLI")
    .with_prompt("/")
    .with_description("A CLI for the Complex File System")
    .with_on_after_command_async(|context| Box::pin(CFSPrompt::update_prompt(context)));

  for (command, callback) in Commands::all()? {
    repl = repl.with_command_async(command, callback);
  }

  cfs::init_library_logger();
  repl.run_async().await
}
