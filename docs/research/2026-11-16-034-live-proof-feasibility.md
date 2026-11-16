# 034 — Autonomous, non-destructive live T8 proof on a separate canonical instance

**Date:** 2026-11-16. **Repo:** `flora` (archived checkout `/home/luis/development/MDMV/projetos/Arquivados/flora`).
**Canonical base:** `github/dev@1be723cc`. **Local HEAD:** `dev@babac3ea` (`1be723cc` is an ancestor; the delta is only other-agent `/metrics` work in `health.rs`/`routes/mod.rs`).
**Type:** read-only research + executable plan. **No source, config, workflow, DB row, or running process was changed by this pass.** Only `SELECT` queries and read-only `curl` GET/OPTIONS probes were issued. This document is the only artifact written.

---

## 0. Verdict

| Step | Feasibility | Authorization |
|---|---|---|
| 1. Build/run separate canonical `flora-api` on `127.0.0.1:3100` against a fresh isolated DB, known CORS allowlist | **FEASIBLE** | Autonomous |
| 2. CORS preflight allowed/disallowed proof with `curl` | **FEASIBLE** | Autonomous |
| 3. Mint Bearer session + `select-organization` round-trip, redacted evidence | **FEASIBLE** | Autonomous (DB writes confined to the new DB) |
| 4. WS `Origin` guard: disallowed→403, allowed→auth path | **BLOCKED until E6 (`origin_allowed` guard in `websocket.rs`) is applied and built** | Source edit needs human sign-off (cycle-034 implementation) |
| Overall 030 T8 + 034 WS extension | **CONDITIONAL GO** | All of 1–3 now; 4 after E6 lands |

**Do not use the process on `:3000` for any of this.** It is `./target/debug/flora-api`, pid `1799842`, stale (pre-032): read-only probes this pass reconfirm `OPTIONS … Origin: https://evil.example` → `access-control-allow-origin: *` and `GET /llms.txt` → `404`. It must not be stopped, restarted, signalled, or reconfigured.

---

## 1. Verified environment facts (read-only this pass)

| Fact | Evidence |
|---|---|
| Two local Postgres instances are live: `127.0.0.1:5432` and `127.0.0.1:15433` | `pg_isready` both "accepting connections" |
| The running API uses **`postgresql://postgres:****@127.0.0.1:15433/flora`** (credentials redacted at source) | `/proc/1799842/environ`, DB URL field only, masked before display |
| `@15433/flora`: 1 org (nil UUID), **0 users, 0 sessions** → cannot support `select-organization`, and is the live process's DB: never write to it | `psql SELECT count(*)` |
| `5432/flora`: 2 orgs (nil + `11111111-…`), 4 users, 3 memberships, 1 session, **only 1 migration applied** | `psql SELECT` |
| `5432/flora_023_test`: 307 orgs, 348 users, 258 memberships, **all 6 applied migrations**, but **no Alice (`22222222-…`) and no org `11111111-…`** | `psql SELECT` |
| `postgres` has `rolcreatedb = t` on both instances | `SELECT rolcreatedb FROM pg_roles` |
| Port `3100` is free; only `:3000` is listening for Flora | `ss -ltnp` |
| Canonical/local source already has the hardened CORS layer (`AllowOrigin::list`, explicit methods/headers, no credentials, `Vary: Origin`); `AppConfig::validate()` rejects `*` | `middleware/cors.rs:26-55`, `config/app.rs:84-117` |
| **No WS `Origin` guard exists** in canonical `1be723cc` or local tree: `websocket_handler` goes straight to `extract_org_context` at `websocket.rs:123`; `grep` finds no `origin_allowed`/`header::ORIGIN` in `flora-api/src` | `git show 1be723cc:…/websocket.rs`, `grep` |
| E6 of `docs/research/2026-11-16-034-origin-boundary-exact-plan.md` is the planned guard; it is **not applied** (single worktree, clean source tree, only two untracked research docs) | `git worktree list`, `git status --short` |
| `flora-mint-session` is a `flora-core` bin gated by `dev-mint` (`required-features`); `flora-api` has no such feature | `flora-core/Cargo.toml:63-72`, `flora-api/Cargo.toml:70-71` |
| Migrations are embedded and applied at API startup (7 SQL files) | `flora-core/src/migrations/mod.rs:18`, `flora-core/src/migrations/sql/*` |

**Why a fresh DB, not `flora_023_test`:** it is a shared, fully-migrated test DB with no seeded Alice/Acme identity, so obtaining a Bearer session would require writing seed rows into another agent's DB. A brand-new DB is isolated, needs no cleanup of shared state, and is reversible with one `DROP DATABASE`.

