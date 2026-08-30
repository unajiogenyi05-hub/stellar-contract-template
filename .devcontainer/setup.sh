#!/usr/bin/env bash
set -euo pipefail

echo "==> Adding wasm32-unknown-unknown Rust target..."
rustup target add wasm32-unknown-unknown

echo "==> Installing stellar-cli..."
cargo install --locked stellar-cli --features opt

echo ""
echo "✅ Dev container setup complete."
echo "   Run 'cp .env.example .env' and fill in your SOURCE_ACCOUNT to get started."
