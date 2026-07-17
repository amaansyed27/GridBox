$ErrorActionPreference = "Stop"

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    throw "Rust/Cargo is required. Install rustup first."
}
if (-not (Get-Command uv -ErrorAction SilentlyContinue)) {
    throw "uv is required. Install it with: winget install astral-sh.uv"
}

uv sync --extra dev
cargo build --workspace
cargo run -p gridbox-cli -- doctor

Write-Host "Run: cargo run -p gridbox-cli"