---

## 2. Autonomous setup — separate instance on :3100

All commands below are **the plan**, not executed in this research pass. Use one shell with these exports so the API, the minter, and the JWT recipe share the exact same secret and allowlist.

```bash
# --- isolated evidence workspace (outside the repo, no git impact) ---
export WORK=/tmp/opencode/flora-034
mkdir -p "$WORK/evidence" "$WORK/storage"

# --- exact Figment env keys (FLORA_ prefix + '__' nesting; flat FLORA_PORT is ignored) ---
export FLORA_ENV=development
export FLORA_APP__ENV=development
export FLORA_APP__HOST=127.0.0.1
export FLORA_APP__PORT=3100
export FLORA_APP__BASE_URL=http://localhost:3100
export FLORA_APP__JWT_SECRET='change-me-to-a-random-secret-at-least-32-characters-long'
export FLORA_APP__SESSION_TTL_SECS=3600
# Minimal, known allowlist. Tauri literals are deliberately NOT included so they stay
# negative controls; adding them is a separate authorized ops decision.
export FLORA_APP__CORS_ORIGINS='["http://127.0.0.1:4173","http://localhost:5173"]'
export FLORA_DATABASE__POSTGRES_URL='postgresql://postgres:postgres@127.0.0.1:5432/flora_034_t8'
export FLORA_DATABASE__MAX_CONNECTIONS=5
export FLORA_DATABASE__MIN_CONNECTIONS=1
export FLORA_MESSAGING__ENABLED=false
export FLORA_SEARCH__ENABLED=false
export FLORA_OIDC__ISSUER_URL='http://localhost:8080'
export FLORA_OIDC__CLIENT_ID='flora@localhost'
export FLORA_OIDC__CLIENT_SECRET='unused-local-no-oidc'
export FLORA_STORAGE__LOCAL_PATH="$WORK/storage"

export ALLOWED='http://127.0.0.1:4173'
export API='http://127.0.0.1:3100'
export ORG_ID='11111111-1111-1111-1111-111111111111'   # seeded Acme; NOT the 030 doc typo …-7111-…
```

### 2.1 Create + seed the isolated DB (all writes confined to `flora_034_t8`)

```bash
export PGPASSWORD=postgres
dropdb -h 127.0.0.1 -p 5432 -U postgres --if-exists flora_034_t8   # only if re-running; safe, dedicated name
createdb -h 127.0.0.1 -p 5432 -U postgres flora_034_t8
```

Then build and start the separate API. Its startup applies all embedded migrations; wait for `/health` before seeding.

```bash
cargo build -p flora-api                       # dev profile; :3000 binary/target untouched by env
nohup ./target/debug/flora-api >"$WORK/api-3100.log" 2>&1 &
echo $! >"$WORK/api-3100.pid"
for i in $(seq 1 30); do curl -fsS "$API/health" >/dev/null 2>&1 && break; sleep 1; done
curl -fsS "$API/health"                        # expect {"status":"ok",...}
```

Now load the dev seed + the fixed pre-selection session (both scripts are transactional and idempotent by `ON CONFLICT`):

```bash
psql -h 127.0.0.1 -p 5432 -U postgres -d flora_034_t8 -v ON_ERROR_STOP=1 -f dev-seed.sql
psql -h 127.0.0.1 -p 5432 -U postgres -d flora_034_t8 -v ON_ERROR_STOP=1 \
  -f scripts/dev-smoke-session.sql
psql -h 127.0.0.1 -p 5432 -U postgres -d flora_034_t8 -Atc \
  "SELECT count(*) FROM _sqlx_migrations; SELECT id FROM organizations WHERE id='$ORG_ID';"
# expect 7 migrations and the Acme org row
```

**Freshness discriminator** (proves this is the new canonical build, not the stale process):

```bash
curl -s -o /dev/null -w 'llms.txt=%{http_code}\n' "$API/llms.txt"   # expect 200 (stale :3000 = 404)
```

Cleanup at the end (reversible, no shared state):

```bash
kill "$(cat "$WORK/api-3100.pid")"             # ONLY the pid file we created; never pid 1799842
dropdb -h 127.0.0.1 -p 5432 -U postgres --if-exists flora_034_t8
```

---

## 3. Step 2 — CORS preflight proof (read-only HTTP)

