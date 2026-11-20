# Tasks: 015 — Minimal Realtime Delivery & Catch-up

- [x] T015.1 Add deterministic cursor catch-up support for channel messages (repo trait/impl + API `after` query) with stable ordering and tenant-safe behavior.
- [x] T015.2 Add in-process realtime hub/fanout in `AppState` and canonical message mutation events emitted after committed REST mutations.
- [x] T015.3 Implement WS subscription registry and authorized fanout delivery; align WS envelopes with canonical message payloads (keep handshake/auth guards).
- [x] T015.4 Align frontend WS client/store/mappers/chat for idempotent upsert, subscribe/unsubscribe, reconnect catch-up, and coherent REST+realtime flow.
- [x] T015.5 Add/update deterministic and DB-gated contract tests for cursor catch-up and realtime/fanout-relevant behavior; keep regressions green.
- [x] T015.6 Run verification (fmt/clippy/tests, DB-gated via CI, frontend checks), SDD evidence recorded, PR #218 para `dev` CI verde (6/6), mergeado/sync (`dev@ec0be14e0`, `origin/dev`), OBJETIVO/ROADMAP atualizados.
