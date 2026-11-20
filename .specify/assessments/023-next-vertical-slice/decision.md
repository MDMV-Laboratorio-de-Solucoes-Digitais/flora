# Decision: 023 — Next vertical slice

**Verdict:** go (scoped)

## Chosen slice

**S1 — Postgres-authoritative search contract + cheap PG FTS hardening (Seed/Postgres-first).** 023 makes PG FTS the sole authoritative backend for `/api/v1/search`, declares Meilisearch a dormant/non-authoritative opt-in, adds an explicit `source` capability field and deterministic DB-gated contracts, normalizes FTS separators, and aligns config/docs.

## Justification
- Roadmap candidate explicitly allows an "operational capability contract + rebuild determinism, preserving authoritative PG fallback" instead of full Meilisearch wiring.
- Constitution defines Seed as Flora + PostgreSQL; Meilisearch is Grove-tier. Debloat is a first principle.
- R1 research converged: the Meilisearch path is unwritten (zero hooks, no `setup_index`) and has a latent plural/singular type-filter bug; serving it as authoritative is a correctness defect. Hooks + awaits + rebuild + CI harness are large effort for a non-Seed component.
- PG FTS is transactional, tenant/soft-delete safe, already CI-enforced, and requires no new services.

## Scope (in)
- `flora-api/src/routes/search.rs`: always call `search_pg`; add `source: "postgres"` to `SearchResponse`; document dormant Meilisearch.
- Additive migration normalizing separators in `messages/tasks/files.search_tsv` generated expressions (+ indexes).
- `flora-core/src/config/search.rs`: default `enabled = false`; `.env.example` search keys; `docs/dev-smoke.md` capability/degradation note.
- DB-gated contracts: authoritative `source` even with a configured-but-unreachable `SearchService`; hyphenated-name sub-token match; preserve existing tenant/deleted/type/limit invariants.
- SDD/validation/release/planning artifacts; PR/CI green; merge/sync; plan 024. No FE changes.

## Risks
1. Removing the Meili branch changes response shape additively (`source`); no FE consumer, but update contracts.
2. Generated-column recreation must preserve partial GIN indexes; verified on disposable PG18.
3. PG FTS relevance remains NV beyond separator normalization.

## NVs / non-goals
- NV: Meilisearch mutation hooks, delete-on-soft-delete, task-await wrapper, rebuild/backfill tooling, blue-green swap, Meilisearch-gated CI, cross-engine parity, outbox/tx-coupling.
- NV: `pg_trgm`/i18n/CJK relevance, search ACL, 2vCPU/4GB perf/load/OOM/restore, frontend adoption/e2e.
- NV: deterministic notification UUIDs/coalescing, unassign events, mentions/push (024 candidates).
- Non-goals: new mandatory services, crate removal, FE changes.
