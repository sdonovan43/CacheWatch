# PowerShell script to create sample database for CacheWatch telemetry system

# Create the database file with sample data
$createDbScript = @"
-- Create sample database with test data for CacheWatch telemetry system

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
(2, 'mistral', 98765432, '2026-11-15', '2026-11-15 13:00:00'),
(3, 'gemma2', 456789123, '2026-11-15', '2026-11-15 14:00:00');

-- Insert sample GPU metrics data
INSERT INTO gpu_metrics VALUES 
(1, 65.5, 2147483648, 8589934592, 72.3, '2026-11-15 12:00:00'),
(2, 72.1, 2345678912, 8589934592, 75.2, '2026-11-15 13:00:00'),
(3, 58.7, 1987654321, 8589934592, 69.8, '2026-11-15 14:00:00');

-- Insert sample System metrics data
INSERT INTO system_metrics VALUES 
(1, 45.2, 4294967296, 17179869184, '2026-11-15 12:00:00'),
(2, 52.8, 4789123456, 17179869184, '2026-11-15 13:00:00'),
(3, 38.5, 3987654321, 17179869184, '2026-11-15 14:00:00');
"@

# Write the script to a file
$createDbScript | Out-File -FilePath "test_data/create_db.sql" -Encoding UTF8

# Create database using sqlite3
sqlite3 test_data/sample.db < test_data/create_db.sql