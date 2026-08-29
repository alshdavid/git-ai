use crate::EnvConfig;
// use crate::platform::openai_api::OpenAIConversation;
// use crate::platform::openai_api::OpenAIConversationOptions;

static SYSTEM_PROMPT: &str = "Generate a one-line Conventional Commit message based on this diff. Output ONLY the commit message text, no markdown, no quotes.";

#[derive(Debug, clap::Parser)]
pub struct CommitCommand {}

pub fn main(
  _env: EnvConfig,
  _args: CommitCommand,
) -> anyhow::Result<()> {
  // dbg!(&env);
  // dbg!(&args);

  let r = crate::platform::git::build_atomic_diff_payload(200)?;

  dbg!(&r);
  // println!("{}", r);

  // let options = OpenAIConversationOptions {
  //   openai_api_url: env.openai_api_url,
  //   openai_api_token: env.openai_api_token,
  //   model: env.model_id,
  //   system_prompt: Some(SYSTEM_PROMPT.to_string()),
  // };

  // let mut conversation = OpenAIConversation::new(options);

  // // Turn 1
  // let response = conversation.submit("Add feature flag support")?;
  // println!("Response: {}", response);

  // // Turn 2 (retains history from Turn 1)
  // let follow_up = conversation.submit("Make it shorter")?;
  // println!("Follow up: {}", follow_up);
  Ok(())
}
