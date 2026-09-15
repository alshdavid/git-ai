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

/// Returns the number of an open pull request for the given branch, if any.
///
/// Queries `gh pr list` for the branch's PR. Returns `None` when no open pull
/// request exists for the branch.
pub fn find_pull_request(branch: &str) -> anyhow::Result<Option<u64>> {
  let output = Command::new("gh")
    .args([
      "pr",
      "list",
      "--head",
      branch,
      "--state",
      "open",
      "--json",
      "number",
      "--jq",
      ".[0].number // empty",
    ])
    .output()
    .context("failed to run gh pr list")?;

  if !output.status.success() {
    let stderr = String::from_utf8_lossy(&output.stderr);
    anyhow::bail!(
      "gh pr list failed (status {}): {}",
      output.status,
      stderr.trim()
    );
  }

  let number = String::from_utf8(output.stdout)
    .context("invalid UTF-8 from gh pr list")?
    .trim()
    .to_string();

  if number.is_empty() {
    return Ok(None);
  }

  let number = number
    .parse::<u64>()
    .context("failed to parse pull request number")?;

  Ok(Some(number))
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

/// Updates an existing pull request's title, body, and base branch.
pub fn edit_pull_request(
  number: u64,
  base: &str,
  title: &str,
  body: &str,
) -> anyhow::Result<()> {
  let status = Command::new("gh")
    .args([
      "pr",
      "edit",
      &number.to_string(),
      "--base",
      base,
      "--title",
      title,
      "--body",
      body,
    ])
    .status()
    .context("failed to run gh pr edit")?;

  if !status.success() {
    anyhow::bail!("gh pr edit failed (status {status})");
  }

  Ok(())
}
