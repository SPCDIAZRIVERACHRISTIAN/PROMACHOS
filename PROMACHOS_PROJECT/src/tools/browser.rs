use anyhow::{anyhow, Result};
use serde_json::json;
use std::process::Command;
//use std::env;

use super::{Tool, ToolContext, ToolResult};

pub struct OpenUrl;

impl Tool for OpenUrl {
    fn name(&self) -> &'static str { "open_url" }
    fn description(&self) -> &'static str {
        "Open a URL in the default browser. args: { url: string }"
    }

    fn run(&self, _ctx: &ToolContext, args: serde_json::Value) -> Result<ToolResult> {
        let url = args
            .get("url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("missing url"))?;

        // Detect WSL or Linux
        let uname = Command::new("uname").arg("-a").output();
        let is_wsl = uname.as_ref()
            .map(|o| String::from_utf8_lossy(&o.stdout).contains("WSL"))
            .unwrap_or(false);

        let result = if cfg!(target_os = "windows") {
            Command::new("cmd").args(["/C", "start", url]).spawn()
        } else if cfg!(target_os = "macos") {
            Command::new("open").arg(url).spawn()
        } else if is_wsl {
            Command::new("wslview").arg(url).spawn()
        } else {
            // Assume Linux / BSD
            Command::new("xdg-open").arg(url).spawn()
        };

        match result {
            Ok(_) => Ok(ToolResult {
                name: self.name().into(),
                output: json!({ "ok": true, "url": url }),
            }),
            Err(e) => Err(anyhow!("failed to open browser: {e}")),
        }
    }
}
