# GridBox commands

## Executable commands

### `gridbox`

Starts the TUI and checks the latest OpenF1 session when auto-detection is enabled.

### `gridbox live`

Starts the TUI on the live timing view and continues polling.

### `gridbox doctor`

Checks:

- platform data directories,
- OpenF1 connectivity,
- Ollama connectivity and installed models,
- Python worker startup and FastF1 availability.

### `gridbox schedule <year>`

Prints the Jolpica season schedule.

### `gridbox analyze <year> <event> <session>`

Loads a FastF1 session summary.

Add `--drivers` to compare fastest laps:

```text
gridbox analyze 2025 Monaco Q --drivers NOR VER LEC
```

## Interactive commands

| Command | Action |
|---|---|
| `/live` | Load latest/active OpenF1 data |
| `/refresh` | Refresh the snapshot immediately |
| `/driver 4` | Focus car number 4 |
| `/schedule 2026` | Load schedule into History |
| `/session 2025 Monaco Q` | Load FastF1 session summary |
| `/compare 2025 Monaco Q NOR VER` | Compare fastest laps |
| `/model qwen3.5:4b` | Select another installed Ollama model |
| `/clear` | Clear the engineer conversation |
| `/help` | Open command help |
| `/quit` | Exit |
