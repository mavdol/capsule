"""
Capsule SDK Export Module for task-runner interface

This module implements the `capsule:host/task-runner` export interface
that the Wasm component provide.
"""

import json

_TASKS = {}
_main_module = None


def register_task(name: str, func, config: dict = None):
    """
    Register a task function by name with its configuration.

    Args:
        name: Task name
        func: The task function
        config: Task configuration (compute, ram, timeout, etc.)
    """
    _TASKS[name] = {
        "func": func,
        "config": config or {}
    }

def get_task(name: str):
    """Get a registered task by name"""
    task_info = _TASKS.get(name)
    if task_info:
        return task_info["func"]
    return None

def get_task_config(name: str):
    """Get the configuration for a registered task"""
    task_info = _TASKS.get(name)
    if task_info:
        return task_info["config"]
    return {}

class TaskRunner:
    """
    Implementation of the capsule:host/task-runner interface.

    This class is instantiated by capsule-core when the component is loaded.
    The Rust host calls `run(args_json)` to execute a task.
    """
    def run(self, args_json: str) -> str:
        """
        Execute a task with the given arguments.

        Args:
            args_json: JSON string containing:
                - task_name: Name of the task to run
                - args: Positional arguments list
                - kwargs: Keyword arguments dict

        Returns:
            JSON string with the task result or error

        The host calls this function to execute a task within this Wasm instance.
        """
        try:
            data = json.loads(args_json)
            task_name = data.get("task_name", "main")
            args = data.get("args", [])
            kwargs = data.get("kwargs", {})

            task_func = get_task(task_name)

            if task_func is None and task_name != "main":
                task_func = get_task("main")

            if task_func is None and _main_module is not None:
                if hasattr(_main_module, 'main') and callable(_main_module.main):
                    task_func = _main_module.main

            if task_func is None and _TASKS:
                first_task_name = next(iter(_TASKS.keys()))
                task_func = get_task(first_task_name)

            if task_func is None:
                return json.dumps({
                    "error": f"No tasks or main() function found. Available tasks: {list(_TASKS.keys())}"
                })

            result = task_func(*args, **kwargs)

            if result is None:
                return json.dumps({"error": "Unknown error"})

            return json.dumps(result)

        except Exception as e:
            import traceback
            return json.dumps({
                "error": str(e),
                "traceback": traceback.format_exc()
            })


exports = TaskRunner()
