# Research: 023 — R1 synthesis (search direction)

**Sources:** 023 R1 research docs (6), codebase audit, frontend/ops audit, 022 artifacts, constitution.

## Convergence
- **Debloat/PG-only (`docs/research/2026-11-16-023-search-debloat-pg-only-vps.md`):** Meilisearch's own docs concede PG FTS may suffice for small datasets/basic keyword/no extra infra — Flora matches. Indexing defaults (2/3 RAM, 1/2 cores) compete with Postgres on 2 vCPU/4 GB; OOM is a documented failure mode. Recommendation: PG FTS as sole supported Seed backend; Meilisearch dormant, explicitly non-authoritative opt-in.
- **Constitution:** Seed = Flora + PostgreSQL; Meilisearch is a Grove-tier component. Reinforces the debloat direction.
- **Hooks audit:** Meilisearch index has **zero write call sites**; `setup_index` is never called; type-token mismatch (plural filter vs singular docs) would yield silent zero results if populated. Therefore the Meilisearch path is currently non-functional, not merely stale — making it authoritative is a correctness risk.
- **Task-await research:** SDK 0.33 `wait_for_task` can return `Ok(Task::Failed)` (must check `is_success()`); `Client::new` performs no network I/O, so an unreachable Meili still yields `Some(search_service)`. Deterministic Meili tests require an awaitable wrapper and a separate advisory CI job — out of 80/20 scope.
- **Rebuild research:** blue-green via `/swap-indexes` and UUIDv7 keyset scans are viable *if* Meilisearch is re-enabled later; outbox-lite requires a transaction refactor first. Both deferred.
- **Macro reference:** replay-over-coordination, stable aliases, generation fences; notification idempotency via caller-supplied UUID (024+ candidate).
- **Frontend/ops audit:** canonical FE has no search/notifications/task-edit consumers → backend-only. No rebuild tooling or Meili CI exists; config drift (`enabled=true` in code vs `false` in `.env.example`) identified.

## Decision drivers
- **Correctness:** PG FTS is transactional (generated `tsvector` + partial GIN), tenant/soft-delete safe, and already CI-enforced. Meilisearch is unwritten and can serve empty-but-successful results (022 fail-open only covers query errors).
- **Cost/80/20:** hooks + normalization + awaits + rebuild + CI harness ≈ large surface for a component outside the Seed tier.
- **Reversibility:** keeping `flora-search` and the compose `search` profile dormant makes re-enablement a gated 024+ decision with documented design.

## Key risks/NVs
- Removing the Meilisearch request path is an additive contract change (`source` field) with no FE consumer; documented as explicit capability contract.
- PG FTS relevance (hyphen/CJK/trigram) remains imperfect; separator normalization addresses the common hyphen case cheaply.
- Meilisearch re-enablement requires hooks, awaits, rebuild, and deterministic CI (documented NVs).
