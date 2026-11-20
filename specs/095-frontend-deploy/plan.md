# Implementation Plan: 095 — Deploy do frontend Flora

**Branch**: `feature/095-frontend-deploy` | **Date**: 2026-11-20 |
**Spec**: [spec.md](./spec.md)

**Input**: `.specify/assessments/095-frontend-deploy/decision.md` (go, causa raiz
determinística)

## Summary

Separar a imagem da API da imagem web: publicar `flora-web` a partir de
`apps/frontend/Dockerfile` por workflow manual, apontar `flora.mdmv.qzz.io` para
o frontend e `flora-api.mdmv.qzz.io` para a API, gerar `config.js` em runtime a
partir de `FLORA_API_BASE_URL` e incluir a origem web na allowlist CORS.

## Technical Context

**Root cause**: `.github/workflows/docker-staging.yml` publica `context: .` com o
`Dockerfile` da raiz (`flora-api` apenas); por isso `/health`, `/metrics` e
`/llms.txt` respondem no FQDN e `/` e `/tasks` dão 404.

**Target Path**: imagem `apps/frontend/Dockerfile` (context `apps/frontend`),
workflow manual `docker-frontend.yml`, config de FQDN/Coolify,
`apps/frontend/static/config.js` em runtime e `FLORA_APP__CORS_ORIGINS`.

**Testing**: build da imagem web, `npm run check`/`npm run build` do frontend,
smoke de rotas nas duas origens, header CORS e conteúdo do `config.js`; deploy
remoto **[NV]** neste snapshot.

**Constraints**: nenhum trigger automático de push/PR/merge, nenhuma mudança de
required checks, API, runtime, schema ou migration; sem tocar em `validate/`,
`release/` ou `STATE`.

## Steps

1. **Evidência**: registrar baseline `29b34aefe`, a cadeia workflow/Dockerfile e
   os 404 observados; confirmar que o Dockerfile do frontend não é publicado
   (T001–T002).
2. **Imagem e publicação**: workflow `workflow_dispatch` para `flora-web`
   (`context: apps/frontend`), tags `:production`/`:sha`, sem alterar gates
   (T003).
3. **Origens**: registrar `flora.mdmv.qzz.io` → web e `flora-api.mdmv.qzz.io` →
   API com o mapa de rotas de cada uma (T004).
4. **Runtime e CORS**: gerar `config.js` de `FLORA_API_BASE_URL` e adicionar
   `https://flora.mdmv.qzz.io` a `FLORA_APP__CORS_ORIGINS` (T005–T006).
5. **Verificação e fecho**: smoke de rotas/CORS/config, fronteira do diff e
   rastreabilidade T001..T008 (T007–T008).

## Constitution Check

Sem violações: nenhum required check, gate, schema, migration ou contrato da API
é alterado; publicação permanece manual (decisão 037) e o diff fica restrito ao
deploy e aos artefatos do ciclo 095.

## Project Structure

```text
.specify/assessments/095-frontend-deploy/decision.md
specs/095-frontend-deploy/
├── spec.md      # FR-001..FR-006 / SC-001..SC-003
├── plan.md
└── tasks.md     # T001..T008
apps/frontend/Dockerfile          # imagem flora-web (node:22-alpine, adapter-node)
apps/frontend/static/config.js    # defaults locais; runtime os sobrescreve
.env.example                      # FLORA_APP__CORS_ORIGINS com a origem web
.github/workflows/                # publicação manual de flora-web no GHCR
```

## Complexity Tracking

Nenhuma. A correção é separação de imagens/origens mais duas configurações
explícitas (runtime config e allowlist CORS), verificáveis por inspeção e smoke.
