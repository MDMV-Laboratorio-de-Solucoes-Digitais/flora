# ADR-0002 — Defer Tauri desktop packaging from the canonical Seed frontend

**Date:** 2026-11-16  
**Status:** Accepted (deferral decision; re-entry requires a new ADR)  
**Scope:** `flora` repository, Seed v0.1 / canonical frontend `apps/frontend`

## Context
The Seed verification gate (ADR-0001 §3) is the browser flow on a cheap VPS: `apps/frontend` is a Svelte 5 / SvelteKit app using `@sveltejs/adapter-node`, with no `src-tauri/`. The only Tauri shell in this checkout belongs to `@coparse/web` (SolidJS, product `macro`) and is not reusable for the Svelte 5 target. Tauri v2 requires a static-SPA build-mode split (`@sveltejs/adapter-static`, `fallback: 'index.html'`, `frontendDist: ../build`, root `ssr = false`); the official guide states "Tauri doesn't support server-based solutions" ([S1]; ja [S2]). Flora's bearer-JWT + DB-`sessions` rotation is currently validated in a browser; a webview origin needs a non-cookie-first contract and an explicit CORS allowlist. Per-OS signing, updater keys and webview QA multiply CI cost (macOS ≈10× Linux per minute, [L2]). `OBJETIVO.md:9,94,104,113` and `ROADMAP.md:4,10` place Tauri in later phases [L15].

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
