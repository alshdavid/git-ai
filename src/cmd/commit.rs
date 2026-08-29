use crate::EnvConfig;

#[derive(Debug, clap::Parser)]
pub struct CommitCommand {}

pub fn main(
  env: EnvConfig,
  args: CommitCommand,
) -> anyhow::Result<()> {
  dbg!(&env);
  dbg!(&args);
  Ok(())
}
