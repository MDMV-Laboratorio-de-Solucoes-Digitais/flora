# Tasks: 095 — Deploy do frontend Flora

**Input**: `specs/095-frontend-deploy/spec.md`, `plan.md`,
`.specify/assessments/095-frontend-deploy/decision.md`

**Date**: 2026-11-20

**Format**: `[ID] Description`

## Phase 1 — Evidência e causa raiz

- [X] T001 Registrar o baseline `29b34aefe` e a cadeia determinística: o workflow
  de staging publica `context: .` com o `Dockerfile` da raiz (`flora-api`), por
  isso `/health`, `/metrics` e `/llms.txt` respondem no FQDN e `/` e `/tasks`
  retornam 404 — descartando falha intermitente de rede/DNS (FR-001, SC-001).
- [X] T002 Confirmar que `apps/frontend/Dockerfile` (`node:22-alpine`, build
  `adapter-node`, `CMD node build/index.js`) não é publicado por nenhum workflow
  e que `apps/frontend/static/config.js` mantém apenas o default local
  `http://localhost:3000` (FR-001, FR-004).

## Phase 2 — Imagem e publicação manual

- [X] T003 Publicar a imagem `flora-web` a partir de `apps/frontend/Dockerfile`
  com `context: apps/frontend`, por `.github/workflows/docker-frontend.yml`, nas tags
  `ghcr.io/mdmv-laboratorio-de-solucoes-digitais/flora-web:production` e
  `:${{ github.sha }}`, sem trigger automático de push/PR/merge e sem alterar
  required checks ou gates (FR-002, SC-003).

## Phase 3 — Origens e contrato de runtime

- [X] T004 Registrar a reatribuição de FQDNs: `flora.mdmv.qzz.io` serve o web
  (`/`, `/tasks`) e `flora-api.mdmv.qzz.io` serve a API (`/health`, `/metrics`,
  `/llms.txt`), com o mapa de rotas de cada origem e o disparo do deploy Coolify
  mantido manual (FR-003, SC-001).
- [X] T005 Gerar o `config.js` em runtime no container web a partir de
  `FLORA_API_BASE_URL`, sobrescrevendo
  `globalThis.__FLORA_CONFIG__.apiBaseUrl` (e a base do websocket quando
  aplicável), preservando os defaults comitados para dev local (FR-004, SC-002).
- [X] T006 Incluir a origem exata `https://flora.mdmv.qzz.io` em
  `FLORA_APP__CORS_ORIGINS`, sem `*`, mantendo as origens locais e a coerência
  com o origin check do websocket em
  `flora-api/src/routes/websocket.rs` (FR-005, SC-002).

## Phase 4 — Verificação e fecho

- [X] T007 Verificar o build da imagem web, `npm run check`/`npm run build` do
  frontend, o HTML de `/` e `/tasks` na origem web, as rotas de API na origem
  `flora-api`, o header `Access-Control-Allow-Origin` e o `config.js` servido
  sem `localhost:3000`; registrar o resultado do deploy remoto/Coolify como
  **[NV]** quando não observado (FR-001, FR-003, FR-004, FR-005, SC-001,
  SC-002).
- [X] T008 Fechar a rastreabilidade T001..T008 em decision/spec/plan/tasks e
  reconfirmar a fronteira: nenhum toque em `validate/`, `release/`,
  `.specify/memory/STATE.md`, required checks, API, runtime, schema ou migration
  (FR-006, SC-003).

## Dependencies & Execution Order

T001 → T002 → T003 → T004 → T005 → T006 → T007 → T008 (sequencial; T005 e T006
são independentes entre si após T004; T007 consolida as verificações e T008 o
rastro documental).
