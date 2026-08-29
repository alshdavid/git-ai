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
  Commit(cmd::commit::CommitCommand),
  PullRequest(cmd::pull_request::PullRequestCommand),
}

fn main() -> anyhow::Result<()> {
  let command = Command::parse();
  match command.command {
    Commands::Commit(args) => cmd::commit::main(args),
    Commands::PullRequest(args) => cmd::pull_request::main(args),
  }
}
