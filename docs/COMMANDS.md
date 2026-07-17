# GridBox commands

## Executable commands

### `gridbox`

Starts the interactive workspace for FastF1 analysis, schedules and the local engineer.

### `gridbox demo-live`

Runs the fully local moving session simulator. It exercises the timing tower, race control, weather, driver focus, strategy signals and local AI context without an API key or network connection.

### `gridbox doctor`

Checks:

- platform data directories,
- Ollama connectivity and installed models,
- Python worker startup and FastF1 availability.

### `gridbox schedule <year>`

Prints the Jolpica season schedule.

### `gridbox analyze <year> <event> <session>`

Loads a FastF1 session summary.

Add `--drivers` to compare fastest laps:

```text
gridbox analyze 2026 Monaco Q --drivers NOR VER LEC
```

### `gridbox config-path`

Prints the resolved platform configuration path.

## Interactive commands

| Command | Action |
|---|---|
| `/driver 4` | Focus car number 4 in local demo analysis |
| `/schedule 2026` | Load a season schedule into History |
| `/session 2026 Monaco Q` | Load a completed FastF1 session summary |
| `/compare 2026 Monaco Q NOR VER` | Compare fastest laps through FastF1 |
| `/model qwen3.5:4b` | Select another installed Ollama model |
| `/clear` | Clear the engineer conversation |
| `/help` | Open command help |
| `/quit` | Exit |
