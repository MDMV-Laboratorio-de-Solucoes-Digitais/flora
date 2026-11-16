# Svelte 5 UI Rewrite/Migration Strategy — Multilingual Research (Cycle 016)

**Date:** 2026-11-16  
**Scope:** Incremental UI rewrite/migration strategy to **Svelte 5 + SvelteKit (runes)** for a collaboration/productivity app (chat/tasks/files), covering component patterns, state/data fetching, realtime UI integration, accessibility/performance on low-end devices, and Tauri readiness.  
**Requested research languages:** EN, pt-BR, es, de, fr, zh-CN, ja.  
**Constraint:** no application source modified; no commit.

## 1) Method and evidence

- **Primary (authoritative, EN):** Svelte 5 docs — migration guide, runes model, component/API changes; SvelteKit docs — state management, load/data, adapters/performance guidance (fetched from svelte.dev/kit docs in this run). Tauri readiness grounded in SvelteKit adapter/static/SSR tradeoffs and repo packaging context.
- **Multilingual coverage (pt-BR, es, de, fr, zh-CN, ja):** targeted web retrieval was limited/unavailable in this environment for non-EN result sets; this note therefore provides a cross-lingual concept map (key terms/operators should use in local-language discovery and runbooks) and applies the same primary-source recommendations. Marked as NV where locale-specific evidence was not directly retrieved.
- **Repo cross-check (evidence):**
  - `OBJETIVO.md`: Flora = AGPL, Rust+Postgres, **UI Svelte 5** canonical, Tauri in later phases, Seed/Postgres-first, MVP P0 (workspace/channel/message/task/files basic, search/notifications minimal), “VPS test” / operation on modest hosts.
  - `apps/frontend` (**canonical target**): Svelte 5.56 / SvelteKit 2.63 / Vite 8 / Tailwind 4, `adapter-node`, `typescript ^6`, runes-based code (`$state`, `$props`, `$effect`, `$derived` patterns), route groups `(app)/(auth)`, domain stores in `src/lib/state/*.svelte.ts` (`MessageStore`, `TaskStore`, `FileStore`, `AuthState`), REST client `src/lib/api/client.ts` (`ApiError`, 401→`lockSession()`), WS client `src/lib/api/websocket.ts` (`/api/v1/ws`, token, reconnect/backoff+jitter, envelope parsing, subscribe/catch-up hooks, auth-failure handling), UI primitives (`bits-ui`/shadcn-style) and feature components (chat/tasks/files/org/workspace). Scope is an early vertical slice, not full parity.
  - `apps/web` (**legacy, non-canonical**): imported `@coparse/web` (SolidJS, large Macro-derived dependency graph, GraphQL/Tauri-plugin deps, custom Vite/e2e tooling). Treat as reference-only; do not port wholesale (debloat/architecture mismatch with Flora Seed goals).

## 2) Repo ↔ goals alignment (what matters for rewrite)

| Dimension | Current `apps/frontend` | Flora goal / gap | Strategy implication |
|---|---|---|---|
| Framework | Svelte 5 runes + SvelteKit 2, node adapter | Canonical Svelte 5 UI, Tauri later | Keep runes-native; isolate adapter/build targets for web vs Tauri |
| App shape | Early slices (auth/session, channels/tasks, WS client/store primitives) | P0 e2e workspace→channel→message/task/files, realtime minimum | Prioritize vertical flows over broad component parity |
| State | Class/store objects with `$state`, idempotent upserts/sorting in `MessageStore` | Reliable core UX, low resource use | Keep stores small, deterministic, and co-located by domain |
| Data/API | Typed REST client, structured errors, 401 session lock; WS envelope/catch-up scaffolding | Stable FE↔BE contracts, tenancy/session authority | Contract-first mappers/DTOs, centralized error/session policy |
| Legacy | `apps/web` SolidJS monolith | Debloat, avoid Macro complexity | Cherry-pick UX/behavior ideas only; no framework coupling |
| Operability | SPA-like client features possible | VPS-cheap, low-end devices | Prefer progressive enhancement, minimal JS on hot paths, SSR/static where beneficial |

## 3) 80/20 recommendations (prioritized)

