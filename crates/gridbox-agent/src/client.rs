use crate::{
    models::{ApiMessage, ChatOptions, ChatRequest, ChatResponse, TagsResponse},
    prompt::SYSTEM_PROMPT,
};
use gridbox_models::{ChatMessage, ChatRole};
use reqwest::Client;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AgentError {
    #[error("local model request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("local model server returned HTTP {status}: {body}")]
    Http { status: u16, body: String },
    #[error("model '{0}' is not installed in Ollama")]
    ModelMissing(String),
}

#[derive(Debug, Clone)]
pub struct OllamaAgent {
    client: Client,
    base_url: String,
    model: String,
}

impl OllamaAgent {
    pub fn new(base_url: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.into().trim_end_matches('/').to_string(),
            model: model.into(),
        }
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn with_model(&self, model: impl Into<String>) -> Self {
        Self {
            client: self.client.clone(),
            base_url: self.base_url.clone(),
            model: model.into(),
        }
    }

    pub async fn health(&self) -> Result<Vec<String>, AgentError> {
        let response = self
            .client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await?;
        let status = response.status();
        if !status.is_success() {
            return Err(AgentError::Http {
                status: status.as_u16(),
                body: response.text().await.unwrap_or_default(),
            });
        }
        let tags: TagsResponse = response.json().await?;
        Ok(tags.models.into_iter().map(|model| model.name).collect())
    }

    pub async fn chat(
        &self,
        history: &[ChatMessage],
        context: &str,
        user_message: &str,
    ) -> Result<String, AgentError> {
        let installed = self.health().await?;
        if !installed
            .iter()
            .any(|name| model_matches(name, &self.model))
        {
            return Err(AgentError::ModelMissing(self.model.clone()));
        }

        let mut messages = vec![ApiMessage {
            role: "system".to_string(),
            content: format!("{SYSTEM_PROMPT}\n\nCURRENT GRIDBOX CONTEXT\n{context}"),
        }];

        let prior_history = prior_history(history, user_message);
        messages.extend(
            prior_history
                .iter()
                .rev()
                .take(12)
                .rev()
                .filter_map(|message| {
                    let role = match message.role {
                        ChatRole::User => "user",
                        ChatRole::Assistant => "assistant",
                        ChatRole::System | ChatRole::Tool => return None,
                    };
                    Some(ApiMessage {
                        role: role.to_string(),
                        content: message.content.clone(),
                    })
                }),
        );
        messages.push(ApiMessage {
            role: "user".to_string(),
            content: user_message.to_string(),
        });

        let request = ChatRequest {
            model: &self.model,
            messages,
            stream: false,
            options: ChatOptions { temperature: 0.2 },
        };

        let response = self
            .client
            .post(format!("{}/api/chat", self.base_url))
            .json(&request)
            .send()
            .await?;
        let status = response.status();
        if !status.is_success() {
            return Err(AgentError::Http {
                status: status.as_u16(),
                body: response.text().await.unwrap_or_default(),
            });
        }
        let response: ChatResponse = response.json().await?;
        Ok(response.message.content)
    }
}

fn prior_history<'a>(history: &'a [ChatMessage], user_message: &str) -> &'a [ChatMessage] {
    if history.last().is_some_and(|message| {
        message.role == ChatRole::User && message.content == user_message
    }) {
        &history[..history.len() - 1]
    } else {
        history
    }
}

fn model_matches(installed: &str, configured: &str) -> bool {
    installed == configured
        || installed.strip_suffix(":latest") == Some(configured)
        || configured.strip_suffix(":latest") == Some(installed)
}

#[cfg(test)]
mod tests {
    use super::{model_matches, prior_history};
    use gridbox_models::ChatMessage;

    #[test]
    fn matches_latest_alias() {
        assert!(model_matches("qwen3:latest", "qwen3"));
        assert!(model_matches("qwen3", "qwen3:latest"));
    }

    #[test]
    fn removes_current_question_from_prior_history() {
        let history = vec![ChatMessage::assistant("Ready"), ChatMessage::user("Compare pace")];
        let prior = prior_history(&history, "Compare pace");
        assert_eq!(prior.len(), 1);
    }
}
