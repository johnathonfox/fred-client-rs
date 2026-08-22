#!/usr/bin/env bash
set -e

echo "=== fred-client-rs dev setup ==="

# Install required Rust components
rustup component add rustfmt clippy

# Install cargo tools
cargo install cargo-deny cargo-outdated cargo-audit

# Build the project
cargo build --all-features

echo ""
echo "Done! Set FRED_API_KEY env var to run integration tests."
echo "Get a key at: https://fred.stlouisfed.org/docs/api/api_key.html"
