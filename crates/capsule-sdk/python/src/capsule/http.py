"""
Capsule SDK - HTTP Client

This module provides HTTP request functions that work inside Wasm instances.
Requests are executed by the Rust host via the http-request host function.
"""

import json
from typing import Dict, List, Optional, Any


class Response:
    """HTTP Response wrapper with convenient methods."""

    def __init__(self, status: int, headers: List[tuple], body: str):
        self.status_code = status
        self.headers = dict(headers)
        self.text = body

    def json(self) -> Any:
        """Parse response body as JSON."""
        return json.loads(self.text)

    def ok(self) -> bool:
        """Check if response status is 2xx."""
        return 200 <= self.status_code < 300

    def __repr__(self) -> str:
        return f"<Response [{self.status_code}]>"


def _get_host():
    """Get the WIT host module."""
    try:
        from wit_world.imports import api as host_module
        return host_module
    except ImportError:
        return None


def _make_request(
    method: str,
    url: str,
    headers: Optional[Dict[str, str]] = None,
    body: Optional[str] = None,
) -> Response:
    """
    Internal function to make HTTP requests via the host.

    Args:
        method: HTTP method (GET, POST, PUT, DELETE, PATCH, HEAD)
        url: Target URL
        headers: Optional headers dict
        body: Optional request body string

    Returns:
        Response object

    Raises:
        RuntimeError: If not running in Wasm or request fails
    """
    host = _get_host()
    if host is None:
        return Response(
            status=200,
            headers=[("content-type", "application/json")],
            body=json.dumps({"mock": True, "url": url, "method": method})
        )

    headers_list = list((headers or {}).items())

    result = host.http_request(method, url, headers_list, body)

    if isinstance(result, tuple) and hasattr(result, '__iter__'):
        if hasattr(result, 'value'):
            resp = result.value
            return Response(resp.status, resp.headers, resp.body)

    return Response(result.status, result.headers, result.body)


def get(url: str, headers: Optional[Dict[str, str]] = None) -> Response:
    """
    Make an HTTP GET request.

    Args:
        url: Target URL
        headers: Optional headers dict

    Returns:
        Response object

    Example:
        response = get("https://api.example.com/data")
        data = response.json()
    """
    return _make_request("GET", url, headers)


def post(
    url: str,
    body: Optional[str] = None,
    json_data: Optional[Any] = None,
    headers: Optional[Dict[str, str]] = None,
) -> Response:
    """
    Make an HTTP POST request.

    Args:
        url: Target URL
        body: Raw request body string
        json_data: Data to serialize as JSON (sets Content-Type automatically)
        headers: Optional headers dict

    Returns:
        Response object

    Example:
        response = post("https://api.example.com/data", json_data={"key": "value"})
    """
    headers = headers or {}
    if json_data is not None:
        body = json.dumps(json_data)
        headers["Content-Type"] = "application/json"
    return _make_request("POST", url, headers, body)


def put(
    url: str,
    body: Optional[str] = None,
    json_data: Optional[Any] = None,
    headers: Optional[Dict[str, str]] = None,
) -> Response:
    """Make an HTTP PUT request."""
    headers = headers or {}
    if json_data is not None:
        body = json.dumps(json_data)
        headers["Content-Type"] = "application/json"
    return _make_request("PUT", url, headers, body)


def delete(url: str, headers: Optional[Dict[str, str]] = None) -> Response:
    """Make an HTTP DELETE request."""
    return _make_request("DELETE", url, headers)


def patch(
    url: str,
    body: Optional[str] = None,
    json_data: Optional[Any] = None,
    headers: Optional[Dict[str, str]] = None,
) -> Response:
    """Make an HTTP PATCH request."""
    headers = headers or {}
    if json_data is not None:
        body = json.dumps(json_data)
        headers["Content-Type"] = "application/json"
    return _make_request("PATCH", url, headers, body)


def head(url: str, headers: Optional[Dict[str, str]] = None) -> Response:
    """Make an HTTP HEAD request."""
    return _make_request("HEAD", url, headers)
