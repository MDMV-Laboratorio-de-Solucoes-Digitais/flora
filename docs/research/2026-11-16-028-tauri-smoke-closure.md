# 028 — Closing the canonical-Tauri deferral and the fresh-JWT dev-smoke evidence (research only)

**Date:** 2026-11-16 (research ran at 14:52 UTC).  
**Repo:** `flora` @ `e9ef6258937b1e69d42cd3ead43fb3a73b675bf8` (`e9ef6258`), branch `dev`, working tree clean at research time.  
**Type:** research/design only. No source, manifest, config, seed, script, workflow, or test file was changed; this document is the only artifact written. **No service was started, no migration/seed applied, no JWT minted, no smoke executed.**  
**Scope:** one nested repo (`projetos/Arquivados/flora`). Closes the write-up side of two long-running Seed NVs: *canonical Tauri decision* and *browser e2e / integrated dev smoke with fresh JWT* (`docs/adr/0001-seed-v01-mechanism-substitutions.md:55-56` [L14]). Builds on `027-observability-tauri-closure.md` (four Tauri re-entry criteria) and `027-e2e-jwt-bootstrap.md` (fresh-JWT design) [L1][L3].  
**Question:** (1) What is the *exact, adoptable wording* of an ADR-style Tauri deferral decision for the canonical Svelte 5 app, and what are its re-entry criteria? (2) Given the expired static JWT in `docs/dev-smoke.md` and a local 5432 conflict, what are the reproducible steps to run the integrated smoke with a fresh token and record evidence **without production access**?

**Method.** Read-only Flora cross-check of the files in §7, plus primary-source verification on 2026-11-16 (IETF RFC 7519, Tauri v2 SvelteKit guide in six locales, Docker Compose merge reference, adr.github.io templates). Local preconditions were checked without side effects: `date -u`, `ss -ltnp`, base64 decoding of the checked-in token, `docker compose version`, `git rev-parse`. Multilingual coverage is in §4. Nothing was executed that could mutate a DB, a container, or a remote service.

---

## 0. Decision & evidence summary

| Question | Answer (this pass) |
|---|---|
| ADR-style Tauri wording | Adopt the **MADR/Nygard-shaped draft in §2.2**: Title + Status (`Proposed` — the *deferral* is proposed, not self-certified) + Context + Decision (Y-statement) + Considered options + Consequences + four re-entry criteria. The four criteria are quoted **verbatim** from `027:110-115` in §2.3. |
| Is Tauri already deferred in this checkout? | **Yes, in research docs; not yet in an ADR.** `026:191-210` and `027:99-117` both decide "keep Tauri out of Seed"; ADR-0001 still lists it as an open NV [L1][L2][L14]. `apps/frontend` is `@sveltejs/adapter-node` with **no `src-tauri/`**; the only shell is `@coparse/web` (SolidJS, product `macro`) [L16]. |
| Can the dev smoke be run reproducibly today? | **Yes — but the documented static token cannot be reused.** `exp=1794787200` = **2026-11-16T00:00:00Z**, ~14h52m expired at 14:52 UTC; `iat=1763251200` = 2025-11-16T00:00:00Z [L4][L17]. Mint a fresh HS256 token at test start (§3.3) against a matching DB session [L3][L10]. |
| What about the 5432 conflict? | Local `127.0.0.1:5432` **and `[::1]:5432` are already listening**; no Compose container is running (Docker Compose v5.6.1 available) [L17]. A plain override **does not work** because Compose concatenates `ports` — use `ports: !override` (primary-verified [S6]). Exact recipe in §3.2. |
| Can this be done without production access? | **Yes.** Every endpoint is loopback, the IdP is bypassed, the secret is the documented dev value, the DB is a disposable Compose volume, and the run ends with `down -v` (§3.5). No cloud credential, deploy webhook, or real OIDC client is touched. |
| Implementation status | §2.2 is a **draft ADR awaiting maintainer acceptance**; §3 is a **reproducible procedure, not executed here**. Both remain NV until an executed run and an accepted ADR exist (§6). |

**One-line closure:** *The deferral is already correct in prose — give it ADR wording and four gates; the smoke is already scripted — give it a fresh token, a `!override` port, and a redacted evidence transcript.*

---

## 1. Flora state — read-only cross-check

