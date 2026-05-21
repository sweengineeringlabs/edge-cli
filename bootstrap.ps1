# Edge CLI project bootstrap script for Windows

$ErrorActionPreference = "Stop"

Write-Host "==> Bootstrap edge CLI"
Write-Host "    Installing Rust toolchain..."

# Verify Rust is installed
if (-not (Get-Command rustc -ErrorAction SilentlyContinue)) {
    Write-Host "Rust is not installed. Please install Rust from https://rustup.rs/"
    exit 1
}

Write-Host "    Rust version: $(rustc --version)"
Write-Host "    Cargo version: $(cargo --version)"

$repoRoot = Split-Path -Parent $MyInvocation.MyCommand.Path

# Fetch dependencies for all workspaces
@("cli/features/main") | ForEach-Object {
    $workspace = $_
    Write-Host "    Fetching dependencies for $workspace..."
    Push-Location (Join-Path $repoRoot $workspace)
    cargo fetch --locked
    Pop-Location
}

Write-Host "`u{2713} Bootstrap complete"