```bash
# (a) allowed preflight: exact ACAO echo, explicit methods/headers, no credentials, Vary: Origin
curl -si -X OPTIONS "$API/api/v1/auth/select-organization" \
  -H "Origin: $ALLOWED" \
  -H 'Access-Control-Request-Method: POST' \
  -H 'Access-Control-Request-Headers: authorization,content-type' \
  | grep -iE '^(HTTP/|access-control-|vary:)' | tee "$WORK/evidence/01-preflight-allowed.txt"
# EXPECT 200; access-control-allow-origin: http://127.0.0.1:4173
#         ACAM: GET,POST,PUT,PATCH,DELETE,OPTIONS; ACAH contains authorization;
#         NO access-control-allow-credentials; Vary: Origin

# (b) negative controls: must print NO ACAO line
for O in 'https://evil.example' 'null' 'tauri://localhost' 'http://tauri.localhost'; do
  printf 'origin %-24s -> ' "$O"
  curl -si -X OPTIONS "$API/health" -H "Origin: $O" \
    -H 'Access-Control-Request-Method: GET' \
    | grep -i '^access-control-allow-origin:' || echo '(no ACAO = denied)'
done | tee "$WORK/evidence/02-preflight-denied.txt"
# EXPECT four "(no ACAO = denied)" lines
```

**Correction to the task wording:** a disallowed **CORS preflight** does **not** return 403 here. `tower-http` answers `OPTIONS` itself with `200` and simply omits `Access-Control-Allow-Origin` — this is asserted by the existing unit test `cors.rs:136-149`. `403` is the **WS Origin guard** signal (step 4) only. Record this distinction in the evidence.

**Caveat to record:** `curl` does not enforce CORS; it proves the server emits the correct headers. Browser/webview enforcement remains T7 and is out of scope.

---

## 4. Step 3 — Bearer session + `select-organization` (redacted evidence)

### 4.1 Mint (no OIDC provider needed)

Primary, as requested — the feature-gated helper:

```bash
cargo run -p flora-core --features dev-mint --bin flora-mint-session -- \
  --storage-state "$WORK/storage-state.json" \
  --frontend-origin "$ALLOWED" --ttl-secs 3600
# Needs the SAME FLORA_APP__JWT_SECRET export as the API; refuses production env.
TOKEN=$(jq -r '.origins[0].localStorage[] | select(.name=="flora.auth.v1") | .value | fromjson | .token' \
  "$WORK/storage-state.json")
```

The minted claims are Alice `22222222-…`, jti `018f0000-0000-7000-8000-000000000001`, nil org, `alice@acme.test` (`flora-mint-session.rs:20-28,59-67`), matching `scripts/dev-smoke-session.sql`. Alternative with no build: the Node HS256 recipe in `docs/dev-smoke.md:70-91` (same secret, same claims).

**Never print `$TOKEN` or `$SCOPED`.** Redaction helper for every captured artifact:

```bash
redact() { sed -E \
  -e 's/(Bearer )[A-Za-z0-9._-]+/\1<REDACTED>/g' \
  -e 's/("session_token":")[^"]+/\1<REDACTED>/g' \
  -e 's/(FLORA_APP__JWT_SECRET=).*/\1<REDACTED>/'; }
```

### 4.2 Round-trip

```bash
RESP=$(curl -si -X POST "$API/api/v1/auth/select-organization" \
  -H "Authorization: Bearer $TOKEN" -H "Origin: $ALLOWED" \
  -H 'Content-Type: application/json' \
  -d "{\"organization_id\":\"$ORG_ID\"}")
printf '%s\n' "$RESP" | grep -iE '^(HTTP/|access-control-allow-origin:|vary:)' \
  | tee "$WORK/evidence/03-select-org-headers.txt"
printf '%s\n' "$RESP" | grep -i '"session_token"' | redact \
  | sed -E 's/("expires_in":[0-9]+).*/\1, "user_id":"<redacted>", "organization_id":"'"$ORG_ID"'" }/' \
  >> "$WORK/evidence/03-select-org-headers.txt"
# EXPECT 200 + ACAO == $ALLOWED; body has a NEW session_token (rotated jti).
SCOPED=$(printf '%s\n' "$RESP" | sed -n 's/.*"session_token":"\([^"]*\)".*/\1/p')

# Prove the scoped token works from the same non-browser origin:
curl -si -H "Authorization: Bearer $SCOPED" -H "Origin: $ALLOWED" "$API/api/v1/workspaces" \
  | grep -iE '^(HTTP/|access-control-allow-origin:|vary:)' | redact \
  | tee "$WORK/evidence/04-scoped-workspaces.txt"
# EXPECT 200 + ACAO == $ALLOWED (seeded Main/Design workspaces in body; redact if captured).
```

