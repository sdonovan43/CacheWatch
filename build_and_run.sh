#!/bin/bash

# Build and run script for CacheWatch-Rust-SQLite on Linux/macOS

echo "Building CacheWatch CLI tool..."
cargo build --bin cli-display

if [ $? -eq 0 ]; then
    echo "Build successful!"
    echo "Running the CLI tool..."
    ./target/debug/cli-display
else
    echo "Build failed!"
    exit 1
fi