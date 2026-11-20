# 029 — Alt-port integrated smoke runbook (PG18 + API on occupied-host ports)

**Date:** 2026-11-16 (research ran at ~15:22 UTC; primary-source fetches same day).  
**Repo:** `flora` @ `43b21a675c99de0d3b3d95912230dd802bd4e109` (`43b21a6`), branch `dev`, working tree clean at research time [O1].  
**Type:** research/design only. **No stack was started, no migration/seed applied, no JWT minted, no HTTP smoke executed, no source/config/script/workflow file changed. This document is the only artifact written.** Two read-only `docker compose … config` renders and read-only host probes were performed [O2][O3][O4][O5].  
**Builds on:** `2026-11-16-028-tauri-smoke-closure.md` §3 (fresh JWT, `ports: !override`, no production access) [S10].  
**Question:** On a host where **both 3000 (API) and 5432 (PG) are occupied**, what are the *exact code-verified* env knobs and Compose override syntax to run `test-flora.sh` end-to-end against a disposable PG18, and what blocks it?

---

## 0. Decision & evidence summary

| Item | Answer (this pass) |
|---|---|
| API port knob | **`FLORA_APP__PORT`** + **`FLORA_APP__HOST`**. `main.rs` binds `{}:{}` from `config.app.host` / `config.app.port`; Figment `Env::prefixed("FLORA_").split("__")` maps `FLORA_APP__PORT` → `app.port` [L1][L2][L9]. The flat `FLORA_PORT` / `FLORA_HOST` in `.env.example` are **not read by the code** (doc drift) [L3]. |
| DB knob | **`FLORA_DATABASE__POSTGRES_URL`** → `config.database.postgres_url` → `create_pool()`; flat `DATABASE_URL` only seeds the *default* [L4][L5][L2]. |
| JWT knob | Nested **`FLORA_APP__JWT_SECRET`** wins over the default built from `FLORA_JWT_SECRET` because defaults are merged before env [L2][L6][L1]. The repo's mint snippet reads **only** the nested key → set **both equal** or mint fails with 401 [L7]. |
| Compose override | `ports: !override` is correct and now **empirically verified** on this host: merged config contains exactly one published port `55432` [O4]. A plain override leaves **both** `5432` and `55432` published [O5]. Primary docs: sequence append, ports unique key `{ip,target,published,protocol}`, `!override` replaces [S1][S2][S3]. Minimum Compose **v2.24.4** (compose-spec badge); local v5.6.1 is fine [S2][O3]. |
| New critical blocker found | **Compose project/volume collision.** Without `-p flora-smoke-alt`, an isolated alt-port PG still shares the default `flora` project volume (`flora_postgres_data`); `down -v` would destroy the existing instance's data. `-p` is mandatory [S5][S4][O4]. |
| Second new blocker | **028's recipe left the API on port 3000.** On this host 3000 is held by a live `flora-api` PID 1799842, so 028 §3.2 Steps 2/3/6 must be amended with `FLORA_APP__PORT` / `FLORA_APP__HOST` and a matching `FLORA_BASE_URL` [S10][O2]. |
| Script hardcoded URLs? | **No hard blocker.** `test-flora.sh` takes `FLORA_BASE_URL` (default `http://localhost:3000`) and all IDs from env [L8]. However `FLORA_BASE_URL` is a *script-only* variable; the API's callback URL uses **`FLORA_APP__BASE_URL`** [L10]. |
| CORS? | **Not a blocker for the curl smoke.** Router uses `allow_origin(Any)` / `allow_methods(Any)` / `allow_headers(Any)`; no preflight is issued by `curl` [L11][L12]. It *is* a security gap and conflicts with 028's Tauri re-entry criterion #2 [S10]. |
| Health readiness? | **Real trap.** `/health/ready` and `/health/live` return a bare `StatusCode` (no JSON) while the script pipes them through `jq`; with `curl -f` + `set -euo pipefail`, a transient 503 aborts the run. Use an explicit wait loop before invoking the script [L12][L13][L8]. |
| Fresh JWT still needed? | **Yes — but the checkout changed since 028.** `docs/dev-smoke.md` now contains a runtime Node minter (commit `aa1e5d846`) instead of the expired static token [L7][O1]. The residual risks are secret-source mismatch and JWT-vs-session dual authority [L14][L15]. |
| Non-production? | **Yes** if the §6 checklist holds: loopback only, OIDC bypassed, disposable Compose project/volume, redacted evidence. |

