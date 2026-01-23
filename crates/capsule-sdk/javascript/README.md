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

> [!NOTE]
> The runtime requires a task named `"main"` as the entrypoint.


When you run `capsule run main.ts`, your code is compiled into a WebAssembly module and executed in a dedicated sandbox.

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

Node.js built-ins like `fs` are not available in the WebAssembly sandbox. Instead, use the `files` API provided by the SDK:

```typescript
import { task, files } from "@capsule-run/sdk";

export const restrictedWriter = task({
    name: "restricted_writer",
    allowedFiles: ["./output"]
}, async () => {
    await files.writeText("./output/result.txt", "result");
});

export const main = task({ name: "main" }, async () => {
    await restrictedWriter();
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

✅ **Supported:**
- npm packages and ES modules work

⚠️ **Not yet supported:**
- Node.js built-ins (`fs`, `path`, `os`) are not available in the sandbox

## Links

- [GitHub](https://github.com/mavdol/capsule)
- [Issues](https://github.com/mavdol/capsule/issues)
