use std::{collections::HashMap, path::PathBuf, process::Command};

pub fn diff_optimized(max_lines_per_file: usize) -> Result<String, std::io::Error> {
  // 1. Get high-level summary
  let stat_output = Command::new("git")
    .args(["diff", "--staged", "--stat"])
    .output()?;
  let stat_str = String::from_utf8_lossy(&stat_output.stdout);

  if stat_str.trim().is_empty() {
    return Ok(String::new());
  }

  // 2. Get list of staged files
  let files_output = Command::new("git")
    .args(["diff", "--staged", "--name-only"])
    .output()?;
  let files_str = String::from_utf8_lossy(&files_output.stdout);
  let files: Vec<&str> = files_str
    .lines()
    .map(|f| f.trim())
    .filter(|f| !f.is_empty())
    .collect();

  // 3. Build per-file truncated diffs
  let mut file_diffs = Vec::new();

  for file in files {
    // Skip lockfiles/binary noise if desired
    if file.contains("lock") || file.ends_with(".png") || file.ends_with(".wasm") {
      file_diffs.push(format!(
        "File: {}\n[Diff skipped: Large/Binary/Lockfile]",
        file
      ));
      continue;
    }

    let diff_output = Command::new("git")
      .args(["diff", "--staged", "--", file])
      .output()?;
    let diff_str = String::from_utf8_lossy(&diff_output.stdout);

    let lines: Vec<&str> = diff_str.lines().collect();
    if lines.len() <= max_lines_per_file {
      file_diffs.push(diff_str.to_string());
    } else {
      let truncated: String = lines
        .into_iter()
        .take(max_lines_per_file)
        .collect::<Vec<&str>>()
        .join("\n");
      file_diffs.push(format!(
        "{}\n... [truncated remaining lines for {}]",
        truncated, file
      ));
    }
  }

  // 4. Combine into final prompt payload
  let payload = format!(
    "Summary of Staged Changes:\n{}\n\nDetailed Per-File Diffs:\n{}",
    stat_str.trim(),
    file_diffs.join("\n\n---\n\n")
  );

  Ok(payload)
}

use git2::{DiffOptions, Patch, Repository};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileDiff {
  Text {
    /// The actual diff snippet (None if skipped or empty)
    details: String,
    /// Total lines in the diff before truncation
    diff_size: usize,
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
) -> Result<HashMap<PathBuf, FileDiff>, git2::Error> {
  let mut file_diffs = HashMap::new();

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
  for i in 0..diff.deltas().len() {
    let delta = diff.deltas().nth(i).unwrap();

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

    // Check 2: Skip lockfiles
    if path_str.contains("lock") {
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
          diff_text.push_str(std::str::from_utf8(hunk.header()).unwrap_or(""));

          for l in 0..num_lines {
            let line = patch.line_in_hunk(h, l)?;
            let origin = line.origin();

            // Only capture line additions, deletions, or context lines
            if origin == '+' || origin == '-' || origin == ' ' {
              if (diff_text.lines().count() - 1) >= max_lines_per_file {
                truncated = true;
                continue 'hunks;
              }

              let content = std::str::from_utf8(line.content()).unwrap_or("");
              diff_text.push_str(&format!("{}{}", origin, content));
            }
          }
        }
      }

      let details = diff_text;

      file_diffs.insert(
        path_buf,
        FileDiff::Text {
          details,
          diff_size: total_lines,
          truncated,
        },
      );
    }
  }

  Ok(file_diffs)
}