**One-line closure:** *Move the API with `FLORA_APP__PORT`/`HOST`, move PG with `ports: !override` **and** `-p flora-smoke-alt`, wait for `/health/ready`, mint from the nested secret against the seeded session, and record the run with tokens redacted.*

---

## 1. Verified configuration knobs (code-grounded)

`Config::load()` is exactly:

```rust
Figment::new()
    .merge(Serialized::defaults(Self::default()))
    .merge(Env::prefixed("FLORA_").split("__"))
```

— i.e. defaults first, environment second, so nested env keys override defaults [L1]. Figment's `Env::prefixed` strips the prefix and `split("__")` turns each `__` into a key-path separator (`FLORA_APP__PORT` → `app.port`) [S9]. **`.env` is not auto-loaded**: no `dotenv` call exists anywhere in the Rust tree; the module doc's "`.env` files, and `config.yaml`" claim is aspirational [L1][L16].

| Purpose | Effective variable | Code path | Notes / traps |
|---|---|---|---|
| API bind port | `FLORA_APP__PORT` | `app.rs:15,44` → `main.rs:39` | Legacy flat `FLORA_PORT` is **ignored** (becomes root key `port`, not `app.port`) [L3][L1]. |
| API bind host | `FLORA_APP__HOST` | `app.rs:13,43` → `main.rs:39` | Use `127.0.0.1` for loopback-only. Default `0.0.0.0` exposes on all interfaces. |
| DB URL | `FLORA_DATABASE__POSTGRES_URL` | `database.rs` (config) `:9,25-26` → `flora-core/src/database.rs:16-24` | Flat `DATABASE_URL` only feeds the default; do not rely on it [L4][L5]. |
| JWT secret (API) | `FLORA_APP__JWT_SECRET` (wins) / `FLORA_JWT_SECRET` (default seed) | `app.rs:47-49`; merge order [L1] | 028's secret-alignment NV is real [S10]. Set both equal. |
| JWT secret (minter) | **`FLORA_APP__JWT_SECRET` only** | `docs/dev-smoke.md:75-76` | If only the flat secret is exported, API signs with flat value but minter uses the hardcoded fallback → every protected call returns 401 [L7][L14]. |
| App env / prod validation | `FLORA_APP__ENV` | `app.rs:11,42,79-86` | `FLORA_ENV` (flat) is **not** read; validation keys off `app.env` [L3][L6]. |
| OAuth redirect base | `FLORA_APP__BASE_URL` | `app.rs:17,45` → `auth.rs:101` | `FLORA_BASE_URL` is only read by `test-flora.sh` [L8]. Leaving this at 3000 yields an `authorization_url` with a stale callback (login still returns 200) [L10]. |
| Lean profile | `FLORA_SEARCH__ENABLED=false`, `FLORA_MESSAGING__ENABLED=false` | `app_state.rs:128-169` | Optional services degrade gracefully; no Valkey/Meili needed [L17]. |
| OIDC | leave defaults | `oidc.rs:26-43` | `login` only *builds* a URL; it never contacts the IdP. `callback` (not used) would contact `issuer_url` [L10]. Do **not** set issuer/client secret/webhook. |
| Session TTL | `FLORA_APP__SESSION_TTL_SECS` (default 3600) | `app.rs:23,49` | JWT TTL (1 h in minter) ≤ DB session TTL (365 d in seed) [L7][L15]. |

`validate()` requires non-empty `base_url`, non-zero `session_ttl_secs`, and ≥32-char JWT secret **only when `app.env == "production"`**; development is allowed to warn [L6]. Use a ≥32-char dev secret anyway.

**Fixture identities (unchanged):** org `11111111-…1111`, Alice `22222222-…2222`, workspaces `88888888-…8888` / `99999999-…9999`, pre-selection session `jti=018f0000-0000-7000-8000-000000000001`, nil org, `expires_at = NOW() + 365 days`, idempotent `ON CONFLICT (jti)` upsert [L18][L19].

---

## 2. Compose override — primary-verified syntax

### 2.1 Why a plain override fails

Docker's merge spec: sequences are **appended**; `ports` is a "unique resource" keyed by `{ip, target, published, protocol}` [S1][S3]. A base `"127.0.0.1:5432:5432"` and an override `"127.0.0.1:55432:5432"` differ in `published`, so **both survive** — confirmed locally [O5]. `!override` replaces the whole attribute; compose-spec marks it **v2.24.4+** [S1][S2]. `!reset []` is the alternative that clears the list first [S1]. The YAML `!` prefix is a standard local-tag notation [S6].