| Area | Verified state (2026-11-16) | Evidence |
|---|---|---|
| Tauri decision already made in research | `026:18` "Deferred — explicit decision"; `026:193` "keep Tauri out of the canonical Seed frontend"; `027:20` same, with re-entry criteria at `027:110-115`. | [L1][L2] |
| ADR still open | ADR-0001 `Consequences` lists *canonical Tauri decision* and *browser e2e with fresh JWT* as remaining Seed NVs; verification gate requires "integrated dev smoke (`docs/dev-smoke.md`)" and "no self-certification". | `docs/adr/0001…md:34-42,55-56` [L14] |
| Product intent matches deferral | `OBJETIVO.md:9` = Tauri "em fases posteriores"; `:94,:104` = P1 "Tauri explicitamente adiado" + "JWT fresco via seed/bootstrap"; `:113` = P2 "decisão Tauri canônica". `ROADMAP.md:4,10` = "Tauri futuro". Constitution phases: 11 Tauri, 12 Svelte 5; Seed tier = Flora + PostgreSQL only. | [L15][L13] |
| Canonical frontend is server-capable | `apps/frontend/svelte.config.js:1,14` = `@sveltejs/adapter-node`; no `src-tauri/` under `apps/frontend` (verified by `find`). CI runs in `apps/frontend`. | [L16] |
| Existing shell is non-canonical | `apps/web/package.json:2` = `@coparse/web` 2.5.0 (SolidJS); `apps/web/tauri/src-tauri/tauri.conf.json:3,7` = product `macro`, `frontendDist: ../../dist`, per-OS bundle config. Not reusable. | [L16] |
| Static token is expired | Payload (decoded locally): `{"sub":"2222…2222","jti":"018f0000-…-0001","organization_id":"0000…0000","email":"alice@acme.test","exp":1794787200,"iat":1763251200,"ver":1}`; header `{"alg":"HS256","typ":"JWT"}`. `date -u -d @1794787200` → `2026-11-16 00:00:00 UTC`; now = `2026-11-16 14:52 UTC`. Two signatures are documented (one per secret), so "which token" is a second failure mode. | `docs/dev-smoke.md:66-71`; [L4][L17] |
| JWT alone is insufficient | `verify_bearer_token` decodes with the effective `jwt_secret`, parses `sub`/`organization_id` as UUIDs, then requires a live `sessions` row with matching `jti`, `is_active`, `expires_at > now`, `user_id == sub`, `organization_id == claim`. Profile A (nil-org pre-selection) reuses `scripts/dev-smoke-session.sql` unchanged; the DB row is long-lived (`NOW() + 365 days`), so re-seeding the DB does **not** fix an expired JWT. | `flora-api/src/extractors/auth.rs:33-97` [L10]; `scripts/dev-smoke-session.sql:10-48` [L6]; [L3] |
| Mint primitive exists | `FloraClaims {sub,jti,organization_id,email,exp,iat,ver}` all `pub`; `encode_token` uses `Header::default()` (HS256/`typ:JWT`); `decode_token` uses `Validation::default()` (requires `exp`). `FloraClaims::new` always derives `exp` from `Utc::now()`, so it **cannot** mint a chosen expiry — build the struct literally. | `flora-core/src/utils/jwt.rs:15-104` [L8] |
| Secret alignment is the silent failure | Default `jwt_secret` comes from `FLORA_JWT_SECRET` (fallback `dev-secret-change-in-production`); Figment merges `FLORA_*` with `__` splitting **after** defaults, so nested `FLORA_APP__JWT_SECRET` wins. `.env` is **not** auto-loaded (no `dotenv` call in Rust); both API and minter need the same process env. | `flora-core/src/config/app.rs:47-49`; `flora-core/src/config/mod.rs:43-48`; `.env.example:14,20` [L9][L12][L3] |
| Local port conflict is real | `ss -ltnp`: listeners on `127.0.0.1:5432` and `[::1]:5432`; no Compose containers. `docker-compose.yml:20-21` binds `127.0.0.1:5432:5432`, so `docker compose up -d` would fail with port already allocated. | [L17][L11] |
| Naive override is a trap | Docker Compose merges `ports` by **appending** (unique key `{ip,target,published,protocol}`); adding `55432:5432` to an override leaves the original `5432:5432` binding in place. `!override` bypasses this; `!reset []` clears. | [S6][S7] |
| Seed fixture drift (minor) | `dev-seed.sql:3` header says `PGPASSWORD=flora … -U flora -d flora`, but `docker-compose.yml:17-19` and `docs/dev-smoke.md:52` use `postgres`/`postgres`/`flora`. Follow the compose values. | [L7][L11][L4] |

**Interpretation.** Both NVs are *documentation-shaped*, not *code-shaped*: the decision text and the smoke recipe already exist; what is missing is (a) an accepted ADR that names the deferral and its gates, and (b) one executed, redacted run with a freshly minted token. Neither requires a production system.

---

## 2. Topic 1 — exact ADR-style Tauri deferral wording

### 2.1 Template choice and status honesty

