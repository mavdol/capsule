"""
Capsule SDK - Host API Interface

This module provides the interface to call host functions from Python Wasm code.
When running in WASM mode, it imports the auto-generated bindings from componentize-py.
When running locally, it provides mock implementations for testing.
"""

import json

IS_WASM = False

# componentize-py generates bindings in wit_world.imports.api
# The module name is based on the WIT world name "capsule-agent" → "wit_world"
try:
    from wit_world.imports import api as host
    IS_WASM = True
except ImportError:
    IS_WASM = False
    host = None

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
            - env_vars: list of (key, value) tuples

    Returns:
        JSON string with the task result

    In WASM mode:
        Calls the Rust host's schedule_task function via WIT bindings

    In local mode:
        Returns mocked result
    """
    if IS_WASM and host is not None:
        try:
            result = host.schedule_task(name, json.dumps(args), json.dumps(config))
            return result
        except Exception as e:
            return json.dumps({"error": f"Host call failed: {str(e)}"})
    else:
        return json.dumps({"result": f"mock_result_for_{name}"})
