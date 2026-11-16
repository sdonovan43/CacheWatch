#!/bin/bash

echo "Building CacheWatch Rust Telemetry System..."

# Build the project
cargo build

if [ $? -eq 0 ]; then
    echo "Build successful!"
    echo "To run the telemetry collector:"
    echo "  cargo run"
    echo ""
    echo "To run with specific features:"
    echo "  cargo run --features \"gpu_support\""
else
    echo "Build failed!"
    exit 1
fi