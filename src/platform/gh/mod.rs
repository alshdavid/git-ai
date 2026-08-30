use std::process::Command;

use anyhow::Context;

/// Creates a pull request with the GitHub CLI (`gh`).
pub fn create_pull_request(
  base: &str,
  title: &str,
  body: &str,
) -> anyhow::Result<()> {
  let status = Command::new("gh")
    .args([
      "pr", "create", "--base", base, "--title", title, "--body", body,
    ])
    .status()
    .context("failed to run gh pr create")?;

  if !status.success() {
    anyhow::bail!("gh pr create failed (status {status})");
  }

  Ok(())
}
