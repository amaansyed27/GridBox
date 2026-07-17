use crate::{normalize::value_to_display, OpenF1Error};
use chrono::{DateTime, Utc};
use gridbox_models::{
    DriverSnapshot, LiveSnapshot, RaceControlEvent, SessionInfo, WeatherSnapshot,
};
use reqwest::{Client, Response};
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct OpenF1Client {
    client: Client,
    base_url: String,
    token: Option<String>,
}

impl OpenF1Client {
    pub fn new(base_url: impl Into<String>, token: Option<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.into().trim_end_matches('/').to_string(),
            token: token.filter(|token| !token.trim().is_empty()),
        }
    }

    pub async fn health(&self) -> Result<SessionInfo, OpenF1Error> {
        self.latest_session().await
    }

    pub async fn latest_session(&self) -> Result<SessionInfo, OpenF1Error> {
        let sessions: Vec<RawSession> = self.get("sessions?session_key=latest").await?;
        sessions
            .into_iter()
            .max_by_key(|session| session.date_start)
            .map(Into::into)
            .ok_or(OpenF1Error::NoSession)
    }

    pub async fn snapshot_latest(&self) -> Result<LiveSnapshot, OpenF1Error> {
        let session = self.latest_session().await?;
        let key = session.session_key;

        let (drivers, positions, intervals, laps, stints, race_control, weather) = tokio::join!(
            self.get_or_default::<RawDriver>(format!("drivers?session_key={key}")),
            self.get_or_default::<RawPosition>(format!("position?session_key={key}")),
            self.get_or_default::<RawInterval>(format!("intervals?session_key={key}")),
            self.get_or_default::<RawLap>(format!("laps?session_key={key}")),
            self.get_or_default::<RawStint>(format!("stints?session_key={key}")),
            self.get_or_default::<RawRaceControl>(format!("race_control?session_key={key}")),
            self.get_or_default::<RawWeather>(format!("weather?session_key={key}")),
        );

        let mut by_number: HashMap<u32, DriverSnapshot> = drivers
            .into_iter()
            .map(|driver| {
                let number = driver.driver_number;
                (
                    number,
                    DriverSnapshot {
                        driver_number: number,
                        acronym: driver.name_acronym,
                        full_name: driver.full_name,
                        team_name: driver.team_name,
                        team_colour: driver.team_colour,
                        ..DriverSnapshot::default()
                    },
                )
            })
            .collect();

        for position in latest_by_driver(positions, |item| item.driver_number, |item| item.date) {
            by_number
                .entry(position.driver_number)
                .or_insert_with(|| placeholder_driver(position.driver_number))
                .position = Some(position.position);
        }

        for interval in latest_by_driver(intervals, |item| item.driver_number, |item| item.date) {
            let driver = by_number
                .entry(interval.driver_number)
                .or_insert_with(|| placeholder_driver(interval.driver_number));
            driver.gap_to_leader = value_to_display(&interval.gap_to_leader);
            driver.interval = value_to_display(&interval.interval);
        }

        for lap in latest_by_driver(laps, |item| item.driver_number, |item| item.date_start) {
            let driver = by_number
                .entry(lap.driver_number)
                .or_insert_with(|| placeholder_driver(lap.driver_number));
            driver.lap_number = Some(lap.lap_number);
            driver.last_lap_duration = lap.lap_duration;
        }

        for stint in latest_stints(stints) {
            let driver = by_number
                .entry(stint.driver_number)
                .or_insert_with(|| placeholder_driver(stint.driver_number));
            driver.compound = stint.compound;
            let completed_laps = stint
                .lap_end
                .unwrap_or(stint.lap_start)
                .saturating_sub(stint.lap_start);
            driver.tyre_age = Some(stint.tyre_age_at_start.unwrap_or(0) + completed_laps);
        }

        let mut driver_snapshots: Vec<_> = by_number.into_values().collect();
        driver_snapshots.sort_by_key(|driver| driver.position.unwrap_or(u32::MAX));

        let mut control_events: Vec<RaceControlEvent> = race_control
            .into_iter()
            .map(|event| RaceControlEvent {
                date: event.date,
                category: event.category,
                flag: event.flag,
                message: event.message,
                driver_number: event.driver_number,
                lap_number: event.lap_number,
            })
            .collect();
        control_events.sort_by_key(|event| event.date);
        if control_events.len() > 20 {
            control_events.drain(0..control_events.len() - 20);
        }

        let weather = weather
            .into_iter()
            .max_by_key(|sample| sample.date)
            .map(|sample| WeatherSnapshot {
                date: sample.date,
                air_temperature: sample.air_temperature,
                track_temperature: sample.track_temperature,
                humidity: sample.humidity,
                rainfall: sample.rainfall,
                wind_speed: sample.wind_speed,
            });

        Ok(LiveSnapshot {
            session,
            drivers: driver_snapshots,
            race_control: control_events,
            weather,
            fetched_at: Utc::now(),
            source: "OpenF1".to_string(),
        })
    }

    async fn get_or_default<T>(&self, path: String) -> Vec<T>
    where
        T: DeserializeOwned,
    {
        self.get(&path).await.unwrap_or_default()
    }

    async fn get<T>(&self, path: &str) -> Result<T, OpenF1Error>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}/{}", self.base_url, path.trim_start_matches('/'));
        let mut request = self.client.get(url);
        if let Some(token) = &self.token {
            request = request.bearer_auth(token);
        }
        parse_response(request.send().await?).await
    }
}

