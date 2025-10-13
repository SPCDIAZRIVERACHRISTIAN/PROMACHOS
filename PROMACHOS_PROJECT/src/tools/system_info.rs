use anyhow::Result;
use serde_json::json;
use sysinfo::{System, SystemExt};


use super::{Tool, ToolContext, ToolResult};


pub struct SystemInfo;


impl Tool for SystemInfo {
fn name(&self) -> &'static str { "system_info" }
fn description(&self) -> &'static str { "Return CPU, memory, and OS info." }
fn run(&self, _ctx: &ToolContext, _args: serde_json::Value) -> Result<ToolResult> {
let mut sys = System::new_all();
sys.refresh_all();
let total_mem = sys.total_memory();
let used_mem = sys.used_memory();
let os = System::name().unwrap_or_else(|| "unknown".into());
let kernel = System::kernel_version().unwrap_or_default();
Ok(ToolResult {
name: self.name().into(),
output: json!({
"os": os,
"kernel": kernel,
"memory_used_mb": used_mem / 1024,
"memory_total_mb": total_mem / 1024,
"cpus": sys.cpus().len(),
})
})
}
}
