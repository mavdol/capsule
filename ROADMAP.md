# 📌 Roadmap

This document tracks the development status of Capsule.

> [!NOTE]
> This roadmap reflects current priorities. Feature requests and ideas are welcome in [Issues](https://github.com/mavdol/capsule/issues).

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

**Status:** ✅ Done

**Goal:** Stability, security, and expanding the ecosystem.

- [x] **TypeScript SDK:** Functional wrapper support (`const t = task(...)`).
- [x] **Better Logs:** Better logging for debugging and monitoring.
- [x] **Distribution:** CI/CD for binaries and Package Registries (PyPI, npm, crates.io).

---

## v0.3.0: Data Access

**Status:** 📅 Planned

**Goal:** Enable agents to work with local files and datasets.

- [ ] **Filesystem:** Local file mounting (`fs_access`) for reading images, CSVs, and datasets.

---