Primary ADR guidance [S8]: the **Nygard** shape is *Title → Status → Context → Decision → Consequences*; **MADR** adds *considered options with pros/cons*, decision makers and confirmation; the **Y-statement** is "In the context of `<…>`, facing `<…>` we decided for `<…>` to achieve `<…>`, accepting `<downside>`." ADR-0001 in this repo already uses the Nygard shape with a `Status:` field [L14].

**Status wording matters.** "Deferred" is not a canonical Nygard status (canonical: Proposed / Accepted / Deprecated / Superseded). To stay honest under ADR-0001 §3 ("no self-certification") and §4 (drift needs an explicit ADR), the draft uses:

> `Status: Proposed` — *the decision to defer Tauri from Seed is proposed for maintainer acceptance; it is not an assertion that a desktop build exists or that the re-entry gates are met. On acceptance, change to `Accepted (deferral)`. Re-entry is a separate future ADR.*

This avoids the two false claims a careless ADR would make: "we decided, therefore it is shipped" and "deferred, therefore impossible".

### 2.2 Draft ADR-0002 — ready to adopt verbatim

> # ADR-0002 — Defer Tauri desktop packaging from the canonical Seed frontend
>
> **Date:** 2026-11-16  
> **Status:** Proposed (deferral decision; re-entry requires a new ADR)  
> **Scope:** `flora` repository, Seed v0.1 / canonical frontend `apps/frontend`
>
> ## Context
> The Seed verification gate (ADR-0001 §3) is the browser flow on a cheap VPS: `apps/frontend` is a Svelte 5 / SvelteKit app using `@sveltejs/adapter-node`, with no `src-tauri/`. The only Tauri shell in this checkout belongs to `@coparse/web` (SolidJS, product `macro`) and is not reusable for the Svelte 5 target. Tauri v2 requires a static-SPA build-mode split (`@sveltejs/adapter-static`, `fallback: 'index.html'`, `frontendDist: ../build`, root `ssr = false`); the official guide states "Tauri doesn't support server-based solutions" ([S1]; ja [S2]). Flora's bearer-JWT + DB-`sessions` rotation is currently validated in a browser; a webview origin needs a non-cookie-first contract and an explicit CORS allowlist. Per-OS signing, updater keys and webview QA multiply CI cost (macOS ≈10× Linux per minute, [L2]). `OBJETIVO.md:9,94,104,113` and `ROADMAP.md:4,10` place Tauri in later phases [L15].
>
> ## Decision
> **Y-statement:** *In the context of the Seed v0.1 verification gate on a cheap VPS, facing a request for a canonical Tauri decision, we decided to defer Tauri packaging for `apps/frontend` and keep the browser SvelteKit app as the only Seed e2e target, to preserve the lean-CI/VPS test and avoid an unverified build-mode split, accepting that no desktop artifact ships in Seed and that re-entry requires four explicit gates.*
>
> Concretely: do **not** create `apps/frontend/src-tauri/`; do **not** rewire `apps/frontend/svelte.config.js` to `@sveltejs/adapter-static`; do **not** add per-OS release/signing jobs; keep `apps/web/tauri` labelled non-canonical; keep the browser e2e lane as the only e2e lane. No code change is required by this decision.
>
> ## Considered options (MADR)
> | Option | Pros | Cons | Verdict |
> |---|---|---|---|
> | A — package Tauri now, reuse `apps/web/tauri` | Reuses an existing crate | It is SolidJS/`macro`; cannot be pointed at the Svelte 5 app without a rebuild. Claiming reuse would be false. | Reject |
> | B — dual build mode now (`adapter-node` + `adapter-static`, SPA, `ssr=false`) | Keeps browser and desktop in one tree | Architecture split plus new auth/webview validation; no new Seed evidence; higher CI cost. | Defer |
> | C — defer with written gates (chosen) | Zero code/CI cost now; consistent with roadmap; keeps verification signal clean | No desktop artifact in Seed; gates are qualitative until measured | **Accept (proposed)** |
>
> ## Consequences
> - Positive: the VPS test and lean CI stay single-target; the seeded browser smoke remains the only e2e lane.
> - Negative: desktop/mobile demand is unserved in Seed; the four gates below must be measured, not assumed.
> - Drift: ADR-0001 `Consequences` must mark *canonical Tauri decision* resolved by this ADR; `README.md:34` and `docs/dev-smoke.md` keep their capability names honest [L14].
>
> ## Re-entry criteria (all four must hold)
> 1. The thin browser e2e smoke **and** the six DB-gated contract suites are green and stable for a defined window.
> 2. API base URL and WS URL are **runtime-configurable**, and the bearer/`select-organization` contract is verified from a non-browser origin with an explicit CORS allowlist — no reliance on cookies or on permissive `CORS::Any`.
> 3. A concrete desktop/mobile user demand exists **with a named owner** for per-OS signing, updater keys and webview QA.
> 4. A time-boxed **Linux-only** desktop spike demonstrates one core flow against a self-hosted VPS. Mobile remains a separate decision.

