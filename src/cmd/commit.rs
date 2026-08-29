#[derive(Debug, clap::Parser)]
pub struct CommitCommand {}

pub fn main(_args: CommitCommand) -> anyhow::Result<()> {
  Ok(())
}