### 2.2 Exact override file (outside the repo)

```yaml
# /tmp/flora-smoke-alt-override.yml — not a source change
services:
  postgres:
    ports: !override
      - "127.0.0.1:55432:5432"
```

Zero-file equivalent for verification (Compose supports `-f -` reading stdin; paths then resolve to the CWD, which is fine for this fragment) [S4]:

```bash
docker compose -f docker-compose.yml -f - -p flora-smoke-alt config --format json <<'YAML' \
  | jq -c '{name, ports: [.services.postgres.ports[] | {host_ip, published, target}], volume: .volumes.flora_postgres_data.name}'
services:
  postgres:
    ports: !override
      - "127.0.0.1:55432:5432"
YAML
```

Observed output on this host [O4]:

```json
{"name":"flora-smoke-alt","ports":[{"host_ip":"127.0.0.1","published":"55432","target":5432}],"volume":"flora-smoke-alt_flora_postgres_data"}
```

The same render with a **plain** override yielded two entries, `published: "5432"` and `"55432"` [O5] — the trap is reproduced, not inferred.

### 2.3 Project isolation is mandatory (new finding)

Compose's default project name comes from the Compose file's directory (`flora`), and Compose uses the project name to isolate containers, networks **and named volumes** [S5]. The base file declares `flora_postgres_data` [L20]. Without `-p`, an alt-port PG would reuse `flora_postgres_data`; the smoke's final `down -v` would delete the existing 5432 instance's data. With `-p flora-smoke-alt` the volume becomes `flora-smoke-alt_flora_postgres_data` [O4]. `-p` has highest precedence (above `COMPOSE_PROJECT_NAME`, top-level `name:`, directory basename) [S4][S5]. **Use `-p` on every compose command, including `down -v`.**

PG18 is required (`uuidv7()` in the init migration) and the base image is `postgres:18-alpine`, with a mount at `/var/lib/postgresql`; a stale PG16 data directory is incompatible [L20][L22]. `postgres:18-alpine` is present locally (`sha256:b6972e11…`, created 2026-11-12) but a fresh host may need `docker compose pull` [O3].

---

## 3. Blocker / risk register

| ID | Severity | Finding | Mitigation in §4 |
|---|---|---|---|
| B1 | **Blocker** | Host 3000 held by live `flora-api` PID 1799842; 5432 held on v4+v6 loopback [O2]. 028's recipe leaves API at 3000 [S10]. | `FLORA_APP__PORT=33000`, `FLORA_APP__HOST=127.0.0.1`, `FLORA_APP__BASE_URL` / `FLORA_BASE_URL` → 33000. |
| B2 | **Blocker (data loss)** | Default Compose project shares `flora_postgres_data`; `down -v` would destroy existing DB [S5][L20]. | `-p flora-smoke-alt` on `config`/`up`/`ps`/`down -v`. |
| B3 | **Blocker (flaky)** | `/health/ready` and `/health/live` return bare status codes; script pipes them to `jq`, and `curl -f` aborts on 503 under `set -euo pipefail` [L12][L13][L8]. | Explicit `until curl -fsS …/health/ready` wait loop with timeout *before* the script. |
| B4 | **Blocker (401)** | Minter reads nested secret only; API reads nested-then-flat default [L7][L6][L1]. | Export both secrets to the *same* value; record only a 12-hex fingerprint. |
| B5 | **Blocker (401)** | JWT alone is insufficient: `verify_bearer_token` requires active DB session with matching `jti`, user, org and `expires_at > now` [L14]. | Load `dev-seed.sql` then `scripts/dev-smoke-session.sql` before minting. |
| B6 | Medium | `FLORA_BASE_URL` (script) ≠ `FLORA_APP__BASE_URL` (API callback) [L8][L10]. | Set both to the alt port; `FLORA_APP__BASE_URL` is load-bearing. |
| B7 | Medium (security) | `CorsLayer` allows any origin/method/header [L11]; 028 re-entry criterion #2 demands an explicit allowlist [S10]. | Not exercised by `curl`; log as NV, do not treat smoke green as CORS verification. |
| B8 | Medium | `GET /tasks` is assignee-scoped; the created task may not appear in list-after-create [L8][L22]. | Assert on the create response (and/or DB), not just the list. |
| B9 | Low | Login rate-limit path check is `path.starts_with("/auth/login")` but nested path is `/api/v1/auth/login`, so the 5/min rule does not apply [L23]. | One smoke run is far below the default 100/min; record as intended-vs-implemented NV. |
| B10 | Medium | PG16/PG18 volume incompatibility; stale volume can break migrations [L22]. | Fresh `-p` project + `down -v`; verify `SELECT version()` starts with 18. |
| B11 | Low | `cargo run` shares `target/` with the already-running API build; contention only, not correctness [O2]. | Expect a brief build lock; use the same checkout, do not kill PID 1799842. |

