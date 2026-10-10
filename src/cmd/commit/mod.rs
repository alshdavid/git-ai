mod prompt;

use crate::EnvConfig;
use crate::platform::agent::openai_api::OpenAIConversation;
use crate::platform::agent::openai_api::OpenAIConversationOptions;
use crate::platform::git;

#[derive(Debug, clap::Parser)]
pub struct CommitCommand {
  /// The max lines per file before they will be truncated
  #[arg(short = 'l', long = "max-lines-per-file", default_value = "400")]
  pub max_lines_per_file: usize,

  #[arg(long = "reasoning-effort")]
  pub reasoning_effort: Option<String>,

  #[arg(long = "dry")]
  pub dry_run: bool,

  #[command(flatten)]
  pub env: EnvConfig,
}

pub fn main(args: CommitCommand) -> anyhow::Result<()> {
  let diffs = crate::platform::git::get_staged_diff(args.max_lines_per_file)?;
  let rendered = prompt::user(&diffs)?;

  println!("{}", rendered);

  let openai_api_url = args.env.openai_api_url;
  let model = args.env.model_id;

  let options = OpenAIConversationOptions {
    openai_api_url,
    openai_api_token: args.env.openai_api_token,
    model,
    system_prompt: Some(prompt::system().to_string()),
    reasoning_effort: args.reasoning_effort,
  };

  let mut conversation = OpenAIConversation::new(options);

  let response = conversation.submit(&rendered)?;
  println!("{}\n", response);

  if args.dry_run {
    println!("Skipping commit");
    return Ok(());
  }

  git::commit(response)?;

  Ok(())
}
