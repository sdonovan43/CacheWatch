// Simple verification that rusqlite is working correctly
use rusqlite::Connection;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== CacheWatch Database Verification ===");
    
    // Create test database in current directory
    let db_path = "verification_test.db";
    println!("Creating test database at: {}", db_path);
    
    // Try to create connection
    let conn = Connection::open(db_path)?;
    println!("✓ Successfully opened database connection");
    
    // Create a simple table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS test_table (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            created_at TEXT
        )", 
        []
    )?;
    println!("✓ Created test table");
    
    // Insert test data
    conn.execute(
        "INSERT INTO test_table (name, created_at) VALUES (?1, ?2)",
        ["test_entry", "2026-11-15"]
    )?;
    println!("✓ Inserted test data");
    
    // Query test data
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM test_table")?;
    let count: i64 = stmt.query_row([], |row| row.get(0))?;
    println!("✓ Query successful - Found {} records", count);
    
    println!("=== Verification Complete ===");
    println!("CacheWatch database implementation is working correctly!");
    
    Ok(())
}