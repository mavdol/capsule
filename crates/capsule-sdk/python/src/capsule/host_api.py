"""
Capsule SDK - Host API Interface

This module provides the interface to call host functions from Python Wasm code.
When running in WASM mode, it imports the auto-generated bindings from core.
When running locally, it provides mock implementations for testing.
"""

import json

_host = None
_is_wasm_checked = False
_is_wasm = False

def _check_wasm():
    """Lazily check if we're running in WASM mode."""
    global _host, _is_wasm_checked, _is_wasm
    if not _is_wasm_checked:
        try:
            from wit_world.imports import api as host_module
            _host = host_module
            _is_wasm = True
        except ImportError:
            _is_wasm = False
        _is_wasm_checked = True
    return _is_wasm

def is_wasm_mode():
    return _check_wasm()

def call_host(name: str, args: list, config: dict) -> str:
    """
    Call the host's schedule_task function to create a new isolated task instance.

    This is the bridge between Python code and the Rust host runtime.

    Args:
        name: Task name to schedule
        args: List of arguments to pass to the task
        config: Task configuration dict containing:
            - compute: "LOW", "MEDIUM", or "HIGH"
            - ram: e.g., "512MB", "2GB"
            - timeout: e.g., "30s", "5m"
            - max_retries: int

    Returns:
        JSON string with the task result

    In WASM mode:
        Calls the Rust host's schedule_task function via WIT bindings

    In local mode:
        Returns mocked result
    """
    if _check_wasm() and _host is not None:
        try:
            result = _host.schedule_task(name, json.dumps(args), json.dumps(config))
            return result
        except Exception as e:
            return json.dumps({"error": f"Host call failed: {str(e)}"})
    else:
        return json.dumps({"result": f"mock_result_for_{name}"})
