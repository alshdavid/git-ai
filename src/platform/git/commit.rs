use std::process::Command;

use anyhow::Context;

pub fn commit(message: &str) -> anyhow::Result<()> {
  let status = Command::new("git")
    .arg("commit")
    .arg("-m")
    .arg(message)
    .status()
    .context("failed to run git commit")?;

  if !status.success() {
    anyhow::bail!("git commit failed (status {status})");
  }

  Ok(())
}
