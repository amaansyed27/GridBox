use chrono::{Duration, Utc};
use gridbox_models::{
    DriverSnapshot, LiveSnapshot, RaceControlEvent, SessionInfo, WeatherSnapshot,
};

pub fn snapshot(tick: u64) -> LiveSnapshot {
    let now = Utc::now();
    let lap = 18 + (tick / 8) as u32;
    let phase = (tick % 20) as f64 / 20.0;
    let gaps = [
        0.0,
        1.35 + phase * 0.8,
        4.7 - phase * 0.6,
        8.4 + phase,
        13.2 - phase * 0.4,
        18.9 + phase * 0.7,
    ];
    let identities = [
        (7, "APX", "Alex Pace", "Velocity Works"),
        (12, "RIV", "Riley Voss", "Northstar Racing"),
        (21, "KAI", "Kai Mercer", "Apex Dynamics"),
        (27, "SOL", "Sofia Lane", "Velocity Works"),
        (33, "DEV", "Dev Arora", "Northstar Racing"),
        (48, "MOR", "Morgan Hale", "Apex Dynamics"),
    ];

    let drivers = identities
        .into_iter()
        .enumerate()
        .map(|(index, (number, acronym, name, team))| {
            let gap = gaps[index];
            DriverSnapshot {
                driver_number: number,
                acronym: acronym.to_string(),
                full_name: name.to_string(),
                team_name: team.to_string(),
                team_colour: None,
                position: Some(index as u32 + 1),
                gap_to_leader: Some(if index == 0 {
                    "LEADER".to_string()
                } else {
                    format!("+{gap:.3}")
                }),
                interval: Some(if index == 0 {
                    "-".to_string()
                } else {
                    format!("+{:.3}", gap - gaps[index - 1])
                }),
                lap_number: Some(lap),
                last_lap_duration: Some(91.2 + index as f64 * 0.31 + phase * 0.12),
                compound: Some(if index % 3 == 0 { "HARD" } else { "MEDIUM" }.to_string()),
                tyre_age: Some(7 + index as u32 * 2 + lap.saturating_sub(18)),
            }
        })
        .collect();

    let mut race_control = vec![RaceControlEvent {
        date: now - Duration::minutes(4),
        category: "Flag".to_string(),
        flag: Some("GREEN".to_string()),
        message: "TRACK CLEAR".to_string(),
        driver_number: None,
        lap_number: Some(lap.saturating_sub(3)),
    }];
    if tick % 40 >= 28 {
        race_control.push(RaceControlEvent {
            date: now - Duration::seconds(12),
            category: "Flag".to_string(),
            flag: Some("YELLOW".to_string()),
            message: "YELLOW FLAG IN SECTOR 2".to_string(),
            driver_number: None,
            lap_number: Some(lap),
        });
    }

    LiveSnapshot {
        session: SessionInfo {
            session_key: -1,
            meeting_key: -1,
            session_name: "Demo Race".to_string(),
            session_type: "Race".to_string(),
            country_name: "Local".to_string(),
            location: "GridBox Test Circuit".to_string(),
            circuit_short_name: "GBX".to_string(),
            year: now.year(),
            date_start: now - Duration::minutes(30),
            date_end: now + Duration::minutes(90),
        },
        drivers,
        race_control,
        weather: Some(WeatherSnapshot {
            date: now,
            air_temperature: Some(24.1),
            track_temperature: Some(36.8 + phase),
            humidity: Some(52.0),
            rainfall: Some(false),
            wind_speed: Some(3.4),
        }),
        fetched_at: now,
        source: "GridBox local demo".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::snapshot;

    #[test]
    fn demo_snapshot_is_live_and_sorted() {
        let snapshot = snapshot(8);
        assert!(snapshot.session.is_live_at(chrono::Utc::now()));
        assert_eq!(snapshot.drivers.len(), 6);
        assert_eq!(snapshot.drivers[0].position, Some(1));
        assert_eq!(snapshot.source, "GridBox local demo");
    }
}
