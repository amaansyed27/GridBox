# Contributing

GridBox is Rust-first and modular. Keep provider-specific response models inside provider crates and expose only `gridbox-models` types to the rest of the application.

Before submitting changes:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
uv run ruff check python
uv run pytest
```

Do not:

- add unrelated logic to `main.rs`,
- make the TUI depend directly on FastF1 Python objects,
- let an LLM generate authoritative telemetry values,
- hard-code driver lineups or team names,
- commit API tokens or local recordings.
