use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "gridbox",
    version,
    about = "Local-first Formula 1 telemetry, live timing and race-engineering TUI",
    long_about = None
)]
pub struct Cli {
    #[arg(long, global = true, value_name = "PATH")]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Start directly in live-session mode using the configured provider.
    Live,
    /// Run a fully local moving live-session simulation with no API key.
    DemoLive,
    /// Check OpenF1, Ollama, FastF1 and local storage.
    Doctor,
    /// Print a season schedule from Jolpica.
    Schedule { year: u16 },
    /// Load a FastF1 session summary or compare fastest laps.
    Analyze {
        year: u16,
        event: String,
        session: String,
        #[arg(short, long, num_args = 0..)]
        drivers: Vec<String>,
    },
    /// Print the resolved configuration path.
    ConfigPath,
}
