# Feature Specification: 019 — Postgres FTS Search Fallback + Reply Notification

**Feature Branch**: `feat/019-postgres-search-reply-notification`  
**Created**: 2026-11-16  
**Status**: In progress  
**Assessment**: `.specify/assessments/019-next-vertical-slice/decision.md` (verdict: go, slice S1)

## Goal
Make core search and reply feedback usable on Seed/VPS without Meilisearch by adding a Postgres-first FTS fallback for messages and emitting one deterministic in-app `Reply` notification for threaded replies, verified with DB-gated tests.

## Scope
- **Search (messages, PG fallback):**
  - Add additive migration introducing `messages.search_tsv` (generated/stored, `to_tsvector('simple', coalesce(content,''))`) and partial GIN index for non-deleted messages.
  - Implement PG-backed message search (tenant/org scoped, non-deleted, safe query parsing with `websearch_to_tsquery` fallback to `plainto_tsquery`, rank+recency ordering, bounded snippet/limit) returning existing API result shape.
  - Wire `GET /api/v1/search` to use Meilisearch when available, otherwise query Postgres (no 503 solely due to missing Meilisearch). Preserve contract (`q`, `type`, `limit`, `{query, results[{type,id,snippet}]}`), org membership checks, and empty-query behavior.
- **Notifications (reply-in-thread):**
  - On successful threaded message creation (`POST /api/v1/channels/{channel_id}/messages` with `thread_id`), resolve parent and create `NotificationType::Reply` for `parent.sender_id` when parent exists, belongs to same org/channel, is not deleted, and sender != parent author.
  - Best-effort/non-fatal: notification creation failure must not fail message creation/realtime response (log warning).
  - Expose via existing `GET/POST /api/v1/notifications` (no API contract changes).
- **Verification:** DB-gated contract tests on disposable PostgreSQL 18 covering search availability/match/isolation/empty/type behavior and reply notification creation/visibility/guards.

## Non-goals
- Tasks/files search, `pg_trgm` fuzzy matching, relevance/i18n tuning, channel-level ACL filtering in search, search backfill/reindex tooling.
- Outbox/workers, Valkey/`flora-notifications` delivery, push/email/realtime notification events, mention/assignment/completion fanout, dedup uniqueness schema (unless trivial).
- Frontend UI for search/notifications, Meilisearch removal, crate convergence, multi-node, 2vCPU/4GB perf benchmarks.

## NV
- zh-CN/ja tokenization/relevance quality with `'simple'` config; channel-level permission filtering beyond org membership in search.
- Notification idempotency under retries/duplicates and tx-coupling (best-effort post-persist).
- 2vCPU/4GB load/OOM/proxy/restore validation, browser e2e/multi-client.
