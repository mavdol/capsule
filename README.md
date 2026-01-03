<div align="center">

# 📦 Capsule

**A secure, durable runtime for agentic workflows**

[![test](https://github.com/mavdol/capsule/actions/workflows/test.yml/badge.svg)](https://github.com/mavdol/capsule/actions/workflows/test.yml)


[Getting Started](#-quick-start) • [Documentation](#-documentation) • [Contributing](#-contributing)

</div>


---

## What is Capsule?

Capsule is a runtime for coordinating AI agent tasks in isolated environments. It is designed to handle, long-running workflows, large-scale processing, autonomous decision-making securely, or even multi-agent systems.

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


When you run `capsule run main.py` (or `main.ts`), your code is primarily compiled into a WebAssembly module and executed in a dedicated to isolate tasks.

Each task operates within its own sandbox with configurable resource limits, ensuring that failures are contained and don't cascade to other parts of your workflow. The host system controls every aspect of execution, from CPU allocation via Wasm fuel metering to memory constraints and timeout enforcement.

## Quick Start

### Prerequisites

- **Rust** (latest stable) – [Install Rust](https://rustup.rs/)
- **Python 3.13+** – [Install Python](https://www.python.org/downloads/) *(required for Python support)*
- **Node.js 22+** – [Install Node.js](https://nodejs.org/) *(required for TypeScript/JavaScript support)*

### Installation

```bash
# Clone the repository
git clone https://github.com/mavdol/capsule.git
cd capsule

# Install the Capsule CLI
cargo install --path crates/capsule-cli
```

Then, install the SDK for your language:

<details>
<summary><strong>🐍 Python</strong></summary>

```bash
pip install -e crates/capsule-sdk/python
```

**Your First Task**:

```python
from capsule import task

@task(name="main", compute="LOW", ram="64MB")
def main() -> str:
    return "Hello from Capsule!"
```

Run it:

```bash
capsule run hello.py --verbose
```

</details>

<details>
<summary><strong>🟦 TypeScript / JavaScript</strong></summary>

```bash
cd crates/capsule-sdk/javascript
npm install
npm run build
npm link # makes the SDK available globally for local development
```

Then, in your project folder:

```bash
npm link @capsule-run/sdk
```

**Your First Task**:

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
capsule run hello.ts --verbose
```

</details>

## 📚 Documentation

### Task Configuration Options

Configure your tasks with these parameters:

| Parameter | Type | Description | Example |
|-----------|------|-------------|---------|
| `name` | `str` | Task identifier (defaults to function name) | `"process_data"` |
| `compute` | `str` | CPU allocation level: `"LOW"`, `"MEDIUM"`, or `"HIGH"` | `"MEDIUM"` |
| `ram` | `str` | Memory limit for the task | `"512MB"`, `"2GB"` |
| `timeout` | `str` | Maximum execution time | `"30s"`, `"5m"`, `"1h"` |
| `max_retries` | `int` | Number of retry attempts on failure (default: 1) | `3` |

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

Capsule also provides an HTTP client for TypeScript/JavaScript via `@capsule-run/sdk`. However, standard libraries like `fetch` already compatible, so you can use whichever approach you prefer.

## 🔧 Compatibility

**Current Version**: v0.2 (Python + TypeScript/JavaScript)

### Python

✅ **Supported:**
- CPython 3.11 inside WebAssembly
- Standard library modules: `json`, `math`, `re`, `datetime`, `collections`, etc.
- Pure Python packages and libraries
- Basic I/O operations

⚠️ **Limitations:**
- Packages with C extensions like `numpy` and `pandas` are not yet supported. Support for compiled extensions is planned for future releases.

### TypeScript / JavaScript

✅ **Supported:**
- npm packages and libraries
- ES modules and modern JavaScript features

⚠️ **Limitations:**
- Only Node.js libraries like `fs`, `path`, `os`, etc. are not supported.

> [!NOTE]
> TypeScript/JavaScript has broader compatibility than Python since it doesn't rely on native bindings.

## Contributing

Contributions are welcome! Here's how you can help:

1. **Fork** the repository
2. **Create** a feature branch: `git checkout -b feature/amazing-feature`
3. **Add tests** if applicable: `cargo test`
4. **Open** a Pull Request

Need help or have questions? [Open an issue](https://github.com/mavdol/capsule/issues)

## Credits

Capsule builds on these open source projects:

- [componentize-py](https://github.com/bytecodealliance/componentize-py) – Python to WebAssembly Component compilation
- [jco](https://github.com/bytecodealliance/jco) – JavaScript toolchain for WebAssembly Components
- [wasmtime](https://github.com/bytecodealliance/wasmtime) – WebAssembly runtime
- [WASI](https://github.com/bytecodealliance/wasi.dev) – WebAssembly System Interface

## License

This project is licensed under the **Apache License 2.0** - see the [LICENSE](LICENSE) file for details.
