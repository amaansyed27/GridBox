#!/usr/bin/env bash
set -euo pipefail

command -v cargo >/dev/null || { echo "Rust/Cargo is required" >&2; exit 1; }
command -v uv >/dev/null || { echo "uv is required" >&2; exit 1; }

uv sync --extra dev
cargo build --workspace
cargo run -p gridbox-cli -- doctor || true

echo "Run: cargo run -p gridbox-cli"
