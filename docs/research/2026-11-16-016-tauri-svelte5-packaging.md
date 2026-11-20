# Tauri + Svelte 5 (SvelteKit) Packaging for Flora — Multilingual 80/20 Research

**Date:** 2026-11-16  
**Scope:** Packaging the canonical Svelte 5/SvelteKit web UI as Tauri v2 desktop/mobile apps for a self-hosted Flora deployment (SMBs, cheap VPS backends). Focus: architecture (shared UI, API base config, auth/session), build/tooling implications, performance/resource constraints, and pragmatic MVP sequencing for a small team.  
**Evidence inspected (repo):** `apps/frontend/package.json` (Svelte 5.56, `@sveltejs/kit` 2.63, `@sveltejs/adapter-node` 5.5.4, Vite 8), `apps/frontend/svelte.config.js` (`adapter-node`), `OBJETIVO.md` (AGPL self-hosted Macro alternative for PMEs, Rust+Postgres, Svelte 5, Tauri later, Seed/VPS ~2 vCPU/4 GB, P0 auth/tenancy/chat/tasks/files/search/notifications), `ROADMAP.md` (Seed usable/secure, Postgres-first/debloat, P2 defers Tauri/mobile-class expansion complexity).  
**External research (multilingual):** EN: Tauri v2 SvelteKit/static-adapter docs, Tauri frontend/mobile/develop guides, SvelteKit adapter-static/SPA/auth docs, Tauri GitHub/issues/discussions, Sik/Urhoba/CODERCOPS/MayhemCode/Nathan Sarrazin/gau/blackthorne analyses (2024–2026). pt-BR: Zeeklog/KernelLab “Tauri 前端配置…” translations/pt cues, CSDN/guia pt-BR de Tauri+SvelteKit (adapter-static, sem SSR, `TAURI_DEV_HOST`), conceitos “URL da API absoluta/base configurável, token em storage seguro nativo”. es: guías de empaquetado Tauri/SvelteKit (SPA estática, `frontendDist build/`, CORS/origen remoto para backend self-hosted, separar servidor/API). de: Sik “Rollen trennen: Browser-UI/API-Aufrufe vs. lokale Gerätefunktionen vs. Server/Geheimnisse; statischer Build/SPA, `TAURI_DEV_HOST`”. fr: principes “adapter-static + SPA, pas de SSR serveur, config API distante, capacités/permissions minimales”. zh-CN: Zeeklog/KernelLab/CSDN “Tauri 前端配置/静态构建+禁用 SSR+fallback，`frontendDist ../build`，移动端用 `TAURI_DEV_HOST`，API 走标准 Client↔API，勿硬编码密钥”. ja: Tauri公式 ja（SvelteKit/static-adapter, 開発/モバイル, `TAURI_DEV_HOST`/`ssr=false`）、Sik/OfLight/T-Creator/記事類（サーバ役割分離、静的ビルド、権限境界、モバイル制約/署名）。

## 1) Architecture (80/20)