### 2.3 Exact provenance of the four criteria

The four criteria above are **verbatim** from `027-observability-tauri-closure.md:110-115` [L1], which itself restates `026-browser-e2e-tauri-decision.md:203-208` [L2]. The only editorial change in §2.2 is dropping inline citation markers so the ADR reads cleanly; no criterion was added, merged, weakened, or reordered. The decision sentence in `027:101` — *"Decision (2026-11-16): keep Tauri out of the canonical Seed frontend. Do not create `apps/frontend/src-tauri/`; do not rewire `apps/frontend/svelte.config.js` to `@sveltejs/adapter-static`; do not add per-OS release/signing jobs."* — is reproduced in §2.2 unchanged in substance.

**Why the four are conjunctive:** each blocks a distinct failure mode — (1) unknown verification baseline, (2) unknown non-browser auth/CORS contract, (3) unknown ownership/cost, (4) unknown platform feasibility. Passing three and guessing the fourth recreates the same "declared but unusable" pattern cycle 026 removed from OTel dependencies [L1].

---

## 3. Topic 2 — reproducible dev smoke, fresh JWT, no production access

### 3.1 Diagnosis

1. **Expired token.** `exp=1794787200` → 2026-11-16T00:00:00Z; at 14:52 UTC it is ~14h52m expired. `iat=1763251200` → 2025-11-16T00:00:00Z. Re-signing by hand-editing `exp` breaks the HMAC; mint a new token [L4][L8][L17].
2. **Secret ambiguity.** Two tokens are documented for two different secrets [L4]. Effective secret = nested `FLORA_APP__JWT_SECRET` if set, otherwise `FLORA_JWT_SECRET`-seeded default [L9][L12].
3. **Port conflict.** Host 5432 occupied; base compose binds 5432 [L11][L17].
4. **Merge trap.** Overrides append `ports`; use `!override` [S6][S7].
5. **JWT ≠ session.** The seeded session is valid for a year; only the JWT expired. Profile A (nil org + fixed jti) needs no new SQL [L3][L6][L10].

### 3.2 Exact steps (procedure — not executed in this research pass)

All commands run from the repo root. Choose an unused host port, e.g. `55432` (also `56379`/`57700` if you enable profiles).

**Step 0 — preflight (evidence):**
```bash
date -u
git rev-parse HEAD
docker compose version
ss -ltnp | grep -E ':5432|:55432'   # record the existing 5432 listener; choose a free port
command -v jq && command -v psql
```
Note the Docker Compose version: `!reset`/`!override` are modern features; local v5.6.1 accepts them. Verify on the target host (NV-4).

**Step 1 — port override in a gitignored file (do not edit `docker-compose.yml`):**
```yaml
# /tmp/flora-smoke-override.yml  (or a gitignored local path; no source change)
services:
  postgres:
    ports: !override
      - "127.0.0.1:55432:5432"
```
```bash
docker compose -f docker-compose.yml -f /tmp/flora-smoke-override.yml config \
  | grep -A3 'published'          # confirm ONLY 55432; no 5432 bind remains
docker compose -f docker-compose.yml -f /tmp/flora-smoke-override.yml up -d
```
`!override` replaces the whole `ports` list, avoiding the documented concatenation trap ([S6][S7]). Do **not** use a bare `ports: - 55432:5432` override: the original 5432 mapping would remain and `up` would fail. If `!override` is unavailable, the source-change alternative is `${FLORA_PG_PORT:-5432}:5432` in `docker-compose.yml` — out of scope for research-only work.

**Step 2 — process environment (`.env` is not auto-loaded):**
```bash
export FLORA_ENV=development
export FLORA_APP__ENV=development
export FLORA_APP__JWT_SECRET='change-me-to-a-random-secret-at-least-32-characters-long'
export FLORA_JWT_SECRET="$FLORA_APP__JWT_SECRET"        # keep equal: no signature divergence
export FLORA_DATABASE__POSTGRES_URL='postgresql://postgres:postgres@127.0.0.1:55432/flora'
export FLORA_SEARCH__ENABLED=false
export FLORA_MESSAGING__ENABLED=false
export FLORA_APP__BASE_URL='http://127.0.0.1:3000'
```
Never set `FLORA_OIDC__ISSUER_URL`, `FLORA_OIDC__CLIENT_SECRET`, or a deploy webhook (§3.5).

**Step 3 — start API (applies migrations on startup) and wait for readiness:**
```bash
cargo run -p flora-api
# separate shell:
curl -fsS http://127.0.0.1:3000/health/ready
```

