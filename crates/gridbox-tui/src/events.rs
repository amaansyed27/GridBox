use gridbox_models::{LiveSnapshot, RaceEvent};
use serde_json::Value;

#[derive(Debug)]
pub enum AppEvent {
    LiveLoaded(Box<LiveSnapshot>),
    AiCompleted(Result<String, String>),
    ScheduleLoaded(Result<Vec<RaceEvent>, String>),
    FastF1Completed(Result<Value, String>),
}

#[derive(Debug)]
pub enum AppAction {
    None,
    Quit,
    AskAi(String),
    LoadSchedule(u16),
    LoadSession {
        year: u16,
        event: String,
        session: String,
    },
    Compare {
        year: u16,
        event: String,
        session: String,
        drivers: Vec<String>,
    },
}
