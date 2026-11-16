# CacheWatch CLI Display Demo

This document demonstrates how to use the command-line interface for displaying telemetry data.

## Running the CLI Tool

To run the CLI display tool, you need to have Rust installed and then execute:

```bash
cargo run --bin cli-display
```

## Expected Output

The CLI tool will display the latest telemetry data from the SQLite database in a structured format:

```
CacheWatch Telemetry Data
=========================

Ollama Metrics:
---------------
ID: 1, Model: llama2, Size: 34567890 bytes, Modified: 2026-01-01T10:00:00Z, Timestamp: 2026-01-01T10:05:00Z

GPU Metrics:
------------
ID: 1, GPU Util: 45.2%, Memory Used: 2147483648 bytes, Memory Total: 8589934592 bytes, Temp: 62.5°C, Timestamp: 2026-01-01T10:05:00Z

System Metrics:
---------------
ID: 1, CPU Util: 23.4%, Memory Used: 4294967296 bytes, Memory Total: 17179869184 bytes, Timestamp: 2026-01-01T10:05:00Z
```

## Database Structure

The CLI tool connects to a SQLite database (`cache_watch.db`) with the following tables:

### ollama_metrics
| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER | Primary key |
| model_name | TEXT | Name of the model |
| size | INTEGER | Model size in bytes |
| modified_at | TEXT | Last modification timestamp |
| timestamp | TEXT | Collection timestamp |

### gpu_metrics
| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER | Primary key |
| gpu_utilization | REAL | GPU utilization percentage |
| memory_used | INTEGER | Memory used in bytes |
| memory_total | INTEGER | Total memory in bytes |
| temperature | REAL | GPU temperature |
| timestamp | TEXT | Collection timestamp |

### system_metrics
| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER | Primary key |
| cpu_usage | REAL | CPU utilization percentage |
| memory_used | INTEGER | Memory used in bytes |
| memory_total | INTEGER | Total memory in bytes |
| timestamp | TEXT | Collection timestamp |