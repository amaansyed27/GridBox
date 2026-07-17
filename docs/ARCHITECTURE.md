# GridBox architecture

## Design principles

1. **Local-first:** model inference, analysis, recordings and application state remain on the device.
2. **Deterministic before generative:** the local LLM explains structured calculations; it is not the source of lap times or live gaps.
3. **Provider isolation:** OpenF1, Jolpica, FastF1 and Ollama are accessed through separate crates or processes.
4. **Failure containment:** a Python or data-provider failure must not crash the terminal UI.
5. **Modularity:** each crate owns one clear responsibility. `main.rs` only bootstraps the application.

## Rust workspace

| Crate | Responsibility |
|---|---|
| `gridbox-cli` | Clap commands, configuration and service wiring |
| `gridbox-tui` | Ratatui rendering, keyboard input and task dispatch |
| `gridbox-models` | Provider-independent domain models |
| `gridbox-storage` | Config paths, TOML loading and local recordings |
| `gridbox-openf1` | OpenF1 REST client and snapshot normalization |
| `gridbox-jolpica` | Historical schedule client |
| `gridbox-analysis` | Deterministic live strategy signals and LLM context |
| `gridbox-agent` | Local Ollama health checks and chat requests |
| `gridbox-fastf1-client` | NDJSON process protocol for the Python worker |

## Python worker

The FastF1 ecosystem remains in Python. Rust launches:

```text
python -m gridbox_fastf1
```

The process reads one JSON request from standard input and returns one JSON response. Each handler lives in its own module:

- `handlers/sessions.py`
- `handlers/compare.py`
- `handlers/telemetry.py`

A worker crash or missing dependency becomes a structured error in the TUI.

## Live flow

```text
OpenF1 REST
    ↓
gridbox-openf1 raw response models
    ↓
provider-independent LiveSnapshot
    ├── timing tower
    ├── race-control panel
    ├── JSONL recorder
    ├── deterministic strategy analysis
    └── compact context for the local LLM
```

The TUI polls only while live mode is active or the latest session falls inside its configured live window.

## Trust boundary

The local model must not be treated as a telemetry calculator. `gridbox-analysis` creates the strategy signals first. The agent receives those signals plus normalized live data and is instructed to identify missing or stale data.

## Planned extensions

- OpenF1 WebSocket and MQTT transports.
- Persistent FastF1 worker pool.
- DuckDB and Parquet telemetry warehouse.
- Unicode/Braille telemetry and track-map widgets.
- Replay engine using recorded session events.
- Monte Carlo pit-window and safety-car simulations.
- Plugin SDK for other motorsport series.
