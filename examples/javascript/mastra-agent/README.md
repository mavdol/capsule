# Mastra + Capsule Code Execution

This example demonstrates how to integrate [Capsule](https://github.com/mavdol/capsule) with [Mastra](https://mastra.ai/) to safely execute AI-generated code in a sandboxed environment.

## How it works

1. The Mastra agent receives a task from the user
2. It generates JavaScript code to solve the problem
3. The code is executed in a Capsule sandbox via `execSync`
4. Results are returned safely to the user

> [!TIP]
> Check [`capsule.ts`](./capsule.ts) to see how the sandbox task executes code.

## Quick Start

```bash
npm install
npm run dev
```

Open http://localhost:4111 and try: *"Calculate fibonacci of 15"*

> [!NOTE]
> In v0.6.0, we'll release an SDK helper to simplify subprocess execution.
