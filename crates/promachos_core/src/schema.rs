use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
pub role: String, // "system" | "user" | "assistant"
pub content: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaChatRequest {
pub model: String,
pub messages: Vec<ChatMessage>,
#[serde(skip_serializing_if = "Option::is_none")]
pub stream: Option<bool>,
#[serde(skip_serializing_if = "Option::is_none")]
pub format: Option<String>, // set to "json" when we demand tool JSON
}


#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaChatResponse {
pub message: ChatMessage,
}


/// Convention for tool calls from the model when format == json
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolCall {
pub tool: String,
#[serde(default)]
pub args: serde_json::Value,
}
