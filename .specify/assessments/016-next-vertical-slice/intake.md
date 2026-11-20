# Intake: 016 — Next vertical slice

**Date:** 2026-11-16  
**Cycle:** 016 (pre-implementation assessment)  
**Repo:** `/home/luis/development/MDMV/projetos/Arquivados/flora` (`dev`, post-015/PR #218)  
**Task constraint:** Assessment and SDD artifacts only — **no application source modified, no commits**.

## Objective (80/20)

Select and scope **one** primary 80/20 vertical slice for cycle 016 that best advances Flora’s north star as a **self-hosted, debloated Macro alternative for SMBs runnable on a cheap VPS (2 vCPU / 4 GB RAM, SSD, single-node)** with **Rust + PostgreSQL + Svelte 5**, while preserving verifiable increments.

## Decision space (requested)

Compare:

- **(B) Minimal safe binary file upload/download** — Axum streaming multipart, Postgres metadata, local disk storage, quotas/path safety/checksums, aligned FE contract.
- **(C) Seed/VPS operability hardening** — make optional services truly optional at boot/config (Meilisearch/Valkey), compose resource limits/healthchecks/profiles, `FLORA_*` env + pool alignment, backup/restore basics.

## Context anchors (read)

- `OBJETIVO.md`, `ROADMAP.md` (015 merged, 016 candidates B/C).
- `.specify/assessments/015-next-vertical-slice/decision.md` (go: slice A realtime/catch-up).
- Research: `docs/research/2026-11-16-015-binary-file-upload-download.md`, `2026-11-16-015-files-codebase-audit.md`, `2026-11-16-015-files-impl-readiness.md`, `2026-11-16-015-seed-vps-operability-quickwins.md`, `2026-11-16-015-macro-debloat-vps-prioritization.md`, `2026-11-16-seed-vps-profile-audit.md`, `2026-11-16-014-seed-vps-operability-international.md`.

## Repo reality (evidence highlights)

- **B (files):** API is metadata-only JSON (`flora-api/src/routes/files.rs`), core `FileService` records DB row without bytes/storage wiring, FE expects `/api/v1/files/upload` (multipart) and `/api/v1/files/download/{id}` (`apps/frontend/src/lib/components/files/*`), schema/config gaps (workspace linkage/quota semantics/path safety). Larger security/contract surface.
- **C (operability):** `flora-api/src/main.rs` creates pool+migrates then `AppState::new` creates another pool; `flora-core/src/database.rs` hardcodes `max_connections(20)` ignoring config; `AppState` hard-fails Meilisearch init despite `search.enabled`/“optional MVP” intent; messaging config requires `valkey_url` while runtime tolerates missing Valkey; `docker-compose.yml` has no profiles/resource limits and exposes all three services by default; `.env.example` mixes `FLORA_*__*` and non-prefixed OIDC vars/comments; PG18 mandatory (`uuidv7`), backup/runbook not operationalized for Seed.

## Constraints / guardrails

- Rust 2024 workspace, strict lints, `Result`/`tracing`, crate-local changes preferred.
- Postgres-first, no new mandatory services, no proprietary deps, no Macro parity/P2 scope.
- Cheap VPS: 2 vCPU / 4 GB RAM budget; avoid JVM/search/broker always-on defaults.
- Deterministic, env-gated tests where DB needed; do not claim NVs as verified.
- Skills discovery attempted: `npx --yes skills find "product research prioritization self-hosted VPS"` (no directly applicable installed skill used; assessment performed via repo + authoritative docs).

## Deliverables

This folder: `intake.md`, `research.md`, `problem.md`, `concept.md`, `decision.md` with `go`/scope verdict, evidence, risks/NVs, debloat notes, file-by-file implementation checklist, deterministic verification plan, explicit non-goals.