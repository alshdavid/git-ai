#[derive(Debug, clap::Parser)]
pub struct CommitCommand {}

pub fn main(args: CommitCommand) -> anyhow::Result<()> {
  dbg!(&args);
  Ok(())
}
