use std::collections::BTreeMap;
use std::path::PathBuf;

use handlebars::Handlebars;
use serde::Serialize;

use crate::EnvConfig;
use crate::platform::git::FileDiff;
use crate::platform::openai_api::OpenAIConversation;
use crate::platform::openai_api::OpenAIConversationOptions;

static SYSTEM_PROMPT: &str = include_str!("./prompt_system.txt");
const DEFAULT_COMMIT_TEMPLATE: &str = include_str!("./prompt_user.hbs");

#[derive(Debug, clap::Parser)]
pub struct CommitCommand {
  /// The max lines per file before they will be truncated
  #[arg(short = 'l', long = "max-lines-per-file", default_value = "400")]
  pub max_lines_per_file: usize,

  #[arg(long = "reasoning-effort", default_value = "high")]
  pub reasoning_effort: String,
}

pub fn main(
  env: EnvConfig,
  args: CommitCommand,
) -> anyhow::Result<()> {
  // dbg!(&env);
  // dbg!(&args);

  let diffs = crate::platform::git::build_atomic_diff_payload(args.max_lines_per_file)?;

  let mut reg = Handlebars::new();

  // Disables HTML entity escaping (e.g. keeps '<' and '>' raw inside code blocks)
  reg.register_escape_fn(handlebars::no_escape);

  let context = prepare_context(&diffs);
  let rendered = reg.render_template(DEFAULT_COMMIT_TEMPLATE, &context)?;

  // dbg!(&r);
  println!("{}", rendered);

  let options = OpenAIConversationOptions {
    openai_api_url: env.openai_api_url,
    openai_api_token: env.openai_api_token,
    model: env.model_id,
    system_prompt: Some(SYSTEM_PROMPT.to_string()),
    reasoning_effort: Some(args.reasoning_effort),
  };

  let mut conversation = OpenAIConversation::new(options);

  let response = conversation.submit(&rendered)?;
  println!("{}", response);

  Ok(())
}

#[derive(Serialize)]
pub struct FileDiffView {
  /// Path to the changed file
  pub path: String,
  /// The type of file "text", "binary", or "skipped"
  pub kind: String,
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

pub fn prepare_context(diffs: &BTreeMap<PathBuf, FileDiff>) -> PromptContext {
  let mut files = Vec::new();

  for (path, diff) in diffs {
    let path_str = path.display().to_string();
    match diff {
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
          details: None,
          diff_size: 0,
          additions: 0,
          deletions: 0,
          truncated: false,
          reason: Some(reason.clone()),
        });
      }
    }
  }

  PromptContext { files }
}
