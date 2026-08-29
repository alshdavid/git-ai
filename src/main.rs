mod cmd;
mod platform;

use clap::Parser;
use clap::Subcommand;

#[derive(Debug, Parser)]
struct Command {
  #[clap(subcommand)]
  command: Commands,

  #[command(flatten)]
  pub env: EnvConfig,
}

#[derive(Debug, clap::Args)]
struct EnvConfig {
  /// Base URL for OpenAI-compatible API endpoints
  #[arg(
    long = "openai-api-url",
    env = "GIT_AI_OPENAI_API_URL",
    env = "OPENAI_BASE_URL",
    env = "OPENAI_API_BASE"
  )]
  pub openai_api_url: String,

  /// API token for OpenAI or compatible provider
  #[arg(
    long = "openai-api-token",
    env = "GIT_AI_OPENAI_API_KEY",
    env = "OPENAI_API_KEY"
  )]
  pub openai_api_token: Option<String>,

  /// Model ID to use (e.g., gpt-4o, deepseek-v4-flash)
  #[arg(long = "model", env = "GIT_AI_MODEL_ID")]
  pub model_id: String,

  /// GitHub Personal Access Token
  #[arg(long = "gh-token", env = "GH_TOKEN")]
  pub gh_token: Option<String>,
}

#[derive(Debug, Subcommand)]
enum Commands {
  /// Create a commit with an automatically generated summary
  #[clap(name = "commit", alias = "c")]
  Commit(cmd::commit::CommitCommand),
  /// Create a GitHub PR with any automatically generated title and description
  #[clap(name = "pull-request", alias = "pr")]
  PullRequest(cmd::pull_request::PullRequestCommand),
}

fn main() -> anyhow::Result<()> {
  let command = Command::parse();
  match command.command {
    Commands::Commit(args) => cmd::commit::main(command.env, args),
    Commands::PullRequest(args) => cmd::pull_request::main(command.env, args),
  }
}
