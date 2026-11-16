//! User Interface for CacheWatch telemetry system with start/stop controls and safety valves

use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::Duration;
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use std::io::{self, Write};

pub struct UiManager {
    is_running: Arc<Mutex<bool>>,
    max_collection_time: Duration,
    max_collection_count: u32,
    collection_count: u32,
}

impl UiManager {
    pub fn new() -> Self {
        Self {
            is_running: Arc::new(Mutex::new(false)),
            max_collection_time: Duration::from_secs(3600), // 1 hour default
            max_collection_count: 1000, // 1000 collections default
            collection_count: 0,
        }
    }

    pub fn set_max_collection_time(&mut self, seconds: u64) {
        self.max_collection_time = Duration::from_secs(seconds);
    }

    pub fn set_max_collection_count(&mut self, count: u32) {
        self.max_collection_count = count;
    }

    pub async fn start_collection(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut is_running = self.is_running.lock().await;
        *is_running = true;
        self.collection_count = 0;
        drop(is_running); // Release the lock
        Ok(())
    }

    pub async fn stop_collection(&mut self) {
        let mut is_running = self.is_running.lock().await;
        *is_running = false;
        println!("Collection stopped by user.");
    }

    pub async fn is_collection_running(&self) -> bool {
        let is_running = self.is_running.lock().await;
        *is_running
    }

    pub async fn increment_collection_count(&mut self) {
        self.collection_count += 1;
    }

    pub async fn should_continue_collection(&self) -> bool {
        let is_running = self.is_running.lock().await;
        if !*is_running {
            return false;
        }
        
        // Check safety valves
        if self.collection_count >= self.max_collection_count {
            println!("Safety valve triggered: Maximum collection count reached ({})", self.max_collection_count);
            return false;
        }
        
        true
    }

    pub async fn display_status(&self, collection_duration: &str) {
        let is_running = self.is_running.lock().await;
        let status = if *is_running { "RUNNING" } else { "STOPPED" };
        println!("CacheWatch Status: {}", status);
        println!("Collection Duration: {}", collection_duration);
        println!("Collection Count: {}", self.collection_count);
        println!("Max Collection Time: {} seconds", self.max_collection_time.as_secs());
        println!("Max Collection Count: {}", self.max_collection_count);
        println!();
    }

    pub async fn clear_screen(&self) {
        let mut stdout = io::stdout();
        execute!(stdout, Clear(ClearType::All)).unwrap();
        stdout.flush().unwrap();
    }
}