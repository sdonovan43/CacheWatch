# Installing Rust and Setting Up Tauri Development Environment

## Prerequisites

Before installing Rust and setting up the Tauri development environment, ensure you have:

1. **Windows 10 or later** (or macOS/Linux)
2. **Visual Studio Build Tools** (already installed based on your note)
3. **Git** (installed and configured)

## Installing Rust

### Method 1: Using rustup (Recommended)

The easiest way to install Rust is using the official installer:

1. Download the rustup installer:
   ```powershell
   # In PowerShell, run:
   [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.SecurityProtocolType]::Tls12
   Invoke-WebRequest -Uri 'https://win.rustup.rs/' -OutFile 'rustup-init.exe'
   ```

2. Run the installer:
   ```powershell
   rustup-init.exe
   ```

3. Follow the prompts:
   - Accept default settings (press Enter)
   - Restart your terminal when prompted

4. Verify installation:
   ```powershell
   rustc --version
   cargo --version
   ```

### Method 2: Manual Installation (if rustup fails)

If the automatic installer doesn't work:

1. Visit [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
2. Download the appropriate installer for your system
3. Run the installer with administrator privileges
4. Add Rust to PATH manually if needed:
   - Add `C:\Users\[username]\.cargo\bin` to your system PATH

### Method 3: Using Package Managers

If you have Chocolatey installed:
```powershell
choco install rustup
```

## Installing Tauri Dependencies

Once Rust is installed, install the required Tauri dependencies:

1. Install Node.js (LTS version recommended)
2. Install Tauri CLI globally:
   ```bash
   npm install -g @tauri-apps/cli
   ```

3. Verify Tauri installation:
   ```bash
   tauri --version
   ```

## Environment Variables Setup

If Rust isn't found after installation, you may need to manually add these to your PATH:

1. Add to System PATH:
   - `C:\Users\[username]\.cargo\bin`
   - `C:\Users\[username]\.rustup\toolchains\[toolchain]\bin`

2. Restart your terminal/command prompt

## Troubleshooting Common Issues

### Issue 1: "The term 'rustc' is not recognized"
- Solution: Ensure Rust is added to PATH or restart your terminal
- Verify with: `where rustc`

### Issue 2: TLS/SSL errors during download
- Solution: Use PowerShell with explicit TLS settings:
  ```powershell
  [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.SecurityProtocolType]::Tls12
  ```

### Issue 3: Build failures due to missing components
- Solution: Install required components:
  ```bash
  rustup component add rust-src
  rustup component add clippy
  rustup component add rustfmt
  ```

## Testing Your Setup

After installation, verify everything works:

1. Create a test project:
   ```bash
   cargo new tauri-test
   cd tauri-test
   ```

2. Initialize Tauri:
   ```bash
   tauri init
   ```

3. Build the project:
   ```bash
   tauri build
   ```

## CacheWatch Specific Setup

For the CacheWatch project, ensure you have:

1. Rust installed with `rustup`
2. Tauri CLI installed globally
3. Node.js installed for frontend dependencies
4. The project's Cargo.toml configured properly

The CacheWatch project should already be set up to work with Tauri once the environment is properly configured.