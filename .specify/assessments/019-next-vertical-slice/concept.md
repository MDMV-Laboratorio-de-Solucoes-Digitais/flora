# Concept: 019 — Postgres FTS (messages) + reply-in-thread notification (S1, 80/20)

**Verdict target:** **go** (scoped)

## 1) Minimal solution concept
Add a Postgres-native search path for messages and a single deterministic notification side effect for threaded replies, using existing DB/APIs and additive migration. Keep Meilisearch optional, avoid outbox/workers/push/new mandatory services, and verify with DB-gated tests.

## 2) Target behavior (in scope)
1. **Postgres FTS for messages (default/fallback):**
   - Add additive migration: `messages.search_tsv tsvector GENERATED ALWAYS AS (to_tsvector('simple', coalesce(content,''))) STORED` and partial GIN index on `search_tsv WHERE is_deleted = false` (optionally composite-friendly with `organization_id` query pattern as needed).
   - Implement PG search query (messages-only for 019): tenant-scoped `organization_id = $org`, `is_deleted = false`, `search_tsv @@ query`, safe parsing (`websearch_to_tsquery('simple', $q)` with fallback to `plainto_tsquery`/empty-safe handling), ordering by rank + recency, bounded snippet, limit capped.
   - Preserve `/api/v1/search` contract/params (`q`, `type`, `limit`) and response shape. If `type` excludes `message`, return empty results (or only supported types). When Meilisearch service is unavailable/disabled, serve via Postgres instead of 503 (503 only if search cannot be served at all).

2. **Reply-in-thread in-app notification:**
   - In `POST /api/v1/channels/{channel_id}/messages`, after successful message persist (and after building response; realtime may proceed), if `thread_id` is set: load parent message; if parent exists, `parent.organization_id == org_id`, `parent.channel_id == channel_id`, not deleted, and `parent.sender_id != created.sender_id`, create `Notification::new(org_id, parent.sender_id, NotificationType::Reply, created.id, target_url: None)`.
   - Best-effort/non-fatal: notification creation failure must not fail the 201 message response/realtime path (log warning). No Valkey/`flora-notifications` crate, no outbox.

3. **DB-gated verification:**
   - Env-gated tests (disposable PG18, migrations applied) covering: PG FTS returns matching message for org member, respects tenant isolation and deleted exclusion/empty query/type filtering as applicable, and remains available without Meilisearch; reply creates `Reply` notification for parent author, no self-notify, no notify on invalid/missing/cross-scope parent, and notifications API exposes it.

4. **Frontend:** no change required (no existing search/notifications UI integration; contracts preserved).

## 3) Likely files/components
- Migration: `flora-core/src/migrations/sql/<timestamp>_add_messages_search_fts.sql`
- Search: `flora-core/src/config/search.rs`, `flora-search/src/lib.rs` and/or new PG search helper/trait/module (e.g. in `flora-core`/`flora-search`), `flora-api/src/state/app_state.rs`, `flora-api/src/routes/search.rs`
- Notifications: `flora-api/src/routes/channels.rs` (+ minimal use of `PgNotificationRepository`/`Notification`), optionally `flora-core/src/services/notification_service.rs` if helpful (not required)
- Tests: `flora-tests/src/contract/search.rs`, `flora-tests/src/contract/notifications.rs` (DB-gated; may add/adjust tasks/messaging harness helpers minimally)
- SDD: `specs/019-postgres-search-reply-notification/*`, `.specify/assessments/019-next-vertical-slice/*`, `.specify/validations/019-*`, `.specify/releases/019-*`

## 4) Risks / mitigations
1. **Search contract/availability behavior change:** preserve response shape and org auth; add explicit fallback and tests for no-Meilisearch case.
2. **FTS perf/tokenization limits on 4GB/multilingual:** messages-only, capped limit/snippet, fixed `simple` config; document zh-CN/ja relevance and 2vCPU/4GB load as NV.
3. **Notification semantics/duplication:** single post-persist hook, self/invalid-parent guards, best-effort failure; idempotency/tx coupling and deleted-parent edge nuances documented/tested as feasible.
4. **Schema/migration portability:** additive DDL, IF NOT EXISTS, no extension dependency required for core `tsvector`/GIN.

## 5) NV / non-goals
- NV: tasks/files search, `pg_trgm` fuzzy matching, relevance ranking tuning, CJK/locale dictionaries, channel-level ACL filtering in search, search backfill/reindex tooling, outbox/workers, push/email/realtime notification delivery, mention/assignment/completion fanout, dedup uniqueness constraints unless trivial, 2vCPU/4GB perf/load/restore benchmarks, browser e2e.
- Non-goals: Meilisearch removal, frontend search/notification UI, crate unification, multi-node consistency, new mandatory services.