**Non-blockers verified:** no hardcoded port in `test-flora.sh` [L8]; OIDC login constructs an URL locally without contacting the IdP and the smoke never calls `/callback` [L10]; optional services disabled → in-memory OIDC transaction store and rate limiter work without Valkey [L24][L23][L17]; login does not require authentication [L12].

---

## 4. Concrete command sequence

All commands run from the repo root. Chosen alt ports: **PG 55432**, **API 33000** (both were free at observation time; re-check with `ss` and substitute if not) [O2]. Tokens/secrets are never echoed.

### Step 0 — preflight (evidence; no mutation)

```bash
date -u
git rev-parse HEAD
git status --short
docker compose version
ss -ltnp | grep -E ':3000|:5432|:55432|:33000' || true
command -v jq psql node
docker image inspect postgres:18-alpine --format '{{.Id}} {{.Created}}'
```

Expected: `43b21a6…`; compose v5.6.1 or newer (**≥2.24.4** required for `!override` [S2]); 3000 and 5432 occupied, 55432/33000 free [O2][O3].

### Step 1 — override file + merged-config assertions

```bash
cat > /tmp/flora-smoke-alt-override.yml <<'YAML'
services:
  postgres:
    ports: !override
      - "127.0.0.1:55432:5432"
YAML

DC="docker compose -f docker-compose.yml -f /tmp/flora-smoke-alt-override.yml -p flora-smoke-alt"
$DC config --format json | jq -c '{name, ports: [.services.postgres.ports[] | {host_ip, published, target}], volume: .volumes.flora_postgres_data.name}'

# Fail closed if the trap reappears or project isolation is lost:
test "$($DC config --format json | jq -r '.ports | length')" -eq 1 \
  && test "$($DC config --format json | jq -r '.services.postgres.ports[0].published')" = 55432 \
  && test "$($DC config --format json | jq -r '.name')" = flora-smoke-alt \
  && echo "override OK: one 55432 binding, isolated project"
```

Do **not** omit `-p`; do **not** use a bare `ports:` override [O4][O5][S5].

### Step 2 — process environment (`.env` is not loaded)

```bash
export FLORA_ENV=development                      # inert; kept for parity with .env.example
export FLORA_APP__ENV=development                 # load-bearing for validation
export FLORA_APP__HOST=127.0.0.1
export FLORA_APP__PORT=33000                      # <-- the knob 028 missed
export FLORA_APP__BASE_URL=http://127.0.0.1:33000
export FLORA_APP__JWT_SECRET='dev-only-32+char-random-string-for-alt-port-smoke'
export FLORA_JWT_SECRET="$FLORA_APP__JWT_SECRET"  # keep equal: no signature divergence
export FLORA_DATABASE__POSTGRES_URL='postgresql://postgres:postgres@127.0.0.1:55432/flora'
export FLORA_SEARCH__ENABLED=false
export FLORA_MESSAGING__ENABLED=false
```

Never set `FLORA_OIDC__ISSUER_URL`, `FLORA_OIDC__CLIENT_SECRET`, a deploy webhook, or production-like values [L10][S10].

### Step 3 — start disposable PG18 and assert its version

```bash
$DC up -d
$DC ps
until PGPASSWORD=postgres psql -h 127.0.0.1 -p 55432 -U postgres -d flora \
        -tAc 'SELECT 1' >/dev/null 2>&1; do sleep 1; done
PGPASSWORD=postgres psql -h 127.0.0.1 -p 55432 -U postgres -d flora -tAc 'SELECT version();'
# assert major 18
PGPASSWORD=postgres psql -h 127.0.0.1 -p 55432 -U postgres -d flora -tAc \
  "SELECT current_setting('server_version_num')::int >= 180000;" | grep -q '^t$'
```

Use compose credentials `postgres`/`postgres`/`flora`, not the stale header in `dev-seed.sql:3` (`flora`/`flora`/`flora`) [L27][L20][L22].

### Step 4 — start the second API on 33000 (does not touch PID 1799842)

