#!/bin/bash

# Build script for Claude Code Monitor Rust version

echo "🔧 Building Claude Code Monitor (Rust + Makepad)..."

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Cargo.toml not found. Please run this script from the project root."
    exit 1
fi

# Build the project
echo "📦 Running cargo build..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo "🚀 You can run the application with: cargo run --release"
else
    echo "❌ Build failed. Please check the errors above."
    exit 1
fi