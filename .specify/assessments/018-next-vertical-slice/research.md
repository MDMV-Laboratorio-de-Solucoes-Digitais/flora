# Research: 018 — File hardening, DB-gated contracts, and wiring readiness

**Date:** 2026-11-16  
**Method:** synthesis of completed multilingual research (EN, pt-BR, es, de, fr, zh-CN, ja) + codebase audits; complementary Macro/outbox research still in-flight.

## Completed inputs
- `docs/research/2026-11-16-018-file-contracts-hardening-vps.md`: DB-gated contract/integration matrix (C1–C8, I1–I6) on disposable PG18, deterministic fixtures, env-gated strategy, and minimal hardening priorities (workspace ownership/authz, multipart robustness, quota aggregate/race-safety, storage containment parity, delete/purge consistency, metadata-only policy).
- `docs/research/2026-11-16-018-search-notifications-wiring-vps.md`: Postgres-first FTS/`pg_trgm` and in-app notifications as 80/20 for Seed/VPS; proposes 018-S1 (PG FTS MVP for messages + one notification event) but defers outbox/workers/push/backfill and notes contract/crate/NV risks.
- Audit `ses_e1d8d03e6ffehEXmO6kEraL1aX` (readiness): recommends **A1** (file authz/tenancy hardening + DB-gated contracts) over B for 018 due to localized high-value security/contract gap and near-zero file coverage; B needs product/crate decisions.
- Audit `ses_e1d8b443cffe9g6ewGSBRnZNNn` (file hardening): confirms gaps — upload/legacy metadata accept `workspace_id` without verifying workspace exists/belongs to org or caller workspace access; read/download/delete are org-scoped (not workspace-scoped) with anti-enumeration behavior; multipart field-order/shape fragility; quota is per-file ceiling (no aggregate usage/reservation), delete best-effort storage-before-soft-delete and purge DB-only, metadata-only path bypasses bytes/checksum invariants, storage trait containment weaker than stream/open, nullable workspace linkage.

## Codebase cross-check (current)
- `flora-api/src/routes/files.rs`: org membership via `ensure_member`; multipart upload + metadata endpoints require `workspace_id` but do not validate workspace∈org/membership; get/download require org match + not deleted, delete owner-only; `POST /` metadata-only still present.
- `flora-core/src/services/file_service.rs`, `repositories/file_repository.rs`, `storage/local.rs`, `config/storage.rs`, migrations `20240818000001_init_schema.sql` + `20261116000002_add_workspace_id_to_files.sql`: streaming local storage/checksum implemented (017) but quota aggregate/accounting and workspace ownership enforcement incomplete; repo has `find_by_workspace_id(org, ws, pagination)` unused by API; soft-delete/purge/storage cleanup semantics partial.
- `flora-tests/src/contract/files.rs`, `flora-tests/src/integration/files.rs`: placeholders/stubs; DB-gated harness patterns exist in `contract/auth.rs`, `messaging.rs` (env-gated `FLORA_DATABASE__POSTGRES_URL`, migrations, seeded state, router `oneshot`).

## Evidence-based 80/20 conclusion
- **Highest value/risk fit for 018:** A1 — harden file authz/tenant/workspace invariants and add DB-gated contracts on disposable PG18 (addresses 017 NVs and concrete security/consistency gaps with deterministic verification).
- **Valuable but riskier/broader:** PG FTS + 1 notification event wiring requires contract/crate/tx decisions and wider mutation touch-points; better as 019 or tightly scoped follow-up after file invariants/contracts are locked.
- **In-flight inputs:** Macro search/notifications parity and PG search/outbox research may refine 019; do not block 018 A1 given sufficient convergent evidence.
