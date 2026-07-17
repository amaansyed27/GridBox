use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RaceEvent {
    pub season: u16,
    pub round: u16,
    pub race_name: String,
    pub circuit_name: String,
    pub locality: String,
    pub country: String,
    pub date: String,
    pub time: Option<String>,
}

impl RaceEvent {
    pub fn display_time(&self) -> String {
        match &self.time {
            Some(time) if !time.is_empty() => format!("{} {}", self.date, time),
            _ => self.date.clone(),
        }
    }
}
