from __future__ import annotations

import importlib.metadata
import json
import os
import sys
import traceback
from collections.abc import Callable
from typing import Any

from gridbox_fastf1.protocol import Request, failure, success

Handler = Callable[[dict[str, Any]], Any]


def ping(_: dict[str, Any]) -> dict[str, Any]:
    try:
        version = importlib.metadata.version("fastf1")
    except importlib.metadata.PackageNotFoundError:
        return {"fastf1_available": False, "version": None, "python": sys.version.split()[0]}
    return {"fastf1_available": True, "version": version, "python": sys.version.split()[0]}


def handlers() -> dict[str, Handler]:
    from gridbox_fastf1.handlers.compare import compare_laps
    from gridbox_fastf1.handlers.sessions import session_summary
    from gridbox_fastf1.handlers.telemetry import telemetry

    return {
        "ping": ping,
        "session_summary": session_summary,
        "compare_laps": compare_laps,
        "telemetry": telemetry,
    }


def process(payload: dict[str, Any]) -> dict[str, Any]:
    request_id = str(payload.get("id", "unknown"))
    try:
        request = Request.from_dict(payload)
        handler = handlers().get(request.method)
        if handler is None:
            raise ValueError(f"unknown method: {request.method}")
        return success(request.id, handler(request.params))
    except Exception as exc:  # worker boundary converts every exception to JSON
        if os.environ.get("GRIDBOX_WORKER_TRACEBACK") == "1":
            traceback.print_exc(file=sys.stderr)
        return failure(request_id, f"{type(exc).__name__}: {exc}")


def main() -> int:
    line = sys.stdin.readline()
    if not line:
        print(json.dumps(failure("unknown", "no request received")), flush=True)
        return 2
    try:
        payload = json.loads(line)
    except json.JSONDecodeError as exc:
        print(json.dumps(failure("unknown", f"invalid JSON: {exc}")), flush=True)
        return 2

    print(json.dumps(process(payload), separators=(",", ":"), default=str), flush=True)
    return 0
