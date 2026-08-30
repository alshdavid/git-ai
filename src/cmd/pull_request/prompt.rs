use handlebars::Handlebars;
use serde::Serialize;

const SYSTEM_PROMPT: &str = include_str!("./prompt_system.txt");
const USER_PROMPT: &str = include_str!("./prompt_user.hbs");

pub fn system() -> &'static str {
  SYSTEM_PROMPT
}

#[derive(Serialize)]
pub struct UserContext {
  pub diff: String,
}

pub fn user(diff: &str) -> anyhow::Result<String> {
  let mut reg = Handlebars::new();
  reg.register_escape_fn(handlebars::no_escape);

  let context = UserContext {
    diff: diff.to_string(),
  };
  let rendered = reg.render_template(USER_PROMPT, &context)?;

  Ok(rendered)
}
