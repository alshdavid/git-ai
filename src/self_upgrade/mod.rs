use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

use self_update::ReleaseAsset;
use self_update::backends::github;

const BIN_NAME: &str = "git-ai";

#[derive(Debug)]
pub struct UpgradeOptions<'a> {
  pub target_repo: &'a str,
  pub current_version: &'a str,
}

#[derive(Debug)]
pub enum UpgradeOutcome {
  /// The running binary is a pre-release build, which is never replaced
  Prerelease,
  /// The running binary already matches the latest release
  UpToDate,
  /// The latest release was installed, carrying the installed version
  Updated(String),
}

/// How the latest release compares to the running binary
#[derive(Debug)]
pub enum LatestRelease {
  /// The running binary is a pre-release build, which is never replaced
  Prerelease,
  /// The running binary already matches the latest release
  UpToDate,
  /// A newer release is available
  Available(semver::Version),
}

/// Compare the running version against the latest GitHub release, without
/// replacing anything
pub fn check_for_update(options: &UpgradeOptions<'_>) -> anyhow::Result<LatestRelease> {
  if is_prerelease(options.current_version)? {
    return Ok(LatestRelease::Prerelease);
  }

  let releases = configure(options)?.build()?.get_latest_release()?;

  let Some(release) = releases.latest() else {
    return Ok(LatestRelease::UpToDate);
  };

  let current_version = semver::Version::parse(options.current_version)?;
  let remote_version = normalize_tag(release.version())?;

  if current_version < remote_version {
    return Ok(LatestRelease::Available(remote_version));
  }

  Ok(LatestRelease::UpToDate)
}

/// Replace the running binary with the latest release
pub fn try_upgrade(
  options: &UpgradeOptions<'_>,
  force: bool,
) -> anyhow::Result<UpgradeOutcome> {
  if is_prerelease(options.current_version)? {
    return Ok(UpgradeOutcome::Prerelease);
  }

  let mut builder = configure(options)?;

  if force {
    // The update path only installs releases that are strictly newer than the
    // running version, so pin the latest tag to reinstall over it
    let Some(latest) = builder.build()?.get_latest_release()?.latest().cloned() else {
      return Ok(UpgradeOutcome::UpToDate);
    };

    builder.release_tag(latest.version());
  }

  let status = builder.progress_callback(progress()).build()?.update()?;

  match status {
    self_update::VersionStatus::Updated(version) => Ok(UpgradeOutcome::Updated(version)),
    _ => Ok(UpgradeOutcome::UpToDate),
  }
}

fn configure(options: &UpgradeOptions<'_>) -> anyhow::Result<github::UpdateBuilder> {
  let Some((owner, name)) = options.target_repo.split_once('/') else {
    anyhow::bail!("Invalid repo name");
  };

  let mut builder = github::Update::configure();
  builder
    .repo_owner(owner)
    .repo_name(name)
    .bin_name(BIN_NAME)
    .current_version(options.current_version)
    .asset_matcher(select_asset)
    .unattended();

  Ok(builder)
}

fn is_prerelease(version: &str) -> anyhow::Result<bool> {
  Ok(!semver::Version::parse(version)?.pre.is_empty())
}

fn normalize_tag(tag: &str) -> anyhow::Result<semver::Version> {
  let tag = tag.strip_prefix('v').unwrap_or(tag);
  Ok(semver::Version::parse(tag)?)
}

fn select_asset(assets: &[ReleaseAsset]) -> Option<ReleaseAsset> {
  let os = match std::env::consts::OS {
    "windows" => &["win32", "windows", "win"][..],
    "macos" => &["macos", "darwin"][..],
    "linux" => &["linux"][..],
    _ => return None,
  };

  let arch = match std::env::consts::ARCH {
    "x86_64" => &["amd64", "x86_64"][..],
    "aarch64" => &["arm64", "aarch64"][..],
    _ => return None,
  };

  assets
    .iter()
    .find(|asset| {
      let name = asset.name().to_ascii_lowercase();
      name.ends_with(".tar.gz")
        && os.iter().any(|s| name.contains(s))
        && arch.iter().any(|s| name.contains(s))
    })
    .cloned()
}

fn progress() -> impl Fn(u64, Option<u64>) + Send + Sync + 'static {
  let started = AtomicBool::new(false);
  let reported = AtomicU64::new(0);

  move |downloaded, total| {
    if !started.swap(true, Ordering::Relaxed) {
      println!("0%");
    }

    let Some(total) = total.filter(|total| *total > 0) else {
      return;
    };

    let step = (downloaded * 4 / total).min(4);
    if step > reported.load(Ordering::Relaxed) {
      reported.store(step, Ordering::Relaxed);
      println!("{}%", step * 25);
    }
  }
}
