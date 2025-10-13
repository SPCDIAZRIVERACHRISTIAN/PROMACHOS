use crate::schema::{OllamaChatRequest, OllamaChatResponse};
use anyhow::{anyhow, Result};
use reqwest::Client;


#[derive(Clone)]
pub struct Ollama {
client: Client,
base: String,
}


impl Ollama {
pub fn new(base: impl Into<String>) -> Self {
Self { client: Client::new(), base: base.into() }
}


pub async fn chat(&self, req: &OllamaChatRequest) -> Result<OllamaChatResponse> {
let url = format!("{}/api/chat", self.base);
let res = self.client.post(url).json(req).send().await?;
if !res.status().is_success() {
return Err(anyhow!("ollama error: {}", res.text().await?));
}
Ok(res.json::<OllamaChatResponse>().await?)
}
}