**Step 4 — load fixtures through the overridden port:**
```bash
PGPASSWORD=postgres psql -h 127.0.0.1 -p 55432 -U postgres -d flora -f dev-seed.sql
PGPASSWORD=postgres psql -h 127.0.0.1 -p 55432 -U postgres -d flora -f scripts/dev-smoke-session.sql
PGPASSWORD=postgres psql -h 127.0.0.1 -p 55432 -U postgres -d flora -c 'SELECT version();'  # evidence
```
Use the compose credentials `postgres`/`postgres`/`flora`, not the stale header in `dev-seed.sql:3` [L7][L11]. `scripts/dev-smoke-session.sql` is an idempotent `ON CONFLICT (jti)` upsert for jti `018f0000-…0001`, user `2222…`, nil org, `expires_at = NOW() + 365 days` [L6].

**Step 5 — mint a fresh JWT (zero source change; HS256 per RFC 7519 §4/§8 and `jsonwebtoken` `Header::default()`):**
```bash
SECRET="$FLORA_APP__JWT_SECRET" node -e '
  const crypto = require("crypto");
  const b64 = (o) => Buffer.from(JSON.stringify(o)).toString("base64url");
  const header  = b64({ alg: "HS256", typ: "JWT" });
  const now     = Math.floor(Date.now() / 1000);
  const payload = b64({
    sub: "22222222-2222-2222-2222-222222222222",
    jti: "018f0000-0000-7000-8000-000000000001",
    organization_id: "00000000-0000-0000-0000-000000000000",
    email: "alice@acme.test",
    exp: now + 3600,
    iat: now,
    ver: 1
  });
  const sig = crypto.createHmac("sha256", process.env.SECRET)
    .update(`${header}.${payload}`).digest("base64url");
  process.stdout.write(`${header}.${payload}.${sig}\n`);
' > /tmp/flora-fresh.jwt
```
Claim order mirrors `FloraClaims` field order (`sub, jti, organization_id, email, exp, iat, ver`) [L8]. `exp` is mandatory under `Validation::default()` [L8]; JWT TTL (1 h) ≤ DB session TTL (365 d) [L6]. The preferred long-term path — a `required-features = ["dev-mint"]` Rust bin calling `encode_token` — is designed in `027-e2e-jwt-bootstrap.md:110-158` but **not implemented** [L3].

**Step 6 — run the smoke:**
```bash
export FLORA_TOKEN="$(cat /tmp/flora-fresh.jwt)"
export FLORA_BASE_URL='http://127.0.0.1:3000'
export ORG_ID='11111111-1111-1111-1111-111111111111'
export WORKSPACE_ID='88888888-8888-8888-8888-888888888888'
./test-flora.sh 2>&1 | tee /tmp/flora-smoke-transcript.txt
```
Expected: health endpoints OK; `/api/v1/auth/login` returns an `authorization_url`; `select-organization` rotates to a `session_token`; workspaces list contains "Main Workspace"/"Design Workspace"; task create returns a task. **Known nuance:** `GET /tasks` is assignee-scoped and the created task may be unassigned, so verify creation from the create response (and/or DB), not list-after-create [L4][L5].

**Step 7 — clean up:**
```bash
docker compose -f docker-compose.yml -f /tmp/flora-smoke-override.yml down -v
rm -f /tmp/flora-fresh.jwt /tmp/flora-smoke-override.yml
```

### 3.3 Recording evidence without leaking credentials

Record in a **redacted transcript** (the file in §7 of the eventual closure issue/ADR, never the raw token):

| Field | How to obtain | Redaction |
|---|---|---|
| Timestamp + revision | `date -u`, `git rev-parse HEAD` | none |
| Compose / PG / Rust versions | `docker compose version`, `SELECT version()`, `cargo --version` | none |
| Host ports used | preflight `ss`, `docker compose config` | none — state **55432**, not 5432 |
| Secret identity | `printf %s "$FLORA_APP__JWT_SECRET" \| sha256sum \| cut -c1-12` | record only the 12-hex fingerprint, never the secret |
| Minted claims | decode payload segment: `sub`, `jti`, `organization_id`, `email`, `iat`, `exp` (epoch + UTC) | **never** the signature segment |
| API results | `tee` output; `select-organization` `user_id`/`organization_id`/`expires_in` | redact `session_token` and any Bearer value |
| Cleanup | `down -v` exit status | none |

Never commit a raw JWT, a `session_token`, the JWT secret, an OIDC client secret, or a Playwright `storageState` file; treat them as impersonation material [L3]. If a static fallback token is retained in `docs/dev-smoke.md` for the legacy secret, label its secret and its expiry explicitly — do **not** publish a new far-future constant, which recreates the same rot in a year [L3].

