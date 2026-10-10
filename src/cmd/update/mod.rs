use crate::platform::self_upgrade::LatestRelease;
use crate::platform::self_upgrade::UpgradeOptions;
use crate::platform::self_upgrade::UpgradeOutcome;
use crate::platform::self_upgrade::check_for_update;
use crate::platform::self_upgrade::try_upgrade;

const TARGET_REPO: &str = "alshdavid/git-ai";
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, clap::Parser)]
pub struct UpdateCommand {
  /// Install the latest version even when already up to date
  #[arg(long = "force")]
  pub force: bool,

  /// Only check whether an update is available, without installing it
  #[arg(long = "check")]
  pub check: bool,
}

pub fn main(args: UpdateCommand) -> anyhow::Result<()> {
  let options = UpgradeOptions {
    target_repo: TARGET_REPO,
    current_version: VERSION,
  };

  if args.check {
    match check_for_update(&options)? {
      LatestRelease::Prerelease => println!("{}", prerelease()),
      LatestRelease::UpToDate => println!("{}", up_to_date()),
      LatestRelease::Available(version) => {
        println!("Update available: {} -> {}", VERSION, version)
      }
    }

    return Ok(());
  }

  match try_upgrade(&options, args.force)? {
    UpgradeOutcome::Prerelease => println!("{}", prerelease()),
    UpgradeOutcome::UpToDate => println!("{}", up_to_date()),
    UpgradeOutcome::Updated(version) => println!("Updated git-ai to {version}"),
  }

  Ok(())
}

fn prerelease() -> String {
  format!("Skipping update, running a pre-release build ({VERSION})")
}

fn up_to_date() -> String {
  format!("Already on the latest version ({VERSION})")
}
