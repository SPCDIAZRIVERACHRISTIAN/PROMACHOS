mod config;
mod ollama;
mod schema;
mod tools;
mod agent;


use anyhow::Result;
use clap::Parser;
use config::Config;
use ollama::Ollama;
use agent::Agent;
use std::io::{self, Write};


#[derive(Parser, Debug)]
#[command(name = "jarvis-local")]
struct Args {
/// Override model, e.g. "mistral:7b-instruct"
#[arg(short, long)]
model: Option<String>,
}


#[tokio::main]
async fn main() -> Result<()> {
let cfg = Config::from_env()?;
let args = Args::parse();
let model = args.model.unwrap_or(cfg.model);


let ollama = Ollama::new(cfg.ollama_host);
let agent = Agent::new(ollama, model);


println!("Jarvis Local (Rust + Ollama) — type :help for commands\n");
let mut history = vec![];


loop {
print!("> ");
io::stdout().flush().ok();
let mut input = String::new();
if io::stdin().read_line(&mut input).is_err() { break; }
let input = input.trim();
if input.is_empty() { continue; }


// Commands
if input == ":quit" || input == ":q" { break; }
if input == ":help" {
println!(":help — this help\n:quit — exit\n:model <name> — switch model at runtime");
continue;
}
if let Some(rest) = input.strip_prefix(":model ") {
let mut a = agent; // shadow not allowed easily; simple message for now
println!("(restart with --model {} for now)", rest);
continue;
}


match agent.step(&mut history, input).await {
Ok(reply) => println!("{}\n", reply),
Err(e) => eprintln!("error: {e}"),
}
}
}
