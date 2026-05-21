#!/bin/bash
# Edge CLI project bootstrap script

set -e

echo "==> Bootstrap edge CLI"
echo "    Installing Rust toolchain..."

# Verify Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "Rust is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi

echo "    Rust version: $(rustc --version)"
echo "    Cargo version: $(cargo --version)"

cd "$(dirname "$0")"

# Fetch dependencies for all workspaces
for workspace in main/features; do
    echo "    Fetching dependencies for $workspace..."
    cd "$workspace"
    cargo fetch --locked
    cd - > /dev/null
done

echo "✓ Bootstrap complete"
