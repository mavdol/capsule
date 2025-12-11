# 📌 Roadmap

This document tracks the development status of Capsule. We follow a "Release Early, Release Often" philosophy.

---

## v0.1.0: The Python Runner (MVP)

**Status:** 🚧 In Progress

**Goal:** Validating the core Host/Guest technology and Python interoperability.

- [x] **Core:** Rust Host capable of loading Wasm Components.
- [x] **SDK (Python):** Basic `@task` decorator with JSON serialization.
- [x] **CLI:** `capsule run main.py` with JIT compilation.
- [x] **Limits:** Basic Fuel metering for CPU protection.

---

## v0.2.0: The Orchestrator

**Status:** 📅 Planned

**Goal:** Turning the runner into a stable, persistent system.

- [ ] **Daemon Mode:** Implement the client-server architecture and IPC communication.
- [ ] **Basic Config:** Support `capsule.toml` to identify projects (Namespacing).
- [ ] **Management:** `list` (Tree View), `restart`, and `stop` commands.
- [x] **Resilience:** Retry logic and real Timeout handling (Wall clock).

---

## v0.3.0: The AI Infrastructure (Data & GPU)

**Status:** 📅 Planned

**Goal:** Powering Data Science and heavy workloads.

- [ ] **Hardware:** Experimental **GPU** support via WASI-NN (Local Inference).
- [ ] **Data:** Local file mounting (`fs_access`) for passing images/CSVs.
- [ ] **Observability:** Structured telemetry output (`traces.jsonl`).

---

## v0.4.0: Production & Polyglot

**Status:** 📅 Planned

**Goal:** Stability, security, and expanding the ecosystem.

- [ ] **TypeScript SDK:** Functional wrapper support (`const t = task(...)`).
- [ ] **Rust SDK:** Native support via `#[task]` macros.
- [ ] **Go SDK:** Registered function support.
- [ ] **Security:** Full `capsule.toml` support (Network Whitelisting).
- [ ] **Distribution:** CI/CD for binaries and Package Registries (PyPI, npm, crates.io).
