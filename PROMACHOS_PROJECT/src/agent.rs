use crate::ollama::Ollama;
use crate::schema::{ChatMessage, OllamaChatRequest, ToolCall};
use crate::tools::{call, ToolContext};
use anyhow::Result;
use regex::Regex;


/// Minimal tool-call protocol:
/// - Normal chat uses free-form text
/// - If the model decides to call a tool, it must reply **JSON only** like:
/// {"tool":"system_info","args":{}}
/// We detect JSON with a quick heuristic and execute the tool.


pub struct Agent {
pub ollama: Ollama,
pub model: String,
pub sys_prompt: String,
}


impl Agent {
pub fn new(ollama: Ollama, model: String) -> Self {
let sys_prompt = r#"
You are a local system assistant. If you need system information or to open a URL,
respond **only** with JSON using this schema: {"tool": string, "args": object}.
Available tools:
- system_info: Returns CPU/memory/OS.
- open_url: args { url: string }
When no tool is needed, answer normally.
"#.to_string();
Self { ollama, model, sys_prompt }
}


pub async fn step(&self, history: &mut Vec<ChatMessage>, user_input: &str) -> Result<String> {
history.push(ChatMessage{ role: "user".into(), content: user_input.into() });
let req = OllamaChatRequest {
model: self.model.clone(),
messages: {
let mut m = vec![ChatMessage{ role: "system".into(), content: self.sys_prompt.clone() }];
m.extend(history.clone());
m
},
stream: None,
format: None, // allow free-form unless we detect a tool loop
};
let res = self.ollama.chat(&req).await?;
let mut reply = res.message.content.clone();


// Try to detect a tool JSON reply (starts with '{' and ends with '}')
let looks_like_json = reply.trim_start().starts_with('{') && reply.trim_end().ends_with('}');
if looks_like_json {
// Try to parse ToolCall
if let Ok(tc) = serde_json::from_str::<ToolCall>(&reply) {
let ctx = ToolContext;
let out = call(&tc.tool, &ctx, tc.args)?;


// Feed the tool result back to the model to produce a natural answer
let tool_msg = format!("Tool `{}` result: {}", out.name, serde_json::to_string_pretty(&out.output)?);
history.push(ChatMessage{ role: "assistant".into(), content: reply });
history.push(ChatMessage{ role: "user".into(), content: tool_msg });


let req2 = OllamaChatRequest {
model: self.model.clone(),
messages: {
let mut m = vec![ChatMessage{ role: "system".into(), content: self.sys_prompt.clone() }];
m.extend(history.clone());
m
},
stream: None,
format: None,
}
