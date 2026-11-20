# Intake: 022 — Next vertical slice (post-021)

**Date:** 2026-11-16  
**Baseline:** `dev@b5be7d16f` (021 merged, PR #224, CI green).  
**Objective:** evolve Flora as a viable, performant, self-hosted Macro-like alternative for SMBs on cheap VPS (Rust + PostgreSQL, Svelte 5; Tauri later) via 80/20 debloat and verified vertical slices.

## Inputs
- `OBJETIVO.md` / `ROADMAP.md`: 022 candidate = paridade Meilisearch e robustez de notificações/busca — (1) wiring mínimo de indexação/mutação para `task/file` (e consistência de entidades núcleo) no modo Meilisearch **ou** contrato operacional explícito de degradação/paridade (rebuild/reindex, delete-on-soft-delete), mantendo fallback PG determinístico e tenant/soft-delete safe; (2) endurecer idempotência/dedup (incl. janela/condição de corrida quando viável), avaliar evento/semântica específica de unassign se contrato exigir, e ampliar cobertura de transições/concorrência com contratos DB-gated; (3) contratos DB-gated (PG descartável) e, quando viável, harness/gates Meilisearch determinísticos para paridade/precedence; (4) CI verde.
- 021 residuals/NVs: dedup unread-only/race-prone (query-before-insert), Meilisearch task/file indexing parity/freshness and Meilisearch-gated tests deferred, PG fallback authoritative when Meilisearch unavailable.
- R1 research (022): `docs/research/2026-11-16-022-meilisearch-indexing-hooks-reindex-parity-vps.md`, `docs/research/2026-11-16-022-meilisearch-pg-coherence-degradation-contract-vps.md`, `docs/research/2026-11-16-022-notification-idempotency-unassign-semantics.md`, `docs/research/2026-11-16-022-db-meilisearch-gated-test-harness.md`, `docs/research/2026-11-16-022-macro-search-notification-reference-flows.md`, `docs/research/2026-11-16-022-meilisearch-vps-resource-ops.md`.
- R1 audits: codebase (`ses_e1c0bc4e5`), frontend canônico (`ses_e1c0bc4c8`, sem consumidores search/unassign-edit/notifications → slice backend-only sem mudança de FE).

## Candidates (80/20)
1. **S1 — DB-enforced notification dedup + search fail-open degradation contract + CI DB-gated enforcement (recommended):** partial unique index for unread notifications `(user_id,event_type,target_id) WHERE is_read=false` with conflict-safe insert (`ON CONFLICT DO NOTHING`) to eliminate TOCTOU/race duplicates; keep U0 unassign semantics and best-effort post-persist delivery. On Meilisearch query error, fall back to PG FTS (fail-open to correctness baseline) while keeping current precedence when healthy; explicitly record mutation-indexing parity/freshness as deferred NV. Extend CI `db-tests` to run `contract::search::db_`, `contract::notifications::db_`, `contract::files::db_`.
2. **S2 — Minimal Meilisearch mutation hooks (task/file/message) + delete-on-soft-delete + rebuild path:** improves freshness but requires task-awaitable Meilisearch APIs, parity normalization, and deterministic Meilisearch-gated harness/CI not present; higher risk on 2vCPU/4GB.
3. **S3 — Outbox-lite indexing/notifications + tx-coupling:** strongest durability/consistency but adds schema/runtime complexity beyond Seed 80/20.

## Constraints
- Seed/VPS (~2 vCPU/4 GB): no new mandatory services, Postgres-first, cheap deterministic tests, additive/safe migrations, conservative runtime behavior.
- AGPL-safe, preserve contracts where feasible, no blind merge, CI green required, keep only `dev`/`main` (+ `testing`/`staging` if existing), stage explicit paths.
- NV: Meilisearch mutation hooks/indexing parity, async task wait/rebuild tooling, Meilisearch-gated CI, cross-engine ranking/snippet parity, `pg_trgm`/i18n/CJK relevance, search ACL; outbox/tx-coupling, unassign-specific event/recipient semantics, coalescing upsert policy, realtime/push/mentions/completion; 2vCPU/4GB load/OOM/restore benchmarks; browser e2e/multi-client.
