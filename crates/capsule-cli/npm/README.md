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

// The "main" task is required as the entrypoint
export const main = task({
    name: "main",
    compute: "HIGH"
}, () => {
  return analyzeData([1, 2, 3, 4, 5]);
});
```

When you run `capsule run main.ts`, your code is compiled into a WebAssembly module and executed in a dedicated sandbox.

## Documentation

### Task Configuration Options

| Parameter | Type | Description | Example |
|-----------|------|-------------|---------|
| `name` | `string` | Task identifier | `"process_data"` |
| `compute` | `string` | CPU level: `"LOW"`, `"MEDIUM"`, `"HIGH"` | `"MEDIUM"` |
| `ram` | `string` | Memory limit | `"512MB"`, `"2GB"` |
| `timeout` | `string` or `number` | Maximum execution time | `"30s"`, `"5m"` |
| `maxRetries` | `number` | Retry attempts on failure | `3` |

### Compute Levels

- **LOW**: Minimal allocation for lightweight tasks
- **MEDIUM**: Balanced resources for typical workloads
- **HIGH**: Maximum fuel for compute-intensive operations
- **CUSTOM**: Specify exact fuel value (e.g., `compute="1000000"`)

## Compatibility

✅ **Supported:**
- npm packages and ES modules work

⚠️ **Not yet supported:**
- Node.js built-ins (`fs`, `path`, `os`) are not available in the sandbox

## Links

- [GitHub](https://github.com/mavdol/capsule)
- [Issues](https://github.com/mavdol/capsule/issues)

## License

Apache-2.0
