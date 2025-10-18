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
#[command(name = "Promachos-local")]
struct Args {
    /// Override model, e.g. "mistral:7b-instruct"
    #[arg(short, long)]
    model: Option<String>,
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("fatal: {e}");
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let cfg = Config::from_env()?;
    let args = Args::parse();
    let mut model = args.model.unwrap_or(cfg.model);
    let ollama = Ollama::new(cfg.ollama_host);
    let mut agent = Agent::new(ollama, model);
    
    //for logging what the program does.
    tracing_subscriber::fmt()
            .with_env_filter("info");
            .init();
    tracing::info!("Agent is starting up!...");

    println!("Promachos Local (Rust + Ollama) — type :help for commands\n");
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
            println!(":help — this help\n:quit — exit\n:model <name> — switch model at runtime\n:tools — list available tools");
            continue;
        }

        if let Some(rest) = input.strip_prefix(":model ") {
            model = rest.to_string();
            agent = Agent::new(ollama.clone(), model.clone());
            tracing::info!("✅ Switched model to {model}");
            println!("(using model: {model})");
            continue;
        }

        if input == ":tools" {
            for t in tools::registry() {
                println!("• {} — {}", t.name(), t.description());
            }
            continue;
        }

        match agent.step(&mut history, input).await {
            Ok(reply) => println!("{reply}\n"),
            Err(e) => eprintln!("error: {e}"),
        }
    }

    Ok(())
}

