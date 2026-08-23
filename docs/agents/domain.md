# Domain Documentation

This repository uses a **multi-context** layout for domain documentation to support its monorepo structure (`flora-*` crates and `apps/*`).

## Layout & Rules

- **Root Map:** A `CONTEXT-MAP.md` at the root of the repository indexes the various packages and points to their specific contexts.
- **Per-Package Context:** Individual packages contain their own `CONTEXT.md` detailing specific constraints, architecture, and testing rules.
- **ADRs (Architecture Decision Records):** Stored globally in `docs/adr/` at the root, or locally within a specific package's `docs/adr/` directory.
- **Agent Rule:** When working on a task, always consult the root `CONTEXT-MAP.md` and the relevant package's `CONTEXT.md` before making architectural changes.
