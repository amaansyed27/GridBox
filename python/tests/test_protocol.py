from gridbox_fastf1.protocol import Request, failure, success
from gridbox_fastf1.worker import process


def test_request_validation() -> None:
    request = Request.from_dict({"id": "1", "method": "ping", "params": {}})
    assert request.method == "ping"


def test_response_helpers() -> None:
    assert success("1", {"ok": True})["ok"] is True
    assert failure("1", "broken")["error"] == "broken"


def test_unknown_method_is_structured_error() -> None:
    response = process({"id": "abc", "method": "missing", "params": {}})
    assert response["ok"] is False
    assert "unknown method" in response["error"]


def test_ping_does_not_require_fastf1_import() -> None:
    response = process({"id": "ping", "method": "ping", "params": {}})
    assert response["ok"] is True
    assert "fastf1_available" in response["result"]
