# 📌 Roadmap

This document tracks the development status of Capsule.

> Note: Features marked as Planned are subject to change based on community feedback and real-world usage patterns.

---

## v0.1.0: The Python Runner

**Status:** ✅ Done

**Goal:** Validating the core Host/Guest technology and Python interoperability.

- [x] **Core:** Rust Host capable of loading Wasm Components.
- [x] **SDK (Python):** Basic `@task` decorator with JSON serialization.
- [x] **CLI:** `capsule run main.py` with JIT compilation.
- [x] **Limits:** Basic Fuel metering for CPU protection.
- [x] **Resilience:** Retry logic and real Timeout handling (Wall clock).

---

## v0.2.0: Production & Polyglot

**Status:** 🛠️ In progress

**Goal:** Stability, security, and expanding the ecosystem.

- [x] **TypeScript SDK:** Functional wrapper support (`const t = task(...)`).
- [ ] **Distribution:** CI/CD for binaries and Package Registries (PyPI, npm, crates.io).

---

## v0.3.0: The AI Infrastructure (Data & GPU)

**Status:** 📅 Planned

**Goal:** Powering Data Science and heavy workloads.

- [ ] **Hardware:** Experimental **GPU** support via WASI-NN (Local Inference).
- [ ] **Data:** Local file mounting (`fs_access`) for passing images/CSVs.

---

