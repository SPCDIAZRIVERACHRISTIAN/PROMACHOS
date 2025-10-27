// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod agent_bridge;

use std::sync::Arc;

#[tokio::main]
async fn main() {
    let cfg = promachos_core::Config::from_env().expect("env");
    let ollama = promachos_core::Ollama::new(cfg.ollama_host.clone());
    let agent = promachos_core::Agent::new(ollama, cfg.model.clone());
    let state = Arc::new(agent_bridge::AgentState::new(agent));

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![agent_bridge::chat])
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}

