# Validation: 034 — ADR-0002 re-entry criterion #2 (origin boundary)

**Branch:** `feat/034-origin-boundary` (from canonical `github/dev@1be723cc`)
**Baseline:** cycles 023–033 merged; no tags/releases. Other-agent `/metrics` divergence in local `dev` untouched.

## Evidence executed (local, 2026-11-16)

### Rust gates
- `cargo fmt --all -- --check`: clean.
- `cargo clippy --workspace --all-targets -- -D warnings`: clean (one `doc_markdown` backtick fix applied to the new `AppState` doc).
- `cargo test --workspace`: 103 passed, 44 ignored (17 suites).
- `cargo test -p flora-api websocket`: 9 passed (3 new `origin_allowed_*` pure tests).
- DB-free contract tests (no `FLORA_DATABASE__POSTGRES_URL`): `ws_disallowed_origin_is_403_without_db` **ok**, `ws_allowed_origin_passes_guard_then_fails_auth_without_db` **ok**.
- Six DB-gated suites against `flora_023_test` (env set, `--ignored --test-threads=1`): auth 12, messaging 4, websocket 8, search 8, notifications 5, files 6 — **all ok**.

### Frontend gates (from `apps/frontend`)
- `bun run check`: 0 errors / 0 warnings.
- `bun run lint`: green (Prettier + ESLint; `config.js` uses `globalThis` with a scoped, reasoned disable for the deliberate runtime-config global).
- `bun run test:unit -- --run --project server`: 10 files / 30 tests passed (5 new resolver tests; 25 from 033).
- `bun run build`: green.

### T8 live non-browser proof — fresh CANONICAL instance, isolated from other-agent processes
Setup: dedicated DB `flora_034_t8` on :5432 + `flora-api` built from this branch on `127.0.0.1:3100`, `FLORA_APP__CORS_ORIGINS='["http://127.0.0.1:4173","http://localhost:5173"]'`, messaging/search disabled. Freshness discriminator `GET /llms.txt` → **200** (the stale, other-agent `:3000` binary returns 404 and `ACAO: *`; it was never touched — `/health/ready` still 200 after cleanup). Cleanup: only the recorded pid killed, `flora_034_t8` dropped.

| Probe | Result |
|---|---|
| Preflight `OPTIONS /api/v1/auth/select-organization`, `Origin: http://127.0.0.1:4173`, ACRM POST, ACAH authorization,content-type | `200`; `ACAO: http://127.0.0.1:4173`; `Vary: Origin`; explicit ACAM/ACAH; **no** `Access-Control-Allow-Credentials` |
| Preflight negative controls: `https://evil.example`, `null`, `tauri://localhost`, `http://tauri.localhost` | **no ACAO** in all four |
| WS handshake, no token, `Origin: https://evil.example` / `null` | **403 / 403** (guard before auth) |
| WS handshake, no token, allowed origin | **401** (guard passed, auth rejected) |
| Bearer (Alice, nil-org jti) `POST /api/v1/auth/select-organization` from allowed origin | **200** + `ACAO` echo + `Vary: Origin`; scoped `session_token` returned |
| Scoped token `GET /api/v1/workspaces` from same non-browser origin | **200** + `ACAO` echo |
| WS handshake, allowed origin + valid scoped token | **101 Switching Protocols** (`sec-websocket-accept` present) |
| WS handshake, allowed origin + invalid token | **401** |

Raw (redacted, no tokens) evidence: `/tmp/opencode/flora-034/evidence/{origin-boundary,bearer-select-org,ws-handshake}.txt` (session-local, outside the repo).

## Contract coverage
`origin_allowed()` exact-matches a present `Origin` against the same `app.cors_origins` list used by `cors_layer`; absent `Origin` is allowed (native/interop), `null` and unlisted origins are denied. Guard runs before session lookup, so 403 (origin) is distinguishable from 401 (auth) and both new router tests are DB-free. `resolveApiBaseUrl()`/`resolveWsBaseUrl()` implement runtime global > Vite env > default without new dependencies.

## Boundary / NV
- `curl` does not enforce CORS: the live proof shows the server emits correct headers and accepts the Bearer contract from a non-browser client; it is **not** browser/webview enforcement. T7 browser lane must be green on the PR for the ADR note.
- Tauri origin literals (`tauri://localhost` vs `http(s)://tauri.localhost`) are per-OS unproven; defaults do not include them.
- Reverse proxy must forward `Origin` unchanged; a stripping proxy would make the absent-Origin allowance bypass the guard.
- `?token=` WS query auth remains a token-in-logs risk; not fixed here.
- The long-running `:3000` dev process remains stale; restarting it is an ops action, not a product defect.

## CI result
PR #248: all checks green — 4 `build`, 2 `db-tests`, `dependency-review`, 2 `supply-chain-vet` (informational), 2 `e2e-browser` (informational, ~4m46s). GitHub runs `43972366904` / `43972379097`; merged as `23ca350c`. Browser lane green completes T7; the ADR gate note records criterion #2 as MET.