| Topic | Recommendation | Rationale / cross-check |
|---|---|---|
| Shared UI | Keep one Svelte 5 UI codebase; **do not share SSR/server runtime**. For Tauri targets use a static SPA build (`@sveltejs/adapter-static`, `fallback: 'index.html'`, root `ssr=false`; avoid prerendered `load` that needs Tauri APIs). Web can remain `adapter-node` short-term, but converge on a **client-first API-driven UI** so both targets share routes/components/stores. | Tauri has no Node/SvelteKit server in webview (official EN/ja/zh). Current `apps/frontend` is `adapter-node` (SSR-capable), so Tauri requires an explicit build-mode split or migration to client loads. |
| API base config | Make backend origin **runtime-configurable** (e.g. `PUBLIC_API_BASE`/`VITE_API_BASE_URL`) and use absolute URLs from Tauri; never assume same-origin. Centralize fetch/WS client so desktop/mobile can point to user’s self-hosted VPS URL (HTTP(S)/WSS). Handle CORS and trusted origins on `flora-api`. | Common pattern in EN/pt/es/zh guides (Node+static dual build, Tauri remote API). Fits Flora’s self-hosted/SMB goal (user owns server). |
| Auth/session | Prefer **short-lived Bearer access tokens in memory + server-side/rotating refresh** over web-cookie reliance in custom webview origins. Store refresh/session material via OS credential vault/stronghold plugin when native shell exists; fall back to minimal storage only if explicitly accepted. Keep `select-organization`/tenant context explicit in client state and revalidate on 401/session-invalid (aligns with recent Flora session-authority work). For OIDC, use system browser/deep-link callback flow on desktop/mobile, not embedded cookie assumptions. | Tauri origin/cookie behavior differs from normal browsers (discussion/workarounds historically fragile); official SvelteKit auth is session/cookie-oriented for web servers, so Tauri needs a token-capable API contract. OBJETIVO/ROADMAP emphasize secure tenancy/session authority. |
| Realtime/WS | Connect WS to configurable backend (`wss://host/api/v1/ws` or current path), send bearer token per Flora contract, handle reconnect/catch-up idempotently. Do not depend on same-origin cookies. | Matches Flora’s recent WS/session work and self-hosted topology; avoids webview cookie edge cases. |

## 2) Build & tooling implications

- **Dual-target frontend shape (pragmatic):** introduce build modes/adapter switch (or separate app entries) so `web-node` and `tauri-static` can coexist without breaking current CI (`bun run check/lint/test:unit/build` in `apps/frontend`). Expect conditional env (`TAURI_ENV_PLATFORM`), `ssr=false` layout for Tauri, and `vite` server tweaks for mobile dev (`TAURI_DEV_HOST`, strict port/HMR) per Tauri/Vite docs (EN/zh/ja).
- **Tauri project scaffolding:** add `src-tauri/` (Rust crate, capabilities/permissions/CSP, icons, updater later), `@tauri-apps/cli` + `@tauri-apps/api`, and CI jobs per platform (Linux/Win/macOS, later Android/iOS). Mobile adds SDK/NDK/Xcode/signing complexity; treat as separate pipeline.
- **No server-only SvelteKit features in Tauri:** `+page.server.ts`, form actions, hooks-as-server, secrets in bundle are out for packaged client (EN/de/zh consensus). Flora backend (`flora-api` on VPS) must remain the authority.
- **Packaging/signing:** Tauri updater requires signing keys; plan artifact naming/version sync and per-OS signing/notarization only when distribution starts (NV for team infra).

## 3) Performance & resource constraints (self-hosted/cheap VPS lens)

- **Client:** Tauri uses system webview (small binaries, lower RAM than Electron typical) — good for SMB desktops; but rendering/WebGL/behavior varies by OS webview (esp. Linux WebKitGTK/Windows WebView2), so test on lowest-supported targets. Mobile is feasible in Tauri v2 but newer/rougher than desktop (plugins, background, storage, store review).
- **Server (Flora VPS, Seed ~2 vCPU/4 GB):** Tauri clients do **not** reduce backend load by themselves; they add more concurrent API/WS clients. Keep Postgres-first, bounded pools, optional Valkey/Meilisearch, and catch-up/reconnect strategies (already roadmap concerns). Avoid chatty per-keystroke IPC-to-server patterns; batch/paginate.
- **Network:** self-hosted instances may be remote/high-latency; design for offline-tolerant UX only where cheap (drafts/cache optional) but keep MVP online-first.

## 4) Pragmatic MVP sequencing (small team, SMBs/cheap VPS)

