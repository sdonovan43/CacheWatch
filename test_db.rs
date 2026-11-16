use rusqlite::Connection;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Test database connection with a simple approach
    println!("Current working directory: {:?}", env::current_dir()?);
    
    // Try to connect to a simple database file in current directory
    let db_path = "test.db";
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
            
            println!("Test completed successfully!");
        }
        Err(e) => {
            println!("Failed to connect to database: {}", e);
            return Err(e.into());
        }
    }
    
    Ok(())
}