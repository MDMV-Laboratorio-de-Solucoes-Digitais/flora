# Problem Definition: Rotas da aplicação ausentes no FQDN

- **Slug**: 095-frontend-deploy
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md

## Problem Statement

O FQDN `flora.mdmv.qzz.io` não serve o frontend: ele serve a API Rust. A cadeia é determinística, não intermitente — `.github/workflows/docker-staging.yml` publica a imagem construída com `context: .` pelo `Dockerfile` da raiz, que compila apenas `-p flora-api` e cita somente `/usr/local/bin/flora-api`; por isso `/health`, `/metrics` e `/llms.txt` respondem como rotas da API, enquanto `/` e `/tasks` retornam 404 (`decision.md:12-34`; `research.md:53`). `apps/frontend/Dockerfile` existe, mas nenhum workflow publicava essa imagem, de modo que o frontend não era servido em nenhum FQDN (`decision.md:24-26`). Mesmo que fosse servido, `apps/frontend/static/config.js` e `DEFAULT_API_BASE_URL` apontam para `http://localhost:3000` e a allowlist `FLORA_APP__CORS_ORIGINS` contém apenas origens de loopback (`decision.md:24-31`). Não há indício de falha de rede, DNS intermitente ou build quebrado: o conteúdo publicado explica integralmente o 404 (`decision.md:33-34`). Medições remotas e configuração de DNS/Coolify não foram observadas nesta avaliação — **[NV]** (`research.md:49`).

## Affected Users & Stakeholders

- **Users**: usuário final que abre a aplicação no FQDN e navega para `/tasks`, recebendo 404 da API Rust (`research.md:55`); maintainer/SE que relatou o defeito (`intake.md:34`).
- **Stakeholders**: dono do gate ADR-0003 (deploy fora do gate de merge); operador de deploy/Coolify; responsável por DNS/FQDN; [ASSUMPTION] consumidores das rotas de API na origem atual, não identificados (`research.md:85`).

## Goals

- Tornar as rotas da aplicação (`/`, `/tasks`) acessíveis em uma origem web própria, distinta da origem da API.
- Fazer com que rotas de API (`/health`, `/metrics`, `/llms.txt`) respondam em uma origem identificável como API.
- Garantir que o browser use a base da API correta em produção, em vez do default de desenvolvimento (`decision.md:50-54`).
- Garantir uma allowlist CORS explícita, sem curinga, coerente com a checagem de origem do websocket (`decision.md:54-56`).
- Manter a publicação como decisão manual, fora do gate de merge, sem trigger automático de push/PR/merge (`decision.md:57-59`).

## Non-Goals

- Alterar required checks, gates de CI, API de aplicação, runtime, schema, migration ou credenciais.
- Criar trigger automático de publicação ou promover deploy a condição de merge (`decision.md:57-59`).
- Afirmar respostas HTTP remotas, configuração de DNS/FQDN ou resultado do deploy sem **[NV]** (`research.md:49`).
- Tocar `validate/`, `release/` ou `.specify/memory/STATE.md`.

## Success Metrics

- `/` e `/tasks` respondem na origem web e `/health`, `/metrics`, `/llms.txt` na origem da API (baseline: 404 em `/`, `/tasks` e respostas de API no mesmo FQDN em 2026-11-20T23:13:59Z — `research.md:50`; remoto atual **[NV]**).
- `config.js` entregue ao browser aponta para a base da API e a allowlist CORS inclui a origem web exata, sem `*` (baseline: `http://localhost:3000` e origens de loopback — `apps/frontend/static/config.js:1-7`; `research.md:56`).
- Preflight CORS e smoke das rotas sem erro (baseline: registro de 200 em `STATE.md:826-830`; remoto não reconsultado — `research.md:58`).
- Publicação manual, sem trigger automático e com diff restrito (baseline: `workflow_dispatch` em `.github/workflows/docker-staging.yml`; `research.md:53`).

## Cost of Inaction

A aplicação continua inacessível no FQDN, com `/` e `/tasks` retornando 404 enquanto rotas de API respondem na mesma origem. Essa mistura impede configurar a base da API e o CORS de forma coerente, e qualquer consumidor não documentado das rotas de API nessa origem pode ser afetado por uma separação improvisada sem diagnóstico prévio.

## Open Questions

- [NEEDS CLARIFICATION: quais são as configurações atuais de DNS/FQDN e de deploy no Coolify, dado que não foram observadas neste snapshot (**[NV]**)?] (`intake.md:37`)
- [NEEDS CLARIFICATION: qual é a resposta HTTP real de `/`, `/tasks`, `/health`, `/metrics` e `/llms.txt` nas duas origens após a mudança (**[NV]**)?] (`intake.md:38`)
- [NEEDS CLARIFICATION: quem detém a autorização para disparar a publicação manual e para promover o deploy?] (`intake.md:39`)
- [NEEDS CLARIFICATION: existem consumidores de `flora.mdmv.qzz.io` que dependam das rotas da API nessa origem?] (`intake.md:40`; `research.md:85`)
- [NEEDS CLARIFICATION: qual estratégia de rollback se aplica se a separação de origens quebrar o login ou o websocket?] (`intake.md:41`; `research.md:87`)
- [NEEDS CLARIFICATION: o header CORS e o `config.js` entregues ao browser foram verificados em produção ou permanecem apenas como contrato especificado (**[NV]**)?] (`intake.md:42`)
- [NEEDS CLARIFICATION: como o desvio entre a publicação prevista no GHCR (FR-002) e o build direto do Git no Coolify deve ser refletido nos artefatos do ciclo?] (`research.md:83`)
