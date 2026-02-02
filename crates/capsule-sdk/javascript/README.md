# capsule

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
npm install -g @capsule-run/cli
npm install @capsule-run/sdk
```

## Quick Start

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

> Use `--verbose` to display real-time task execution details.

## How It Works

Simply use a wrapper function to define your tasks:

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

export const main = task({
    name: "main",
    compute: "HIGH"
}, () => {
  return analyzeData([1, 2, 3, 4, 5]);
});
```

> The runtime requires a task named `"main"` as the entry point.

When you run `capsule run main.ts`, your code is compiled into a WebAssembly module and executed in a dedicated sandbox.

### Response Format

Every task returns a structured JSON envelope containing both the result and execution metadata:
```json
{
  "success": true,
  "result": { "processed": 5, "status": "complete" },
  "error": null,
  "execution": {
    "task_name": "data_processor",
    "duration_ms": 1523,
    "retries": 0,
    "fuel_consumed": 45000
  }
}
```

**Response fields:**
- `success` — Boolean indicating whether the task completed successfully
- `result` — The actual return value from your task (json, string, null on failure etc..)
- `error` — Error details if the task failed (`{ error_type: string, message: string }`)
- `execution` — Performance metrics:
  - `task_name` — Name of the executed task
  - `duration_ms` — Execution time in milliseconds
  - `retries` — Number of retry attempts that occurred
  - `fuel_consumed` — CPU resources used (see [Compute Levels](#compute-levels))

## Documentation

### Task Configuration Options

| Parameter | Description | Type | Default | Example |
|-----------|-------------|------|---------|---------|
| `name` | Task identifier | `string` | *required* | `"process_data"` |
| `compute` | CPU level: `"LOW"`, `"MEDIUM"`, `"HIGH"` | `string` | `"MEDIUM"` | `"HIGH"` |
| `ram` | Memory limit | `string` | unlimited | `"512MB"`, `"2GB"` |
| `timeout` | Maximum execution time | `string` or `number` | unlimited | `"30s"`, `"5m"` |
| `maxRetries` | Retry attempts on failure | `number` | `0` | `3` |
| `allowedFiles` | Folders accessible in the sandbox | `string[]` | `[]` | `["./data", "./output"]` |
| `envVariables` | Environment variables accessible in the sandbox | `string[]` | `[]` | `["API_KEY"]` |

### Compute Levels

- **LOW**: Minimal allocation for lightweight tasks
- **MEDIUM**: Balanced resources for typical workloads
- **HIGH**: Maximum fuel for compute-intensive operations
- **CUSTOM**: Specify exact fuel value (e.g., `compute="1000000"`)

### Project Configuration (Optional)

Create a `capsule.toml` file in your project root to set default options:

```toml
[workflow]
name = "My AI Workflow"
version = "1.0.0"
entrypoint = "src/main.ts"  # Run `capsule run` without specifying a file

[tasks]
default_compute = "MEDIUM"
default_ram = "256MB"
default_timeout = "30s"
```

Task-level options always override these defaults.

### File Access

Tasks can read and write files within directories specified in `allowedFiles`. Any attempt to access files outside these directories is not possible.

Common Node.js built-ins are available. Use the standard `fs` module:

```typescript
import { task } from "@capsule-run/sdk";
import fs from "fs/promises";

export const restrictedWriter = task({
    name: "restricted_writer",
    allowedFiles: ["./output"]
}, async () => {
    await fs.writeFile("./output/result.txt", "result");
});

export const main = task({ name: "main", allowedFiles: ["./data"] }, async () => {
    await restrictedWriter();
    return await fs.readFile("./data/input.txt", "utf8");
});
```

### Environment Variables

Tasks can access environment variables to read configuration, API keys, or other runtime settings. Use the standard `process.env`:

```typescript
import { task } from "@capsule-run/sdk";

export const main = task({
    name: "main",
    envVariables: ["API_KEY"]
}, () => {
    const apiKey = process.env.API_KEY;
    return { apiKeySet: apiKey !== undefined };
});
```

## Compatibility

✅ **Supported:**
- npm packages and ES modules
- Common Node.js built-ins. If you have any trouble with a built-in do not hesitate to open an issue.

⚠️ **Not supported:**
- Native Node.js addons (C++ bindings)

## Links

- [GitHub](https://github.com/mavdol/capsule)
- [Issues](https://github.com/mavdol/capsule/issues)
