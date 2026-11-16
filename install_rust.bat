@echo off
echo Installing Rust and Tauri development tools
echo.

REM Check if PowerShell is available
powershell.exe -Command "Get-Command powershell" >nul 2>&1
if %errorlevel% neq 0 (
    echo PowerShell not found. Please install PowerShell and try again.
    pause
    exit /b 1
)

echo Downloading rustup installer...
powershell.exe -Command "[System.Net.ServicePointManager]::SecurityProtocol = [System.Net.SecurityProtocolType]::Tls12; Invoke-WebRequest -Uri 'https://win.rustup.rs/' -OutFile '%TEMP%\rustup-init.exe'"

if %errorlevel% equ 0 (
    echo Rustup installer downloaded successfully.
    echo Installing Rust...
    
    REM Run the installer
    "%TEMP%\rustup-init.exe" -y
    
    if %errorlevel% equ 0 (
        echo Rust installed successfully!
        echo Please restart your terminal/command prompt to complete installation.
        echo Then run 'rustc --version' and 'cargo --version' to verify installation.
    ) else (
        echo Failed to install Rust. Please check the error above.
        pause
        exit /b 1
    )
) else (
    echo Failed to download rustup installer.
    echo Please manually download from https://win.rustup.rs/ and run it.
    pause
    exit /b 1
)

echo.
echo Installing Tauri CLI...
npm install -g @tauri-apps/cli

if %errorlevel% equ 0 (
    echo Tauri CLI installed successfully!
) else (
    echo Warning: Failed to install Tauri CLI. You may need to install Node.js first.
)

echo.
echo Installation process complete!
echo Check the INSTALL_RUST_TAURI.md file for detailed installation instructions.
pause