"""
Example demonstrating recursive/nested task execution with config.

This example shows:
1. How task configuration is properly passed from decorator to host
2. How tasks can call other tasks (recursive execution)
3. How the host creates isolated Wasm instances for each task
"""

from capsule import task


@task(name="add", compute="LOW", ram="64MB", timeout="5s")
def add(a: int, b: int) -> int:
    """A simple addition task with LOW compute requirements."""
    return a + b


@task(name="multiply", compute="MEDIUM", ram="128MB", timeout="10s")
def multiply(a: int, b: int) -> int:
    """A multiplication task with MEDIUM compute requirements."""
    return a * b


@task(name="complex_calculation", compute="HIGH", ram="512MB", timeout="30s", max_retries=3)
def complex_calculation(x: int, y: int) -> dict:
    """
    A complex calculation that spawns sub-tasks.

    This demonstrates recursive task execution:
    1. The host creates a Wasm instance for this task
    2. This task calls add() and multiply()
    3. Each call creates a NEW Wasm instance with its own resource limits
    4. Results are returned back up the chain
    """
    sum_result = add(x, y)
    product_result = multiply(x, y)

    return {
        "sum": sum_result,
        "product": product_result,
        "total": sum_result + product_result
    }


@task(name="main", compute="MEDIUM", ram="256MB")
def main() -> dict:
    """Main entry point - orchestrates the workflow."""
    print("Starting complex calculation workflow...")

    result = complex_calculation(10, 5)

    print(f"Calculation complete: {result}")
    return result