**DB side effect (isolated):** `select_organization` (`auth.rs:356-415`) revokes the current jti and inserts a new rotated-jti org-scoped session, and checks Alice's membership in Acme. Because the API points at `flora_034_t8`, **no row in the live `:3000` DB (`@15433/flora`) or in `flora_023_test` is touched.** Re-running the proof requires re-loading `scripts/dev-smoke-session.sql` into `flora_034_t8` (it reactivates the fixed jti). This is the only mutable part of T8 and is fully reversible by dropping the dedicated DB.

---

## 5. Step 4 — WS Origin guard (BLOCKED until E6 lands)

The live separate instance currently has **no** `Origin` guard. With no token, all origins return `401` from `extract_org_context` (`websocket.rs:161-179`); with a valid scoped token, all origins upgrade. That is **not** acceptable evidence for "disallowed → 403".

**Prerequisite (human-authorized source edit, from the 034 exact plan E6):** insert `origin_allowed(&HeaderMap, &[String])` and the early guard at the top of `websocket_handler` (`websocket.rs:122`), reusing `state.config.app.cors_origins`; then rebuild the separate instance. No new dependency, no signature change; `StatusCode`/`IntoResponse`/`HeaderMap` are already imported. Its 3 pure unit tests + 2 DB-free oneshot tests (E7(i)/(ii)) must be green first.

After E6 is applied and the :3100 binary rebuilt with the same env exports:

```bash
# (a) absent token + allowed Origin -> passes guard, reaches auth -> 401 (DB-free)
# (b) absent token + disallowed Origin -> guard -> 403 (DB-free)
for O in "$ALLOWED" 'https://evil.example' 'null'; do
  printf 'ws origin %-24s -> ' "$O"
  curl -si -m 5 "$API/api/v1/ws" \
    -H 'Connection: Upgrade' -H 'Upgrade: websocket' \
    -H 'Sec-WebSocket-Version: 13' \
    -H 'Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==' \
    -H "Origin: $O" | head -1
done | tee "$WORK/evidence/05-ws-origin-guard.txt"
# EXPECT: allowed -> 401; evil/null -> 403

# (c) valid scoped token + allowed Origin -> auth path -> 101 Switching Protocols
curl -si -m 5 "$API/api/v1/ws" \
  -H 'Connection: Upgrade' -H 'Upgrade: websocket' \
  -H 'Sec-WebSocket-Version: 13' \
  -H 'Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==' \
  -H "Origin: $ALLOWED" -H "Authorization: Bearer $SCOPED" \
  | head -1 | tee "$WORK/evidence/06-ws-allowed-upgrade.txt"
# EXPECT 101. (curl is not a full WS client; guard+auth run before on_upgrade,
# so the status line alone is sufficient for this proof. Token never logged.)
```

Before E6, the same commands yield 401/401/401 and 101 — record that as "guard absent", **not** as a pass. The deterministic Rust oneshot tests in E7(ii) are the authoritative DB-free proof of 403 vs 401; the live curl probe is corroboration on the actual binary.

---

## 6. Evidence artifact (proposed, not written in this research pass)

Target: `docs/research/2026-11-16-034-live-proof-evidence.md`. Assemble only redacted text:

1. Header block: UTC timestamp, local HEAD `babac3ea`, canonical `1be723cc`, `API=http://127.0.0.1:3100`, DB name `flora_034_t8` (no password), CORS allowlist literal, and the explicit statement that `:3000` (pid `1799842`) was untouched.
2. Freshness discriminator: `/llms.txt` 200 (new) vs 404 (stale :3000).
3. `01-preflight-allowed.txt`, `02-preflight-denied.txt` (verbatim, redacted through `redact`).
4. `03-select-org-headers.txt`, `04-scoped-workspaces.txt` — status + ACAO + Vary only, tokens replaced by `<REDACTED>`.
5. `05-ws-origin-guard.txt`, `06-ws-allowed-upgrade.txt` — first status line only.
6. Caveats: curl does not enforce CORS; disallowed preflight is 200-without-ACAO, not 403; T7 browser/webview regression remains separate; `?token=` WS query leakage (030 NV-6) is unaddressed.

No real JWT, JWT secret, DB password, or `/proc` secret may appear in the artifact. The pipe through `redact()` is mandatory for every block that could carry a token.

---

## 7. Authorization boundary

