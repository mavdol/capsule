# capsule-run

**A secure, durable runtime for agentic workflows**

## Overview

Capsule is a runtime for coordinating AI agent tasks in isolated environments. It is designed to handle long-running workflows, large-scale processing, autonomous decision-making securely, or even multi-agent systems.

Each task runs inside its own WebAssembly sandbox, providing:

- **Isolated execution**: Each task runs isolated from your host system
- **Resource limits**: Set CPU, memory, and timeout limits per task
- **Automatic retries**: Handle failures without manual intervention
- **Lifecycle tracking**: Monitor which tasks are running, completed, or failed

## Installation

```bash
pip install capsule-run
```

## Quick Start

Create `hello.py`:

```python
from capsule import task

@task(name="main", compute="LOW", ram="64MB")
def main() -> str:
    return "Hello from Capsule!"
```

Run it:

```bash
capsule run hello.py
```

## How It Works

Simply annotate your Python functions with the `@task` decorator:

```python
from capsule import task

@task(name="analyze_data", compute="MEDIUM", ram="512MB", timeout="30s", max_retries=1)
def analyze_data(dataset: list) -> dict:
    """Process data in an isolated, resource-controlled environment."""
    return {"processed": len(dataset), "status": "complete"}
```

When you run `capsule run main.py`, your code is compiled into a WebAssembly module and executed in a dedicated sandbox.

## Documentation

### Task Configuration Options

| Parameter | Type | Description | Example |
|-----------|------|-------------|---------|
| `name` | `str` | Task identifier | `"process_data"` |
| `compute` | `str` | CPU level: `"LOW"`, `"MEDIUM"`, `"HIGH"` | `"MEDIUM"` |
| `ram` | `str` | Memory limit | `"512MB"`, `"2GB"` |
| `timeout` | `str` | Maximum execution time | `"30s"`, `"5m"` |
| `max_retries` | `int` | Retry attempts on failure | `3` |

### Compute Levels

- **LOW**: Minimal allocation for lightweight tasks
- **MEDIUM**: Balanced resources for typical workloads
- **HIGH**: Maximum fuel for compute-intensive operations
- **CUSTOM**: Specify exact fuel value (e.g., `compute="1000000"`)

### HTTP Client

Standard `requests` library isn't compatible with WASM. Use Capsule's HTTP client:

```python
from capsule import task
from capsule.http import get, post

@task(name="fetch", compute="MEDIUM", timeout="30s")
def main() -> dict:
    response = get("https://api.example.com/data")
    return {"status": response.status_code, "ok": response.ok()}
```

## Compatibility

✅ **Supported:**
- Pure Python packages and standard library
- `json`, `math`, `re`, `datetime`, `collections`, etc.

⚠️ **Not yet supported:**
- Packages with C extensions (e.g `numpy`, `pandas`)

## Links

- [GitHub](https://github.com/mavdol/capsule)
- [Issues](https://github.com/mavdol/capsule/issues)

## License

Apache-2.0
