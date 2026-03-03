#!/bin/bash
set -e

echo "Running tests for all games..."

echo "1. Testing bubbles..."
cargo test -p bubbles

echo "2. Testing jetpac..."
cargo test -p jetpac

echo "3. Testing zookeeper..."
cargo test -p zookeeper_wasm

echo "All tests passed successfully!"
