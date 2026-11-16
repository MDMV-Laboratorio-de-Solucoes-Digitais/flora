# Decision: 014 — Next vertical slice

**Verdict:** **go** (scoped)

## Chosen slice
**A — WebSocket session authority + protocol alignment.**  
Primary 80/20 for cycle 014: enforce DB-backed session authority on `/ws` (parity with HTTP), align endpoint/token transport and minimal frontend↔backend WS protocol/close behavior so authenticated, tenant-correct realtime use is possible and safe-by-default. **Do not** take on full fanout/registry/sync (B) or broad operability refactors (C) as the primary deliverable, except narrowly required compatibility hooks.

## Justification (evidence)
- **Security/P0:** HTTP extractors already validate `jti` in Postgres (012, PR #215), but `flora-api/src/routes/websocket.rs` performs JWT-only decode pre-upgrade with no DB session/revocation/expiry enforcement — inconsistent trust boundary (ROADMAP P0.1; `docs/research/2026-11-16-013-websocket-session-authority.md`).
- **Usability/contract:** frontend `WsClient` targets `/api/v1/ws?token=` and a divergent event/sync model while backend serves `/ws` with header auth and tagged envelopes; browser interop is effectively broken (`apps/frontend/src/lib/api/websocket.ts`, 013 research).
- **Sequencing leverage:** 013 delivered REST persistence; A is prerequisite for reliable realtime delivery/fanout (B) and avoids building push features on unauthenticated/misaligned foundations.
- **VPS fit:** reuses existing Postgres/session services, no new mandatory infra; handshake-only or low-frequency revalidation fits 2 vCPU/4 GB better than premature stateful fanout complexity.
- **Research consensus:** multilingual/industry practice favors pre-upgrade auth, DB-backed revocation checks (jti/session), generic failures, bounded revocation windows, and careful browser token transport (see `research.md`).

## Scope (in) — file-by-file implementation checklist
> Planned targets (confirm exact symbols/paths at implementation time; keep diff minimal).

### Backend (authority & transport)
- `flora-api/src/extractors/auth.rs` — factor shared token verification helper (e.g. `verify_bearer_token(token, state) -> Result<VerifiedSession, _>` or equivalent) reusing existing JWT decode + `SessionService::validate_session(jti)` + active/expiry/user/org checks; keep HTTP extractor behavior/messages unchanged.
- `flora-api/src/extractors/mod.rs` (and any re-export/call sites) — expose helper for WS/other callers as needed.
- `flora-api/src/routes/websocket.rs` — resolve token from supported transports (prefer `Authorization: Bearer` + documented browser fallback such as query/subprotocol; never log token), call shared verifier **before** `on_upgrade`; return generic 401/500 consistent with HTTP; pass verified `user_id/org_id/jti` into socket handler; retain/ensure channel-org ownership checks on message actions; define nil-org handling; optionally add minimal periodic `jti` revalidation/`SESSION_INVALID`+`1008` close if low-risk (or document handshake-only window).
- `flora-api/src/routes/mod.rs` — align/alias WS route path with frontend (`/ws` and/or `/api/v1/ws`) as decided.
- `flora-api/src/state/*` (only if required) — no new services expected; wire existing `session_service` access already available.

### Frontend (protocol/transport/lifecycle)
- `apps/frontend/src/lib/api/websocket.ts` — point to agreed WS URL/path, send token via agreed transport, parse backend tagged envelopes (`Subscribed/Unsubscribed/NewMessage/MessageSent/Pong/Error`) minimally, remove/guard broken assumptions (`/messages/sync` call, `workspace.user.removed` contract) with safe behavior, stop blind reconnect on auth/policy failure (invoke auth invalidation/lock as appropriate), add bounded backoff+jitter for retryable disconnects.
- `apps/frontend/src/routes/(app)/+layout.svelte` (and auth/org state integration points) — ensure disconnect/reconnect on logout, token refresh, and org switch so socket context matches verified session/org.
- `apps/frontend/src/lib/api/client.ts` / auth state modules (only if needed) — no behavior change required unless exposing token/refresh hooks for WS lifecycle.

### Tests & docs
- `flora-api/src/extractors/auth.rs` (tests) and/or `flora-api/src/routes/websocket.rs` (tests) — unit tests for token transport parsing/precedence and WS envelope (de)serialization helpers where applicable.
- `flora-tests/src/contract/*` or `flora-tests/src/integration/*` (add `websocket`/session module as appropriate) — **DB-gated** handshake/session matrix: accept valid; reject revoked/inactive, expired, missing, user mismatch, org mismatch (generic); nil-org parity; same-org send/persist ok; cross-org channel action forbidden. Use disposable Postgres 18 + deterministic session fixtures.
- `.github/workflows/rust.yml` — include new DB-gated tests in `db-tests` job (env-gated/`--ignored` pattern consistent with repo) when implemented.
- `apps/frontend/tests/unit/*` (e.g. websocket client/mappers) — URL/transport, envelope handling, auth-failure no-infinite-retry, reconnect classification.
- `docs/dev-smoke.md` and/or `docs/research/*` (assessment follow-up) — document WS auth path/transport, revocation window, and manual smoke steps (NV if not executed).

## Deterministic verification plan
1. `cargo fmt --all -- --check`
2. `cargo clippy --workspace --all-targets -- -D warnings` (record any scoped fallback if required by preexisting lint surface)
3. `cargo test -p flora-core --lib`
4. `cargo test -p flora-api` (unit tests, incl. new extractor/ws unit cases)
5. `cargo test -p flora-tests --lib`
6. **DB-gated (Postgres 18 disposable, alt-port ok):** `FLORA_DATABASE__POSTGRES_URL=... cargo test -p flora-tests <ws/session contract filter> -- --ignored --test-threads=1` (and wire into CI `db-tests` job)
7. `cd apps/frontend && bun run check && bun run lint && bun run test:unit -- --run --project server && bun run build`
8. **NV (environment-dependent / follow-up):** live browser e2e against running API+DB (+Valkey if enabled), real OIDC/JWKS flows, multi-client realtime fanout/delivery, `/messages/sync` behavior, long-lived revocation push semantics, default-port smoke when 5432 unavailable conflicts absent, `apps/web`.

## Risks and mitigations
| Risk | Impact | Mitigation |
|---|---|---|
| Browser token transport tradeoff (query/subprotocol vs header/cookie) | Leakage if logged/cached; implementation complexity | Never log tokens, prefer short-lived credentials, document choice, support safest feasible transport; keep generic errors. |
| Handshake-only revocation leaves residual window | Revoked session usable until reconnect/expiry | Document explicitly; prefer low-frequency periodic/jittered recheck if cheap; bound TTLs. |
| FE/BE contract churn | Regressions in chat/auth UX | Minimal envelope mapping, isolate client changes, unit tests, keep REST as correctness path. |
| DB dependency on handshake/recheck | Added latency/load, fail-closed on DB issues | Reuse indexed `jti` lookup, low recheck frequency, preserve 500-on-unavailable semantics; VPS pool sizing noted. |
| Scope creep into fanout/sync | Slips schedule, adds stateful complexity | Hard non-goals; `Subscribe` remains ack-only unless trivial; defer registry/NOTIFY/Valkey fanout to B. |
| Nil-org / pre-selection sessions | Ambiguous WS access | Align with HTTP `require_org`/product rule; test explicitly. |

## Non-goals (014)
- Full realtime fanout/subscription registry, multi-client broadcast correctness guarantees, cross-node delivery, Valkey/NOTIFY-driven push architecture (beyond documenting options).
- `/api/v1/messages/sync` implementation, message history replay/catch-up protocol, presence/typing/read receipts, edit/delete realtime events taxonomy expansion.
- File upload/download/binary storage, attachments, DM UX, channel membership/roles UI, search/notifications wiring.
- OIDC/JWKS e2e, browser Playwright e2e, `apps/web`, deploy/merge/release/secrets, production rollout.
- Broad Seed/VPS refactor (compose profiles, pool rewiring, making Meilisearch optional at boot, SQLite claims) except incidental doc notes — **candidate follow-up slice C**.

## Debloat / VPS notes
- **No new mandatory services.** Reuse Postgres + existing `sessions`/`session_service`; avoid introducing ticket stores/pubsub dependencies in 014.
- Prefer a small shared verifier over duplicating auth logic or adding auth frameworks.
- Keep revalidation (if added) infrequent and jittered; handshake-only is acceptable for Seed if residual window is documented.
- Realtime correctness remains **DB/REST-authoritative**; push is best-effort (aligns with LISTEN/NOTIFY & pubsub fire-and-forget realities). For 2 vCPU/4 GB, defer heavy fanout/state until measured need.
- Seed operability gaps (optional services, pool/config, mem limits) remain tracked in `docs/research/2026-11-16-seed-vps-profile-audit.md` for subsequent slice prioritization.

## NV / assumptions
- No live runtime measurements, browser e2e, or CI remote execution performed in this assessment.
- Exact DB schema/index sufficiency for WS revalidation frequency assumed adequate given 012 usage; confirm during implementation.
- ja/ko/ru-language primary sources limited in this run; no conflicting guidance identified.