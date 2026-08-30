use std::process::Command;

use anyhow::Context;

/// Returns the name of the currently checked-out branch.
///
/// Errors when not inside a git repository or when in a detached HEAD state.
pub fn current_branch() -> anyhow::Result<String> {
  let output = Command::new("git")
    .args(["branch", "--show-current"])
    .output()
    .context("failed to run git branch --show-current")?;

  if !output.status.success() {
    anyhow::bail!(
      "git branch --show-current failed (status {})",
      output.status
    );
  }

  let branch = String::from_utf8(output.stdout)
    .context("invalid UTF-8 from git branch --show-current")?
    .trim()
    .to_string();

  if branch.is_empty() {
    anyhow::bail!("not on a branch (detached HEAD)");
  }

  Ok(branch)
}

/// Runs `git diff <base>...HEAD` and returns the combined diff text.
pub fn get_branch_diff(base: &str) -> anyhow::Result<String> {
  let output = Command::new("git")
    .args(["diff", &format!("{base}...HEAD")])
    .output()
    .context("failed to run git diff")?;

  if !output.status.success() {
    anyhow::bail!("git diff failed (status {})", output.status);
  }

  let diff = String::from_utf8(output.stdout).context("invalid UTF-8 from git diff")?;
  Ok(diff)
}

/// Pushes the current branch to `origin` and tracks it as the upstream.
pub fn push_branch(branch: &str) -> anyhow::Result<()> {
  let status = Command::new("git")
    .args(["push", "-u", "origin", branch])
    .status()
    .context("failed to run git push")?;

  if !status.success() {
    anyhow::bail!("git push failed (status {status})");
  }

  Ok(())
}
