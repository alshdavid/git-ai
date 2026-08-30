use std::collections::BTreeMap;
use std::path::PathBuf;

use handlebars::Handlebars;
use serde::Serialize;

use crate::platform::git::ChangeStatus;
use crate::platform::git::FileChange;
use crate::platform::git::FileDiff;

const SYSTEM_PROMPT: &str = include_str!("./prompt_system.txt");
const USER_PROMPT: &str = include_str!("./prompt_user.hbs");

pub fn system() -> &'static str {
  SYSTEM_PROMPT
}

pub fn user(diffs: &BTreeMap<PathBuf, FileChange>) -> anyhow::Result<String> {
  let mut reg = Handlebars::new();

  reg.register_escape_fn(handlebars::no_escape);

  let context = prepare_user_context(diffs);
  let rendered = reg.render_template(USER_PROMPT, &context)?;

  Ok(rendered)
}

#[derive(Serialize)]
pub struct FileDiffView {
  /// Path to the changed file
  pub path: String,
  /// The type of file "text", "binary", or "skipped"
  pub kind: String,
  /// The git change status (added, modified, deleted, renamed)
  pub status: String,
  /// The contents of the file
  pub details: Option<String>,
  /// How many lines were changed
  pub diff_size: usize,
  /// Number of added lines (`+`)
  pub additions: usize,
  /// Number of deleted lines (`-`)
  pub deletions: usize,
  /// If the file was truncated
  pub truncated: bool,
  /// If the file was truncated, the reason it was truncated
  pub reason: Option<String>,
}

#[derive(Serialize)]
pub struct PromptContext {
  pub files: Vec<FileDiffView>,
}

pub fn prepare_user_context(diffs: &BTreeMap<PathBuf, FileChange>) -> PromptContext {
  let mut files = Vec::new();

  for change in diffs.values() {
    let path_str = change.path.display().to_string();
    let status_str = match &change.status {
      ChangeStatus::Added => "added".to_string(),
      ChangeStatus::Modified => "modified".to_string(),
      ChangeStatus::Deleted => "deleted".to_string(),
      ChangeStatus::Renamed { from } => {
        format!("renamed from {}", from.display())
      }
    };
    match &change.diff {
      FileDiff::Text {
        details,
        diff_size,
        additions,
        deletions,
        truncated,
      } => {
        files.push(FileDiffView {
          path: path_str,
          kind: "text".to_string(),
          status: status_str.clone(),
          details: Some(details.clone()),
          diff_size: *diff_size,
          additions: *additions,
          deletions: *deletions,
          truncated: *truncated,
          reason: None,
        });
      }
      FileDiff::Binary => {
        files.push(FileDiffView {
          path: path_str,
          kind: "binary".to_string(),
          status: status_str.clone(),
          details: None,
          diff_size: 0,
          additions: 0,
          deletions: 0,
          truncated: false,
          reason: None,
        });
      }
      FileDiff::Skipped { reason } => {
        files.push(FileDiffView {
          path: path_str,
          kind: "skipped".to_string(),
          status: status_str.clone(),
          details: None,
          diff_size: 0,
          additions: 0,
          deletions: 0,
          truncated: false,
          reason: Some(reason.trim().to_string()),
        });
      }
    }
  }

  PromptContext { files }
}
