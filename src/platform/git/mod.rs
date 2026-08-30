use std::collections::BTreeMap;
use std::path::PathBuf;

use git2::DiffOptions;
use git2::Patch;
use git2::Repository;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileDiff {
  Text {
    /// The actual diff snippet
    details: String,
    /// Total lines in the diff before truncation
    diff_size: usize,
    /// Number of added lines (`+`) across the whole diff
    additions: usize,
    /// Number of deleted lines (`-`) across the whole diff
    deletions: usize,
    /// Whether `details` was capped at `max_lines_per_file`
    truncated: bool,
  },
  Binary,
  Skipped {
    reason: String,
  },
}

pub fn build_atomic_diff_payload(
  max_lines_per_file: usize
) -> Result<BTreeMap<PathBuf, FileDiff>, git2::Error> {
  let mut file_diffs = BTreeMap::new();

  // 1. Open current repo and retrieve index
  let repo = Repository::open_from_env()?;
  let index = repo.index()?;

  // 2. Resolve HEAD tree to diff staged changes against current HEAD
  let head_tree = match repo.head() {
    Ok(head) => Some(head.peel_to_tree()?),
    Err(_) => None, // Initial commit scenario (no HEAD commit yet)
  };

  // 3. Generate staged diff
  let mut opts = DiffOptions::new();
  let diff = repo.diff_tree_to_index(head_tree.as_ref(), Some(&index), Some(&mut opts))?;

  // 4. Iterate over each modified delta
  for (i, delta) in diff.deltas().enumerate() {
    let path_buf = delta
      .new_file()
      .path()
      .or_else(|| delta.old_file().path())
      .map(PathBuf::from)
      .unwrap_or_else(|| PathBuf::from("unknown"));

    let path_str = path_buf.to_string_lossy();

    // Check 1: Binary files
    if delta.flags().is_binary() {
      file_diffs.insert(path_buf, FileDiff::Binary);
      continue;
    }

    if matches!(
      path_str.as_ref(),
      "Cargo.lock"
        | "yarn.lock"
        | "package-lock.json"
        | "npm-shrinkwrap.json"
        | "pnpm-lock.yaml"
        | "Gemfile.lock"
        | "poetry.lock"
        | "Pipfile.lock"
        | "composer.lock"
        | "go.sum"
        | "bun.lockb"
        | "bun.lock"
        | "uv.lock"
    ) {
      file_diffs.insert(
        path_buf,
        FileDiff::Skipped {
          reason: "Lockfile ignored".to_string(),
        },
      );
      continue;
    }

    // Parse patch hunks
    let patch = Patch::from_diff(&diff, i)?;
    if let Some(patch) = patch {
      let mut diff_text = String::new();
      let mut total_lines = 0;
      let mut captured_lines = 0;
      let mut additions = 0;
      let mut deletions = 0;
      let mut truncated = false;

      let old_file = patch
        .delta()
        .old_file()
        .path()
        .and_then(|p| p.to_str())
        .unwrap_or("dev/null");
      let new_file = patch
        .delta()
        .new_file()
        .path()
        .and_then(|p| p.to_str())
        .unwrap_or("dev/null");

      diff_text.push_str(&format!("--- a/{}\n+++ b/{}\n", old_file, new_file));

      'hunks: for h in 0..patch.num_hunks() {
        let (hunk, num_lines) = patch.hunk(h)?;

        // Track total line count (excluding headers)
        total_lines += num_lines;

        if !truncated {
          // Trim trailing space on hunk header for cleaner diff output
          let header = std::str::from_utf8(hunk.header()).unwrap_or("").trim_end();
          diff_text.push_str(header);
          diff_text.push('\n');

          for l in 0..num_lines {
            let line = patch.line_in_hunk(h, l)?;
            let origin = line.origin();

            if origin == '+' {
              additions += 1;
            } else if origin == '-' {
              deletions += 1;
            }

            // Only capture line additions, deletions, or context lines
            if origin == '+' || origin == '-' || origin == ' ' {
              if captured_lines > max_lines_per_file {
                truncated = true;
                continue 'hunks;
              }

              let content = std::str::from_utf8(line.content()).unwrap_or("");

              // Trim CRLF (`\r\n`) or trailing `\n` to prevent double-newlines
              let trimmed_content = content.trim_end_matches(['\r', '\n']);

              diff_text.push(origin);
              diff_text.push_str(trimmed_content);
              diff_text.push('\n');

              captured_lines += 1;
            }
          }
        }
      }

      // Trim leading/trailing whitespace around the complete file diff payload
      let details = diff_text.trim().to_string();

      file_diffs.insert(
        path_buf,
        FileDiff::Text {
          details,
          diff_size: total_lines,
          additions,
          deletions,
          truncated,
        },
      );
    }
  }

  Ok(file_diffs)
}