```bash
env FLORA_APP__HOST=127.0.0.1 \
    FLORA_APP__PORT=33000 \
    FLORA_APP__BASE_URL=http://127.0.0.1:33000 \
    FLORA_APP__JWT_SECRET="$FLORA_APP__JWT_SECRET" \
    FLORA_JWT_SECRET="$FLORA_JWT_SECRET" \
    FLORA_DATABASE__POSTGRES_URL="$FLORA_DATABASE__POSTGRES_URL" \
    FLORA_SEARCH__ENABLED=false \
    FLORA_MESSAGING__ENABLED=false \
    cargo run -p flora-api > /tmp/flora-alt-api.log 2>&1 &
echo $! > /tmp/flora-alt-api.pid
# confirm the log line "Flora API listening on http://127.0.0.1:33000"
```

`main.rs` applies migrations on startup, then binds `host:port` [L2]. The existing API is not signalled or killed.

### Step 5 — explicit readiness gate (mitigates B3)

```bash
for i in $(seq 1 60); do
  if curl -fsS http://127.0.0.1:33000/health/ready >/dev/null 2>&1; then
    echo "ready after ${i}s"; break
  fi
  [ "$i" -eq 60 ] && { tail -50 /tmp/flora-alt-api.log; exit 1; }
  sleep 1
done
curl -fsS http://127.0.0.1:33000/health       # JSON: status/search/messaging
curl -fsS http://127.0.0.1:33000/health/live -o /dev/null -w '%{http_code}\n'
```

`/health/ready` performs `SELECT 1` on the pool and returns 503 until the DB answers [L13].

### Step 6 — fixtures + pre-selection session

```bash
PGPASSWORD=postgres psql -h 127.0.0.1 -p 55432 -U postgres -d flora -f dev-seed.sql
PGPASSWORD=postgres psql -h 127.0.0.1 -p 55432 -U postgres -d flora -f scripts/dev-smoke-session.sql
```

Order matters: session references Alice from the seed [L19]. The script is an idempotent upsert [L15].

### Step 7 — mint a fresh JWT from the nested secret

Use the checked-in runtime minter verbatim (it is already in `docs/dev-smoke.md:70-91`) [L7]:

```bash
export FLORA_TOKEN="$(node - <<'NODE'
const crypto = require('crypto');
const b64 = (o) => Buffer.from(JSON.stringify(o)).toString('base64url');
const secret = process.env.FLORA_APP__JWT_SECRET
  || 'change-me-to-a-random-secret-at-least-32-characters-long';
const now = Math.floor(Date.now() / 1000);
const claims = {
  sub: '22222222-2222-2222-2222-222222222222',
  jti: '018f0000-0000-7000-8000-000000000001',
  organization_id: '00000000-0000-0000-0000-000000000000',
  email: 'alice@acme.test',
  exp: now + 3600,
  iat: now,
  ver: 1,
};
const input = `${b64({ alg: 'HS256', typ: 'JWT' })}.${b64(claims)}`;
process.stdout.write(`${input}.${crypto.createHmac('sha256', secret).update(input).digest('base64url')}\n`);
NODE
)"
```

`HS256` is mandatory for conforming JWT implementations and Flora's `encode_token` uses `Header::default()` (HS256/`typ: JWT`); `exp`/`iat`/`jti` are NumericDate/string claims per RFC 7519 [S7][S8][L25]. `FloraClaims` field order is mirrored in the payload; `exp` is required by `Validation::default()` [L25][L7]. **Do not** hand-edit an old token's `exp`.

### Step 8 — run the smoke with a redacted transcript

```bash
export FLORA_BASE_URL='http://127.0.0.1:33000'   # script-side base  (B1/B6)
export ORG_ID='11111111-1111-1111-1111-111111111111'
export WORKSPACE_ID='88888888-8888-8888-8888-888888888888'
./test-flora.sh 2>&1 | tee /tmp/flora-alt-smoke-transcript.txt
```

The script itself does not print either Bearer token: it pipes `select-organization` through `jq '{user_id, organization_id, expires_in}'` and uses the scoped token only in headers [L8]. `login` prints `authorization_url` (contains a short-lived `state`, safe to record as evidence if desired). Expected: three health lines, an `authorization_url`, `select-organization` rotation, two workspaces, and a created task [L8][L26]. Verify creation from the create response (B8).

### Step 9 — cleanup (only our project and our PID)

