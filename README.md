# CacheWatch-Rust-SQLite

A Rust-based telemetry system for monitoring local LLMs (Ollama, LM Studio, llama.cpp) with real-time metrics display.

## Architecture

```
┌────────────────────────┐
│ React Dashboard        │
│ Charts & Widgets       │
└──────────┬─────────────┘
           │
           ▼
┌────────────────────────┐
│ Rust Core              │
│                        │
│ Poll Ollama            │
│ Poll LM Studio         │
│ Poll llama.cpp         │
│ Poll GPU metrics       │
│ Poll System metrics    │
└──────────┬─────────────┘
           │
           ▼
┌────────────────────────┐
│ Storage Layer          │
│ SQLite                 │
└────────────────────────┘
```

## Features

- **Concurrent Telemetry Collection**: Uses Tokio async runtime for efficient concurrent polling
- **Multi-Platform Support**: Collects metrics from Ollama, LM Studio, and llama.cpp
- **GPU Monitoring**: Real-time GPU utilization and memory metrics
- **System Metrics**: CPU and memory usage tracking
- **Persistent Storage**: SQLite database for storing telemetry data
- **Extensible Design**: Easy to add new collector types
- **Interactive UI**: Start/stop collection with safety valves
- **Data Visualization**: CLI-based display of collected metrics

## Components

### Backend (Rust Core)
- `src/main.rs`: Entry point with main collection loop
- `src/collector/`: Individual collectors for different telemetry sources
- `src/storage/`: SQLite storage implementation
- `src/telemetry/`: Metrics data structures

### Frontend (React Dashboard)
- Real-time charts and widgets
- Data visualization components
- API communication with Rust backend

## Usage

### Starting Data Collection

Run the main application to start collecting telemetry data:
```bash
cargo run
```

The system provides an interactive UI with the following controls:
- `s` - Start collection
- `t` - Stop collection  
- `q` - Quit
- `c` - Clear screen
- `h` - Show help

### Safety Valves

The system includes built-in safety valves to prevent resource overuse:
1. **Maximum Collection Count**: Stops after 1000 collections (configurable)
2. **Maximum Collection Time**: Stops after 1 hour (configurable)

### Viewing Data

To view collected data, run the CLI display tool:
```bash
cargo run --bin cli-display
```

## Design Principles

### Why Rust?

Telemetry collection is a perfect Rust workload because:

- **Background threads**: Efficient concurrent polling
- **Concurrent operations**: Multiple collectors running simultaneously  
- **Socket connections**: Network communication with LLM APIs
- **Process monitoring**: System resource tracking
- **GPU monitoring**: Direct hardware access capabilities
- **Memory safety**: Prevents runtime errors in telemetry collection

### Example Concurrent Operations

```rust
tokio::spawn(async {
    poll_ollama();
});

tokio::spawn(async {
    poll_gpu();
});

tokio::spawn(async {
    poll_system_stats();
});
```

Everything runs simultaneously with proper error handling and resource management.

## Project Structure

```
CacheWatch-Rust-SQLite/
├── backend/                 # Rust core application
│   ├── src/
│   │   ├── main.rs          # Entry point
│   │   ├── ui.rs            # UI controls and safety valve logic
│   │   ├── collector/       # Collection modules
│   │   │   ├── mod.rs
│   │   │   ├── ollama_collector.rs
│   │   │   ├── lm_studio_collector.rs
│   │   │   ├── llama_cpp_collector.rs
│   │   │   ├── gpu_collector.rs
│   │   │   └── system_collector.rs
│   │   ├── storage/         # Storage modules
│   │   │   ├── mod.rs
│   │   │   └── sqlite_storage.rs
│   │   ├── telemetry/       # Telemetry handling
│   │   │   ├── mod.rs
│   │   │   └── metrics.rs
│   │   └── lib.rs           # Library exports
│   ├── Cargo.toml           # Rust project configuration
│   └── README.md            # Backend documentation
├── frontend/                # CLI display tool
│   ├── cli_display.rs       # Display implementation
│   └── README.md            # Frontend documentation
├── docs/                    # Documentation
└── README.md                # Project overview
```

## Implementation Details

### Concurrent Execution

The system uses Tokio tasks to collect metrics from all sources concurrently:

```rust
let ollama_metrics = tokio::spawn(async {
    ollama_collector.collect().await
});

let lm_studio_metrics = tokio::spawn(async {
    lm_studio_collector.collect().await
});

let llama_cpp_metrics = tokio::spawn(async {
    llama_cpp_collector.collect().await
});

// Wait for all collections to complete
let (ollama_result, lm_studio_result, llama_cpp_result) = tokio::try_join!(ollama_metrics, lm_studio_metrics, llama_cpp_metrics)?;
```

### Database Schema

The system creates three tables in the SQLite database:

#### ollama_metrics
| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER | Primary key |
| model_name | TEXT | Name of the model |
| size | INTEGER | Model size in bytes |
| modified_at | TEXT | Last modification timestamp |
| timestamp | TEXT | Collection timestamp |

#### gpu_metrics
| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER | Primary key |
| gpu_utilization | REAL | GPU utilization percentage |
| memory_used | INTEGER | Memory used in bytes |
| memory_total | INTEGER | Total memory in bytes |
| temperature | REAL | GPU temperature |
| timestamp | TEXT | Collection timestamp |

#### system_metrics
| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER | Primary key |
| cpu_usage | REAL | CPU utilization percentage |
| memory_used | INTEGER | Memory used in bytes |
| memory_total | INTEGER | Total memory in bytes |
| timestamp | TEXT | Collection timestamp |

## Planned Enhancements

1. **Configuration Management**: Environment variables or config files for settings
2. **Comprehensive Error Handling**: Detailed logging throughout the system
3. **System Tray Integration**: Desktop widget for real-time monitoring
4. **Package Distribution**: setup.py, pyproject.toml for PyPI publication
5. **Advanced GPU Monitoring**: More detailed NVIDIA/AMD metrics
6. **Web API Interface**: RESTful endpoints for frontend communication
7. **Cross-Platform Dashboard**: Tauri-based React/TypeScript dashboard with real-time charts and widgets

## Display Implementation

The CacheWatch system now includes a Tauri-based cross-platform dashboard with the following features:

### Backend API Endpoints
- `/api/ollama/metrics` - Retrieve Ollama telemetry data
- `/api/gpu/metrics` - Retrieve GPU telemetry data  
- `/api/system/metrics` - Retrieve system telemetry data
- `/api/metrics/latest` - Retrieve latest metrics from all sources

### Dashboard Features
- Real-time charts for CPU, GPU, and memory usage
- Model tracking widgets showing Ollama models
- System monitoring dashboard with live metrics
- Cross-platform compatibility (Windows, macOS, Linux)

## License

MIT License