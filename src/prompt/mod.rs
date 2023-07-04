use crate::context::CFSContext;
use reedline_repl_rs::Result as ReplResult;

pub struct CFSPrompt;

impl CFSPrompt {
  pub async fn update_prompt(context: &mut CFSContext) -> ReplResult<Option<String>> {
    // let pwd = context.pwd();
    let pwd = "/";

    let prompt = String::from(pwd);

    Ok(Some(prompt))
  }
}
