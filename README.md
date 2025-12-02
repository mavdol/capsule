# 📦 Capsule

![License](https://img.shields.io/badge/license-Apache_2.0-blue)
![Rust](https://img.shields.io/badge/built_with-Rust-orange)

**Fine-grained isolation and orchestration for multi-agent systems**


> ⚠️ **Status**: Request for Comments (RFC) Capsule is currently in active research & development. The code in this repository represents the Architecture Design and API Specification.
>
> I am building the core Rust engine in public.<br>
> 👉 **Subscribe to the [Engineering Log](https://capsuleruntime.substack.com)** to follow the journey from design to v0.1.

## 🎯 What is Capsule?

Capsule is a Wasm-based runtime that runs each AI agent task in an isolated sandbox with resource limits and full execution tracing. Think of it as a specialized orchestrator for safely running thousands of agent workflows in parallel.

## ⚡ Why Capsule?

When you scale from one agent to thousands running in parallel, simple scripts break down:
- **Security:** How do you isolate untrusted code safely?
- **Resources:** How do you track which tasks are consuming resources?
- **Observability:** How do you debug failures in distributed workflows?

**Capsule** is a specialized runtime that gives you fine-grained control over agent execution through WebAssembly isolation, real-time monitoring, and complete task traceability.

---

## 💻 Quick Example

Define isolated, crash-proof agent tasks with simple decorators.

```python
from capsule import task

@task(
    name="web_search",
    compute="MEDIUM",
    timeout="5m",
    max_retries=3
)
def search_web(query: str) -> dict:
    results = search_api.query(query)
    return {"results": results, "count": len(results)}

@task(name="summarize", compute="HIGH", ram="512MB")
def summarize_results(data: dict) -> str:
    return llm.summarize(data["results"])

# Orchestrate tasks - each runs in isolation
async def research_agent(topic: str):
    data = await search_web(topic)
    summary = await summarize_results(data)
    return summary
```

**What you get:**
- 🛡️ Sandboxing: Each task runs in its own Wasm instance.
- 📉 Metering: Automatic CPU & RAM resource limiting.
- 🔍 Traceability: Full execution traces for debugging.
- 💾 Fault Tolerance: Built-in retry logic and state persistence.

👉 See [/examples](https://github.com/mavdol/capsule/tree/main/examples) for complete multi-agent workflows.

---

## 🛠️ Technical Stack (Planned)

Capsule bridges your language with the safety of Rust.

- **🦀 Core Engine**: Written in **Rust** (Tokio) for high-performance async scheduling.
- **💾 Persistence**: Uses a **Write-Ahead Log (WAL)** to persist every step to disk.
- **🛡️ Sandboxing**: Executes agent logic inside **WebAssembly** (Wasmtime) containers for isolation.

---

## 📌 Roadmap
### v0.1.0: The Python Runner

**Status:** 🚧 In Progress

**Goal:** Validating the core Host/Guest technology and Python interoperability.

- [ ] **Core:** Rust Host capable of loading Wasm Components.
- [ ] **SDK (Python):** Basic `@task` decorator with JSON serialization.
- [ ] **CLI:** `capsule run main.py` with JIT compilation.
- [ ] **Limits:** Basic Fuel metering for CPU protection.

Check the full [Roadmap](ROADMAP.md) for details.

---

## 🤝 Contributing
Capsule is not yet ready for code contributions, as the core engine is being scaffolded. However, I am looking for feedback on the API Design.

- Does the Python syntax feel idiomatic?
- What features from LangChain/CrewAI/Temporal would you want?
- What isolation/monitoring capabilities matter most for your use case?

Open an Issue to discuss

---

## 📄 License

This project is licensed under the Apache 2.0 License

see the [LICENSE](LICENSE) file for details.

---
