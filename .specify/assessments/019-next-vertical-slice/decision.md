# Decision: 019 — Next vertical slice

**Verdict:** **go** (scoped)

## Chosen slice

**S1 — Postgres FTS fallback for messages + reply-in-thread in-app notification (disposable PostgreSQL 18, Seed/VPS).**  
Primary 80/20 for cycle 019: make `/api/v1/search` usable without Meilisearch via additive PG FTS for messages (stable contract, tenant-safe, degraded-to-PG instead of 503) and emit one high-value in-app notification (`Reply` to parent message author on threaded reply) through existing core notification storage/API, verified by DB-gated tests.

## Justification (evidence)

- **Roadmap fit:** `OBJETIVO.md`/`ROADMAP.md` define 019 as essential search/notifications wiring (Postgres-first, verifiable, no new mandatory services).
- **Convergent research/audits:** `docs/research/2026-11-16-019-postgres-fts-search-vps.md`, `docs/research/2026-11-16-019-inapp-notification-wiring-vps.md`, `docs/research/2026-11-16-019-macro-mutation-search-notification-flows.md`, 018 wiring research, and audits (`ses_e1d793fa1ffeg18iLwVe1BQrV1`, `ses_e1d77e0a0ffe9WZE984FSoaAv7`, `ses_e1d78fb75ffezHWpPzuDQ60xJS`) converge on messages-only FTS + reply-in-thread as the smallest verifiable, high-value, low-infra slice; frontend requires no change for contract-compatible backend delivery.
- **Value/80/20:** restores search availability on lean/cheap VPS and adds a concrete collaboration feedback loop using existing schemas/APIs, with deterministic DB-gated evidence and no Kafka/SQS/OpenSearch/outbox/workers/push.

## Scope summary (in)

- Additive migration: `messages.search_tsv` (`tsvector`, generated/stored, `'simple'`) + partial GIN index for non-deleted rows.
- PG search implementation and fallback wiring (state/config/route) preserving `GET /api/v1/search` contract and tenant checks; degrade to PG when Meilisearch unavailable/disabled.
- Reply notification hook in message creation path: notify parent author on valid threaded reply (`NotificationType::Reply`), with self/cross-scope/invalid-parent guards and best-effort/non-fatal behavior.
- DB-gated contract/integration tests on disposable PG18 for search (match, isolation, empty/type behavior, no-Meilisearch availability) and reply notifications (create, no self/invalid cases, API visibility).
- SDD artifacts, local verification evidence, PR/CI green, merge/sync, planning updates.

## Key risks

1. **Fallback/config semantics drift:** mitigate with additive config handling and contract-preserving route behavior/tests.
2. **FTS relevance/perf/tokenization limits:** mitigate with messages-only scope, fixed `'simple'`, capped/ranked queries; record NVs (CJK, 2vCPU/4GB, channel ACL).
3. **Notification correctness/duplication/tx boundaries:** mitigate with strict guards, best-effort side effect, deterministic DB-gated assertions; document NVs.

## NVs / non-goals

- NV: tasks/files search, `pg_trgm`, relevance/i18n tuning and zh-CN/ja quality, channel-level ACL in search, outbox/workers/backfill/push/realtime delivery, mention/assignment/completion fanout, dedup uniqueness (unless trivial), 2vCPU/4GB perf/load/restore benchmarks, browser e2e/multi-client.
- Non-goals: remove Meilisearch, frontend UI for search/notifications, crate convergence, multi-node infra, new mandatory services.