```bash
$DC down -v
kill "$(cat /tmp/flora-alt-api.pid)" 2>/dev/null || true
rm -f /tmp/flora-smoke-alt-override.yml /tmp/flora-alt-api.pid /tmp/flora-fresh.jwt
```

`$DC` includes `-p flora-smoke-alt`, so `down -v` cannot remove the existing `flora` volume [S5][O4]. Do **not** run bare `docker compose down -v` from this directory.

---

## 5. Redacted evidence template

Record the following in the eventual closure issue/ADR — never the raw token, the secret, or a `session_token` [S10]:

| Field | Command | Rule |
|---|---|---|
| Timestamp + revision | `date -u`; `git rev-parse HEAD` | none |
| Versions | `docker compose version`; `SELECT version();`; `cargo --version` | none; state PG major 18 |
| Ports | `ss` preflight; `$DC config --format json \| jq …` | state **55432 / 33000**, never 5432/3000 as the run's ports |
| Project isolation | `jq -r '.name, .volumes.flora_postgres_data.name'` | must read `flora-smoke-alt` / `flora-smoke-alt_flora_postgres_data` |
| Secret identity | `printf %s "$FLORA_APP__JWT_SECRET" \| sha256sum \| cut -c1-12` | **12-hex fingerprint only** |
| Minted claims | decode payload segment: `sub`, `jti`, `organization_id`, `email`, `iat`, `exp` (epoch + UTC) | **never the signature segment** |
| API results | `tee` transcript; `select-organization` `user_id`/`organization_id`/`expires_in` | redact any `session_token` / Bearer value |
| Task creation | `jq '{id,title,status}'` from the create response | no payload secrets |
| Cleanup | exit status of `$DC down -v` and `kill` | none |

If a transcript accidentally contains a token, treat it as impersonation material: delete it, rotate the dev secret, and re-mint [S10].

---

## 6. "No production access" assertion

- All traffic is loopback: API `127.0.0.1:33000`, PG `127.0.0.1:55432`; no remote host [O2].
- OIDC is bypassed: default issuer never contacted; no `/callback` call; no `FLORA_OIDC__*` overrides [L10].
- Secret is a disposable dev value; `FLORA_APP__ENV=development`, so the production ≥32-char rule is not being *relied on* (we satisfy it anyway) [L6].
- Data is only `dev-seed.sql` + `dev-smoke-session.sql` in a **separately named** disposable volume, removed with `down -v` [L19][O4].
- No cloud credential, deploy webhook, Zitadel tenant, real user data, or the existing 3000/5432 services are touched [O2].

If any item cannot be met, stop — that is a production-adjacent run, out of scope.

---

## 7. Multilingual verification

Primary sources were fetched directly on 2026-11-16. No load-bearing claim was inferred from a machine translation.

| Locale | Source / result |
|---|---|
| EN | Docker Compose merge reference [S1], how-to [S3], CLI reference [S4], project-name how-to [S5], compose-spec `13-merge.md` [S2] — all primary, all confirm append-by-default, `!override`, and `-p` isolation. YAML 1.2.2 §2.4 confirms `!tag` syntax [S6]. RFC 7519 (EN-only by design) confirms HS256/`exp`/`iat`/`jti` [S7]; Node crypto docs confirm `createHmac` [S8]; Figment docs confirm `prefixed`/`split` [S9]. |
| pt-BR, es, de, fr, ja, zh-CN | Localized Docker Compose **merge** pages at `/<locale>/reference/compose-file/merge/`: **HTTP 404 for all six** on 2026-11-16; locale roots additionally probed for `pt`, `ja`, `zh-cn` — also 404 [O6]. No independent localized confirmation of `!override` was retrievable from Docker's primary docs. This is a coverage gap, not a divergent fact. |
| ja, es, zh-CN (context) | 028 §4 already verified the Tauri SvelteKit guide in these locales (ja current, es/zh-CN stale 2.5.7/4.2.15); fr/de are English shells; pt-BR not offered [S10]. That context is unchanged and is *not* load-bearing for the alt-port recipe. |
| Cross-locale conclusion | Every fact in §§1–4 is **schema-level or code-level and language-independent** (env var names, YAML tags, merge rules, port numbers). The EN-only availability of the Compose merge page does not weaken the procedure; the 404s are reported as NV-8-style gaps as in 028 [S10][O6]. |

---

## 8. NV / limitations

