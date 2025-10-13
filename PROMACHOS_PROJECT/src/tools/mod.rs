use anyhow::Result;
use serde_json::json;


pub mod system_info;
pub mod browser;


#[derive(Clone)]
pub struct ToolContext;


#[derive(Debug, Clone)]
pub struct ToolResult {
pub name: String,
pub output: serde_json::Value,
}


pub trait Tool: Send + Sync {
fn name(&self) -> &'static str;
fn description(&self) -> &'static str;
fn run(&self, ctx: &ToolContext, args: serde_json::Value) -> Result<ToolResult>;
}


pub fn registry() -> Vec<Box<dyn Tool>> {
vec![
Box::new(system_info::SystemInfo),
Box::new(browser::OpenUrl),
]
}


pub fn call(name: &str, ctx: &ToolContext, args: serde_json::Value) -> Result<ToolResult> {
for t in registry() {
if t.name() == name { return t.run(ctx, args); }
}
Ok(ToolResult { name: name.to_string(), output: json!({"error": "unknown tool"}) })
}
