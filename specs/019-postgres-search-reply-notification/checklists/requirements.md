# Requirements Checklist: 019

- [x] Additive migration: `messages.search_tsv` + GIN (non-deleted) for PG FTS
- [x] `/api/v1/search` uses PG fallback when Meilisearch unavailable/disabled (no 503 for missing Meilisearch), preserves contract and tenant/org checks
- [x] Threaded reply creates `Reply` notification for valid parent author (not self, same org/channel, not deleted), best-effort/non-fatal
- [x] DB-gated tests (disposable PG18) cover search match/isolation/empty/type behavior and reply notification creation/visibility/guards
- [x] No new mandatory services; Postgres-first/Seed-safe; verification evidence recorded
- [ ] PR to `dev`, CI green (no blind merge), merge/sync `github/dev`+`origin/dev`, only `dev`/`main`, planning updated
