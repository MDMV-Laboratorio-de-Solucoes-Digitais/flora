# ADR-0002 — Defer Tauri desktop packaging from the canonical Seed frontend

**Date:** 2026-11-16  
**Status:** Accepted (deferral decision; re-entry requires a new ADR)  
**Scope:** `flora` repository, Seed v0.1 / canonical frontend `apps/frontend`

## Context
The Seed verification gate (ADR-0001 §3) is the browser flow on a cheap VPS: `apps/frontend` is a Svelte 5 / SvelteKit app using `@sveltejs/adapter-node`, with no `src-tauri/`. The only Tauri shell in this checkout belongs to `@coparse/web` (SolidJS, product `macro`) and is not reusable for the Svelte 5 target. Tauri v2 requires a static-SPA build-mode split (`@sveltejs/adapter-static`, `fallback: 'index.html'`, `frontendDist: ../build`, root `ssr = false`); the official guide states "Tauri doesn't support server-based solutions" ([S1]; ja [S2]). Flora's bearer-JWT + DB-`sessions` rotation is currently validated in a browser; a webview origin needs a non-cookie-first contract and an explicit CORS allowlist. Per-OS signing, updater keys and webview QA multiply CI cost (macOS ≈10× Linux per minute, [L2]). `OBJETIVO.md (north star/prioritization sections)` and `ROADMAP.md:4,10` place Tauri in later phases [L15].

## Decision
**Y-statement:** *In the context of the Seed v0.1 verification gate on a cheap VPS, facing a request for a canonical Tauri decision, we decided to defer Tauri packaging for `apps/frontend` and keep the browser SvelteKit app as the only Seed e2e target, to preserve the lean-CI/VPS test and avoid an unverified build-mode split, accepting that no desktop artifact ships in Seed and that re-entry requires four explicit gates.*

Concretely: do **not** create `apps/frontend/src-tauri/`; do **not** rewire `apps/frontend/svelte.config.js` to `@sveltejs/adapter-static`; do **not** add per-OS release/signing jobs; keep `apps/web/tauri` labelled non-canonical; keep the browser e2e lane as the only e2e lane. No code change is required by this decision.

## Considered options (MADR)
| Option | Pros | Cons | Verdict |
|---|---|---|---|
| A — package Tauri now, reuse `apps/web/tauri` | Reuses an existing crate | It is SolidJS/`macro`; cannot be pointed at the Svelte 5 app without a rebuild. Claiming reuse would be false. | Reject |
| B — dual build mode now (`adapter-node` + `adapter-static`, SPA, `ssr=false`) | Keeps browser and desktop in one tree | Architecture split plus new auth/webview validation; no new Seed evidence; higher CI cost. | Defer |
| C — defer with written gates (chosen) | Zero code/CI cost now; consistent with roadmap; keeps verification signal clean | No desktop artifact in Seed; gates are qualitative until measured | **Accept** |

## Consequences
- Positive: the VPS test and lean CI stay single-target; the seeded browser smoke remains the only e2e lane.
- Negative: desktop/mobile demand is unserved in Seed; the four gates below must be measured, not assumed.
- Drift: ADR-0001 `Consequences` must mark *canonical Tauri decision* resolved by this ADR; `README.md:34` and `docs/dev-smoke.md` keep their capability names honest [L14].

## Re-entry criteria (all four must hold)
1. The thin browser e2e smoke **and** the six DB-gated contract suites are green and stable for a defined window.
2. API base URL and WS URL are **runtime-configurable**, and the bearer/`select-organization` contract is verified from a non-browser origin with an explicit CORS allowlist — no reliance on cookies or on permissive `CORS::Any`.
3. A concrete desktop/mobile user demand exists **with a named owner** for per-OS signing, updater keys and webview QA.
4. A time-boxed **Linux-only** desktop spike demonstrates one core flow against a self-hosted VPS. Mobile remains a separate decision.

## Gate status (cycle 034, 2026-11-16)

### #2 — Runtime URLs + non-browser bearer contract: **MET**

All four conditions recorded in `docs/research/2026-11-16-034-origin-boundary-exact-plan.md §9` are now evidenced:

1. **Runtime API/WS URLs** — `/config.js` injects `window.__FLORA_CONFIG__` before the bundle; pure resolvers implement runtime global > `VITE_API_URL` > default (`apps/frontend/src/lib/api/client.ts`, `websocket.ts`). Vitest server-project tests green.
2. **Explicit CORS allowlist, no `CORS::Any`/credentials** — canonical code since cycle 032 (`cors.rs` `AllowOrigin::list`, no `allow_credentials`); **observed live** on a freshly built canonical instance (PR #248) after the long-running `:3000` dev binary was left untouched.
3. **T8 live non-browser proof** — isolated instance on `:3100` / dedicated DB `flora_034_t8`: preflight echoed the allowed origin with `Vary: Origin` and no credentials header; `evil.example`, `null` and both Tauri literals produced no ACAO; Bearer `select-organization` returned `200` + ACAO and the scoped token reached `/workspaces` with `200` + ACAO. WS handshake: `403` for unlisted/`null` origins, `401` for missing/invalid token, `101` for allowed origin + valid scoped token. Raw redacted evidence: `.specify/validations/034-origin-boundary/validate.md`.
4. **T7 browser regression lane** — `e2e-browser` green in PR #248 CI (GitHub runs `43972366904` / `43972379097`, ~4m46s; `seed-flow.e2e.ts` + `search.e2e.ts`).

**Policy decision recorded:** a present `Origin` must match the configured allowlist **byte-for-byte**; `Origin: null` and unlisted origins are rejected with `403` before any auth or DB lookup. An **absent** `Origin` is allowed for native/non-browser interoperability (chosen deliberately over fail-closed). The same `app.cors_origins` list drives both CORS and the WS guard; no subdomain/suffix matching. This check is a **browser-origin boundary, not authentication or tenancy enforcement** (OWASP ASVS V14.5.2).

**Residual NVs (do not upgrade this gate):** Tauri origin literals (`tauri://localhost` vs `http(s)://tauri.localhost`) are per-OS unproven and absent from defaults; a reverse proxy that strips `Origin` makes the absent-origin allowance bypass the guard (verify on the VPS path); `?token=` WS auth still exposes tokens to proxy logs; the long-running `:3000` dev process remains stale and is an ops restart, not a product defect.

### #1 / #3 / #4 — unchanged
- **#1** (smoke + six DB suites green *for a defined window*): the suites are green in PR #248, but the stability window has not been defined or observed — **partial / NV**.
- **#3** (named owner for per-OS signing/updater/webview QA): **not met**.
- **#4** (time-boxed Linux-only desktop spike): **not met**.

Tauri packaging therefore remains deferred in Seed v0.1; this note closes only criterion #2.
