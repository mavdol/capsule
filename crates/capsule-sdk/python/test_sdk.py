"""
Test the SDK locally (without WASM compilation).

This tests:
1. Task registration with config
2. Task execution in local mode
3. Config storage and retrieval
"""

import sys
import os

# Add the SDK to path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'crates', 'capsule-sdk', 'python'))

from capsule import task
from capsule import app


def test_task_registration():
    """Test that tasks are registered with their config."""

    @task(name="test_task", compute="HIGH", ram="1GB", timeout="60s", max_retries=5)
    def my_test_task(x: int) -> int:
        return x * 2

    # Check that task is registered
    assert "test_task" in app._TASKS, "Task should be registered"

    # Check that config is stored
    config = app.get_task_config("test_task")
    assert config["name"] == "test_task"
    assert config["compute"] == "HIGH"
    assert config["ram"] == "1GB"
    assert config["timeout"] == "60s"
    assert config["max_retries"] == 5

    print("✓ Task registration test passed")


def test_task_execution_local():
    """Test task execution in local mode."""

    @task(name="add_task", compute="LOW")
    def add_task(a: int, b: int) -> int:
        return a + b

    # Execute in local mode
    result = add_task(5, 3)
    assert result == 8, f"Expected 8, got {result}"

    print("✓ Task execution (local mode) test passed")


def test_task_config_defaults():
    """Test that default config values are applied."""

    @task(name="default_task")
    def default_task():
        pass

    config = app.get_task_config("default_task")
    assert config["compute"] == "MEDIUM", "Default compute should be MEDIUM"

    print("✓ Config defaults test passed")


def test_task_config_env_vars():
    """Test that env_vars are properly formatted."""

    @task(name="env_task", env_vars={"KEY1": "value1", "KEY2": "value2"})
    def env_task():
        pass

    config = app.get_task_config("env_task")
    assert "env_vars" in config
    assert isinstance(config["env_vars"], list)
    assert ("KEY1", "value1") in config["env_vars"]
    assert ("KEY2", "value2") in config["env_vars"]

    print("✓ Env vars test passed")


if __name__ == "__main__":
    print("Testing Capsule SDK...\n")

    test_task_registration()
    test_task_execution_local()
    test_task_config_defaults()
    test_task_config_env_vars()

    print("\n✨ All tests passed!")
