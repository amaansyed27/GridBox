use crate::analyze_live_strategy;
use gridbox_models::LiveSnapshot;
use std::fmt::Write;

pub fn snapshot_context(snapshot: Option<&LiveSnapshot>, selected_driver: Option<u32>) -> String {
    let Some(snapshot) = snapshot else {
        return "No live session snapshot is currently loaded. Never invent current positions, gaps, tyre data or race-control events.".to_string();
    };

    let mut output = String::new();
    let _ = writeln!(output, "Session: {}", snapshot.session.title());
    let _ = writeln!(output, "Fetched: {}", snapshot.fetched_at.to_rfc3339());
    let _ = writeln!(output, "Source: {}", snapshot.source);
    let _ = writeln!(output, "Drivers:");

    for driver in snapshot.sorted_drivers().into_iter().take(20) {
        let marker = if Some(driver.driver_number) == selected_driver {
            "*"
        } else {
            "-"
        };
        let _ = writeln!(
            output,
            "{marker} P{} {} (#{}), gap {}, interval {}, lap {}, tyre {} age {}",
            driver.position.map_or("?".into(), |value| value.to_string()),
            driver.display_name(),
            driver.driver_number,
            driver.gap_to_leader.as_deref().unwrap_or("unknown"),
            driver.interval.as_deref().unwrap_or("unknown"),
            driver.lap_number.map_or("?".into(), |value| value.to_string()),
            driver.compound.as_deref().unwrap_or("unknown"),
            driver.tyre_age.map_or("?".into(), |value| value.to_string()),
        );
    }

    let _ = writeln!(output, "Deterministic strategy signals:");
    for insight in analyze_live_strategy(snapshot).into_iter().take(5) {
        let _ = writeln!(output, "- {}: {}", insight.title, insight.detail);
    }

    if let Some(weather) = &snapshot.weather {
        let _ = writeln!(
            output,
            "Weather: air {:?}C, track {:?}C, rain {:?}, humidity {:?}%",
            weather.air_temperature,
            weather.track_temperature,
            weather.rainfall,
            weather.humidity
        );
    }

    output
}
