"""
Capsule SDK - SDK Runner

Provides the `run` function for running Capsule tasks
from third party applications.
"""

import asyncio
import json
from typing import Any, Optional, TypedDict


class RunnerOptions(TypedDict, total=False):
    file: str
    args: Optional[list[str]]
    cwd: Optional[str]
    capsule_path: Optional[str]


class ExecutionInfo(TypedDict):
    task_name: str
    duration_ms: int
    retries: int
    fuel_consumed: int


class ErrorInfo(TypedDict):
    error_type: str
    message: str


class RunnerResult(TypedDict):
    success: bool
    result: Any
    error: Optional[ErrorInfo]
    execution: ExecutionInfo


async def run(
    *,
    file: str,
    args: Optional[list[str]] = None,
    cwd: Optional[str] = None,
    capsule_path: str = "capsule",
) -> RunnerResult:
    """Run a Capsule task from a third-party application.

    Args:
        file: Path to the Capsule task file
        args: Arguments to pass to the task
        cwd: Working directory for the task
        capsule_path: Path to capsule CLI binary

    Returns:
        RunnerResult with task result and execution metadata
    """
    args = args or []
    cmd = [capsule_path, "run", file, "--json", *args]

    try:
        process = await asyncio.create_subprocess_exec(
            *cmd,
            stdout=asyncio.subprocess.PIPE,
            stderr=asyncio.subprocess.PIPE,
            cwd=cwd,
        )

        stdout, stderr = await process.communicate()

        if process.returncode != 0 and not stdout:
            error_msg = stderr.decode("utf-8") if stderr else "Unknown error"
            raise RuntimeError(error_msg)

        try:
            return json.loads(stdout.decode("utf-8"))
        except json.JSONDecodeError:
            raise ValueError(
                f"Failed to parse Capsule output: {stdout.decode('utf-8')}"
            )

    except FileNotFoundError:
        raise FileNotFoundError(
            "Capsule CLI not found. Use 'pip install capsule' to install it."
        )
