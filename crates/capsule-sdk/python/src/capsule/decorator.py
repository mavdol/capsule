import json
import functools
from .host_api import call_host, IS_WASM
from . import app


def task(name=None, compute="MEDIUM", ram=None, timeout=None, max_retries=None, env_vars=None):
    """
    Decorator to mark a function as a Capsule task.

    Args:
        name: Task name (defaults to function name)
        compute: Compute level - "LOW", "MEDIUM", or "HIGH"
        ram: RAM limit - e.g., "512MB", "2GB"
        timeout: Timeout duration - e.g., "30s", "5m"
        max_retries: Maximum number of retries (default: 1)
        env_vars: Environment variables as dict

    In WASM mode:
    - The function is registered in the task registry with its config
    - When called from within a task, it schedules a new isolated instance
    - The host creates a new Wasm instance with resource limits

    In non-WASM mode:
    - The function executes directly
    """
    def decorator(func):
        task_name = name if name is not None else func.__name__

        task_config = {
            "name": task_name,
            "compute": compute.upper() if compute else "MEDIUM",
        }

        if ram is not None:
            task_config["ram"] = ram
        if timeout is not None:
            task_config["timeout"] = timeout
        if max_retries is not None:
            task_config["max_retries"] = max_retries
        if env_vars is not None:
            task_config["env_vars"] = list(env_vars.items()) if isinstance(env_vars, dict) else env_vars

        @functools.wraps(func)
        def wrapper(*args, **kwargs):
            if not IS_WASM:
                return func(*args, **kwargs)

            args_data = list(args)
            result_json = call_host(task_name, args_data, task_config)

            try:
                result = json.loads(result_json)
                if "error" in result:
                    raise RuntimeError(f"Task {task_name} failed: {result['error']}")
                return result.get("result")
            except json.JSONDecodeError:
                return result_json

        app.register_task(task_name, func, task_config)

        return wrapper

    return decorator
