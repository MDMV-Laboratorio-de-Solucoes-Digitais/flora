# Feature Specification: 023 — Postgres-Authoritative Search Contract

**Feature Branch**: `feat/023-postgres-authoritative-search`  
**Created**: 2026-11-16  
**Status**: In progress  
**Assessment**: `.specify/assessments/023-next-vertical-slice/decision.md` (go, S1)

## Goal
Eliminate the correctness risk of an unwritten, healthy-looking Meilisearch index being authoritative: make Postgres FTS the sole authoritative backend for `GET /api/v1/search`, document Meilisearch as a dormant non-authoritative capability, and harden PG FTS for common filename patterns — with deterministic DB-gated contracts and no new services.

## Scope

### Search authority + capability contract
- `GET /api/v1/search` always executes `search_pg` (messages/tasks/files). The Meilisearch request branch is removed; `flora-search`/compose `search` profile remain dormant for gated re-enablement.
- `SearchResponse` gains `source: "postgres"` (additive; no canonical FE consumer).
- Keep tenant/membership checks, empty-query behavior, type normalization, limit cap, snippets, and PG FTS invariants unchanged.

### PG FTS hardening
- Additive migration recreating generated `tsvector` expressions for `messages`, `tasks`, `files`, replacing `-`, `.`, `_` with spaces before `to_tsvector('simple', ...)`, and recreating the partial GIN indexes `WHERE is_deleted = false`. Derived columns only — no source data loss.

### Config/docs
- `SearchConfig::default().enabled = false` to match `.env.example` and the Seed profile.
- Document all `FLORA_SEARCH__*` keys in `.env.example`; add the capability/degradation contract and Meilisearch re-enablement gates to `docs/dev-smoke.md` (or focused doc).

### Deterministic verification
- DB-gated contract: response `source == "postgres"` even when `AppState.search_service` is `Some(...)` pointing at an unreachable endpoint (no Meili service required; `Client::new` is lazy).
- DB-gated contract: hyphenated file/task names match sub-token queries after separator normalization.
- Existing search/notifications/files/auth/messaging/websocket DB-gated suites remain green; CI `db-tests` unchanged.

## Non-goals
- Meilisearch mutation-indexing hooks, delete-on-soft-delete, task-await wrapper, rebuild/backfill/blue-green swap, Meilisearch-gated CI, cross-engine ranking/snippet parity.
- Outbox/tx-coupling, deterministic notification UUIDs, unassign events, mentions/push/realtime.
- `pg_trgm`/i18n/CJK relevance, search ACL, new mandatory services, frontend changes, 2vCPU/4GB load benchmarks.

## NV
- Meilisearch re-enablement requires the deferred hooks/await/rebuild/CI work.
- PG FTS relevance remains limited for CJK/multilingual corpora and typo tolerance.
