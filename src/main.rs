mod cmd;

use clap::Parser;
use clap::Subcommand;

#[derive(Debug, Parser)]
struct Command {
  #[clap(subcommand)]
  command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
  /// Create a commit with an AI-generated summary
  #[clap(name = "commit", alias = "c")]
  Commit(cmd::commit::CommitCommand),
  /// Create a GitHub PR with AI title and description
  #[clap(name = "pull-request", alias = "pr")]
  PullRequest(cmd::pull_request::PullRequestCommand),
}

fn main() -> anyhow::Result<()> {
  let command = Command::parse();
  match command.command {
    Commands::Commit(args) => cmd::commit::main(args),
    Commands::PullRequest(args) => cmd::pull_request::main(args),
  }
}
