# GridBox

```text
  ██████╗ ██████╗ ██╗██████╗ ██████╗  ██████╗ ██╗  ██╗
 ██╔════╝ ██╔══██╗██║██╔══██╗██╔══██╗██╔═══██╗╚██╗██╔╝
 ██║  ███╗██████╔╝██║██║  ██║██████╔╝██║   ██║ ╚███╔╝
 ██║   ██║██╔══██╗██║██║  ██║██╔══██╗██║   ██║ ██╔██╗
 ╚██████╔╝██║  ██║██║██████╔╝██████╔╝╚██████╔╝██╔╝ ██╗
  ╚═════╝ ╚═╝  ╚═╝╚═╝╚═════╝ ╚═════╝  ╚═════╝ ╚═╝  ╚═╝
```

**GridBox is a local-first Formula 1 telemetry, race-analysis and strategy TUI.** It combines completed-session FastF1 data, historical schedules, deterministic analysis, a local Ollama engineer and a fully local moving session simulator in one terminal application.

GridBox is unofficial and is not associated with Formula One Licensing B.V., the FIA, FastF1, Jolpica or Ollama.

## Current capabilities

- Full-screen Rust TUI built with Ratatui and Crossterm.
- FastF1 session summaries, fastest-lap comparisons and telemetry extraction.
- Jolpica season schedules.
- Deterministic gap, tyre-age, flag and strategy signals.
- Local Ollama engineer receiving structured GridBox context.
- Fully local `demo-live` mode for testing timing, weather, race control and strategy views.
- Modular Rust workspace with an isolated Python FastF1 worker.
- ASCII branding, setup scripts, tests and CI.

## Architecture

```text
┌───────────────────────────────────────────────────────────────┐
│ gridbox-cli                                                   │
│ Clap commands + application bootstrap                         │
├───────────────────────────────────────────────────────────────┤
│ gridbox-tui                                                   │
│ Ratatui views, input, async dispatch and local demo stream     │
├──────────────┬───────────────┬────────────────────────────────┤
│ gridbox-agent│ gridbox-jolpica│ gridbox-analysis               │
│ local Ollama │ schedules      │ deterministic strategy/context │
├──────────────┴───────────────┴────────────────────────────────┤
│ gridbox-fastf1-client  ⇄  Python gridbox_fastf1 worker        │
├───────────────────────────────────────────────────────────────┤
│ gridbox-storage + gridbox-models                               │
└───────────────────────────────────────────────────────────────┘
```

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for module boundaries.

## Requirements

- Rust stable, 1.81 or newer.
- Python 3.11 or 3.12.
- `uv` for the Python environment.
- Ollama for local AI features.
- Internet access when downloading FastF1 or Jolpica data.

## Setup

### Windows PowerShell

```powershell
winget install Rustlang.Rustup
winget install astral-sh.uv
winget install Ollama.Ollama

uv sync --extra dev
ollama pull qwen3.5:4b
cargo run -p gridbox-cli -- doctor
cargo run -p gridbox-cli
```

Or:

```powershell
./scripts/bootstrap.ps1
```

### Linux or macOS

```bash
uv sync --extra dev
ollama pull qwen3.5:4b
cargo run -p gridbox-cli -- doctor
cargo run -p gridbox-cli
```

Or:

```bash
./scripts/bootstrap.sh
```

## CLI

```text
gridbox                         Start the interactive workspace
gridbox demo-live               Run the fully local moving session simulator
gridbox doctor                  Check storage, Ollama and FastF1
gridbox schedule 2026           Print a Jolpica season schedule
gridbox analyze 2026 Monaco Q   Print a FastF1 session summary
gridbox analyze 2026 Monaco Q --drivers NOR VER
gridbox config-path             Print the platform config path
```

During local development, prefix commands with:

```text
cargo run -p gridbox-cli --
```

## TUI commands

```text
/driver 4
/schedule 2026
/session 2026 Monaco Q
/compare 2026 Monaco Q NOR VER
/model qwen3.5:4b
/clear
/quit
```

Input without a leading slash is sent to the configured local Ollama model. The model receives compact structured context rather than raw telemetry frames.

## Data modes

### Completed-session analysis

FastF1 is the primary race-data engine. GridBox uses it for session loading, lap comparisons and telemetry. Availability follows the upstream FastF1 data source and cache.

### Local moving simulation

```bash
cargo run -p gridbox-cli -- demo-live
```

This mode requires no account, token, paid API or internet connection. It feeds continuously changing typed `LiveSnapshot` values through the same TUI and strategy paths used by session-style data.

### Schedules

Jolpica provides season calendars and historical schedule metadata.

## Configuration

Copy `config.example.toml` to the location printed by:

```bash
cargo run -p gridbox-cli -- config-path
```

Environment overrides:

```text
GRIDBOX_OLLAMA_URL
GRIDBOX_MODEL
GRIDBOX_PYTHON
GRIDBOX_PYTHON_ROOT
GRIDBOX_FASTF1_CACHE
```

## Local data

GridBox uses the operating system's application directories for configuration, FastF1 cache, logs and future exports. No GridBox analytics or conversation data is sent to Dawnlight Labs.

## Development

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
uv run ruff check python
uv run pytest
```

The repository deliberately avoids one-file implementations. New providers, views and analysis features must remain behind clear module boundaries.

## Current limitations

- FastF1 data is intended for completed or upstream-available sessions rather than guaranteed real-time coverage.
- The local moving session is synthetic and clearly marked as a demo.
- Strategy output is heuristic, not a substitute for team-grade simulation.
- FastF1 event names in TUI slash commands currently use one token, such as `AbuDhabi`.
- Python is spawned per FastF1 request; a persistent worker pool is planned.

## License

MIT. Data providers retain their own terms, licenses and trademarks.
