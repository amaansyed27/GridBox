mod cli;
mod doctor;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command};
use gridbox_agent::OllamaAgent;
use gridbox_fastf1_client::FastF1Client;
use gridbox_jolpica::JolpicaClient;
use gridbox_openf1::OpenF1Client;
use gridbox_storage::{Config, LiveRecorder};
use gridbox_tui::{run_tui, AppServices, LaunchMode};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    init_tracing();

    let (config, paths) = Config::load(cli.config.as_deref())?;
    let openf1 = OpenF1Client::new(
        config.openf1.base_url.clone(),
        config.openf1.token.clone(),
    );
    let jolpica = JolpicaClient::default();
    let agent = OllamaAgent::new(config.llm.base_url.clone(), config.llm.model.clone());
    let fastf1 = FastF1Client::new(
        config.fastf1.python_command.clone(),
        config.fastf1.module.clone(),
        config.fastf1.python_root.clone(),
    );
    let recorder = LiveRecorder::new(paths.recordings_dir.clone());

    match cli.command {
        Some(Command::Doctor) => {
            let healthy = doctor::run(&paths, &openf1, &agent, &fastf1).await;
            if !healthy {
                std::process::exit(1);
            }
        }
        Some(Command::Schedule { year }) => {
            for race in jolpica.schedule(year).await? {
                println!(
                    "{:>2}. {:<28} {:<24} {}",
                    race.round,
                    race.race_name,
                    race.locality,
                    race.display_time()
                );
            }
        }
        Some(Command::Analyze {
            year,
            event,
            session,
            drivers,
        }) => {
            let result = if drivers.len() >= 2 {
                fastf1
                    .compare_laps(year, &event, &session, &drivers)
                    .await?
            } else {
                fastf1.session_summary(year, &event, &session).await?
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        Some(Command::ConfigPath) => println!("{}", paths.config_file.display()),
        Some(Command::Live) => {
            run_tui(
                config.clone(),
                build_services(&config, openf1, jolpica, agent, fastf1, recorder),
                LaunchMode::Live,
            )
            .await?;
        }
        None => {
            run_tui(
                config.clone(),
                build_services(&config, openf1, jolpica, agent, fastf1, recorder),
                LaunchMode::Auto,
            )
            .await?;
        }
    }

    Ok(())
}

fn build_services(
    config: &Config,
    openf1: OpenF1Client,
    jolpica: JolpicaClient,
    agent: OllamaAgent,
    fastf1: FastF1Client,
    recorder: LiveRecorder,
) -> AppServices {
    AppServices {
        openf1,
        jolpica,
        agent,
        fastf1,
        recorder,
        record_live_sessions: config.openf1.record_live_sessions,
    }
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"));
    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .try_init();
}
