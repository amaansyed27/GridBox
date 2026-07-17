use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub session_key: i64,
    pub meeting_key: i64,
    pub session_name: String,
    pub session_type: String,
    pub country_name: String,
    pub location: String,
    pub circuit_short_name: String,
    pub year: i32,
    pub date_start: DateTime<Utc>,
    pub date_end: DateTime<Utc>,
}

impl SessionInfo {
    pub fn title(&self) -> String {
        let place = if self.location.is_empty() {
            self.country_name.as_str()
        } else {
            self.location.as_str()
        };
        format!("{} — {}", place, self.session_name)
    }

    pub fn is_live_at(&self, now: DateTime<Utc>) -> bool {
        let early_window = self.date_start - Duration::minutes(20);
        let late_window = self.date_end + Duration::minutes(45);
        now >= early_window && now <= late_window
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DriverSnapshot {
    pub driver_number: u32,
    pub acronym: String,
    pub full_name: String,
    pub team_name: String,
    pub team_colour: Option<String>,
    pub position: Option<u32>,
    pub gap_to_leader: Option<String>,
    pub interval: Option<String>,
    pub lap_number: Option<u32>,
    pub last_lap_duration: Option<f64>,
    pub compound: Option<String>,
    pub tyre_age: Option<u32>,
}

impl DriverSnapshot {
    pub fn display_name(&self) -> &str {
        if self.acronym.is_empty() {
            self.full_name.as_str()
        } else {
            self.acronym.as_str()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaceControlEvent {
    pub date: DateTime<Utc>,
    pub category: String,
    pub flag: Option<String>,
    pub message: String,
    pub driver_number: Option<u32>,
    pub lap_number: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherSnapshot {
    pub date: DateTime<Utc>,
    pub air_temperature: Option<f64>,
    pub track_temperature: Option<f64>,
    pub humidity: Option<f64>,
    pub rainfall: Option<bool>,
    pub wind_speed: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveSnapshot {
    pub session: SessionInfo,
    pub drivers: Vec<DriverSnapshot>,
    pub race_control: Vec<RaceControlEvent>,
    pub weather: Option<WeatherSnapshot>,
    pub fetched_at: DateTime<Utc>,
    pub source: String,
}

impl LiveSnapshot {
    pub fn is_stale(&self, now: DateTime<Utc>, threshold_seconds: i64) -> bool {
        now.signed_duration_since(self.fetched_at).num_seconds() > threshold_seconds
    }

    pub fn sorted_drivers(&self) -> Vec<&DriverSnapshot> {
        let mut drivers: Vec<_> = self.drivers.iter().collect();
        drivers.sort_by_key(|driver| driver.position.unwrap_or(u32::MAX));
        drivers
    }

    pub fn driver(&self, number: u32) -> Option<&DriverSnapshot> {
        self.drivers
            .iter()
            .find(|driver| driver.driver_number == number)
    }
}
