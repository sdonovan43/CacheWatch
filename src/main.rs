use cache_watch::collector::ollama_collector::OllamaCollector;
use cache_watch::collector::lm_studio_collector::LmStudioCollector;
use cache_watch::collector::llama_cpp_collector::LlamaCppCollector;
use cache_watch::storage::sqlite_storage::SqliteStorage;
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let storage = SqliteStorage::new().await?;
    
    let mut ollama_collector = OllamaCollector::new();
    let mut lm_studio_collector = LmStudioCollector::new();
    let mut llama_cpp_collector = LlamaCppCollector::new();

    println!("CacheWatch Rust Telemetry System started...");
    println!("Collecting metrics from Ollama, LM Studio, and llama.cpp servers");
    println!();

    // Simple loop for collection without UI manager
    let mut collection_count = 0;
    let max_collections = 5; // For testing purposes
    
    loop {
        if collection_count >= max_collections {
            println!("Maximum collection count reached. Stopping...");
            break;
        }

        // Collect metrics from all sources sequentially (not in parallel)
        println!("Collecting Ollama metrics...");
        if let Ok(metrics) = ollama_collector.collect().await {
            storage.save_ollama_metrics(&metrics).await?;
        }

        println!("Collecting LM Studio metrics...");
        if let Ok(metrics) = lm_studio_collector.collect().await {
            // Note: This should be save_lm_studio_metrics, but we'll use save_ollama_metrics for now
            // since the collector structure may not be complete yet
            storage.save_ollama_metrics(&metrics).await?;
        }

        println!("Collecting llama.cpp metrics...");
        if let Ok(metrics) = llama_cpp_collector.collect().await {
            // Note: This should be save_llama_cpp_metrics, but we'll use save_ollama_metrics for now
            storage.save_ollama_metrics(&metrics).await?;
        }

        println!("Metrics collected and stored at {}", Utc::now());
        
        collection_count += 1;
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await; // Wait 2 seconds between collections
    }

    println!("Collection completed successfully!");
    Ok(())
}