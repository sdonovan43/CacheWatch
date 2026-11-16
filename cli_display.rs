//! Simple command-line interface for displaying CacheWatch telemetry data

use std::error::Error;
use rusqlite::{Connection, Result as SqliteResult};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to the SQLite database using our new approach
    let conn = Connection::open("cache_watch.db")?;
    
    println!("CacheWatch Telemetry Data");
    println!("=========================");
    println!();
    
    // Display Ollama metrics
    display_ollama_metrics(&conn).await?;
    
    // Display GPU metrics  
    display_gpu_metrics(&conn).await?;
    
    // Display System metrics
    display_system_metrics(&conn).await?;
    
    // Show summary
    show_summary(&conn).await?;
    
    Ok(())
}

async fn display_ollama_metrics(conn: &Connection) -> Result<(), Box<dyn Error>> {
    println!("Ollama Metrics:");
    println!("---------------");
    
    let mut stmt = conn.prepare("SELECT * FROM ollama_metrics ORDER BY timestamp DESC LIMIT 5")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, i64>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, String>(4)?
        ))
    })?;
    
    let mut found = false;
    for row in rows {
        let (id, model_name, size, modified_at, timestamp) = row?;
        println!("ID: {}, Model: {}, Size: {} bytes, Modified: {}, Timestamp: {}", 
                 id, model_name, size, modified_at, timestamp);
        found = true;
    }
    
    if !found {
        println!("No Ollama metrics found.");
    }
    
    println!();
    Ok(())
}

async fn display_gpu_metrics(conn: &Connection) -> Result<(), Box<dyn Error>> {
    println!("GPU Metrics:");
    println!("------------");
    
    let mut stmt = conn.prepare("SELECT * FROM gpu_metrics ORDER BY timestamp DESC LIMIT 5")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, f64>(1)?,
            row.get::<_, i64>(2)?,
            row.get::<_, i64>(3)?,
            row.get::<_, f64>(4)?,
            row.get::<_, String>(5)?
        ))
    })?;
    
    let mut found = false;
    for row in rows {
        let (id, gpu_utilization, memory_used, memory_total, temperature, timestamp) = row?;
        println!("ID: {}, GPU Util: {:.1}%, Memory Used: {} bytes, Memory Total: {} bytes, Temp: {:.1}°C, Timestamp: {}", 
                 id, gpu_utilization, memory_used, memory_total, temperature, timestamp);
        found = true;
    }
    
    if !found {
        println!("No GPU metrics found.");
    }
    
    println!();
    Ok(())
}

async fn display_system_metrics(conn: &Connection) -> Result<(), Box<dyn Error>> {
    println!("System Metrics:");
    println!("---------------");
    
    let mut stmt = conn.prepare("SELECT * FROM system_metrics ORDER BY timestamp DESC LIMIT 5")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, f64>(1)?,
            row.get::<_, i64>(2)?,
            row.get::<_, i64>(3)?,
            row.get::<_, String>(4)?
        ))
    })?;
    
    let mut found = false;
    for row in rows {
        let (id, cpu_usage, memory_used, memory_total, timestamp) = row?;
        println!("ID: {}, CPU Util: {:.1}%, Memory Used: {} bytes, Memory Total: {} bytes, Timestamp: {}", 
                 id, cpu_usage, memory_used, memory_total, timestamp);
        found = true;
    }
    
    if !found {
        println!("No system metrics found.");
    }
    
    println!();
    Ok(())
}

async fn show_summary(conn: &Connection) -> Result<(), Box<dyn Error>> {
    println!("Data Summary:");
    println!("-------------");
    
    // Get counts for each table
    let ollama_count: i64 = conn.query_row("SELECT COUNT(*) FROM ollama_metrics", [], |row| row.get(0))?;
    
    let gpu_count: i64 = conn.query_row("SELECT COUNT(*) FROM gpu_metrics", [], |row| row.get(0))?;
    
    let system_count: i64 = conn.query_row("SELECT COUNT(*) FROM system_metrics", [], |row| row.get(0))?;
    
    println!("Ollama metrics: {} records", ollama_count);
    println!("GPU metrics: {} records", gpu_count);
    println!("System metrics: {} records", system_count);
    
    // Get latest timestamps
    let latest_ollama: Option<String> = conn.query_row("SELECT MAX(timestamp) FROM ollama_metrics", [], |row| row.get(0))?;
    
    let latest_gpu: Option<String> = conn.query_row("SELECT MAX(timestamp) FROM gpu_metrics", [], |row| row.get(0))?;
    
    let latest_system: Option<String> = conn.query_row("SELECT MAX(timestamp) FROM system_metrics", [], |row| row.get(0))?;
    
    println!("Latest Ollama data: {:?}", latest_ollama);
    println!("Latest GPU data: {:?}", latest_gpu);
    println!("Latest System data: {:?}", latest_system);
    
    println!();
    Ok(())
}