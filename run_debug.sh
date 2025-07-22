#!/bin/bash
echo "Starting Claude Code Monitor..."
export RUST_BACKTRACE=1
cargo run 2>&1 | tee debug_output.log