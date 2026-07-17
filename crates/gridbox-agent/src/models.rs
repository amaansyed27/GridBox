use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ChatRequest<'a> {
    pub model: &'a str,
    pub messages: Vec<ApiMessage>,
    pub stream: bool,
    pub options: ChatOptions,
}

#[derive(Debug, Serialize)]
pub struct ChatOptions {
    pub temperature: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub message: ApiMessage,
}

#[derive(Debug, Deserialize)]
pub struct TagsResponse {
    #[serde(default)]
    pub models: Vec<ModelTag>,
}

#[derive(Debug, Deserialize)]
pub struct ModelTag {
    pub name: String,
}
