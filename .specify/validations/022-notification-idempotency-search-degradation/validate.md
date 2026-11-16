# Validation: 022 — Notification Idempotency, Search Degradation, CI DB-Gated Enforcement

**Date:** 2026-11-16  
**Branch:** `feat/022-notification-idempotency-search-degradation` (from `dev@b5be7d16f`)  
**Scope:** S1 per `.specify/assessments/022-next-vertical-slice/decision.md`

## Evidence executed (local, disposable PostgreSQL 18 on 127.0.0.1:5432/flora_022_test)

| Check | Result |
|---|---|
| `cargo fmt --all -- --check` | OK |
| `cargo clippy -p flora-core -p flora-api -p flora-tests --all-targets -- -D warnings` | No issues |
| `cargo test -p flora-core --lib` | 8 passed |
| `cargo test -p flora-api` | 11 passed, 1 ignored |
| `cargo test -p flora-tests --lib` | 44 passed, 41 ignored |
| `cargo test -p flora-tests contract::notifications::db_ -- --ignored --test-threads=1` | 5 passed |
| `cargo test -p flora-tests contract::search::db_ -- --ignored --test-threads=1` | 6 passed |
| `cargo test -p flora-tests contract::files::db_ -- --ignored --test-threads=1` | 6 passed |
| `cargo test -p flora-tests contract::auth::db_ -- --ignored --test-threads=1` | 12 passed |
| `cargo test -p flora-tests contract::messaging::db_ -- --ignored --test-threads=1` | 4 passed |
| `cargo test -p flora-tests contract::websocket::db_ -- --ignored --test-threads=1` | 8 passed |

CI `db-tests` job extended with `search`, `notifications`, and `files` DB-gated suites (in addition to
existing `auth`/`messaging`/`websocket`) on disposable PostgreSQL 18. CI result is recorded in the PR.

## Contract coverage

- **Unread notification dedup (DB-enforced):** migration adds partial unique index
  `idx_notifications_unread_user_event_target_unique ON notifications (user_id, event_type, target_id)
  WHERE is_read = false`. `PgNotificationRepository::create` uses `INSERT ... ON CONFLICT (user_id,
  event_type, target_id) WHERE is_read = false DO NOTHING RETURNING ...` via `fetch_optional`, returning
  `Option<Notification>` (`None` = suppressed, not an error). New DB-gated contract
  `db_notification_create_is_idempotent_for_unread_dedup` verifies first insert → `Some`, duplicate unread
  → `None`, and a fresh unread insert after `mark_as_read` → `Some` (unread-only scope). Producers (task
  assignment, thread reply) treat suppression as success; the 021 TOCTOU precheck was removed. Existing
  020/021 transition contracts (create/self/no-op/reassign/unassign) remain green.
- **Search fail-open degradation:** `GET /api/v1/search` still prefers Meilisearch when configured; on a
  Meilisearch query error it logs a warning and falls back to `search_pg` for the same request. PG fallback
  invariants (tenant isolation, `is_deleted=false`, type normalization, limit cap, deterministic merged
  results) remain green under the DB-gated search suite.
- **CI enforcement:** all six DB-gated contract suites (`auth`, `messaging`, `websocket`, `search`,
  `notifications`, `files`) are now executed by the `db-tests` CI job.

## Incidental defects found while enabling CI (fixed in this cycle)

1. **Tri-state `assignee_id` PATCH was broken:** serde maps explicit JSON `null` on `Option<Option<Uuid>>`
   to the outer `None`, identical to an omitted field, so `PATCH ... {"assignee_id": null}` never cleared the
   assignee. Added a `deserialize_with` helper that maps missing → `None`, `null` → `Some(None)`, UUID →
   `Some(Some(uuid))`. This makes the documented 021 contract actually hold and the 021 DB-gated unassign
   test pass.
2. **Files DB contract status expectations drifted:** the API maps `Error::Validation` to HTTP 422, while
   four assertions expected 400. Updated the assertions to 422 (no API behavior change).
3. **Search DB contract test-data/URI issues:** file fixtures used hyphenated names that the `simple`
   text-search config tokenizes as a single token (so sub-token queries like `beta` could not match), and
   one test URI contained raw spaces (invalid URI). Fixtures now use space-separated tokens and the URI is
   percent-encoded. These are test-only corrections; they do not change API behavior.

## NV / residual

- Meilisearch mutation-indexing hooks, delete-on-soft-delete propagation, async task waiting/completion,
  rebuild/backfill tooling, and Meilisearch-gated deterministic CI remain deferred.
- Meilisearch staleness on *successful* queries remains possible; only query errors fall back.
- Hyphen/CJK/locale sub-token relevance in PG FTS (`pg_trgm`, i18n) remains an explicit NV.
- Delivery is still best-effort/post-persist; no tx-coupling/outbox, no coalescing/epoch idempotency,
  unassign event/recipients unchanged (U0).