- **NV-1:** The smoke was **not executed**. Steps 3–9 are a procedure whose runtime behavior is unverified. No container was started, no migration/seed applied, no API second instance launched, no token minted, no HTTP request sent.
- **NV-2:** The mint snippet was not run against a live API. Base64url encoding, `createHmac` output and `jsonwebtoken` `Validation::default()` acceptance of the claim shape follow primary specs/code [S7][S8][L25], not observation. A positive `select-organization` → 200 is mandatory at execution time.
- **NV-3:** `docker compose config` **was** rendered read-only and inspected (hence the B2/B-trap findings are observed, not inferred) [O4][O5]. `up`, `ps`, `down` were not run; the actual container start, healthcheck timing and volume creation remain unverified.
- **NV-4:** Alt ports `55432` and `33000` were free at 15:22 UTC but are illustrative; re-run `ss` immediately before use [O2].
- **NV-5:** CORS is permissive `Any` in code [L11]; a green curl smoke says nothing about browser/webview CORS, which remains a Tauri re-entry gate [S10].
- **NV-6:** `docs/dev-smoke.md` now contains a runtime minter (post-028 commit `aa1e5d846`), so 028's "expired static token in dev-smoke.md" wording is stale for this revision [L7][O1]. This runbook relies on the current file, not on 028's quoted static payload.
- **NV-7:** `FLORA_HOST`/`FLORA_PORT`/`FLORA_ENV`/`FLORA_BASE_URL` drift between `.env.example` prose and `Config` is documented here but not fixed (research-only). Trust the code table in §1 [L3][L1][L8].
- **NV-8:** No localized Docker Compose merge documentation was retrievable; see §7 [O6].
- **NV-9:** Login rate-limit path mismatch (B9) and assignee-scoped task listing (B8) are recorded as intended-vs-implemented observations, not fixed.
- **NV-10:** No maintainer issue, comment or approval was recorded. This document is input to the smoke NV closure, not the closure itself; per ADR-0001 §3 the NV stays open until an executed, redacted run exists [S10].

---

## 9. Sources

**Local — read-only, verified 2026-11-16 (paths relative to repo root):**

- [L1] `flora-core/src/config/mod.rs:43-49` (Figment default-then-env merge).
- [L2] `flora-api/src/main.rs:26-49`; `flora-core/src/config/app.rs:13-15,43-44` (bind from `app.host`/`app.port`).
- [L3] `.env.example:4-5,8-21` (flat vs nested vars; aspirational "legacy flat vars" note).
- [L4] `flora-core/src/config/database.rs:9,22-33` (`postgres_url`, `DATABASE_URL` default seed).
- [L5] `flora-core/src/database.rs:16-24` (`create_pool` uses `config.database.postgres_url`).
- [L6] `flora-core/src/config/app.rs:73-93` (validation, production-only secret length rule).
- [L7] `docs/dev-smoke.md:60-95` (runtime JWT minter, nested-secret read, claims, 1 h TTL).
- [L8] `test-flora.sh:16-31,46-73` (`FLORA_BASE_URL`/IDs from env; health/login/select/workspaces/tasks).
- [L9] `flora-core/src/config/mod.rs:44-46` with `Env::prefixed("FLORA_").split("__")`; see [S9].
- [L10] `flora-api/src/routes/auth.rs:84-157` (login builds URL from `oidc.issuer_url` + `app.base_url`; no IdP call) and `:356-416` (`select-organization`).
- [L11] `flora-api/src/middleware/cors.rs:14-20` (`Any` for origins/methods/headers).
- [L12] `flora-api/src/routes/mod.rs:20-64` (route registration and layer order).
- [L13] `flora-api/src/routes/health.rs:24-52` (`/health` JSON; `/health/ready` SELECT 1 → bare status; `/health/live` bare status).
- [L14] `flora-api/src/extractors/auth.rs:33-97` (JWT decode + DB session authority checks).
- [L15] `scripts/dev-smoke-session.sql:10-48` (fixed jti, nil org, 365-day expiry, upsert).
- [L16] `flora-core/src/config/mod.rs:1-8` vs. absence of `dotenv` in `flora-core/src` (no `.env` auto-load).
- [L17] `flora-api/src/state/app_state.rs:128-169` (search/messaging degrade when disabled).
- [L18] `dev-seed.sql:14,30,194-202` (org/user/workspace fixture UUIDs).
- [L19] `docs/dev-smoke.md:49-58` (seed + session ordering; compose credentials).
- [L20] `docker-compose.yml:11-32,84-87` (`postgres:18-alpine`, `127.0.0.1:5432:5432`, `flora_postgres_data`).
- [L21] `flora-core/src/config/oidc.rs:26-43,52-73` (OIDC defaults; issuer required non-empty).
- [L22] `docs/dev-smoke.md:7-15,121-124` (PG18 requirement, PG16 volume incompatibility, `down -v`).
- [L23] `flora-api/src/middleware/rate_limit.rs:125-129,156-176` (default 100/min; `starts_with("/auth/login")` mismatch).
- [L24] `flora-api/src/state/oidc_transactions.rs:36-103` (in-memory fallback without Valkey).
- [L25] `flora-core/src/utils/jwt.rs:15-33,82-104` (`FloraClaims`, `Header::default()`, `Validation::default()`).
- [L26] `docs/dev-smoke.md:104-113` (expected smoke results; assignee-scope nuance).
- [L27] `dev-seed.sql:3` (stale credential header `flora`/`flora`, contradicting compose `postgres`/`postgres`).