| Action | Class | Why |
|---|---|---|
| `createdb`/`dropdb flora_034_t8`, migrations, `dev-seed.sql`, `dev-smoke-session.sql` | **Autonomous** | Dedicated DB; reversible; no shared rows |
| `cargo build -p flora-api`, start/kill a new pid on `:3100` with a pid file | **Autonomous** | Separate process/port; `:3000` and pid `1799842` never signalled |
| `OPTIONS`/`GET`/WS-handshake curl probes | **Autonomous** | Read-only HTTP (except select-org below) |
| Node/`dev-mint` token minting to `/tmp` | **Autonomous** | Uses a known dev secret; no production env; no secret logged |
| `POST /api/v1/auth/select-organization` | **Autonomous within this plan** | Writes only to `flora_034_t8` (session revoke + rotate); re-seed or drop to reverse |
| Apply E6 WS guard + E7 tests to `websocket.rs` / `flora-tests`, rebuild | **Needs human authorization** | Source change in an archived repo; cycle-034 implementation scope |
| Add Tauri origins to `FLORA_APP__CORS_ORIGINS` in any deployed config | **Needs human authorization** | Ops/security decision after a real webview smoke |
| ADR-0002 re-entry note / gate closure | **Needs human authorization** | Governance artifact, not a research doc |
| Stop/restart/reconfigure the `:3000` process | **Forbidden here** | Another agent's stale process; explicitly out of scope |

---

## 8. Risks / NVs

| ID | Item |
|---|---|
| **NV-1 (H)** | WS guard absent in canonical/local tree → step 4 cannot be honestly proven before E6 lands. Treat any pre-E6 "403" claim as invalid. |
| **NV-2 (M)** | Absent-`Origin` policy (allowed for native interop) is an explicit ADR decision; exact-match means a wrong Tauri literal silently blocks the webview. Not changed here. |
| **NV-3 (M)** | `5432/flora_023_test` lacks Alice/Acme and is shared; using it would require writes to another agent's DB. Fresh DB avoids this. |
| **NV-4 (M)** | `5432/flora` has only 1 migration applied; never point the new instance at it (would apply 6 migrations to a seeded shared DB). |
| **NV-5 (M)** | Seed compatibility with all 7 embedded migrations is source-derived, not executed in this pass. If `dev-seed.sql` fails after migrations, fallback: `CREATE DATABASE flora_034_t8 TEMPLATE <fully-migrated clean DB>` with no active connections, then load only `dev-smoke-session.sql` and seed Alice/Acme rows. |
| **NV-6 (M)** | Secret mismatch would 401 the minted token. Mitigated by exporting the same `FLORA_APP__JWT_SECRET` to API and minter; never copy the live process's secret. |
| **NV-7 (M)** | `curl` proves header emission and Bearer round-trip, not browser enforcement; T7 remains owed. |
| **NV-8 (L)** | `?token=` WS query token may reach proxy/access logs; unrelated to the Origin guard (030 NV-6). |
| **NV-9 (L)** | Port `3100` was free at research time; re-check `ss -ltnp` before starting. |
| **NV-10 (L)** | Config is startup-only; allowlist changes require restarting only the new `:3100` instance. |

---

## 9. Compact verdict

**FEASIBLE for steps 1–3 now, autonomously and non-destructively; step 4 BLOCKED on the E6 WS-guard source edit (human-authorized).**
Separate stack: `createdb flora_034_t8` on `127.0.0.1:5432` → export `FLORA_APP__HOST=127.0.0.1`, `FLORA_APP__PORT=3100`, `FLORA_APP__CORS_ORIGINS='["http://127.0.0.1:4173","http://localhost:5173"]'`, `FLORA_APP__JWT_SECRET=<known dev secret>`, `FLORA_DATABASE__POSTGRES_URL=…/flora_034_t8` → `cargo build -p flora-api` → run with pid file → seed `dev-seed.sql` + `dev-smoke-session.sql` → `/llms.txt` 200 freshness check → preflight allowed/denied → `cargo run -p flora-core --features dev-mint --bin flora-mint-session …` → `select-organization` with `ORG_ID=11111111-1111-1111-1111-111111111111` → WS probes (403 only after E6).
**Never touch** `:3000` / pid `1799842` / `@15433/flora` / `flora_023_test`. Cleanup = kill recorded pid + `dropdb flora_034_t8`.
**Blockers:** E6 source edit + E7 tests for WS-guard evidence; ADR note for gate closure. **NVs:** absent-Origin policy, Tauri literals, seed/7-migration compatibility, curl≠browser enforcement, secret parity.
