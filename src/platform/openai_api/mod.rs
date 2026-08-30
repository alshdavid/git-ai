use reqwest::blocking::Client;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
  System,
  User,
  Assistant,
  Developer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
  pub role: Role,
  pub content: String,
}

#[derive(Debug, Clone)]
pub struct OpenAIConversationOptions {
  pub openai_api_url: String,
  pub openai_api_token: Option<String>,
  pub model: String,
  pub system_prompt: Option<String>,
  pub reasoning_effort: Option<String>,
}

#[derive(Serialize)]
struct ChatCompletionRequest<'a> {
  model: &'a str,
  messages: &'a [ChatMessage],
  #[serde(skip_serializing_if = "Option::is_none")]
  reasoning_effort: Option<&'a str>,
}

// Response payload structure
#[derive(Deserialize)]
struct ChatCompletionChoice {
  message: ChatMessage,
}

#[derive(Deserialize)]
struct ChatCompletionResponse {
  choices: Vec<ChatCompletionChoice>,
}

pub struct OpenAIConversation {
  options: OpenAIConversationOptions,
  client: Client,
  history: Vec<ChatMessage>,
}

impl OpenAIConversation {
  pub fn new(options: OpenAIConversationOptions) -> Self {
    let mut history = Vec::new();
    if let Some(sys) = &options.system_prompt {
      history.push(ChatMessage {
        role: Role::System,
        content: sys.clone(),
      });
    }

    Self {
      options,
      client: Client::new(),
      history,
    }
  }

  pub fn submit(
    &mut self,
    message: &str,
  ) -> anyhow::Result<&str> {
    self.history.push(ChatMessage {
      role: Role::User,
      content: message.to_string(),
    });

    let payload = ChatCompletionRequest {
      model: &self.options.model,
      messages: &self.history,
      reasoning_effort: self.options.reasoning_effort.as_deref(),
    };

    let endpoint = if self.options.openai_api_url.ends_with("/chat/completions") {
      self.options.openai_api_url.clone()
    } else {
      format!(
        "{}/chat/completions",
        self.options.openai_api_url.trim_end_matches('/')
      )
    };

    let mut req = self.client.post(&endpoint).json(&payload);

    if let Some(token) = &self.options.openai_api_token {
      req = req.bearer_auth(token);
    }

    let response = req.send()?;

    if !response.status().is_success() {
      let status = response.status();
      let err_body = response.text().unwrap_or_default();
      anyhow::bail!("API request failed (status {}): {}", status, err_body);
    }

    let parsed: ChatCompletionResponse = response.json()?;

    let assistant_msg = parsed
      .choices
      .into_iter()
      .next()
      .map(|c| c.message)
      .ok_or_else(|| anyhow::anyhow!("Received empty choices array from API"))?;

    self.history.push(assistant_msg);

    Ok(&self.history.last().unwrap().content)
  }

  pub fn history(&self) -> &[ChatMessage] {
    &self.history
  }
}
