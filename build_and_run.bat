@echo off
REM Build and run CacheWatch telemetry system

echo Building CacheWatch project...
cargo build

if %errorlevel% equ 0 (
    echo Build successful!
    echo Running CacheWatch telemetry collector...
    cargo run --bin cache-watch
) else (
    echo Build failed!
    exit /b 1
)