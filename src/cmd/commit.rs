use crate::EnvConfig;
use crate::platform::openai_api::OpenAIConversation;
use crate::platform::openai_api::OpenAIConversationOptions;

#[derive(Debug, clap::Parser)]
pub struct CommitCommand {}

pub fn main(
  env: EnvConfig,
  args: CommitCommand,
) -> anyhow::Result<()> {
  dbg!(&env);
  dbg!(&args);

  let options = OpenAIConversationOptions {
    openai_api_url: env.openai_api_url,
    openai_api_token: env.openai_api_token,
    model: env.model_id,
    system_prompt: Some("You generate concise git commit messages.".to_string()),
  };

  let mut conversation = OpenAIConversation::new(options);

  // Turn 1
  let response = conversation.submit("Add feature flag support")?;
  println!("Response: {}", response);

  // Turn 2 (retains history from Turn 1)
  let follow_up = conversation.submit("Make it shorter")?;
  println!("Follow up: {}", follow_up);
  Ok(())
}
