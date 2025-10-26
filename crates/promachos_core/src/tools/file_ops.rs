use anyhow::{anyhow, Result};
use serde_json::json;
use std::{fs, path::{Path, PathBuf}};
use similar::TextDiff;

use super::{Tool, ToolContext, ToolResult};

/// Adjust this to your repo root (or pass it later via ToolContext)
const BASE_DIR: &str = "."; // current directory by default

fn resolve_safe_path(rel: &str) -> Result<PathBuf> {
    let base = Path::new(BASE_DIR).canonicalize()?;
    let p = base.join(rel).canonicalize()?;
    if !p.starts_with(&base) {
        return Err(anyhow!("path escapes base directory"));
    }
    Ok(p)
}

pub struct ReadFile;
pub struct PlanWrite;

impl Tool for ReadFile {
    fn name(&self) -> &'static str { "read_file" }
    fn description(&self) -> &'static str { "Read a UTF-8 text file. args: { path: string }" }
    fn run(&self, _ctx: &ToolContext, args: serde_json::Value) -> Result<ToolResult> {
        let path = args.get("path").and_then(|v| v.as_str()).ok_or_else(|| anyhow!("missing path"))?;
        let abs = resolve_safe_path(path)?;
        let content = fs::read_to_string(&abs)
            .map_err(|e| anyhow!("read error {}: {e}", abs.display()))?;

        // avoid spamming the model if file is huge
        let (preview, truncated) = if content.len() > 32_000 {
            (content.chars().take(32_000).collect::<String>(), true)
        } else {
            (content, false)
        };

        Ok(ToolResult {
            name: self.name().into(),
            output: json!({
                "path": abs.to_string_lossy(),
                "truncated": truncated,
                "content": preview
            }),
        })
    }
}

impl Tool for PlanWrite {
    fn name(&self) -> &'static str { "plan_write" }
    fn description(&self) -> &'static str {
        "Plan a file change (no write). args: { path: string, content: string } -> returns unified diff"
    }
    fn run(&self, _ctx: &ToolContext, args: serde_json::Value) -> Result<ToolResult> {
        let path = args.get("path").and_then(|v| v.as_str()).ok_or_else(|| anyhow!("missing path"))?;
        let new_content = args.get("content").and_then(|v| v.as_str()).ok_or_else(|| anyhow!("missing content"))?;
        let abs = resolve_safe_path(path)?;

        let old = fs::read_to_string(&abs).unwrap_or_default();
        let new_owned = new_content.to_string();
        let diff = TextDiff::from_lines(&old, &new_owned)
            .unified_diff()
            .context_radius(3)
            .header(path, path)
            .to_string();

        Ok(ToolResult {
            name: self.name().into(),
            output: json!({
                "path": abs.to_string_lossy(),
                "diff": diff,
                "note": "This is a dry run. Review the diff. To actually write, use Phase 2 (:apply workflow)."
            }),
        })
    }
}
