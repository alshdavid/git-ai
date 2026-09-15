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

/// Resolves a branch name to the best available remote tracking ref.
///
/// Prefers `origin/<base>` when it exists so diffs work even when the local
/// base branch is stale or missing entirely (e.g. fresh clones).
fn resolve_base_ref(base: &str) -> anyhow::Result<String> {
  // Explicit refs (e.g. `origin/main`, `HEAD~3`) are used as-is.
  if base.contains('/') || base.contains("~") || base.contains("@") {
    return Ok(base.to_string());
  }

  let candidates = [
    format!("refs/remotes/origin/{base}"),
    format!("refs/heads/{base}"),
  ];

  for candidate in candidates {
    let status = Command::new("git")
      .args(["rev-parse", "--verify", "--quiet", &candidate])
      .stdout(std::process::Stdio::null())
      .stderr(std::process::Stdio::null())
      .status()
      .context("failed to run git rev-parse")?;

    if status.success() {
      // Prefer `origin/x` for network-fresh comparison, `x` for local branches.
      return Ok(if candidate.starts_with("refs/remotes/") {
        format!("origin/{base}")
      } else {
        base.to_string()
      });
    }
  }

  anyhow::bail!(
    "Could not find base branch '{base}'. Check `git branch -a` and pass \
     an existing branch via --base (e.g. --base main)."
  )
}

/// Runs `git diff <base>...HEAD` and returns the combined diff text.
pub fn get_branch_diff(base: &str) -> anyhow::Result<String> {
  let base_ref = resolve_base_ref(base)?;

  let output = Command::new("git")
    .args(["diff", &format!("{base_ref}...HEAD")])
    .output()
    .context("failed to run git diff")?;

  if !output.status.success() {
    let stderr = String::from_utf8_lossy(&output.stderr);
    anyhow::bail!(
      "git diff {base_ref}...HEAD failed (status {}): {}",
      output.status,
      stderr.trim()
    );
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
