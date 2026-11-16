//! Example usage of the CacheWatch Rust Telemetry System

/// This file demonstrates how to use the CacheWatch telemetry system
/// 
/// To run this example:
/// 1. Ensure you have Rust installed
/// 2. Start your LLM servers (Ollama, LM Studio, or llama.cpp) on their default ports
/// 3. Run with: cargo run --example example_usage

use cache_watch::collector::ollama_collector::OllamaCollector;
use cache_watch::collector::lm_studio_collector::LmStudioCollector;
use cache_watch::collector::llama_cpp_collector::LlamaCppCollector;
use cache_watch::storage::sqlite_storage::SqliteStorage;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("CacheWatch Rust Telemetry System - Example Usage");
    println!("================================================");
    
    // Initialize collectors
    let mut ollama_collector = OllamaCollector::new();
    let mut lm_studio_collector = LmStudioCollector::new();
    let mut llama_cpp_collector = LlamaCppCollector::new();
    
    // Initialize storage
    let storage = SqliteStorage::new().await?;
    
    println!("Collectors and storage initialized successfully!");
    println!("Starting concurrent collection from all sources...");
    
    // Demonstrate concurrent collection (this would normally run in a loop)
    let ollama_task = tokio::spawn(async {
        println!("Starting Ollama collection...");
        match ollama_collector.collect().await {
            Ok(metrics) => {
                println!("Ollama collection successful!");
                Some(metrics)
            }
            Err(e) => {
                println!("Ollama collection failed: {}", e);
                None
            }
        }
    });
    
    let lm_studio_task = tokio::spawn(async {
        println!("Starting LM Studio collection...");
        match lm_studio_collector.collect().await {
            Ok(metrics) => {
                println!("LM Studio collection successful!");
                Some(metrics)
            }
            Err(e) => {
                println!("LM Studio collection failed: {}", e);
                None
            }
        }
    });
    
    let llama_cpp_task = tokio::spawn(async {
        println!("Starting llama.cpp collection...");
        match llama_cpp_collector.collect().await {
            Ok(metrics) => {
                println!("llama.cpp collection successful!");
                Some(metrics)
            }
            Err(e) => {
                println!("llama.cpp collection failed: {}", e);
                None
            }
        }
    });
    
    // Wait for all collections to complete
    let (ollama_result, lm_studio_result, llama_cpp_result) = 
        tokio::try_join!(ollama_task, lm_studio_task, llama_cpp_task)?;
    
    // Process results
    if let Some(metrics) = ollama_result {
        println!("Storing Ollama metrics...");
        match storage.save_ollama_metrics(&metrics).await {
            Ok(_) => println!("Ollama metrics stored successfully!"),
            Err(e) => println!("Failed to store Ollama metrics: {}", e),
        }
    }
    
    if let Some(metrics) = lm_studio_result {
        println!("Storing LM Studio metrics...");
        match storage.save_ollama_metrics(&metrics).await {
            Ok(_) => println!("LM Studio metrics stored successfully!"),
            Err(e) => println!("Failed to store LM Studio metrics: {}", e),
        }
    }
    
    if let Some(metrics) = llama_cpp_result {
        println!("Storing llama.cpp metrics...");
        match storage.save_ollama_metrics(&metrics).await {
            Ok(_) => println!("llama.cpp metrics stored successfully!"),
            Err(e) => println!("Failed to store llama.cpp metrics: {}", e),
        }
    }
    
    println!("Example usage completed!");
    Ok(())
}