### P0 — Incremental migration & architecture (highest leverage)
1. **Strangler-by-route/feature, not big-bang:** evolve `apps/frontend` slice-by-slice (auth/org → workspace/channel → messages/realtime → tasks → files). Keep each slice end-to-end verifiable against API contracts (aligns with OBJETIVO “fatias verticais verificáveis”). Do not import `apps/web` components/runtime.
2. **Runes-native baseline, avoid legacy mixing in new code:** new/rewritten modules use `$state/$derived/$effect/$props`, snippets, callback props, `.svelte.ts` for shared reactive logic. Use `sv migrate svelte-5` only as an aid if/when porting Svelte 4 code; treat `svelte/legacy` shims as temporary. (Svelte 5 migration guide.)
3. **Domain boundaries over UI sprawl:** keep `lib/state` (domain), `lib/api` (transport/mappers), `lib/components/{chat,tasks,files,...}` (presentational/feature) separated. Current stores (`MessageStore` sorted/idempotent upsert) are a good pattern to standardize.

### P0 — State and data fetching (SvelteKit-correct)
4. **Server/client state split (SvelteKit state-management rules):** no shared mutable server state; no side effects in `load`. Use `load` for request-scoped/SSR-safe data, return serializable data, and pass/derive in components. Keep ephemeral UI state local; use context for tree-scoped state; reserve module-level reactive state for true client-only singletons and avoid cross-user leakage (N/A in SPA areas but critical for SSR/Tauri-webview parity).
5. **Cache/invalidation strategy, minimal libraries:** for Seed 80/20 prefer SvelteKit `load`/`invalidate` + lightweight domain stores and explicit refresh on mutations/reconnect; add TanStack Query only when cache complexity justifies it. Normalize entities by id (messages/tasks/files) to make realtime merges cheap.
6. **Typed API + error/session contract at edge:** preserve centralized handling (current `ApiClient`/`ApiError`, 401 lock). Map DTOs once (`mappers.ts`), keep UI models stable, and surface field/code errors for forms.

### P0 — Realtime UI integration (chat/messages first)
7. **Idempotent event application + catch-up:** treat WS as best-effort delivery; persist/canonicalize via REST and reconcile on reconnect using cursor/`lastKnown` (pattern already emerging in `websocket.ts`/`MessageStore.upsertMessage`). Deduplicate by message id, maintain deterministic order, and tolerate out-of-order/duplicate events.
8. **Connection lifecycle and UX states:** explicit states (connecting/connected/degraded/offline/reauth), bounded backoff+jitter (present), stop retry on auth/session-invalid, and resubscribe active channel on open. For multi-channel scale later, gate subscriptions to visible/active resources.
9. **Low-risk transport evolution:** keep envelopes typed/validated (`parseWsEnvelope` pattern), version payloads, and avoid coupling UI rendering to raw socket messages—apply through domain store actions.

### P1 — Component patterns (collaboration UI)
10. **Composable, accessible primitives:** build on headless/accessible primitives (`bits-ui` already in stack) and snippets for composition (replace slot-heavy patterns in any ported code). Favor list virtualization/windowing for long chat/task/file views (not yet evidenced) as data grows.
11. **Optimistic UI only where safe:** optimistic send/status for messages/tasks with rollback on typed API failure; never optimistic for auth/tenancy/permission decisions.
12. **Progressive enhancement for forms/actions:** where SvelteKit form actions fit, use them for resilience; otherwise keep fetch flows but preserve accessible submit/error/focus behavior.

### P1 — Accessibility and performance on low-end devices
13. **Accessibility baseline (a11y):** semantic HTML first, keyboard/focus management in dialogs/menus/overlays (session lock overlays in `+layout.svelte` currently block interaction—ensure focus trap, ARIA roles/labels, Escape/return paths, and screen-reader announcements when hardened), visible focus, color contrast, `prefers-reduced-motion`, and non-color status cues. Align with SvelteKit a11y guidance and WCAG practices. (Multilingual terms: pt-BR acessibilidade, es accesibilidad, de Barrierefreiheit, fr accessibilité, zh-CN 无障碍, ja アクセシビリティ.)
14. **Low-end/VPS-client perf budget:** minimize JS/hydration cost (code-split routes, lazy-load heavy panels/editors, avoid large client-only deps—important given `apps/web` bloat), prefer CSS/Tailwind utilities over runtime-heavy styling, keep derived computations cheap, virtualize large lists, debounce/throttle typing/presence, and compress/cache static assets (adapter-node behind proxy). Test on CPU-throttled/mobile-class profiles.
15. **Media/files efficiency:** thumbnails/streaming/range or chunked transfers when files grow; avoid loading binary previews eagerly (cross-check: files slice currently basic per OBJETIVO/recent audits).

