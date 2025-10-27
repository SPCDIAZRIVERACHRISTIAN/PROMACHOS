use std::sync::Arc;
use tokio::sync::Mutex;
use promachos_core::{Agent, Config, Ollama, schema::ChatMessage};

pub struct AgentState {
    pub agent: Agent,
    pub history: Mutex<Vec<ChatMessage>>,
}

impl AgentState {
    pub fn new(agent: Agent) -> Self {
        Self { agent, history: Mutex::new(vec![]) }
    }
}

#[tauri::command]
pub async fn chat(state: tauri::State<'_, Arc<AgentState>>, text: String) -> Result<String, String> {
    let mut hist = state.history.lock().await;
    state.agent.step(&mut hist, &text).await.map_err(|e| e.to_string())
}