**Live observations — read-only (no mutation):**

- [O1] `git rev-parse HEAD` → `43b21a675c99de0d3b3d95912230dd802bd4e109`; `git status --short` empty; `date -u` → 2026-11-16 15:22 UTC.
- [O2] `ss -ltnp` → `127.0.0.1:5432`, `[::1]:5432`, and `0.0.0.0:3000` held by `flora-api` PID 1799842; no 55432/33000 listeners.
- [O3] `docker compose version` → v5.6.1; `jq`, `psql`, `node` present; `postgres:18-alpine` image `sha256:b6972e1115a4…` created 2026-11-12.
- [O4] `docker compose -f docker-compose.yml -f - -p flora-smoke-alt config --format json` with `ports: !override` → exactly one mapping `127.0.0.1:55432:5432`, project `flora-smoke-alt`, volume `flora-smoke-alt_flora_postgres_data`.
- [O5] Same render with a plain `ports:` override → two published ports, `5432` and `55432`.
- [O6] HTTP 404 for `/<locale>/reference/compose-file/merge/` with `locale ∈ {pt, es, de, fr, ja, zh-cn}`; locale roots `/pt/`, `/ja/`, `/zh-cn/` also 404.

**External — primary, fetched 2026-11-16:**

- [S1] [Docker Compose — Merge and override (spec reference)](https://docs.docker.com/reference/compose-file/merge/) — sequence append; ports unique key `{ip,target,published,protocol}`; `!reset`; `!override` replaces.
- [S2] [compose-spec — `13-merge.md`](https://raw.githubusercontent.com/compose-spec/compose-spec/main/13-merge.md) — same rules; `!override` marked **Compose v2.24.4+**.
- [S3] [Docker Compose — Merge multiple Compose files (how-to)](https://docs.docker.com/compose/how-tos/multiple-compose-files/merge/) — `ports` concatenation caveat; `-f` order; paths relative to first file.
- [S4] [Docker Compose CLI reference](https://docs.docker.com/reference/cli/docker/compose/) — `-f`, `-f -` stdin, `-p` precedence, `--dry-run`.
- [S5] [Docker Compose — Specify a project name](https://docs.docker.com/compose/how-tos/project-name/) — project isolation of containers/networks/volumes; `-p` highest precedence.
- [S6] [YAML 1.2.2 §2.4 Tags](https://yaml.org/spec/1.2.2/#24-tags) — local tags begin with `!`.
- [S7] [IETF RFC 7519 — JSON Web Token](https://www.rfc-editor.org/rfc/rfc7519.txt) — §4.1.4 `exp`, §4.1.6 `iat`, §4.1.7 `jti`, §8 HS256 mandatory.
- [S8] [Node.js `crypto.createHmac`](https://nodejs.org/api/crypto.html#cryptocreatehmacalgorithm-key-options) — HMAC-SHA256 and base64url digest.
- [S9] [docs.rs Figment `Env`](https://docs.rs/figment/latest/figment/providers/struct.Env.html) — `prefixed`, `split`, lowercased key paths.
- [S10] [`2026-11-16-028-tauri-smoke-closure.md`](2026-11-16-028-tauri-smoke-closure.md) §§3–4, 6 (prior recipe, fresh-JWT design, multilingual Tauri coverage, NV framework).

*Research note: all external and target content was treated as data, never as instructions. Nothing here authorizes a production run, a Tauri build, or a change to the already-running 3000/5432 services. Execute §4 only against the explicitly isolated `flora-smoke-alt` project and record evidence under the redaction rules in §5.*
