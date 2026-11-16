# Plan: 019 — Postgres FTS Search Fallback + Reply Notification

## Inputs
- Assessment: `.specify/assessments/019-next-vertical-slice/decision.md`
- Research: `docs/research/2026-11-16-018-{search-notifications-wiring-vps,postgres-search-outbox-vps,macro-search-notifications-parity}.md`, `docs/research/2026-11-16-019-{postgres-fts-search-vps,inapp-notification-wiring-vps,macro-mutation-search-notification-flows,db-gated-tests-search-notifications,pg18-fts-notifications-vps-ops}.md`
- Audits: `ses_e1d793fa1ffeg18iLwVe1BQrV1`, `ses_e1d78fb75ffezHWpPzuDQ60xJS`, `ses_e1d77e0a0ffe9WZE984FSoaAv7`

## Design
1. **Migration:** `20261117000001_add_messages_search_fts.sql` adds generated stored `search_tsv` (`simple`) and partial GIN index `WHERE is_deleted = false`.
2. **PG search:** add PG message search query (sqlx, tenant/org scoped, safe tsquery parse, rank+snippet, limit/type handling) accessible from API route; reuse existing `SearchResult`/`SearchResponse` DTOs.
3. **Fallback wiring:** in `flora-api/src/routes/search.rs`, if `state.search_service` (Meilisearch) is present use it; otherwise execute PG search against `state.db_pool` and return same contract (no 503 for missing Meilisearch). Keep membership/empty-query checks.
4. **Reply notification:** in `flora-api/src/routes/channels.rs::post_message`, after successful persist, when `created.thread_id` set resolve parent and create `Reply` notification for parent author if valid/same org+channel/not deleted/not self; best-effort with warning log on failure.
5. **Tests:** DB-gated env-gated tests in `flora-tests/src/contract/search.rs` and `flora-tests/src/contract/notifications.rs` using disposable PG, migrations, seeded fixtures, and router/API flows.

## Verification
- `cargo fmt --all -- --check`
- `cargo clippy -p flora-core -p flora-search -p flora-api -p flora-tests --all-targets -- -D warnings`
- `cargo test -p flora-core --lib`, `cargo test -p flora-api`, `cargo test -p flora-tests --lib`
- DB-gated search/notification tests when `FLORA_DATABASE__POSTGRES_URL`/`DATABASE_URL` available (expected in CI db-tests)
- Frontend checks not required (no FE changes expected)

## Risks/NVs
- FTS quality/perf/tokenization (notably CJK) and channel-level ACL in search remain NV.
- Notification is best-effort post-persist (no idempotency key/tx coupling in 019).
- 2vCPU/4GB runtime behavior unvalidated.
