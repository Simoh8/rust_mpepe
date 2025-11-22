#!/bin/bash
# Build script for Vercel

# Install Rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

# Build the project in release mode
cargo build --release

# Create the public directory structure expected by Vercel
mkdir -p public
cp target/release/mpesa_web public/
cp -r public/* public/ 2>/dev/null || true