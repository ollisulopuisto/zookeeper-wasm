#!/bin/bash
set -e

echo "Adding WASM target..."
rustup target add wasm32-unknown-unknown

echo "Building project..."
cargo build --target wasm32-unknown-unknown --release

echo "Preparing docs directory for GitHub Pages..."
rm -rf docs
mkdir -p docs

echo "Copying WASM binary..."
cp target/wasm32-unknown-unknown/release/zookeeper_wasm.wasm docs/zookeeper_wasm.wasm

echo "Copying index.html..."
cp index.html docs/index.html

echo "Build complete! You can serve the 'docs' directory to test."
