use crate::EnvConfig;

#[derive(Debug, clap::Parser)]
pub struct UpdateCommand {
  /// Install the latest version even when already up to date
  #[arg(long = "force")]
  pub force: bool,

  /// Only check whether an update is available, without installing it
  #[arg(long = "check")]
  pub check: bool,
}

pub fn main(
  env: EnvConfig,
  args: UpdateCommand,
) -> anyhow::Result<()> {
  Ok(())
}