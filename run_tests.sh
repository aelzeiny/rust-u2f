#!/bin/bash
set -e

echo "Running tests for Pi U2F components"

# Run tests for pi-hid
echo "Testing pi-hid..."
cargo test -p pi-hid -- --nocapture

# Run tests for pi-u2f-adapter
echo "Testing pi-u2f-adapter..."
cargo test -p pi-u2f-adapter -- --nocapture

# Run tests for pi-u2f-daemon
echo "Testing pi-u2f-daemon..."
cargo test -p pi-u2f-daemon -- --nocapture

# Run integration tests
echo "Running integration tests..."
cargo test -p pi-u2f-tests -- --nocapture

echo "All tests completed!"