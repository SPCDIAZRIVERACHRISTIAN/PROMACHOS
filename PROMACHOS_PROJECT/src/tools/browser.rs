use anyhow::{anyhow, Result};
use opener; // opens default system browser
use serde_json::json;


use super::{Tool, ToolContext, ToolResult};


pub struct OpenUrl;


impl Tool for OpenUrl {
fn name(&self) -> &'static str { "open_url" }
fn description(&self) -> &'static str { "Open a URL in the default browser. args: { url: string }" }
fn run(&self, _ctx: &ToolContext, args: serde_json::Value) -> Result<ToolResult> {
let url = args.get("url").and_then(|v| v.as_str()).ok_or_else(|| anyhow!("missing url"))?;
opener::open(url)?;
Ok(ToolResult { name: self.name().into(), output: json!({"ok": true, "url": url}) })
}
}
