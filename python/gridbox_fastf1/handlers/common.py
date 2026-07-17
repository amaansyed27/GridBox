from __future__ import annotations

import os
from pathlib import Path
from typing import Any


def import_fastf1() -> Any:
    try:
        import fastf1
    except ImportError as exc:
        raise RuntimeError(
            "FastF1 is not installed. Run `uv sync` from the GridBox repository."
        ) from exc

    cache_dir = Path(
        os.environ.get(
            "GRIDBOX_FASTF1_CACHE",
            Path.home() / ".cache" / "gridbox" / "fastf1",
        )
    )
    cache_dir.mkdir(parents=True, exist_ok=True)
    fastf1.Cache.enable_cache(str(cache_dir))
    return fastf1


def duration_seconds(value: Any) -> float | None:
    if value is None:
        return None
    if hasattr(value, "total_seconds"):
        try:
            return float(value.total_seconds())
        except (TypeError, ValueError):
            return None
    try:
        if value != value:  # NaN / NaT
            return None
    except TypeError:
        pass
    try:
        return float(value)
    except (TypeError, ValueError):
        return None


def scalar(value: Any) -> Any:
    if value is None:
        return None
    if hasattr(value, "item"):
        try:
            value = value.item()
        except (TypeError, ValueError):
            pass
    if hasattr(value, "isoformat"):
        try:
            return value.isoformat()
        except (TypeError, ValueError):
            pass
    try:
        if value != value:
            return None
    except (TypeError, ValueError):
        pass
    if isinstance(value, (str, int, float, bool)):
        return value
    return str(value)
