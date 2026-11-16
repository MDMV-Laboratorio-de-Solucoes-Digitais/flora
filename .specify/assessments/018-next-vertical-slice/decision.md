# Decision: 018 — Next vertical slice

**Verdict:** **go** (scoped)

## Chosen slice

**A1 — File slice hardening + DB-gated contracts (disposable PostgreSQL 18, Seed/VPS).**  
Primary 80/20 for cycle 018: harden 017 file invariants (workspace∈org validation and consistent authz/tenant checks, robust multipart/contract semantics, explicit minimal quota/cleanup behavior) and replace placeholder file tests with deterministic DB-gated contract/integration coverage using disposable PG + isolated temp storage.

## Justification (evidence)

- **Roadmap/NV fit:** `OBJETIVO.md`/`ROADMAP.md` and 017 assessment/validation mark DB-gated file contracts and hardening (workspace membership/authz, quota coherence, storage/error semantics) as P0 NVs for the next cycle.
- **Convergent research/audits:** `docs/research/2026-11-16-018-file-contracts-hardening-vps.md` provides C1–C8/I1–I6 matrix and minimal hardening priorities; readiness audit (`ses_e1d8d03e6ffehEXmO6kEraL1aX`) and focused audit (`ses_e1d8b443cffe9g6ewGSBRnZNNn`) confirm concrete gaps in `flora-api/src/routes/files.rs`, `flora-core/src/services/file_service.rs`, repository/storage/config, and placeholder `flora-tests` file contracts, recommending A1 as highest value/lowest risk for 018.
- **Value/80/20:** secures and verifies a core, security-sensitive P0 path (binary files) with deterministic evidence and no new mandatory services, aligned with cheap-VPS/Postgres-first debloat goals.
- **Deferred alternative:** search/notifications wiring (018-S1 ideas in `docs/research/2026-11-16-018-search-notifications-wiring-vps.md`, `2026-11-16-018-postgres-search-outbox-vps.md`, `2026-11-16-018-macro-search-notifications-parity.md`) is valuable but broader (contract/crate/tx/product decisions); defer to 019 unless trivially isolated.

## Scope summary (in)

- Enforce workspace existence + belongs-to-org (and policy-consistent caller access) for upload/legacy metadata and consistent tenant/workspace-aware checks for read/download/delete.
- Harden multipart parsing/validation (field order tolerance, single-file/required-field determinism) and keep FE-compatible responses/URLs; make legacy metadata-only behavior explicit/tested.
- Make quota/integrity/cleanup semantics explicit: per-file caps + checksum/size as implemented; add minimal aggregate usage guard if feasible, else codify/test per-file/config behavior and record NV; improve best-effort storage/DB consistency on failure/delete/purge paths where low-risk.
- Add DB-gated contract/integration tests (disposable PG18, migrations, seeded fixtures, temp storage, multipart helpers) covering happy paths, isolation/authz, validation/limits, and cleanup/delete/quota cases per implemented semantics.
- SDD artifacts, verification evidence, PR/CI green, merge/sync, planning updates.

## Key risks

1. **Behavioral tightening breaks implicit flows:** mitigate with explicit error policy, compatibility-minded tests, and minimal surface change.
2. **Concurrency-correct aggregate quota complexity:** mitigate with minimal transactional/advisory approach or clearly scoped/tested per-file semantics + NV.
3. **DB-gated test reliability:** mitigate with env-gated, isolated, deterministic fixtures and cheap payloads.

## NVs / non-goals

- NV: 2vCPU/4GB load/OOM/proxy-limit/restore drills, browser e2e, full concurrent aggregate quota proof (unless implemented), AV/infer-grade sniffing, resumable/S3/presigned, content indexing/previews, multi-node storage, outbox/workers/push, frontend redesign.
- Non-goals: Macro search/notification parity, broad crate/service refactors, new mandatory services.
