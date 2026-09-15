use std::process::Command;

use anyhow::Context;

/// Returns the repository's default branch as reported by GitHub.
///
/// Queries the API via `gh repo view`, so it reflects the remote's current
/// default (e.g. `main` or `master`) even when local refs are stale or absent.
pub fn default_branch() -> anyhow::Result<String> {
  let output = Command::new("gh")
    .args([
      "repo",
      "view",
      "--json",
      "defaultBranchRef",
      "--jq",
      ".defaultBranchRef.name",
    ])
    .output()
    .context("failed to run gh repo view")?;

  if !output.status.success() {
    let stderr = String::from_utf8_lossy(&output.stderr);
    anyhow::bail!(
      "gh repo view failed (status {}): {}",
      output.status,
      stderr.trim()
    );
  }

  let branch = String::from_utf8(output.stdout)
    .context("invalid UTF-8 from gh repo view")?
    .trim()
    .to_string();

  if branch.is_empty() {
    anyhow::bail!("gh repo view returned an empty default branch");
  }

  Ok(branch)
}

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
