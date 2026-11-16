//! llama.cpp collector for gathering LLM metrics

use reqwest;
use serde::{Deserialize, Serialize};
use crate::telemetry::metrics::{OllamaMetrics, OllamaModel};

#[derive(Debug, Serialize, Deserialize)]
pub struct LlamaCppResponse {
    pub models: Vec<OllamaModel>,
}

pub struct LlamaCppCollector {
    client: reqwest::Client,
    base_url: String,
}

impl LlamaCppCollector {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: "http://localhost:8080".to_string(), // Default llama.cpp port
        }
    }

    pub async fn collect(&mut self) -> Result<OllamaMetrics, Box<dyn std::error::Error>> {
        println!("Collecting llama.cpp metrics...\n");
        
        let response = self.client
            .get(&format!("{}/models", self.base_url))
            .send()
            .await?;
            
        let llama_cpp_response: LlamaCppResponse = response.json().await?;
        
        // Process and return metrics
        Ok(OllamaMetrics {
            models: llama_cpp_response.models,
            timestamp: chrono::Utc::now(),
        })
    }
}