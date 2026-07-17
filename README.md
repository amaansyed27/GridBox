# GridBox

```text
  ██████╗ ██████╗ ██╗██████╗ ██████╗  ██████╗ ██╗  ██╗
 ██╔════╝ ██╔══██╗██║██╔══██╗██╔══██╗██╔═══██╗╚██╗██╔╝
 ██║  ███╗██████╔╝██║██║  ██║██████╔╝██║   ██║ ╚███╔╝
 ██║   ██║██╔══██╗██║██║  ██║██╔══██╗██║   ██║ ██╔██╗
 ╚██████╔╝██║  ██║██║██████╔╝██████╔╝╚██████╔╝██╔╝ ██╗
  ╚═════╝ ╚═╝  ╚═╝╚═╝╚═════╝ ╚═════╝  ╚═════╝ ╚═╝  ╚═╝
```

**GridBox is a local-first Formula 1 race-engineering TUI.** It combines live-session data, historical schedules, FastF1 telemetry analysis, deterministic strategy signals and a local Ollama-powered engineer inside one terminal application.

GridBox is unofficial and is not associated with Formula One Licensing B.V., the FIA, OpenF1, FastF1 or Jolpica.

## What works in the first end-to-end release

- Full-screen Rust TUI built with Ratatui and Crossterm.
- Automatic detection of the latest or active OpenF1 session.
- Live timing tower assembled from positions, intervals, laps and stints.
- Race-control and weather panels.
- Deterministic DRS, gap, tyre-age and flag strategy signals.
- Local live-session recording as JSON Lines.
- Historical season schedules through Jolpica.
- FastF1 session summaries, fastest-lap comparisons and telemetry extraction.
- Local Ollama engineer that receives only structured GridBox context.
- CLI commands for live mode, diagnostics, schedules and FastF1 analysis.
- Modular Rust workspace and isolated Python worker.

## Architecture

```text
┌───────────────────────────────────────────────────────────────┐
│ gridbox-cli                                                   │
│ Clap commands + application bootstrap                         │
├───────────────────────────────────────────────────────────────┤
│ gridbox-tui                                                   │
│ Ratatui views, keyboard input, event loop, async task routing  │
├──────────────┬───────────────┬───────────────┬────────────────┤
│ gridbox-agent│ gridbox-openf1│ gridbox-jolpica│ gridbox-analysis│
│ Ollama local │ live/session  │ schedules      │ deterministic  │
│ chat         │ data          │ and history    │ strategy       │
├──────────────┴───────────────┴───────────────┴────────────────┤
│ gridbox-fastf1-client  ⇄  Python gridbox_fastf1 worker        │
├───────────────────────────────────────────────────────────────┤
│ gridbox-storage + gridbox-models                               │
└───────────────────────────────────────────────────────────────┘
```

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for module boundaries and data flow.

## Requirements

- Rust stable, 1.81 or newer.
- Python 3.11 or 3.12.
- `uv` for the FastF1 worker environment.
- Ollama for local AI features.
- Internet access only when downloading F1 data or consuming a live session.

Real-time OpenF1 data may require a paid OpenF1 subscription and token. Historical OpenF1 data from 2023 onward does not require authentication.

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

Or run the bootstrap helper:

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
gridbox live                    Open directly in live-session mode
gridbox doctor                  Check storage, OpenF1, Ollama and FastF1
gridbox schedule 2026           Print a season schedule
gridbox analyze 2025 Monaco Q   Print a FastF1 session summary
gridbox analyze 2025 Monaco Q --drivers NOR VER
gridbox config-path             Print the platform config path
```

## TUI commands

```text
/live
/refresh
/driver 4
/schedule 2026
/session 2025 Monaco Q
/compare 2025 Monaco Q NOR VER
/model qwen3.5:4b
/clear
/quit
```

Any input without a leading slash is sent to the configured local Ollama model. The model receives a compact structured snapshot; it does not receive raw million-point telemetry or any cloud-hosted context.

## Configuration

Copy `config.example.toml` to the location printed by:

```bash
gridbox config-path
```

Environment overrides:

```text
GRIDBOX_OLLAMA_URL
GRIDBOX_MODEL
OPENF1_TOKEN
GRIDBOX_PYTHON
GRIDBOX_PYTHON_ROOT
GRIDBOX_FASTF1_CACHE
```

Tokens are currently accepted through the environment or config file. OS credential-store integration is planned before a stable release; environment variables are recommended in the meantime.

## Local data

GridBox uses the operating system's application directories. It stores:

- configuration,
- FastF1 cache,
- live JSONL recordings,
- logs and later exports.

No GridBox analytics or conversation data is sent to Dawnlight Labs.

## Development

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
uv run ruff check python
uv run pytest
```

The repository deliberately avoids one-file implementations. New providers, views and analysis features should be added behind existing module boundaries.

## Current limitations

- OpenF1 live access depends on the account and plan associated with the supplied token.
- Live polling currently uses REST snapshots; authenticated WebSocket/MQTT transports are a later milestone.
- Strategy output is heuristic, not a substitute for team-grade simulation.
- FastF1 event names in TUI slash commands currently use one token, such as `AbuDhabi`.
- Python is spawned per FastF1 request in this release; a persistent worker pool is planned.

## License

MIT. Data providers retain their own terms, licenses and trademarks.