### P1 — Tauri readiness (later phase, plan now)
16. **Isolate platform/runtime concerns:** keep API base URL, auth token storage, websocket origin, and file/system access behind adapters/services (current `API_BASE_URL` env-driven is a start). Avoid direct dependence on browser-only globals in shared logic.
17. **Rendering/adapter strategy for Tauri:** Tauri typically serves a static/webview bundle; plan `adapter-static` (SPA fallback) or a Tauri-specific build path alongside `adapter-node` for web deploys. Audit SSR-only assumptions, `load` usage, and cookies/session flows for webview context; gate Node-only features.
18. **Security/capabilities:** least-privilege Tauri capabilities (fs/network/shell), strict CSP, and reuse backend session/RBAC (no trust in client). Validate deep links/updates separately when Tauri work begins.

## 4) Incremental execution plan (80/20 sequence)

1. **Stabilize contracts + domain stores** for messages/tasks/files (DTO mappers, idempotent reducers, session/error handling) — enables safe UI iteration.
2. **Ship core vertical UX**: workspace→channel→message (realtime + catch-up) and task basics with accessible, low-JS components.
3. **Harden a11y/perf**: focus/overlay fixes, list virtualization, bundle/code-split checks on modest hardware.
4. **Abstract platform layer** (config/storage/network) and add Tauri build/profile spike **without** blocking Seed web delivery.

## 5) NVs (not verified / limits)

- **NV-multilingual-primary:** non-EN (pt-BR/es/de/fr/zh-CN/ja) primary web sources were not directly retrievable via automated search in this run; recommendations rely on official EN docs + repo evidence. Locale-specific operator docs/runbooks should be added when available.
- **NV-runtime-measurements:** no Lighthouse/CPU-throttle traces, bundle-size baseline, memory/WS soak, or Tauri prototype executed in this environment.
- **NV-backend-contract-drift:** not all frontend route/store assumptions were validated against live `flora-api` in this task; prior cycles indicate ongoing contract evolution (see OBJETIVO §§3.2–3.4).
- **NV-`apps/web` reuse:** legacy app was only high-level cross-checked (structure/deps/README); no component-level portability audit performed—intentional, given debloat/non-canonical status.
- **NV-advanced-realtime:** multi-node fanout (Valkey/NOTIFY), full replay semantics, presence/typing, offline queue, and conflict resolution beyond minimal catch-up/upsert were out of scope.

## 6) Multilingual discovery keywords (for follow-up research)

- **EN:** Svelte 5 runes migration, SvelteKit state management load invalidate, realtime WebSocket reconnection idempotent UI, accessibility low-end performance, Tauri SvelteKit adapter-static.
- **pt-BR:** migração incremental Svelte 5 runas, SvelteKit gerenciamento de estado e carregamento de dados, UI em tempo real WebSocket reconexão, acessibilidade e desempenho em dispositivos modestos, Tauri com SvelteKit.
- **es:** migración incremental a Svelte 5 con runas, gestión de estado y carga de datos en SvelteKit, interfaz en tiempo real con WebSocket, accesibilidad y rendimiento en dispositivos de gama baja, Tauri y SvelteKit.
- **de:** schrittweise Migration zu Svelte 5 (Runes), Zustandsverwaltung und Datenladen in SvelteKit, Echtzeit-UI mit WebSocket, Barrierefreiheit und Leistung auf schwachen Geräten, Tauri-Bereitschaft.
- **fr:** migration incrémentale vers Svelte 5 (runes), gestion d’état et chargement de données avec SvelteKit, UI temps réel WebSocket, accessibilité et performance sur appareils modestes, préparation à Tauri.
- **zh-CN:** Svelte 5 runes 渐进式迁移、SvelteKit 状态管理与数据加载、WebSocket 实时界面与重连补偿、低端设备可访问性与性能优化、Tauri 适配准备。
- **ja:** Svelte 5 runes への段階的移行、SvelteKit の状態管理とデータ取得、WebSocket によるリアルタイムUIと再接続/差分同期、低性能端末向けアクセシビリティと性能、Tauri 対応。

## 7) Sources (primary)

- Svelte 5 migration guide: https://svelte.dev/docs/svelte/v5-migration-guide
- Svelte 5 runes (concept): https://svelte.dev/docs/svelte/what-are-runes
- SvelteKit state management: https://svelte.dev/docs/kit/state-management
- SvelteKit project types/adapters (Tauri/SPA/static context): https://svelte.dev/docs/kit/project-types, https://svelte.dev/docs/kit/adapters, https://svelte.dev/docs/kit/single-page-apps, https://svelte.dev/docs/kit/adapter-static
- Repo: `OBJETIVO.md`, `apps/frontend/{package.json,svelte.config.js,src/**}`, `apps/web/{package.json,README.md}` (cross-check only)
