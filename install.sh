#!/bin/bash
set -e

# Build the binary
echo "Building vin-ui..."
cargo build --release

# Create config directory
CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.cargo/bin/}"
mkdir -p "$CONFIG_DIR"

# Copy templates
echo "Installing templates..."
cp -r templates "$CONFIG_DIR/"

# Install binary to ~/.cargo/bin or use cargo install
echo "Installing binary..."
cargo install --path .

echo -e "\n\033[32mInstallation complete!\033[0m"
echo "Run 'vin-ui --help' to get started."
