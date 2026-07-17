from __future__ import annotations

from typing import Any

from gridbox_fastf1.handlers.common import duration_seconds, import_fastf1, scalar


def session_summary(params: dict[str, Any]) -> dict[str, Any]:
    year = int(params["year"])
    event = str(params["event"])
    session_name = str(params["session"])

    fastf1 = import_fastf1()
    session = fastf1.get_session(year, event, session_name)
    session.load(telemetry=False, weather=True, messages=True)

    drivers: list[dict[str, Any]] = []
    for _, row in session.results.iterrows():
        drivers.append(
            {
                "position": scalar(row.get("Position")),
                "driver_number": scalar(row.get("DriverNumber")),
                "abbreviation": scalar(row.get("Abbreviation")),
                "full_name": scalar(row.get("FullName")),
                "team": scalar(row.get("TeamName")),
                "status": scalar(row.get("Status")),
                "points": scalar(row.get("Points")),
            }
        )

    fastest_laps: list[dict[str, Any]] = []
    if session.laps is not None and not session.laps.empty:
        for driver in session.drivers:
            driver_laps = session.laps.pick_drivers(driver)
            if driver_laps.empty:
                continue
            fastest = driver_laps.pick_fastest()
            if fastest is None:
                continue
            fastest_laps.append(
                {
                    "driver": scalar(fastest.get("Driver")),
                    "lap_number": scalar(fastest.get("LapNumber")),
                    "lap_time_seconds": duration_seconds(fastest.get("LapTime")),
                    "compound": scalar(fastest.get("Compound")),
                    "tyre_life": scalar(fastest.get("TyreLife")),
                }
            )

    return {
        "event": {
            "year": year,
            "name": scalar(session.event.get("EventName")),
            "location": scalar(session.event.get("Location")),
            "country": scalar(session.event.get("Country")),
            "session": session_name,
        },
        "drivers": drivers,
        "fastest_laps": sorted(
            fastest_laps,
            key=lambda lap: lap["lap_time_seconds"] or float("inf"),
        ),
        "weather_samples": 0 if session.weather_data is None else len(session.weather_data),
        "race_control_messages": (
            0 if session.race_control_messages is None else len(session.race_control_messages)
        ),
    }
