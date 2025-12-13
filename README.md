<div align="center">

# 📦 Capsule

**A durable runtime for agentic workflows**

[![License](https://img.shields.io/badge/license-Apache_2.0-blue)](LICENSE)
[![Rust](https://img.shields.io/badge/built_with-Rust-orange)](https://www.rust-lang.org/)


https://github.com/user-attachments/assets/dc0043d5-da42-4ead-b66e-0b7025046aa1

[Getting Started](#-quick-start) • [Examples](#-examples) • [Documentation](#-documentation) • [Contributing](#-contributing)

</div>

---

## 🎯 What is Capsule?

Modern AI agents are evolving from simple, single-threaded scripts into **distributed multi-agent architectures** that coordinate dozens of sub-agents in parallel. These systems handle long-running workflows, large-scale data processing, and complex decision-making tasks that require:

- 🔒 **Robust isolation** between untrusted code and your host system
- 📊 **Fine-grained resource control** (CPU, RAM, timeout limits)
- 🔄 **Automatic retry mechanisms** for resilient execution
- 📈 **Observable task execution** with comprehensive monitoring

**Capsule** is a durable runtime for agentic workflows that gives you complete control over task execution through WebAssembly sandboxing. Build reliable, scalable agent systems with confidence.


## 🚀 How It Works

Capsule leverages **WebAssembly (Wasm)** to create secure, isolated execution environments for each task. Here's what makes it powerful:

1. **Decorator-Based**: Simply annotate your Python functions with `@task`
2. **Automatic Compilation**: Python code is compiled to Wasm on-the-fly
3. **Isolated Execution**: Each task runs in its own sandboxed Wasm instance
4. **Resource Limits**: Enforce CPU, RAM, and time constraints per task
5. **Fault Recovery**: Automatic retries on failure without affecting other tasks

```python
from capsule import task

@task(name="analyze_data", compute="MEDIUM", ram="512MB", timeout="30s")
def analyze_data(dataset: list) -> dict:
    """Process data in an isolated, resource-controlled environment."""
    # Your code runs safely in a Wasm sandbox
    return {"processed": len(dataset), "status": "complete"}
```

This diagram illustrates the complete journey of a task from your command line invocation to execution within the Rust core:

![Process Flow](./assets/task-execution-flow.png)

When you run `capsule run main.py`, your Python code is compiled into a WebAssembly module and executed in a dedicated, isolated Wasm instance managed by Capsule's Rust runtime. Each task operates within its own sandbox with configurable resource limits, ensuring that failures are contained and don't cascade to other parts of your workflow. The host system controls every aspect of execution, from CPU allocation via Wasm fuel metering to memory constraints and timeout enforcement.

## 🏁 Quick Start

### Prerequisites

- **Rust** (latest stable) - [Install Rust](https://rustup.rs/)
- **Python 3.11+** - [Install Python](https://www.python.org/downloads/) (Only for the V0.1)

### Installation

```bash
# Clone the repository
git clone https://github.com/mavdol/capsule.git
cd capsule

# Install the Capsule CLI
cargo install --path crates/capsule-cli

# Install the Python SDK in your workspace
pip install -e crates/capsule-sdk/python
```

### Your First Capsule Task

Create a file called `hello.py`:

```python
from capsule import task

@task(name="hello_capsule", compute="LOW", ram="64MB")
def main() -> str:
    """Your first isolated task!"""
    return "Hello from Capsule! 📦✨"
```

Run it:

```bash
capsule run hello.py
```

That's it! Your Python function just ran in a secure WebAssembly sandbox. 🎉

## 📖 Examples

### Basic Task with Resource Limits

```python
from capsule import task

@task(
    name="process_batch",
    compute="MEDIUM",
    ram="256MB",
    timeout="60s",
    max_retries=3
)
def process_batch(items: list) -> dict:
    """Process a batch of items with resource constraints."""
    results = []
    for item in items:
        # Process each item
        results.append(process_item(item))

    return {"processed": len(results), "results": results}
```

### Making HTTP Requests

Standard libraries like `requests` are disabled for security. Use Capsule's built-in HTTP client:

```python
from capsule import task
from capsule.http import get, post

@task(name="fetch_data", compute="MEDIUM", timeout="30s")
def fetch_data(api_url: str) -> dict:
    """Fetch data from an external API."""
    response = get(api_url)

    if response.ok():
        data = response.json()
        print(f"✅ Fetched {len(data)} items")
        return data
    else:
        print(f"❌ Request failed: {response.status_code}")
        return {"error": response.status_code}

@task(name="post_results", compute="LOW", timeout="15s")
def post_results(api_url: str, payload: dict) -> bool:
    """Post results to an API endpoint."""
    response = post(api_url, json=payload)
    return response.ok()
```

### Task with Environment Variables

```python
from capsule import task

@task(
    name="authenticated_task",
    compute="LOW",
    env_vars={"API_KEY": "your-api-key", "ENVIRONMENT": "production"}
)
def authenticated_task() -> str:
    """Access environment variables within the task."""
    import os
    api_key = os.getenv("API_KEY")
    env = os.getenv("ENVIRONMENT")

    return f"Running in {env} with key: {api_key[:4]}..."
```

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
| `env_vars` | `dict` | Environment variables accessible within the task | `{"KEY": "value"}` |

### Compute Levels

Capsule controls CPU usage through WebAssembly's **fuel mechanism**, which meters instruction execution. The compute level determines how much fuel your task receives.
- **LOW** provides minimal allocation for lightweight tasks
- **MEDIUM** offers balanced resources for typical workloads
- **HIGH** grants maximum fuel for compute-intensive operations
- **CUSTOM** to specify an exact fuel value (e.g., `compute="1000000"`) for precise control over execution limits.

### HTTP Client API

Standard Python networking relies on sockets, which aren't natively compatible with WebAssembly's sandbox model. For security and portability, Capsule provides its own HTTP client that works seamlessly within the Wasm environment while maintaining strict isolation boundaries:

```python
from capsule.http import get, post, put, delete

# GET request
response = get("https://api.example.com/data")

# POST with JSON body
response = post("https://api.example.com/submit", json={"key": "value"})

# Response methods
response.ok()           # Returns True if status code is 2xx
response.status_code    # Get the HTTP status code
response.json()         # Parse response as JSON
response.text()         # Get response as text
```

## 🔧 Compatibility

**Current Version**: v0.1 (Python support only)

### What Works

✅ **Supported:**
- CPython 3.11 inside WebAssembly
- Standard library modules: `json`, `math`, `re`, `datetime`, `collections`, etc.
- Pure Python packages and libraries
- Basic I/O operations

### Important Limitations

Packages with C extensions like `numpy` and `pandas` are not yet supported in the current version. Support for compiled extensions is planned for future releases as we expand WebAssembly compatibility.

## 📅 What's Next

#### v0.2.0: The Orchestrator

> 💡 Community Driven: The path from v0.2 onwards is flexible. While we have a vision for a Daemon mode your feedback defines the priority.

**Status:** Planned

**Goal:** Turning the runner into a stable, persistent system.

- [ ] **Daemon Mode:** Implement the client-server architecture and IPC communication.
- [ ] **Basic Config:** Support `capsule.toml` to identify projects (Namespacing).
- [ ] **Management:** `list` (Tree View), `restart`, and `stop` commands.
- [x] **Resilience:** Retry logic and real Timeout handling (Wall clock).

Check out the [roadmap](./ROADMAP.md) for more details.

## 📝 Engineering Logs

Want to follow along with Capsule's development journey? I share detailed engineering logs, technical deep-dives, and behind-the-scenes insights on Substack:

**[📖 Read the Engineering Logs](https://capsuleruntime.substack.com/)**


## 🤝 Contributing

We welcome contributions from the community! Here's how you can help:

1. **Fork** the repository
2. **Create** a feature branch: `git checkout -b feature/amazing-feature`
3. **Add tests** if applicable: `cargo test`
4. **Open** a Pull Request

Need help or have questions? [Open an issue](https://github.com/mavdol/capsule/issues) !


## 📄 License

This project is licensed under the **Apache License 2.0** - see the [LICENSE](LICENSE) file for details.

---

<div align="center">
[⭐ Star us on GitHub](https://github.com/mavdol/capsule)
</div>