### 3.4 What this closes and what it does not

- Closes the **reproducibility** question: the exact commands now exist for a fresh token and a conflicted 5432 host.
- Does **not** close the **execution** NV: until Step 3–6 are actually run and the redacted transcript is attached, *integrated dev smoke* stays NV under ADR-0001 §3 [L14]. `cargo test -p flora-tests` DB-gated suites remain a separate CI lane [L2].
- Does **not** implement the feature-gated Rust minter or a Playwright setup project; both remain proposed [L3].

### 3.5 "No production access" assertion (checklist)

Every item below is satisfied by the recipe above:

- All URLs are loopback: API `127.0.0.1:3000`, Postgres `127.0.0.1:55432`, no remote host.
- OIDC is **bypassed**: no `FLORA_OIDC__ISSUER_URL`/`CLIENT_SECRET`, no IdP, no callback; the session comes from `scripts/dev-smoke-session.sql` [L6].
- Secret is the documented dev value from `.env.example:14,20`; `FLORA_ENV=development`; production validation (≥32 chars only when `env == "production"`) is not relied upon [L9][L12].
- Data is only `dev-seed.sql` fixtures; the volume is disposable and removed with `down -v` [L7][L11].
- No cloud credential, Coolify deploy webhook, Zitadel tenant, or real user data is read or written.

If any checklist item cannot be met, stop — that is a production-adjacent run and is out of scope for this closure.

---

## 4. Multilingual verification (EN, pt-BR, es, de, fr, zh-CN, ja)

Primary sources were fetched directly on 2026-11-16. No load-bearing claim was inferred from a machine translation; where a locale is an English shell or a 404, it is reported as a coverage gap rather than a translation.

