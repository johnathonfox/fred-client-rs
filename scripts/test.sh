#!/usr/bin/env bash
set -e

echo "=== Running fred-client-rs test suite ==="

echo "Checking formatting..."
cargo fmt -- --check

echo "Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings

echo "Running tests..."
cargo test --all-features

echo "Building docs..."
cargo doc --no-deps --all-features

echo "Running cargo-deny..."
cargo deny check

echo ""
echo "All checks passed!"
