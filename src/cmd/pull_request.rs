#[derive(Debug, clap::Parser)]
pub struct PullRequestCommand {}

pub fn main(args: PullRequestCommand) -> anyhow::Result<()> {
  dbg!(&args);
  Ok(())
}
