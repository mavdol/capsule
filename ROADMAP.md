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

## v0.3.0: Data Access (Filesystem)

**Status:** ✅ Done

**Goal:** Enable agents to work with local files and datasets.

- [x] **Filesystem:** Local file mounting (`fs_access`) for reading images, CSVs, and datasets.

---

## v0.4.0: Configuration & Observability

**Status:** 🚧 Planned

**Goal:** Improve developer workflow with project configuration and richer task feedback.

- [ ] **Config File:** `capsule.toml` for default task options.
- [ ] **Structured Output:** Tasks return a detailed JSON envelope with execution metadata:

```json
{
  "success": true,
  "result": "<task return value>",
  "error": null,
  "execution": {
    "task_name": "analyze_data",
    "duration_ms": 1523,
    "retries": 0,
    "fuel_consumed": 45000
  }
}
```

On failure:

```json
{
  "success": false,
  "result": null,
  "error": {
    "type": "timeout",
    "message": "Task exceeded 30s timeout limit"
  },
  "execution": {
    "task_name": "analyze_data",
    "duration_ms": 30000,
    "retries": 1,
    "fuel_consumed": null
  }
}
```

---


