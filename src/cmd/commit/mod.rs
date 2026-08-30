mod prompt;

use crate::EnvConfig;
use crate::platform::git;
use crate::platform::openai_api::OpenAIConversation;
use crate::platform::openai_api::OpenAIConversationOptions;

#[derive(Debug, clap::Parser)]
pub struct CommitCommand {
  /// The max lines per file before they will be truncated
  #[arg(short = 'l', long = "max-lines-per-file", default_value = "400")]
  pub max_lines_per_file: usize,

  #[arg(long = "reasoning-effort")]
  pub reasoning_effort: Option<String>,

  #[arg(long = "dry")]
  pub dry_run: bool,
}

pub fn main(
  env: EnvConfig,
  args: CommitCommand,
) -> anyhow::Result<()> {
  let diffs = crate::platform::git::get_staged_diff(args.max_lines_per_file)?;
  let rendered = prompt::user(&diffs)?;

  println!("{}", rendered);

  let options = OpenAIConversationOptions {
    openai_api_url: env.openai_api_url,
    openai_api_token: env.openai_api_token,
    model: env.model_id,
    system_prompt: Some(prompt::system().to_string()),
    reasoning_effort: args.reasoning_effort,
  };

  let mut conversation = OpenAIConversation::new(options);

  let response = conversation.submit(&rendered)?;
  println!("{}", response);

  if args.dry_run {
    println!("Skipping commit");
    return Ok(());
  }

  git::commit(response)?;

  Ok(())
}
