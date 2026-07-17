use anyhow::{Context, Result};
use gridbox_models::LiveSnapshot;
use std::{
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct LiveRecorder {
    root: PathBuf,
}

impl LiveRecorder {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn record_snapshot(&self, snapshot: &LiveSnapshot) -> Result<PathBuf> {
        let slug = slugify(&format!(
            "{}-{}-{}",
            snapshot.session.year, snapshot.session.location, snapshot.session.session_name
        ));
        let session_dir = self.root.join(slug);
        std::fs::create_dir_all(&session_dir)
            .with_context(|| format!("failed to create {}", session_dir.display()))?;

        let path = session_dir.join("snapshots.jsonl");
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .with_context(|| format!("failed to open {}", path.display()))?;

        serde_json::to_writer(&mut file, snapshot).context("failed to serialize live snapshot")?;
        file.write_all(b"\n")
            .context("failed to terminate live snapshot record")?;
        Ok(path)
    }

    pub fn root(&self) -> &Path {
        &self.root
    }
}

fn slugify(value: &str) -> String {
    value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::slugify;

    #[test]
    fn creates_stable_recording_slug() {
        assert_eq!(slugify("2026-British GP-Race"), "2026-british-gp-race");
    }
}