| Phase | Deliverable | Exit signal (80/20) |
|---|---|---|
| 0. Web-core hardening (now, aligns P0) | Stable API contracts, token/session semantics usable outside browser cookies, configurable API base/CORS, core flows (auth/org/workspace/channel/message/task) verified. | Web SPA-style client can talk to arbitrary self-hosted base URL without hidden SSR coupling. |
| 1. Desktop spike (Linux/Windows first) | `adapter-static` Tauri build, login/select-org against remote VPS, read/write one core flow (messages or tasks), basic error/reconnect handling. No native plugins beyond essentials. | Internal dogfood install connects to cheap VPS over HTTPS/WSS. |
| 2. Desktop beta hardening | Secure token storage, deep-link/OIDC callback if needed, updater/signing decision, platform QA matrix (webview quirks), logging/crash diagnostics. | Repeatable release artifacts for 1–2 OSes. |
| 3. Mobile evaluation (optional, after desktop proves value) | Android-first prototype reusing same UI/API config; assess auth/deep-link/push/background needs. | Go/no-go with measured effort (plugins/signing/store). |

## 5) 80/20 recommendations

1. **Decouple UI from `adapter-node` for Tauri:** move Tauri target to `adapter-static` SPA + `ssr=false`; keep all data access via `flora-api` (absolute base URL). Treat this as an architecture prerequisite, not a packaging tweak.
2. **Standardize auth for non-browser shells:** make Bearer-token + refresh/`select-organization` flows first-class; plan system-browser OIDC + deep link and OS-backed secure storage for packaged apps.
3. **One configurable network layer:** env/runtime `API_BASE_URL` + WS URL derivation, CORS/origin policy, and uniform 401/session-expired handling (build on cycle 011/012 client/session work).
4. **Ship desktop before mobile:** smallest team path = Linux/Windows Tauri desktop against existing VPS backend; defer iOS/Android until core flows and signing/support costs are justified (matches ROADMAP P2 caution).
5. **Protect cheap VPS:** enforce paging/limits, reuse WS catch-up semantics, and avoid feature flags that force Meilisearch/Valkey/search heaviness on Seed by default.

## NVs (needs verification / out-of-scope for this pass)

- **NV-1:** `apps/frontend` currently has no Tauri config and uses `adapter-node`; exact refactor cost (SSR/load usage, route guards, env plumbing) not audited file-by-file.
- **NV-2:** `flora-api` auth contract for Tauri (cookie vs bearer/refresh/OIDC redirect/deep-link, CORS allowlist for `tauri://`/custom origins, WS token transport) needs endpoint-level verification against implementation.
- **NV-3:** No packaged desktop/mobile prototype, CI matrix, signing/updater keys, or OS-webview compatibility matrix executed in-repo.
- **NV-4:** Binary size/startup/RAM claims are literature-level (Tauri generally smaller than Electron); project-specific measurements on target OSes/cheap client hardware are pending.
- **NV-5:** Mobile feasibility for Flora (background realtime, notifications, file picker/upload, secure storage plugin maturity) requires spike; store compliance/signing effort unestimated.
- **NV-6:** Non-EN sources were cross-checked via available summaries/snippets and multilingual official docs where accessible; some regional articles may have version drift—revalidate against current Tauri/SvelteKit releases before implementation.

## Sources (selected)

- Tauri v2: [SvelteKit frontend guide](https://v2.tauri.app/start/frontend/sveltekit), [Frontend config](https://v2.tauri.app/start/frontend), [Develop](https://v2.tauri.app/develop), [Tauri 2.0 release](https://v2.tauri.app/blog/tauri-20), [tauri-apps/tauri](https://github.com/tauri-apps/tauri), ja docs (`/ja/start/frontend/sveltekit`, `/ja/develop`).
- SvelteKit: [adapter-static](https://svelte.dev/docs/kit/adapter-static), [single-page apps](https://svelte.dev/docs/kit/single-page-apps), [adapters/page options/project types/auth](https://svelte.dev/docs/kit).
- Practice/guides: Sik (EN/KO, static build/roles/permissions), Nathan Sarrazin (dual node+static/API base), blackthorne/Stijn-B templates, gau (deep-link/OIDC in Tauri), Urhoba/CODERCOPS/MayhemCode (perf/mobile tradeoffs), Zeeklog/KernelLab/CSDN (zh-CN config/mobile), T-Creator/OfLight/Qiita/app-tatsujin (ja API/mobile/build).
- Repo: `apps/frontend/{package.json,svelte.config.js}`, `OBJETIVO.md`, `ROADMAP.md`.
