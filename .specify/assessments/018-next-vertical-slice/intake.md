# Intake: 018 — Next vertical slice (post-017)

**Date:** 2026-11-16  
**Baseline:** `dev@b6440c534` (017 slice B merged, PR #220, CI green).  
**Objective:** advance Flora as a viable, performant, self-hosted Macro-like alternative for SMBs on cheap VPS (Rust + PostgreSQL, Svelte 5; Tauri later) via 80/20 debloat and verified vertical slices.

## Inputs
- `.specify/assessments/017-next-vertical-slice/decision.md` and 017 artifacts: slice B (binary upload/download) delivered; NVs include no DB-gated file contracts (PG18), aggregate quota minimal/concurrency NV, workspace membership/authz not deeply validated in file routes, no 2vCPU/4GB/proxy/load/restore evidence.
- `OBJETIVO.md` / `ROADMAP.md`: 018 candidate = "contratos DB-gated/hardening do slice de arquivos e wiring essencial busca/notificações" (Seed/Postgres-first, verificável).
- Research (completed): `docs/research/2026-11-16-018-file-contracts-hardening-vps.md`, `docs/research/2026-11-16-018-search-notifications-wiring-vps.md`, `docs/research/2026-11-16-018-postgres-search-outbox-vps.md`.
- Audits (completed): 018 codebase readiness (`ses_e1d8d03e6ffehEXmO6kEraL1aX`, recommends A1 file hardening+DB-gated contracts), focused 017 hardening gap audit (`ses_e1d8b443cffe9g6ewGSBRnZNNn`).
- Pending complementary research (in-flight): `docs/research/2026-11-16-018-macro-search-notifications-parity.md` (inform 019+/deferred wiring; not blocking 018 A1).

## Candidates (80/20)
1. **A1 — File slice hardening + DB-gated contracts (PG18):** enforce workspace∈org and consistent authz/tenant checks on upload/get/download/delete/legacy metadata paths, harden multipart/validation semantics, add minimal aggregate quota/consistency safeguards where feasible, and implement deterministic DB-gated contract/integration tests with disposable PG + temp storage.
2. **B — Search/notifications wiring (minimal):** Postgres FTS fallback/path for messages (+ optional tasks/files) and/or 1 high-value in-app notification event wired from mutations, with DB-gated tests; outbox/Meilisearch hooks/workers deferred.
3. **A1+B combined:** higher risk/surface for one cycle; not preferred.

## Constraints
- Seed/VPS (~2 vCPU/4 GB): no new mandatory services, cheap/fast deterministic tests, conservative behavior.
- AGPL-safe, Postgres-first, no blind merge, CI green required, keep only `dev`/`main`, stage explicit paths.
- NV: real load/OOM/proxy/restore drills, browser e2e/multi-client, full search relevance/i18n tuning, outbox/workers/push delivery.
