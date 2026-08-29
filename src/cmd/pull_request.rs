use crate::EnvConfig;

#[derive(Debug, clap::Parser)]
pub struct PullRequestCommand {}

pub fn main(
  env: EnvConfig,
  args: PullRequestCommand,
) -> anyhow::Result<()> {
  dbg!(&env);
  dbg!(&args);
  Ok(())
}
