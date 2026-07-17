from __future__ import annotations

from typing import Any

from gridbox_fastf1.handlers.common import import_fastf1, scalar


def telemetry(params: dict[str, Any]) -> dict[str, Any]:
    year = int(params["year"])
    event = str(params["event"])
    session_name = str(params["session"])
    driver = str(params["driver"]).upper()

    fastf1 = import_fastf1()
    session = fastf1.get_session(year, event, session_name)
    session.load(weather=False, messages=False)
    fastest = session.laps.pick_drivers(driver).pick_fastest()
    if fastest is None:
        raise ValueError(f"no lap found for {driver}")

    data = fastest.get_car_data().add_distance()
    if data.empty:
        raise ValueError(f"no telemetry found for {driver}")

    step = max(1, len(data) // 240)
    sampled = data.iloc[::step]
    channels = [
        channel
        for channel in ["Distance", "Speed", "Throttle", "Brake", "nGear", "RPM", "DRS"]
        if channel in sampled.columns
    ]
    points = [
        {channel: scalar(row[channel]) for channel in channels}
        for _, row in sampled.iterrows()
    ]
    return {
        "event": {"year": year, "name": event, "session": session_name},
        "driver": driver,
        "lap_number": scalar(fastest.get("LapNumber")),
        "channels": channels,
        "points": points,
    }
