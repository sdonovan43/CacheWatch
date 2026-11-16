use rusqlite::Connection;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Current working directory: {:?}", env::current_dir()?);
    
    // Test database connection with a simple approach
    let db_path = "simple_test.db";
    println!("Attempting to connect to: {}", db_path);
    
    match Connection::open(db_path) {
        Ok(conn) => {
            println!("Successfully connected to database!");
            
            // Try to create a simple table
            conn.execute("CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY, name TEXT)", [])?;
            
            println!("Table created successfully!");
            
            // Insert and select data
            conn.execute("INSERT INTO test (name) VALUES (?)", ["test"])?;
            
            let mut stmt = conn.prepare("SELECT * FROM test")?;
            let rows = stmt.query_map([], |row| {
                Ok((
                    row.get::<_, i32>(0)?,
                    row.get::<_, String>(1)?
                ))
            })?;
            
            println!("Rows found: {}", rows.count());
            
            println!("Simple test completed successfully!");
        }
        Err(e) => {
            println!("Failed to connect to database: {}", e);
            return Err(e.into());
        }
    }
    
    Ok(())
}