use anyhow::Result;
use std::env;


#[derive(Clone, Debug)]
pub struct Config {
pub ollama_host: String,
pub model: String,
}


impl Config {
pub fn from_env() -> Result<Self> {
dotenvy::dotenv().ok();
let ollama_host = env::var("OLLAMA_HOST").unwrap_or_else(|_| "http://127.0.0.1:11434".into());
let model = env::var("OLLAMA_MODEL").unwrap_or_else(|_| "llama3.1:8b-instruct".into());
Ok(Self { ollama_host, model })
}
}
