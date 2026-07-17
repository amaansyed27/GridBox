use gridbox_models::{DriverSnapshot, LiveSnapshot};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StrategySeverity {
    Info,
    Watch,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyInsight {
    pub severity: StrategySeverity,
    pub title: String,
    pub detail: String,
}

pub fn analyze_live_strategy(snapshot: &LiveSnapshot) -> Vec<StrategyInsight> {
    let mut insights = Vec::new();
    let drivers = snapshot.sorted_drivers();

    for pair in drivers.windows(2) {
        let ahead = pair[0];
        let behind = pair[1];
        if let Some(interval) = numeric_gap(behind.interval.as_deref()) {
            if interval <= 1.0 {
                insights.push(StrategyInsight {
                    severity: StrategySeverity::Critical,
                    title: format!("{} is in DRS range", behind.display_name()),
                    detail: format!(
                        "{} is {:.3}s behind {}.",
                        behind.display_name(),
                        interval,
                        ahead.display_name()
                    ),
                });
            } else if interval <= 2.5 {
                insights.push(StrategyInsight {
                    severity: StrategySeverity::Watch,
                    title: format!("{} is closing", behind.display_name()),
                    detail: format!(
                        "The interval to {} is {:.3}s; monitor the next pit cycle.",
                        ahead.display_name(),
                        interval
                    ),
                });
            }
        }
    }

    for driver in &snapshot.drivers {
        if let Some(age) = driver.tyre_age {
            let threshold = tyre_watch_threshold(driver);
            if age >= threshold {
                insights.push(StrategyInsight {
                    severity: StrategySeverity::Watch,
                    title: format!("{} tyre age", driver.display_name()),
                    detail: format!(
                        "{} has completed approximately {} laps on {} tyres.",
                        driver.display_name(),
                        age,
                        driver.compound.as_deref().unwrap_or("unknown")
                    ),
                });
            }
        }
    }

    if snapshot.race_control.iter().rev().take(5).any(|event| {
        event
            .flag
            .as_deref()
            .is_some_and(|flag| flag.contains("YELLOW") || flag.contains("RED"))
            || event.category.contains("SafetyCar")
    }) {
        insights.push(StrategyInsight {
            severity: StrategySeverity::Critical,
            title: "Race-control intervention".to_string(),
            detail: "A recent flag or safety-car event may materially change the pit window."
                .to_string(),
        });
    }

    if insights.is_empty() {
        insights.push(StrategyInsight {
            severity: StrategySeverity::Info,
            title: "No immediate strategy alert".to_string(),
            detail: "Current gaps, tyre ages and recent race-control messages are stable."
                .to_string(),
        });
    }

    insights.truncate(8);
    insights
}

fn tyre_watch_threshold(driver: &DriverSnapshot) -> u32 {
    match driver.compound.as_deref() {
        Some("SOFT") => 15,
        Some("MEDIUM") => 25,
        Some("HARD") => 38,
        Some("INTERMEDIATE") => 25,
        Some("WET") => 30,
        _ => 30,
    }
}

fn numeric_gap(value: Option<&str>) -> Option<f64> {
    value?.trim_start_matches('+').parse().ok()
}

#[cfg(test)]
mod tests {
    use super::numeric_gap;

    #[test]
    fn parses_numeric_intervals() {
        assert_eq!(numeric_gap(Some("0.845")), Some(0.845));
        assert_eq!(numeric_gap(Some("+1 LAP")), None);
    }
}
