# Problem: 022

After 021, assignment notifications still rely on a best-effort query-before-insert unread guard and have no database-enforced uniqueness, leaving them vulnerable to duplicate unread notifications under concurrent/retried effective transitions. Reply/notification creation paths share the same non-idempotent insert behavior.

Search behavior remains ambiguous in failure/staleness cases: Meilisearch is preferred when configured but mutation indexing hooks and async task guarantees are absent, so freshness/parity can drift; query-time failures are not explicitly degraded to the Postgres FTS correctness baseline. Deterministic DB-gated contracts for search/notifications/files are not all enforced in CI, reducing regression protection for core Seed flows.