async fn parse_response<T>(response: Response) -> Result<T, OpenF1Error>
where
    T: DeserializeOwned,
{
    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(OpenF1Error::Http {
            status: status.as_u16(),
            body,
        });
    }
    Ok(response.json().await?)
}

fn placeholder_driver(number: u32) -> DriverSnapshot {
    DriverSnapshot {
        driver_number: number,
        acronym: number.to_string(),
        full_name: format!("Car {number}"),
        ..DriverSnapshot::default()
    }
}

fn latest_by_driver<T, FNumber, FDate>(items: Vec<T>, number: FNumber, date: FDate) -> Vec<T>
where
    FNumber: Fn(&T) -> u32,
    FDate: Fn(&T) -> DateTime<Utc>,
{
    let mut latest: HashMap<u32, T> = HashMap::new();
    for item in items {
        let driver_number = number(&item);
        let replace = latest
            .get(&driver_number)
            .map(|existing| date(&item) >= date(existing))
            .unwrap_or(true);
        if replace {
            latest.insert(driver_number, item);
        }
    }
    latest.into_values().collect()
}

fn latest_stints(items: Vec<RawStint>) -> Vec<RawStint> {
    let mut latest: HashMap<u32, RawStint> = HashMap::new();
    for stint in items {
        let replace = latest
            .get(&stint.driver_number)
            .map(|current| {
                stint.lap_end.unwrap_or(stint.lap_start)
                    >= current.lap_end.unwrap_or(current.lap_start)
            })
            .unwrap_or(true);
        if replace {
            latest.insert(stint.driver_number, stint);
        }
    }
    latest.into_values().collect()
}

#[derive(Debug, Deserialize)]
struct RawSession {
    session_key: i64,
    meeting_key: i64,
    session_name: String,
    session_type: String,
    #[serde(default)]
    country_name: String,
    #[serde(default)]
    location: String,
    #[serde(default)]
    circuit_short_name: String,
    year: i32,
    date_start: DateTime<Utc>,
    date_end: DateTime<Utc>,
}

impl From<RawSession> for SessionInfo {
    fn from(value: RawSession) -> Self {
        Self {
            session_key: value.session_key,
            meeting_key: value.meeting_key,
            session_name: value.session_name,
            session_type: value.session_type,
            country_name: value.country_name,
            location: value.location,
            circuit_short_name: value.circuit_short_name,
            year: value.year,
            date_start: value.date_start,
            date_end: value.date_end,
        }
    }
}

#[derive(Debug, Deserialize)]
struct RawDriver {
    driver_number: u32,
    #[serde(default)]
    name_acronym: String,
    #[serde(default)]
    full_name: String,
    #[serde(default)]
    team_name: String,
    team_colour: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RawPosition {
    date: DateTime<Utc>,
    driver_number: u32,
    position: u32,
}

#[derive(Debug, Deserialize)]
struct RawInterval {
    date: DateTime<Utc>,
    driver_number: u32,
    gap_to_leader: Option<Value>,
    interval: Option<Value>,
}

#[derive(Debug, Deserialize)]
struct RawLap {
    date_start: DateTime<Utc>,
    driver_number: u32,
    lap_number: u32,
    lap_duration: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct RawStint {
    driver_number: u32,
    lap_start: u32,
    lap_end: Option<u32>,
    compound: Option<String>,
    tyre_age_at_start: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct RawRaceControl {
    date: DateTime<Utc>,
    #[serde(default)]
    category: String,
    flag: Option<String>,
    #[serde(default)]
    message: String,
    driver_number: Option<u32>,
    lap_number: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct RawWeather {
    date: DateTime<Utc>,
    air_temperature: Option<f64>,
    track_temperature: Option<f64>,
    humidity: Option<f64>,
    rainfall: Option<bool>,
    wind_speed: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::latest_stints;

    #[test]
    fn keeps_latest_stint_per_driver() {
        let stints = vec![
            super::RawStint {
                driver_number: 4,
                lap_start: 1,
                lap_end: Some(20),
                compound: Some("MEDIUM".into()),
                tyre_age_at_start: Some(0),
            },
            super::RawStint {
                driver_number: 4,
                lap_start: 21,
                lap_end: Some(35),
                compound: Some("HARD".into()),
                tyre_age_at_start: Some(0),
            },
        ];
        let latest = latest_stints(stints);
        assert_eq!(latest.len(), 1);
        assert_eq!(latest[0].compound.as_deref(), Some("HARD"));
    }
}
