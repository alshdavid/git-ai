mod prompt;

use crate::EnvConfig;
use crate::platform::gh;
use crate::platform::git;
use crate::platform::openai_api::OpenAIConversation;
use crate::platform::openai_api::OpenAIConversationOptions;

#[derive(Debug, clap::Parser)]
pub struct PullRequestCommand {
  /// The branch to open the pull request against
  #[arg(long = "base", default_value = "master")]
  pub base: String,

  #[arg(long = "reasoning-effort", default_value = "high")]
  pub reasoning_effort: String,

  #[arg(long = "dry")]
  pub dry_run: bool,
}

struct GeneratedPr {
  title: String,
  body: String,
}

/// Parses the model's `TITLE:`/`BODY:` output into a title and body.
/// Falls back to a conventional title and to the raw response when parsing fails.
fn parse_generated(
  response: &str,
  fallback_branch: &str,
) -> GeneratedPr {
  let title = response
    .lines()
    .map(str::trim)
    .find(|line| line.to_lowercase().starts_with("title:"))
    .map(|line| line[6..].trim().to_string())
    .filter(|title| !title.is_empty())
    .unwrap_or_else(|| format!("feat: updates from {fallback_branch}"));

  let mut body_lines = Vec::new();
  let mut in_body = false;
  for line in response.lines() {
    let trimmed = line.trim();
    if in_body {
      body_lines.push(trimmed.to_string());
    } else if trimmed.to_lowercase().starts_with("body:") {
      in_body = true;
    }
  }

  let body = if body_lines.is_empty() {
    response.trim().to_string()
  } else {
    body_lines.join("\n").trim().to_string()
  };

  GeneratedPr { title, body }
}

pub fn main(
  env: EnvConfig,
  args: PullRequestCommand,
) -> anyhow::Result<()> {
  // 1. Ensure inside a git repo and determine the current branch
  let current_branch = git::current_branch()?;

  // 2. Refuse to open a PR from the target base branch
  if current_branch == args.base {
    anyhow::bail!(
      "You are currently on '{base}'. Switch to a feature branch first.",
      base = args.base
    );
  }

  // 3. Get diff against the target branch
  let diff = git::get_branch_diff(&args.base)?;
  if diff.trim().is_empty() {
    anyhow::bail!("No diff found between HEAD and '{}'.", args.base);
  }

  // 4. Generate the PR title and description using the model
  let rendered = prompt::user(&diff)?;

  let options = OpenAIConversationOptions {
    openai_api_url: env.openai_api_url,
    openai_api_token: env.openai_api_token,
    model: env.model_id,
    system_prompt: Some(prompt::system().to_string()),
    reasoning_effort: Some(args.reasoning_effort),
  };

  let mut conversation = OpenAIConversation::new(options);
  let response = conversation.submit(&rendered)?;

  let generated = parse_generated(response, &current_branch);

  println!("----------------------------------------");
  println!("Target Branch: {}", args.base);
  println!("Generated Title: {}", generated.title);
  println!("Generated Body:");
  println!("{}", generated.body);
  println!("----------------------------------------");

  if args.dry_run {
    println!("Skipping push and pull request creation");
    return Ok(());
  }

  // 5. Push the branch and create the PR via the GitHub CLI
  println!("Pushing branch '{}' to remote...", current_branch);
  git::push_branch(&current_branch)?;

  println!("Creating Pull Request...");
  gh::create_pull_request(&args.base, &generated.title, &generated.body)?;

  println!("PR created successfully!");
  Ok(())
}
