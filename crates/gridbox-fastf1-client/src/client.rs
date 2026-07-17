use crate::{FastF1Request, FastF1Response};
use anyhow::{anyhow, Context, Result};
use serde_json::{json, Value};
use std::{path::PathBuf, process::Stdio};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::Command,
};

#[derive(Debug, Clone)]
pub struct FastF1Client {
    python_command: String,
    module: String,
    python_root: PathBuf,
}

impl FastF1Client {
    pub fn new(
        python_command: impl Into<String>,
        module: impl Into<String>,
        python_root: impl Into<PathBuf>,
    ) -> Self {
        Self {
            python_command: python_command.into(),
            module: module.into(),
            python_root: python_root.into(),
        }
    }

    pub async fn ping(&self) -> Result<Value> {
        self.request("ping", json!({})).await
    }

    pub async fn session_summary(&self, year: u16, event: &str, session: &str) -> Result<Value> {
        self.request(
            "session_summary",
            json!({"year": year, "event": event, "session": session}),
        )
        .await
    }

    pub async fn compare_laps(
        &self,
        year: u16,
        event: &str,
        session: &str,
        drivers: &[String],
    ) -> Result<Value> {
        self.request(
            "compare_laps",
            json!({
                "year": year,
                "event": event,
                "session": session,
                "drivers": drivers,
            }),
        )
        .await
    }

    pub async fn telemetry(
        &self,
        year: u16,
        event: &str,
        session: &str,
        driver: &str,
    ) -> Result<Value> {
        self.request(
            "telemetry",
            json!({
                "year": year,
                "event": event,
                "session": session,
                "driver": driver,
            }),
        )
        .await
    }

    pub async fn request(&self, method: &str, params: Value) -> Result<Value> {
        let id = format!("{}-{}", method, std::process::id());
        let request = FastF1Request {
            id: id.clone(),
            method: method.to_string(),
            params,
        };
        let encoded = serde_json::to_string(&request)?;

        let mut command = self.worker_command();
        command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = command.spawn().with_context(|| {
            format!(
                "failed to start '{}' for the FastF1 worker",
                self.python_command
            )
        })?;

        let mut stdin = child
            .stdin
            .take()
            .context("FastF1 worker stdin unavailable")?;
        stdin.write_all(encoded.as_bytes()).await?;
        stdin.write_all(b"\n").await?;
        stdin.shutdown().await?;

        let stdout = child
            .stdout
            .take()
            .context("FastF1 worker stdout unavailable")?;
        let mut lines = BufReader::new(stdout).lines();
        let response_line = lines
            .next_line()
            .await?
            .context("FastF1 worker returned no response")?;

        let output = child.wait_with_output().await?;
        if !output.status.success() && response_line.trim().is_empty() {
            return Err(anyhow!(
                "FastF1 worker failed: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ));
        }

        let response: FastF1Response = serde_json::from_str(&response_line)
            .with_context(|| format!("invalid FastF1 response: {response_line}"))?;
        if response.id != id {
            return Err(anyhow!("FastF1 response id mismatch"));
        }
        if !response.ok {
            return Err(anyhow!(
                "FastF1 worker error: {}",
                response.error.unwrap_or_else(|| "unknown error".into())
            ));
        }
        Ok(response.result)
    }

    fn worker_command(&self) -> Command {
        let mut command = Command::new(&self.python_command);
        if is_uv_command(&self.python_command) {
            command.arg("run").arg("python");
        }
        command
            .arg("-m")
            .arg(&self.module)
            .env("PYTHONPATH", &self.python_root);
        command
    }
}

fn is_uv_command(command: &str) -> bool {
    let executable = command.rsplit(['/', '\\']).next().unwrap_or(command);
    let executable = executable.strip_suffix(".exe").unwrap_or(executable);
    executable.eq_ignore_ascii_case("uv")
}

#[cfg(test)]
mod tests {
    use super::is_uv_command;

    #[test]
    fn detects_cross_platform_uv_launchers() {
        assert!(is_uv_command("uv"));
        assert!(is_uv_command("/usr/local/bin/uv"));
        assert!(is_uv_command("C:\\Tools\\uv.exe"));
        assert!(!is_uv_command("python"));
    }
}
