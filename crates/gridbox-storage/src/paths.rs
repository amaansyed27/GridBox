use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct AppPaths {
    pub config_file: PathBuf,
    pub data_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub recordings_dir: PathBuf,
    pub logs_dir: PathBuf,
}

impl AppPaths {
    pub fn discover() -> Result<Self> {
        let project = ProjectDirs::from("com", "dawnlightlabs", "GridBox")
            .context("unable to determine a platform data directory")?;

        let data_dir = project.data_dir().to_path_buf();
        let cache_dir = project.cache_dir().to_path_buf();
        let config_dir = project.config_dir().to_path_buf();

        Ok(Self {
            config_file: config_dir.join("config.toml"),
            recordings_dir: data_dir.join("recordings"),
            logs_dir: data_dir.join("logs"),
            data_dir,
            cache_dir,
        })
    }

    pub fn ensure(&self) -> Result<()> {
        for directory in [
            self.config_file.parent(),
            Some(self.data_dir.as_path()),
            Some(self.cache_dir.as_path()),
            Some(self.recordings_dir.as_path()),
            Some(self.logs_dir.as_path()),
        ]
        .into_iter()
        .flatten()
        {
            std::fs::create_dir_all(directory)
                .with_context(|| format!("failed to create {}", directory.display()))?;
        }
        Ok(())
    }

    pub fn config_path_or<'a>(&'a self, override_path: Option<&'a Path>) -> &'a Path {
        override_path.unwrap_or(self.config_file.as_path())
    }
}
