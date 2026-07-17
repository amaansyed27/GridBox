# GridBox architecture

## Design principles

1. **Local-first:** model inference, analysis, cache and application state remain on the device.
2. **Deterministic before generative:** the local LLM explains structured calculations; it is not the source of lap times or telemetry.
3. **Provider isolation:** FastF1, Jolpica and Ollama are accessed through separate crates or processes.
4. **Failure containment:** a Python or data-provider failure must not crash the terminal UI.
5. **Modularity:** each crate owns one clear responsibility. `main.rs` only bootstraps the application.

## Rust workspace

| Crate | Responsibility |
|---|---|
| `gridbox-cli` | Clap commands, configuration and service wiring |
| `gridbox-tui` | Ratatui rendering, keyboard input, task dispatch and local simulation |
| `gridbox-models` | Provider-independent domain models |
| `gridbox-storage` | Config paths, TOML loading and local application directories |
| `gridbox-jolpica` | Historical and current season schedule client |
| `gridbox-analysis` | Deterministic strategy signals and LLM context |
| `gridbox-agent` | Local Ollama health checks and chat requests |
| `gridbox-fastf1-client` | NDJSON process protocol for the Python worker |

## Python worker

The FastF1 ecosystem remains in Python. Rust launches:

```text
uv run python -m gridbox_fastf1
```

The process reads one JSON request from standard input and returns one JSON response. Each handler lives in its own module:

- `handlers/sessions.py`
- `handlers/compare.py`
- `handlers/telemetry.py`

A worker crash or missing dependency becomes a structured error in the TUI.

## Completed-session flow

```text
FastF1
    ↓
Python handler modules
    ↓ NDJSON request/response
Rust gridbox-fastf1-client
    ↓
provider-independent analysis result
    ├── engineer conversation
    ├── lap and telemetry comparisons
    └── future exports and local warehouse
```

## Local demo flow

```text
Typed local session generator
    ↓
provider-independent LiveSnapshot
    ├── timing tower
    ├── race-control panel
    ├── weather panel
    ├── deterministic strategy analysis
    └── compact context for the local LLM
```

The demo generator is synthetic, continuously changing and explicitly labeled. It exercises the actual TUI and analysis paths without presenting generated data as a real Formula 1 session.

## Trust boundary

The local model must not be treated as a telemetry calculator. `gridbox-analysis` creates strategy signals first. The agent receives those signals plus normalized data and is instructed to identify missing or stale information.

## Planned extensions

- Persistent FastF1 worker pool.
- DuckDB and Parquet telemetry warehouse.
- Unicode/Braille telemetry and track-map widgets.
- Replay engine using locally stored session artifacts.
- Monte Carlo pit-window and safety-car simulations.
- Plugin SDK for other motorsport series and authorized data sources.
