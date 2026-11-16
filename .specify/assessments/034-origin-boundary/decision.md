# Decision: 034 — Close ADR-0002 re-entry criterion #2

**Verdict:** go (scoped)

## Chosen slice
**S1 — Origin boundary:** WS `Origin` guard (403 before auth/DB) + runtime FE API/WS config + deterministic Rust/FE tests + isolated live non-browser proof + ADR/gate note.

## Scope (in)
- `flora-api/src/routes/websocket.rs`: pure `origin_allowed()` + early 403; 3 pure unit tests.
- `flora-tests/src/contract/websocket.rs`: 2 DB-free oneshot tests (`connect_lazy`).
- FE: `static/config.js`, `app.html`, `app.d.ts`, `client.ts` resolvers, `websocket.ts` + mock, `tests/unit/resolver.test.ts`, `.env.example`.
- Isolated canonical instance on :3100 / `flora_034_t8` for T8 preflight + Bearer `select-organization` + WS Origin probes; ADR-0002 gate note with evidence.

## Non-goals / non-claims
No FASE 7 Documents, no frontend affordance cleanup, no cargo-vet promotion, no tag/release, no load/restore. Do NOT claim ADR-0002 #2 met until E1–E8 green + live evidence + T7 browser regression lane green + ADR note. Never touch the running `:3000` process or other-agent `/metrics` work.

## NV
Tauri origin literals per-OS unproven; reverse proxy must forward `Origin`; `?token=` leakage unchanged; running binary is stale; config is startup-only.
