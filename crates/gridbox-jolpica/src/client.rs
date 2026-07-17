use crate::models::RootResponse;
use gridbox_models::RaceEvent;
use reqwest::Client;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JolpicaError {
    #[error("Jolpica request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Jolpica returned HTTP {status}: {body}")]
    Http { status: u16, body: String },
}

#[derive(Debug, Clone)]
pub struct JolpicaClient {
    client: Client,
    base_url: String,
}

impl Default for JolpicaClient {
    fn default() -> Self {
        Self::new("https://api.jolpi.ca/ergast/f1")
    }
}

impl JolpicaClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.into().trim_end_matches('/').to_string(),
        }
    }

    pub async fn schedule(&self, year: u16) -> Result<Vec<RaceEvent>, JolpicaError> {
        let url = format!("{}/{}.json?limit=100", self.base_url, year);
        let response = self.client.get(url).send().await?;
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(JolpicaError::Http {
                status: status.as_u16(),
                body,
            });
        }

        let root: RootResponse = response.json().await?;
        Ok(root
            .mr_data
            .race_table
            .races
            .into_iter()
            .map(|race| RaceEvent {
                season: race.season.parse().unwrap_or(year),
                round: race.round.parse().unwrap_or_default(),
                race_name: race.race_name,
                circuit_name: race.circuit.circuit_name,
                locality: race.circuit.location.locality,
                country: race.circuit.location.country,
                date: race.date,
                time: race.time,
            })
            .collect())
    }
}
