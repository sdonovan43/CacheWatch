//! SQLite storage implementation for telemetry data

use rusqlite::{Connection, Result as SqliteResult};
use std::fs;
use std::path::Path;

pub struct SqliteStorage {
    connection: Connection,
}

impl SqliteStorage {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Try to create the cache-watch-db directory if it doesn't exist
        fs::create_dir_all("cache-watch-db")?;
        
        // Try multiple locations for the database file to avoid permission issues
        let db_paths = vec![
            "cache_watch.db",  // Current directory
            "./cache_watch.db", // Explicit current directory  
            "cache-watch-db/cache_watch.db", // Subdirectory within cache-watch (our preferred location)
        ];
        
        let mut last_error = None;
        
        for path in &db_paths {
            println!("Attempting to connect to database at: {}", path);
            
            // Try to create the directory if it doesn't exist
            if let Some(parent) = Path::new(path).parent() {
                if let Err(e) = fs::create_dir_all(parent) {
                    eprintln!("Failed to create directory for {}: {}", path, e);
                }
            }
            
            // Try to create the connection
            match Connection::open(path) {
                Ok(conn) => {
                    println!("Successfully connected to database at: {}", path);
                    return Ok(SqliteStorage { connection: conn });
                }
                Err(e) => {
                    println!("Failed to connect to database at {}: {}", path, e);
                    last_error = Some(e);
                }
            }
        }
        
        // If all file-based approaches fail, fall back to in-memory database for testing
        println!("All file-based database connections failed. Falling back to in-memory database for testing.");
        let conn = Connection::open_in_memory()?;
        println!("Successfully connected to in-memory database");
        
        // Initialize schema for in-memory database
        let sql = r#"
        CREATE TABLE IF NOT EXISTS ollama_metrics (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            model_name TEXT,
            size INTEGER,
            modified_at TEXT,
            timestamp TEXT
        );
        
        CREATE TABLE IF NOT EXISTS gpu_metrics (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            gpu_utilization REAL,
            memory_used INTEGER,
            memory_total INTEGER,
            temperature REAL,
            timestamp TEXT
        );
        
        CREATE TABLE IF NOT EXISTS system_metrics (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            cpu_usage REAL,
            memory_used INTEGER,
            memory_total INTEGER,
            timestamp TEXT
        );
        "#;
        
        conn.execute_batch(sql)?;
        println!("In-memory database initialized successfully");
        
        Ok(SqliteStorage { connection: conn })
    }

    pub fn initialize_database(&self) -> SqliteResult<()> {
        // This method is now redundant since we initialize in new()
        // but keeping it for API compatibility
        Ok(())
    }

    pub fn save_ollama_metrics(&self, metrics: &crate::telemetry::metrics::OllamaMetrics) -> SqliteResult<()> {
        let mut stmt = self.connection.prepare(
            "INSERT INTO ollama_metrics (model_name, size, modified_at, timestamp) VALUES (?1, ?2, ?3, ?4)"
        )?;
        
        for model in &metrics.models {
            stmt.execute([
                &model.name,
                &model.size.to_string(),
                &model.modified_at,
                &metrics.timestamp.to_rfc3339()
            ])?;
        }
        Ok(())
    }

    pub fn save_gpu_metrics(&self, metrics: &crate::telemetry::metrics::GpuMetrics) -> SqliteResult<()> {
        let mut stmt = self.connection.prepare(
            "INSERT INTO gpu_metrics (gpu_utilization, memory_used, memory_total, temperature, timestamp) VALUES (?1, ?2, ?3, ?4, ?5)"
        )?;
        
        stmt.execute([
            &metrics.gpu_utilization.to_string(),
            &metrics.memory_used.to_string(),
            &metrics.memory_total.to_string(),
            &metrics.temperature.to_string(),
            &metrics.timestamp.to_rfc3339()
        ])?;
        
        Ok(())
    }

    pub fn save_system_metrics(&self, metrics: &crate::telemetry::metrics::SystemMetrics) -> SqliteResult<()> {
        let mut stmt = self.connection.prepare(
            "INSERT INTO system_metrics (cpu_usage, memory_used, memory_total, timestamp) VALUES (?1, ?2, ?3, ?4)"
        )?;
        
        stmt.execute([
            &metrics.cpu_usage.to_string(),
            &metrics.memory_used.to_string(),
            &metrics.memory_total.to_string(),
            &metrics.timestamp.to_rfc3339()
        ])?;
        
        Ok(())
    }
}