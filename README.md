<div align="center">

# ```Capsule```

**A secure, durable runtime for AI agents**

[![CI](https://github.com/mavdol/capsule/actions/workflows/ci.yml/badge.svg)](https://github.com/mavdol/capsule/actions/workflows/ci.yml)

[Getting Started](#quick-start) • [Documentation](#documentation) • [Contributing](#contributing)

</div>

---

## Overview

```Capsule``` is a runtime for coordinating AI agent tasks in isolated environments. It is designed to handle, long-running workflows, large-scale processing, autonomous decision-making securely, or even multi-agent systems.

Each task runs inside its own WebAssembly sandbox, providing:

- **Isolated execution**: Each task runs isolated from your host system
- **Resource limits**: Set CPU, memory, and timeout limits per task
- **Automatic retries**: Handle failures without manual intervention
- **Lifecycle tracking**: Monitor which tasks are running, completed, or failed

This enables safe task-level execution of untrusted code within AI agent systems.

## How It Works

Capsule leverages Wasm to create secure, isolated execution environments.

### With Python

Simply annotate your Python functions with the `@task` decorator:

```python
from capsule import task

@task(name="analyze_data", compute="MEDIUM", ram="512MB", timeout="30s", max_retries=1)
def analyze_data(dataset: list) -> dict:
    """Process data in an isolated, resource-controlled environment."""
    # Your code runs safely in a Wasm sandbox
    return {"processed": len(dataset), "status": "complete"}
```

### With TypeScript / JavaScript

Capsule now supports TypeScript and JavaScript with the `task()` wrapper function. This offers compatibility with the entire JavaScript ecosystem.

```typescript
import { task } from "@capsule-run/sdk";

export const analyzeData = task({
  name: "analyze_data",
  compute: "MEDIUM",
  ram: "512MB",
  timeout: "30s",
  maxRetries: 1
}, (dataset: number[]): object => {
  // Your code runs safely in a Wasm sandbox
  return { processed: dataset.length, status: "complete" };
});

// The "main" task is required as the entrypoint
export const main = task({
    name: "main",
    compute: "HIGH"
}, () => {
  return analyzeData([1, 2, 3, 4, 5]);
});

```
> [!NOTE]
> TypeScript/JavaScript projects require a task named `"main"` as the entrypoint.


When you run `capsule run main.py` (or `main.ts`), your code is compiled into a WebAssembly module and executed in a dedicated sandbox to isolate tasks.

Each task operates within its own sandbox with configurable resource limits, ensuring that failures are contained and don't cascade to other parts of your workflow. The host system controls every aspect of execution, from CPU allocation via Wasm fuel metering to memory constraints and timeout enforcement.

## Quick Start

### Python

```bash
pip install capsule-run
```

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

### TypeScript / JavaScript

```bash
npm install -g @capsule-run/cli
npm install @capsule-run/sdk
```

Create `hello.ts`:

```typescript
import { task } from "@capsule-run/sdk";

export const main = task({
  name: "main",
  compute: "LOW",
  ram: "64MB"
}, (): string => {
  return "Hello from Capsule!";
});
```

Run it:

```bash
capsule run hello.ts
```

> [!TIP]
> Use `--verbose` to display real-time task execution details.

## Documentation

### Task Configuration Options

Configure your tasks with these parameters:

| Parameter | Type | Description | Example |
|-----------|------|-------------|---------|
| `name` | `str` | Task identifier (defaults to function name) | `"process_data"` |
| `compute` | `str` | CPU allocation level: `"LOW"`, `"MEDIUM"`, or `"HIGH"` | `"MEDIUM"` |
| `ram` | `str` | Memory limit for the task | `"512MB"`, `"2GB"` |
| `timeout` | `str` | Maximum execution time | `"30s"`, `"5m"`, `"1h"` |
| `max_retries` | `int` | Number of retry attempts on failure (default: 1) | `3` |
| `allowed_files` | `list` | Files or folders accessible in the sandbox | `["./data", "./output"]` |

### Compute Levels

Capsule controls CPU usage through WebAssembly's **fuel mechanism**, which meters instruction execution. The compute level determines how much fuel your task receives.
- **LOW** provides minimal allocation for lightweight tasks
- **MEDIUM** offers balanced resources for typical workloads
- **HIGH** grants maximum fuel for compute-intensive operations
- **CUSTOM** to specify an exact fuel value (e.g., `compute="1000000"`) for precise control over execution limits.

### HTTP Client API

#### Python

The standard Python `requests` library and socket-based networking aren't natively compatible with WebAssembly's sandboxed I/O model. Capsule provides its own HTTP client that works within the Wasm environment:

```python
from capsule import task
from capsule.http import get, post, put, delete

@task(name="http_example", compute="MEDIUM", timeout="30s")
def main() -> dict:
    """Example demonstrating HTTP client usage within a task."""

    # GET request
    response = get("https://api.example.com/data")

    # POST with JSON body
    response = post("https://api.example.com/submit", json={"key": "value"})

    # Response methods
    is_ok = response.ok()           # Returns True if status code is 2xx
    status = response.status_code    # Get the HTTP status code
    data = response.json()           # Parse response as JSON
    text = response.text()           # Get response as text

    return {"status": status, "success": is_ok}
```

#### TypeScript / JavaScript

Standard libraries like `fetch` are already compatible, so no custom HTTP client is needed for TypeScript/JavaScript.

```typescript
import { task } from "@capsule-run/sdk";

export const main = task({
    name: "main",
    compute: "MEDIUM"
}, async () => {
    const response = await fetch("https://api.example.com/data");
    return response.json();
});
```

### File Access

By default, tasks run in a fully isolated sandbox with no filesystem access. Use the `allowed_files` parameter to grant read and write access to specific files or directories.

#### Python

Python's standard file operations work normally. Use `open()`, `os`, `pathlib`, or any file manipulation library.

```python
from capsule import task

@task(name="main", allowed_files=["./data"])
def main() -> str:
    with open("./data/input.txt") as f:
        return f.read()
```

#### TypeScript / JavaScript

Node.js built-ins like `fs` are not available in the WebAssembly sandbox. Instead, use the `files` API provided by the SDK.

```typescript
import { task, files } from "@capsule-run/sdk";

export const main = task({
    name: "main",
    allowedFiles: ["./data"]
}, async () => {
    return await files.readText("./data/input.txt");
});
```

Available methods:
- `files.readText(path)` — Read file as string
- `files.readBytes(path)` — Read file as `Uint8Array`
- `files.writeText(path, content)` — Write string to file
- `files.writeBytes(path, data)` — Write bytes to file
- `files.list(path)` — List directory contents
- `files.exists(path)` — Check if file exists

## Compatibility

> [!NOTE]
> TypeScript/JavaScript has broader compatibility than Python since it doesn't rely on native bindings.

**Python:** Pure Python packages and standard library modules work. Packages with C extensions (`numpy`, `pandas`) are not yet supported.

**TypeScript/JavaScript:** npm packages and ES modules work. Node.js built-ins (`fs`, `path`, `os`) are not available in the sandbox.

## Contributing

Contributions are welcome!

### Development setup

**Prerequisites:** Rust (latest stable), Python 3.13+, Node.js 22+

```bash
git clone https://github.com/mavdol/capsule.git
cd capsule

# Build and install CLI
cargo install --path crates/capsule-cli

# Python SDK (editable install)
pip install -e crates/capsule-sdk/python

# TypeScript SDK (link for local dev)
cd crates/capsule-sdk/javascript
npm install && npm run build && npm link

# Then in your project: npm link @capsule-run/sdk
```

### How to contribute

1. **Fork** the repository
2. **Create** a feature branch: `git checkout -b feature/amazing-feature`
3. **Run tests**: `cargo test`
4. **Open** a Pull Request

Need help? [Open an issue](https://github.com/mavdol/capsule/issues)

## Credits

Capsule builds on these open source projects:

- [componentize-py](https://github.com/bytecodealliance/componentize-py) – Python to WebAssembly Component compilation
- [jco](https://github.com/bytecodealliance/jco) – JavaScript toolchain for WebAssembly Components
- [wasmtime](https://github.com/bytecodealliance/wasmtime) – WebAssembly runtime
- [WASI](https://github.com/WebAssembly/WASI) – WebAssembly System Interface

## License

This project is licensed under the **Apache License 2.0** - see the [LICENSE](LICENSE) file for details.
