use crate::AppPaths;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub llm: LlmConfig,
    pub openf1: OpenF1Config,
    pub fastf1: FastF1Config,
    pub ui: UiConfig,
}

impl Config {
    pub fn load(override_path: Option<&Path>) -> Result<(Self, AppPaths)> {
        let paths = AppPaths::discover()?;
        paths.ensure()?;
        let config_path = paths.config_path_or(override_path);

        let mut config = if config_path.exists() {
            let raw = std::fs::read_to_string(config_path)
                .with_context(|| format!("failed to read {}", config_path.display()))?;
            toml::from_str(&raw)
                .with_context(|| format!("failed to parse {}", config_path.display()))?
        } else {
            Self::default()
        };

        config.apply_environment();
        Ok((config, paths))
    }

    fn apply_environment(&mut self) {
        if let Ok(value) = std::env::var("GRIDBOX_OLLAMA_URL") {
            self.llm.base_url = value;
        }
        if let Ok(value) = std::env::var("GRIDBOX_MODEL") {
            self.llm.model = value;
        }
        if let Ok(value) = std::env::var("OPENF1_TOKEN") {
            if !value.trim().is_empty() {
                self.openf1.token = Some(value);
            }
        }
        if let Ok(value) = std::env::var("GRIDBOX_PYTHON") {
            self.fastf1.python_command = value;
        }
        if let Ok(value) = std::env::var("GRIDBOX_PYTHON_ROOT") {
            self.fastf1.python_root = PathBuf::from(value);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LlmConfig {
    pub enabled: bool,
    pub provider: String,
    pub base_url: String,
    pub model: String,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            provider: "ollama".to_string(),
            base_url: "http://127.0.0.1:11434".to_string(),
            model: "qwen3.5:4b".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct OpenF1Config {
    pub base_url: String,
    pub token: Option<String>,
    pub poll_interval_secs: u64,
    pub auto_detect: bool,
    pub record_live_sessions: bool,
}

impl Default for OpenF1Config {
    fn default() -> Self {
        Self {
            base_url: "https://api.openf1.org/v1".to_string(),
            token: None,
            poll_interval_secs: 5,
            auto_detect: true,
            record_live_sessions: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FastF1Config {
    pub enabled: bool,
    pub python_command: String,
    pub module: String,
    pub python_root: PathBuf,
}

impl Default for FastF1Config {
    fn default() -> Self {
        Self {
            enabled: true,
            python_command: "uv".to_string(),
            module: "gridbox_fastf1".to_string(),
            python_root: PathBuf::from("python"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct UiConfig {
    pub tick_rate_ms: u64,
    pub compact_logo: bool,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            tick_rate_ms: 100,
            compact_logo: false,
        }
    }
}