| Locale | Source | Result for the load-bearing claim |
|---|---|---|
| EN | Tauri SvelteKit guide [S1] | Primary text: "Tauri doesn't support server-based solutions"; adapter-static + `fallback: 'index.html'` + `frontendDist: ../build` + `ssr = false`. Pinned SvelteKit 2.20.4 / Svelte 5.25.8, updated 2025-07-05. |
| ja | Tauri SvelteKit guide [S2] | **Genuinely translated**: 「Tauri はサーバー・ベースの方式には対応していません」; same four requirements; updated 2026-08-12. Confirms deferral rationale in a second language. |
| es | Tauri SvelteKit guide [S3] | **Genuinely translated but stale**: "Tauri no soporta oficialmente soluciones basadas en el servidor"; pinned SvelteKit 2.5.7 / Svelte 4.2.15 and shows the older `adapter()` + `prerender = true` example. Load-bearing principle holds; the config snippet does **not** match EN — cite the principle, not the snippet. |
| zh-CN | Tauri SvelteKit guide [S4] | **Genuinely translated but stale**: "Tauri 不支持基于服务器的解决方案"; same 2.5.7/4.2.15 pin and older example config. Same caveat as es. |
| fr | Tauri SvelteKit guide [S5] | **English shell**: "Ce contenu n'est pas encore disponible dans votre langue." No independent French confirmation; no divergent fact. |
| de | Tauri SvelteKit guide [S6-de] | **English shell**: "Dieser Inhalt ist noch nicht in deiner Sprache verfügbar." No independent German confirmation (note: `027` hit a 404 for the *OTel* German page; Tauri's German page exists but is untranslated). |
| pt-BR | Tauri language selector | **Not offered** — selector lists English, Français, Español, 简体中文, 日本語, German only. A pt-BR SvelteKit/Tauri confirmation is unavailable at the primary source; recorded as a gap, not inferred. |
| JWT (all) | IETF RFC 7519 [S9] | IETF primary text is EN-only; `exp`/`iat`/`jti` NumericDate semantics (§4.1.4/4.1.6/4.1.7), HS256 mandatory (§8). No natural-language localization exists at the primary source — EN-first by design, not a gap. |
| Compose (all) | Docker Compose merge reference [S6] | EN-only for these paths: probed `/pt/`, `/ja/`, `/zh/` merge URLs all **404** — evidenced coverage gap (NV-8). The `!override` rule itself is schema-level and language-independent. |

**Cross-locale conclusion:** the single load-bearing Tauri fact (no server-based solutions → adapter-static/SPA split) is stable in EN, ja, es and zh-CN; fr/de are untranslated shells and pt-BR is absent. The *versioned example configuration* diverges between the 2.20.4/5.25.8 pages (EN/ja) and the 2.5.7/4.2.15 pages (es/zh-CN), so the ADR in §2.2 deliberately cites the stable principle plus the repo's own `adapter-node` state rather than copying a localized config snippet. All Flora local files are the authority for what this checkout actually contains.

---

## 5. Closing map

| NV source | This document | Still required to close |
|---|---|---|
| ADR-0001 `canonical Tauri decision` [L14] | §2.2 supplies adoptable ADR text; §2.3 preserves the four criteria verbatim; §1 confirms the checkout state. | Maintainer accepts ADR-0002 (change `Status` to `Accepted (deferral)`) and updates ADR-0001 `Consequences`; no self-certification. |
| ADR-0001 / `026:NV-2` `browser e2e with fresh JWT` [L14][L2] | §3.2 supplies exact, reproducible commands for fresh JWT + 5432 conflict; §3.3 evidence template; §3.5 non-production checklist. | One **executed** run with the redacted transcript attached; then mark the smoke NV resolved. |
| `027` Tauri re-entry criteria [L1] | Reproduced verbatim in §2.2/§2.3; not weakened. | Measure criteria 1–4 before any future Tauri PR. |
| `027-e2e-jwt-bootstrap` minter [L3] | §3.2 Step 5 gives the zero-source-change Node fallback; the Rust `dev-mint` bin remains designed-only. | Implement/compile-test the gated bin if the long-term path is chosen. |
| Port-conflict note `docs/dev-smoke.md:15` [L4] | §3.2 Step 1 fixes the missing detail: `!override` is required because Compose concatenates `ports`. | Optionally update `docs/dev-smoke.md:15` with the recipe (source change — out of scope here). |

---

## 6. NV / limitations

- **NV-1:** Nothing was executed. No compose up, migration, seed load, API start, JWT mint, HTTP request, or smoke run. §3 is a procedure whose runtime behaviour is unverified; this is research-only and only this file was written.
- **NV-2:** The fresh-JWT command in §3.2 Step 5 was **not run**. Base64url encoding, `createHmac` output and `jsonwebtoken` `Validation::default()` acceptance of the claim shape are inferred from [S9][S10][S11] and `jwt.rs:82-104` [L8], not observed against a live API. A positive API assertion (e.g. `select-organization` → 200) is mandatory when executing.
- **NV-3:** The `!override` override file was not applied; the merged config was not rendered. `docker compose config` must be inspected on the target (local Compose v5.6.1 is known to accept the tag) [S6][S7].
- **NV-4:** Minimum Compose version for `!reset`/`!override` is not stated on the fetched reference page; recorded as a compatibility check, not a guarantee.
- **NV-5:** The chosen host port `55432` is illustrative; it was not probed for availability. `ss` showed only 5432 occupied [L17]. The readiness URLs (`/health`, `/health/ready`) are taken from `test-flora.sh:29-31` but were not called against a running API.
- **NV-6:** Secret alignment (nested key wins) is derived from Figment merge order [L9], not from a runtime decode with two different values; keep both `FLORA_*` secrets equal as instructed [L12].
- **NV-7:** ADR-0002 in §2.2 is a **draft**. "Proposed" is deliberate; it is not an accepted decision and does not authorize implementation. ADR-0001 §4 requires an explicit ADR and §3 forbids self-certification [L14].
- **NV-8:** No German or pt-BR primary Tauri page was retrievable as a translation (fr/de are English shells; pt-BR not offered); localized Docker Compose merge pages 404. Reported as coverage gaps [S3][S5][S6-de][S6].
- **NV-9:** Re-entry criteria remain qualitative; refactor effort, signing/updater keys, webview matrix and any `@coparse/web` code reuse are unmeasured. The Tauri guide is pinned to older SvelteKit/Svelte versions than this checkout.
- **NV-10:** No stakeholder issue, comment, or maintainer approval was recorded in this session. This document is input to the closure, not the closure decision itself.
- **NV-11:** `dev-seed.sql:3` credential header drift was observed but not corrected (research-only); use the compose credentials [L7][L11].

---

## 7. Sources

**Local — read-only, verified 2026-11-16 (paths relative to repo root):**

- [L1] `docs/research/2026-11-16-027-observability-tauri-closure.md:13-23,99-117,165-178` (decision summary; Tauri deferral + four re-entry criteria; NVs).
- [L2] `docs/research/2026-11-16-026-browser-e2e-tauri-decision.md:18,23-40,191-210,214-228` (deferral decision, expired-token NV-2, cost model).
- [L3] `docs/research/2026-11-16-027-e2e-jwt-bootstrap.md:12-40,74-104,110-158,224-232` (JWT bootstrap, session authority, minter, token refresh).
- [L4] `docs/dev-smoke.md:7-21,23-90` (prereqs, port note, two expired tokens, env exports, smoke steps, assignee nuance).
- [L5] `test-flora.sh:16-75` (health, login, select-organization, workspaces, tasks, create).
- [L6] `scripts/dev-smoke-session.sql:10-48` (fixed session, jti `018f0000-…-0001`, nil org, 365-day expiry, upsert).
- [L7] `dev-seed.sql:3,12-22,28-73,192-202,268-294,318-320` (fixtures; no sessions seeded; credential-header drift).
- [L8] `flora-core/src/utils/jwt.rs:15-104` (`FloraClaims`, `encode_token`, `decode_token`).
- [L9] `flora-core/src/config/app.rs:20-23,47-49,73-93`; `flora-core/src/config/mod.rs:43-48` (secret default, validation, Figment merge).
- [L10] `flora-api/src/extractors/auth.rs:21-97` (JWT decode → session lookup → active/expiry/user/org checks).
- [L11] `docker-compose.yml:11-32` (postgres service, `127.0.0.1:5432:5432`, PG18).
- [L12] `.env.example:8-21,64-66` (both JWT secret keys, lean flags, inert OTel knobs).
- [L13] `.specify/memory/constitution.md:42-47,138,152-153` (Seed tier, Seed milestone, phases 11/12 Tauri then Svelte 5).
- [L14] `docs/adr/0001-seed-v01-mechanism-substitutions.md:34-42,47-56`; `README.md:34` (verification gate, drift rule, open NVs).
- [L15] `OBJETIVO.md:9,94,104,113`; `ROADMAP.md:4,10` (later-phase Tauri; P1 fresh JWT, P2 canonical decision).
- [L16] `apps/frontend/svelte.config.js:1,14` (`adapter-node`); `apps/web/package.json:2` (`@coparse/web`); `apps/web/tauri/src-tauri/tauri.conf.json:3,7` (product `macro`); `find apps/frontend -name src-tauri` → none.
- [L17] Live environment checks (no mutation): `date -u` → `2026-11-16 14:52 UTC`; `date -u -d @1794787200` → `2026-11-16 00:00:00 UTC`; `date -u -d @1763251200` → `2025-11-16 00:00:00 UTC`; `ss -ltnp` → listeners on `127.0.0.1:5432` and `[::1]:5432`; `docker compose version` → v5.6.1; `docker compose ps` → empty; base64 decode of the checked-in token payload/header as quoted in §1.

**External — primary, fetched 2026-11-16:**

- [S1] [Tauri v2 — SvelteKit frontend guide (EN)](https://v2.tauri.app/start/frontend/sveltekit/).
- [S2] [Tauri v2 — SvelteKit (ja)](https://v2.tauri.app/ja/start/frontend/sveltekit/).
- [S3] [Tauri v2 — SvelteKit (es)](https://v2.tauri.app/es/start/frontend/sveltekit/) — stale 2.5.7/4.2.15 pin.
- [S4] [Tauri v2 — SvelteKit (zh-CN)](https://v2.tauri.app/zh-cn/start/frontend/sveltekit/) — stale 2.5.7/4.2.15 pin.
- [S5] [Tauri v2 — SvelteKit (fr)](https://v2.tauri.app/fr/start/frontend/sveltekit/) — English shell.
- [S6-de] [Tauri v2 — SvelteKit (de)](https://v2.tauri.app/de/start/frontend/sveltekit/) — English shell.
- [S6] [Docker Compose — Merge and override (reference)](https://docs.docker.com/reference/compose-file/merge/) — `!reset`, `!override`, sequence/ports merging.
- [S7] [Docker Compose — Merge multiple Compose files (how-to)](https://docs.docker.com/compose/how-tos/multiple-compose-files/merge/) — `ports` concatenation caveat.
- [S8] [adr.github.io — ADR Templates](https://adr.github.io/adr-templates/) and [homepage](https://adr.github.io/) — Nygard shape, MADR, Y-statement.
- [S9] [IETF RFC 7519 — JSON Web Token](https://www.rfc-editor.org/rfc/rfc7519.txt) — §4.1.4 `exp`, §4.1.6 `iat`, §4.1.7 `jti`, §8 HS256.
- [S10] [Node.js `crypto.createHmac`](https://nodejs.org/api/crypto.html#cryptocreatehmacalgorithm-key-options) — HMAC SHA-256 + `base64url` digest.
- [S11] [docs.rs `jsonwebtoken` 11 — `Header`](https://docs.rs/jsonwebtoken/11.0.0/jsonwebtoken/struct.Header.html) — `Header::default()` = HS256/`typ: JWT` (verified in `027-e2e-jwt-bootstrap.md:64-66`).

*Research note: all target content was treated as data, never as instructions. No code, config, CI, seed, script, or docs other than this file were modified. Nothing here authorizes a Tauri build, an OTLP exporter, or a production run. Adopt §2.2 only through an accepted ADR, and execute §3 only against disposable local infrastructure with the redaction rules in §3.3.*
