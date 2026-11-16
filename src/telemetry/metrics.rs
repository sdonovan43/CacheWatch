//! Telemetry metrics data structures

use serde::{Deserialize, Serialize};
use chrono::DateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaModel {
    pub name: String,
    pub size: i64, // Changed from u64 to i64 for SQLx compatibility
    pub modified_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaMetrics {
    pub models: Vec<OllamaModel>,
    pub timestamp: DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuMetrics {
    pub gpu_utilization: f64,
    pub memory_used: i64,
    pub memory_total: i64,
    pub temperature: f64,
    pub timestamp: DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_usage: f64,
    pub memory_used: i64,
    pub memory_total: i64,
    pub timestamp: DateTime<chrono::Utc>,
}
