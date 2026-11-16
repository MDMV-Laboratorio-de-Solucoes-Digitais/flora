# Requirements Checklist: 015

- [x] Slice A scope only (no B/C, no new mandatory infra, no multi-node delivery)
- [x] Postgres-authoritative; events emitted after commit; cursor-deterministic catch-up
- [x] WS auth/tenant isolation preserved; in-process fanout with bounded behavior
- [x] Frontend idempotent upsert + subscribe/reconnect/catch-up aligned
- [x] DB-gated and deterministic verification evidence recorded
- [ ] PR to `dev`, CI green, merge/sync, only `dev`/`main`, docs updated
