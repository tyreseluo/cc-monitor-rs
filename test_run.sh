#!/bin/bash

echo "Building and running Claude Code Monitor..."
cargo build 2>&1

if [ $? -eq 0 ]; then
    echo "Build successful. Running application..."
    cargo run 2>&1 | tee debug_output.log
else
    echo "Build failed"
fi