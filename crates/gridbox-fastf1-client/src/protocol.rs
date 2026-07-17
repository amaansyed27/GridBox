use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize)]
pub struct FastF1Request {
    pub id: String,
    pub method: String,
    pub params: Value,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FastF1Response {
    pub id: String,
    pub ok: bool,
    #[serde(default)]
    pub result: Value,
    pub error: Option<String>,
}
