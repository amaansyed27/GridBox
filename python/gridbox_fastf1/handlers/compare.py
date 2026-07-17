from __future__ import annotations

from typing import Any

from gridbox_fastf1.handlers.common import duration_seconds, import_fastf1, scalar


def compare_laps(params: dict[str, Any]) -> dict[str, Any]:
    year = int(params["year"])
    event = str(params["event"])
    session_name = str(params["session"])
    drivers = [str(driver).upper() for driver in params.get("drivers", [])]
    if len(drivers) < 2:
        raise ValueError("compare_laps requires at least two drivers")

    fastf1 = import_fastf1()
    session = fastf1.get_session(year, event, session_name)
    session.load(weather=False, messages=False)

    comparisons: list[dict[str, Any]] = []
    for driver in drivers:
        laps = session.laps.pick_drivers(driver)
        if hasattr(laps, "pick_accurate"):
            accurate = laps.pick_accurate()
            if not accurate.empty:
                laps = accurate
        fastest = laps.pick_fastest()
        if fastest is None:
            comparisons.append({"driver": driver, "available": False})
            continue

        telemetry = fastest.get_car_data().add_distance()
        comparisons.append(
            {
                "driver": driver,
                "available": True,
                "lap_number": scalar(fastest.get("LapNumber")),
                "lap_time_seconds": duration_seconds(fastest.get("LapTime")),
                "sector_1_seconds": duration_seconds(fastest.get("Sector1Time")),
                "sector_2_seconds": duration_seconds(fastest.get("Sector2Time")),
                "sector_3_seconds": duration_seconds(fastest.get("Sector3Time")),
                "compound": scalar(fastest.get("Compound")),
                "tyre_life": scalar(fastest.get("TyreLife")),
                "max_speed_kph": _series_max(telemetry, "Speed"),
                "average_speed_kph": _series_mean(telemetry, "Speed"),
                "full_throttle_percent": _full_throttle_percent(telemetry),
                "braking_samples": _braking_samples(telemetry),
            }
        )

    available = [item for item in comparisons if item.get("available")]
    available.sort(key=lambda item: item["lap_time_seconds"] or float("inf"))
    leader = available[0] if available else None
    for item in comparisons:
        lap_time = item.get("lap_time_seconds")
        leader_time = None if leader is None else leader.get("lap_time_seconds")
        item["delta_to_fastest_seconds"] = (
            None if lap_time is None or leader_time is None else round(lap_time - leader_time, 4)
        )

    return {
        "event": {"year": year, "name": event, "session": session_name},
        "fastest_driver": None if leader is None else leader["driver"],
        "drivers": comparisons,
    }


def _series_max(frame: Any, column: str) -> float | None:
    if column not in frame or frame.empty:
        return None
    value = frame[column].max()
    return None if value != value else round(float(value), 3)


def _series_mean(frame: Any, column: str) -> float | None:
    if column not in frame or frame.empty:
        return None
    value = frame[column].mean()
    return None if value != value else round(float(value), 3)


def _full_throttle_percent(frame: Any) -> float | None:
    if "Throttle" not in frame or frame.empty:
        return None
    return round(float((frame["Throttle"] >= 99).mean() * 100), 2)


def _braking_samples(frame: Any) -> int | None:
    if "Brake" not in frame or frame.empty:
        return None
    return int(frame["Brake"].astype(bool).sum())
