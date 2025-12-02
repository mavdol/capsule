import asyncio
from capsule import task

@task(name="greet", compute="LOW", ram="64MB", timeout="5s", max_retries=2)
def greet(name: str) -> str:
    """A simple greeting task running in isolation."""
    return f"Hello, {name}!"

@task(name="calculate", compute="LOW", ram="64MB", timeout="5s")
def calculate(x: int, y: int) -> str:
    """A calculation task with resource limits."""
    result = x + y
    return f"The answer is {result}"

@task(name="format_message", compute="LOW", ram="64MB", timeout="5s", max_retries=1)
def format_message(prefix: str, suffix: str) -> str:
    """A message formatting task."""
    return f"{prefix} to {suffix}"

async def main():

    task1 = greet("Capsule")
    task2 = calculate(40, 2)
    task3 = format_message("Welcome", "isolated execution")

    results = await asyncio.gather(task1, task2, task3)

    print(f"Task 1: {results[0]}")
    print(f"Task 2: {results[1]}")
    print(f"Task 3: {results[2]}")
    print("\n✨ All tasks completed successfully!")

if __name__ == "__main__":
    asyncio.run(main())
