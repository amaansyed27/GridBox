from __future__ import annotations

from dataclasses import dataclass
from typing import Any


@dataclass(frozen=True)
class Request:
    id: str
    method: str
    params: dict[str, Any]

    @classmethod
    def from_dict(cls, payload: dict[str, Any]) -> Request:
        request_id = payload.get("id")
        method = payload.get("method")
        params = payload.get("params", {})
        if not isinstance(request_id, str) or not request_id:
            raise ValueError("request.id must be a non-empty string")
        if not isinstance(method, str) or not method:
            raise ValueError("request.method must be a non-empty string")
        if not isinstance(params, dict):
            raise ValueError("request.params must be an object")
        return cls(id=request_id, method=method, params=params)


def success(request_id: str, result: Any) -> dict[str, Any]:
    return {"id": request_id, "ok": True, "result": result, "error": None}


def failure(request_id: str, error: str) -> dict[str, Any]:
    return {"id": request_id, "ok": False, "result": None, "error": error}
