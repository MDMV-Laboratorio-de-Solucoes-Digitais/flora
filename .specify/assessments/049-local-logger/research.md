# Research: 049 — Local logger

**Data:** 2026-11-17T15:05Z · **Evidência local**; remoto **[NV]**.

## Primária

- `apps/frontend/src/lib/utils/logger.ts:1` importa `ApiClient`; `:19-30` agenda flush
  a cada 5 s; `:40` usa `navigator.sendBeacon('/api/v1/telemetry/logs', ...)`;
  `:106` usa `ApiClient.request('/api/v1/telemetry/logs', POST)`.
- `rg -n 'telemetry' flora-api/src` → **0 ocorrências**; `routes/mod.rs` registra
  13 módulos, nenhum de telemetria. `/metrics` (`:26`) é estático.
- `rg -l 'utils/logger' apps/frontend/src` → 8 arquivos (websocket, callback,
  WorkspaceCreate, OrganizationManager, FileUpload, ChatLayout, ThreadReply, UserManager).
- `rg '\.flush\('` → apenas 2 chamadas internas em `logger.ts`; nenhum chamador externo.
- `apps/frontend/vite.config.ts:38-44` — projeto `server` (node) inclui
  `src/**/*.{test,spec}.{js,ts}`; `logger.ts` não usa `window` fora do browser.
- `specs/002-svelte-frontend-rewrite/contracts/logger-contract.md:20-34` descreve
  `flush(): Promise<void>` e endpoint Backend Ingestion API — ambos inexistentes.

## Decisão técnica

Tornar o logger local-only: buffer em memória limitado + `snapshot()` para
diagnóstico, sem `fetch`/`sendBeacon`/`setInterval`/`beforeunload`. API pública
(`debug/info/warn/error`) inalterada — os 8 call sites não mudam. Isso elimina
404s silenciosos sem introduzir `console.log` (regra no-console preservada).

## NV

Comportamento real do `sendBeacon` em produção e qualquer coletor OTel externo: **[NV]**
— não há backend nem exporter neste checkout.
