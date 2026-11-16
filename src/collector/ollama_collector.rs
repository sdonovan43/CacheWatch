//! Ollama collector for gathering LLM metrics

use reqwest;
use serde::{Deserialize, Serialize};
use crate::telemetry::metrics::{OllamaMetrics, OllamaModel};

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaResponse {
    pub models: Vec<OllamaModel>,
}

pub struct OllamaCollector {
    client: reqwest::Client,
    base_url: String,
}

impl OllamaCollector {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: "http://localhost:11434".to_string(),
        }
    }

    pub async fn collect(&mut self) -> Result<OllamaMetrics, Box<dyn std::error::Error>> {
        println!("Collecting Ollama metrics...\n");
        
        let response = self.client
            .get(&format!("{}/api/ps", self.base_url))
            .send()
            .await?;
            
        let ollama_response: OllamaResponse = response.json().await?;
        
        // Process and return metrics
        Ok(OllamaMetrics {
            models: ollama_response.models,
            timestamp: chrono::Utc::now(),
        })
    }
}