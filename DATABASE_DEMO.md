# CacheWatch Database Demonstration

This document demonstrates how to create and use a SQLite database with the CacheWatch telemetry system.

## Creating Sample Database

To demonstrate the CLI display functionality, we need to create a sample database file with test data:

```sql
-- Create Ollama metrics table
CREATE TABLE IF NOT EXISTS ollama_metrics (
    id INTEGER PRIMARY KEY,
    model_name TEXT,
    size INTEGER,
    modified_at TEXT,
    timestamp TEXT
);

-- Create GPU metrics table  
CREATE TABLE IF NOT EXISTS gpu_metrics (
    id INTEGER PRIMARY KEY,
    gpu_utilization REAL,
    memory_used INTEGER,
    memory_total INTEGER,
    temperature REAL,
    timestamp TEXT
);

-- Create System metrics table
CREATE TABLE IF NOT EXISTS system_metrics (
    id INTEGER PRIMARY KEY,
    cpu_usage REAL,
    memory_used INTEGER,
    memory_total INTEGER,
    timestamp TEXT
);

-- Insert sample Ollama metrics data
INSERT INTO ollama_metrics VALUES 
(1, 'llama3', 123456789, '2026-11-15', '2026-11-15 12:00:00'),
(2, 'mistral', 98765432, '2026-11-15', '2026-11-15 13:00:00');

-- Insert sample GPU metrics data
INSERT INTO gpu_metrics VALUES 
(1, 65.5, 2147483648, 8589934592, 72.3, '2026-11-15 12:00:00'),
(2, 72.1, 2345678912, 8589934592, 75.2, '2026-11-15 13:00:00');

-- Insert sample System metrics data
INSERT INTO system_metrics VALUES 
(1, 45.2, 4294967296, 17179869184, '2026-11-15 12:00:00'),
(2, 52.8, 4789123456, 17179869184, '2026-11-15 13:00:00');
```

## Running the CLI Display Tool

After creating the database file, you can run the CLI display tool:

```bash
# Run with default database path (cache_watch.db)
cli-display.exe

# Or specify a custom database path
cli-display.exe --db-path /path/to/your/database.db
```

## Expected Output

The CLI tool should display:
1. Ollama model information (names, sizes, timestamps)
2. GPU utilization metrics (utilization percentage, memory usage, temperature)
3. System metrics (CPU usage, memory usage)

## Complete Workflow

1. **Data Collection**: Run the main cache-watch binary to collect telemetry data
2. **Database Storage**: Data is automatically stored in SQLite database
3. **Data Viewing**: Use cli-display.exe to view collected telemetry information
4. **Analysis**: Review metrics for performance monitoring and optimization

## Cross-Platform Compatibility

The system works on:
- Windows (using build_and_run.bat)
- Linux/macOS (using build_and_run.sh)

Both scripts will automatically:
1. Build the project
2. Run data collection
3. Generate database files
4. Display results through CLI interface