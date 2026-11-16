# Check Rust and Tauri Setup
# This script verifies that Rust and Tauri development tools are properly installed

Write-Host "Checking Rust installation..." -ForegroundColor Green

# Check if rustc is available
$rustcPath = Get-Command rustc -ErrorAction SilentlyContinue
if ($rustcPath) {
    Write-Host "✓ rustc found at: $($rustcPath.Source)" -ForegroundColor Green
    $rustVersion = rustc --version
    Write-Host "Rust version: $rustVersion" -ForegroundColor Yellow
} else {
    Write-Host "✗ rustc not found. Please install Rust using rustup." -ForegroundColor Red
}

# Check if cargo is available
$cargoPath = Get-Command cargo -ErrorAction SilentlyContinue
if ($cargoPath) {
    Write-Host "✓ cargo found at: $($cargoPath.Source)" -ForegroundColor Green
    $cargoVersion = cargo --version
    Write-Host "Cargo version: $cargoVersion" -ForegroundColor Yellow
} else {
    Write-Host "✗ cargo not found. Please install Rust using rustup." -ForegroundColor Red
}

# Check if tauri is available
$tauriPath = Get-Command tauri -ErrorAction SilentlyContinue
if ($tauriPath) {
    Write-Host "✓ Tauri CLI found at: $($tauriPath.Source)" -ForegroundColor Green
    $tauriVersion = tauri --version
    Write-Host "Tauri version: $tauriVersion" -ForegroundColor Yellow
} else {
    Write-Host "⚠ Tauri CLI not found. Install with: npm install -g @tauri-apps/cli" -ForegroundColor Yellow
}

# Check if Node.js is available (required for Tauri)
$nodePath = Get-Command node -ErrorAction SilentlyContinue
if ($nodePath) {
    Write-Host "✓ Node.js found at: $($nodePath.Source)" -ForegroundColor Green
    $nodeVersion = node --version
    Write-Host "Node version: $nodeVersion" -ForegroundColor Yellow
} else {
    Write-Host "⚠ Node.js not found. Install from https://nodejs.org/" -ForegroundColor Yellow
}

Write-Host "`nSetup check complete!" -ForegroundColor Green

# If any critical tools are missing, suggest installation steps
if (-not $rustcPath -or -not $cargoPath) {
    Write-Host "`nTo install Rust:" -ForegroundColor Cyan
    Write-Host "1. Download rustup-init.exe from https://win.rustup.rs/" -ForegroundColor Cyan
    Write-Host "2. Run rustup-init.exe and follow the prompts" -ForegroundColor Cyan
    Write-Host "3. Restart your terminal" -ForegroundColor Cyan
}