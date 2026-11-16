//! LM Studio collector for gathering LLM metrics

use reqwest;
use serde::{Deserialize, Serialize};
use crate::telemetry::metrics::{OllamaMetrics, OllamaModel};

#[derive(Debug, Serialize, Deserialize)]
pub struct LmStudioResponse {
    pub models: Vec<OllamaModel>,
}

pub struct LmStudioCollector {
    client: reqwest::Client,
    base_url: String,
}

impl LmStudioCollector {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: "http://localhost:1234".to_string(), // Default LM Studio port
        }
    }

    pub async fn collect(&mut self) -> Result<OllamaMetrics, Box<dyn std::error::Error>> {
        println!("Collecting LM Studio metrics...\n");
        
        let response = self.client
            .get(&format!("{}/v1/models", self.base_url))
            .send()
            .await?;
            
        let lm_studio_response: LmStudioResponse = response.json().await?;
        
        // Process and return metrics
        Ok(OllamaMetrics {
            models: lm_studio_response.models,
            timestamp: chrono::Utc::now(),
        })
    }
}