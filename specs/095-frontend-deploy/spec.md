# Feature Specification: 095 — Deploy do frontend Flora

**Feature Branch**: `feature/095-frontend-deploy`

**Created**: 2026-11-20

**Status**: Implemented

**Input**: Decisão `go` de `.specify/assessments/095-frontend-deploy/decision.md`

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Frontend servido na origem pública (Priority: P1)

Como usuário do Flora, preciso abrir `https://flora.mdmv.qzz.io/` e navegar para
`/tasks` e ver a aplicação SvelteKit, em vez de um 404 vindo da API Rust.

**Why this priority**: é o defeito relatado e a causa raiz determinística — o FQDN
serve a imagem da API construída pelo `Dockerfile` da raiz (`context: .` no
workflow de staging), que não contém nenhum asset do frontend.

**Independent Test**: consultar as rotas no FQDN web e confirmar que `/` e
`/tasks` retornam 200 com HTML do SvelteKit, enquanto `/health`, `/metrics` e
`/llms.txt` são respondidos apenas na origem da API.

**Acceptance Scenarios**:

1. **Given** o FQDN apontando para o serviço da imagem `flora-web`, **When** uma
   requisição a `/` ou `/tasks` é feita, **Then** a resposta é 200 com o HTML da
   aplicação e não um 404 da API.
2. **Given** a API publicada em `flora-api.mdmv.qzz.io`, **When** `/health`,
   `/metrics` ou `/llms.txt` são consultados nessa origem, **Then** respondem como
   antes; essas rotas não são mais o conteúdo de `flora.mdmv.qzz.io`.

---

### User Story 2 — Browser aponta para a API correta (Priority: P1)

Como usuário do frontend publicado, preciso que o `config.js` entregue em runtime
informe a origem real da API, para que login e chamadas `/api/v1/*` não tentem
`http://localhost:3000`.

**Why this priority**: `src/lib/api/client.ts` resolve `API_BASE_URL` a partir de
`globalThis.__FLORA_CONFIG__`; sem a geração do `config.js` no deploy, o bundle
usaria o default de desenvolvimento mesmo em produção.

**Independent Test**: ler o `config.js` servido pelo container web e conferir que
`apiBaseUrl` é igual a `FLORA_API_BASE_URL`, sem `localhost:3000`.

**Acceptance Scenarios**:

1. **Given** `FLORA_API_BASE_URL=https://flora-api.mdmv.qzz.io` no container web,
   **When** o container inicia, **Then** o `config.js` servido contém essa origem
   em `apiBaseUrl`.
2. **Given** os valores comitados em `apps/frontend/static/config.js` como
   defaults locais, **When** o deploy ocorre, **Then** a renderização em runtime
   sobrescreve tais valores sem alterar o arquivo versionado.

---

### User Story 3 — API aceita a origem do frontend (Priority: P1)

Como usuário autenticado, preciso que o browser consiga chamar a API a partir de
`https://flora.mdmv.qzz.io` sem bloqueio de CORS, inclusive no upgrade do
websocket.

**Why this priority**: a allowlist `FLORA_APP__CORS_ORIGINS` só contém origens de
loopback; o origin check do websocket usa a mesma lista. Sem a origem web nela,
as requisições cross-origin e o realtime falhariam mesmo com o frontend servido.

**Independent Test**: enviar uma preflight/request com
`Origin: https://flora.mdmv.qzz.io` e conferir
`Access-Control-Allow-Origin: https://flora.mdmv.qzz.io`.

**Acceptance Scenarios**:

1. **Given** a allowlist da API sem a origem web, **When** a origem
   `https://flora.mdmv.qzz.io` é adicionada, **Then** a API responde com o header
   CORS refletindo exatamente essa origem.
2. **Given** a regra de que `*` é proibido, **When** a allowlist é revisada,
   **Then** ela mantém apenas origens exatas, incluindo as locais e a nova origem
   web.

---

### User Story 4 — Publicação e rastreabilidade (Priority: P2)

Como mantenedor, preciso publicar o frontend por um workflow manual explícito e
manter o registro do ciclo auditável sem alterar gates nem artefatos de
governança.

**Why this priority**: o deploy permanece fora da automação de push/PR/merge
(decisão do ciclo 037); torná-lo manual evita promover por acidente e preserva a
fronteira de governança.

**Independent Test**: inspecionar o workflow e os quatro artefatos do ciclo,
confirmando `workflow_dispatch`, tags da imagem `flora-web` e ausência de toques
em `validate/`, `release/` e `STATE`.

**Acceptance Scenarios**:

1. **Given** o workflow de publicação, **When** ele é disparado manualmente,
   **Then** constrói `apps/frontend/Dockerfile` com `context: apps/frontend` e
   publica `flora-web:production` e `flora-web:<sha>` no GHCR.
2. **Given** os artefatos do ciclo 095, **When** o diff é revisado, **Then**
   decision/spec/plan/tasks registram T001..T008 e nenhum arquivo de
   `validate/`, `release/` ou `STATE` foi modificado.

---

### Edge Cases

- Se `FLORA_API_BASE_URL` estiver ausente no container, o `config.js` deve manter
  o default local seguro em vez de gerar uma URL inválida.
- Se a origem web não estiver na allowlist CORS, o login e o websocket falham no
  browser mesmo com as rotas da app servidas; a origem exata deve ser conferida.
- Se o workflow continuar publicando apenas o `Dockerfile` da raiz, o 404 em `/`
  e `/tasks` retorna — a imagem da API não pode servir de imagem web.
- A configuração de FQDN/DNS e o deploy remoto são externos ao repositório e
  permanecem **[NV]** neste snapshot; a especificação cobre o contrato, não a
  observação remota.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: O deploy DEVE servir o frontend SvelteKit por uma imagem
  `flora-web` construída com `apps/frontend/Dockerfile` e `context:
  apps/frontend` (`node:22-alpine`, build `adapter-node`, `CMD node
  build/index.js`); a imagem da API (`Dockerfile` da raiz) NÃO DEVE ser usada
  como imagem web.
- **FR-002**: A publicação da imagem `flora-web` DEVE ocorrer por workflow manual
  (`.github/workflows/docker-frontend.yml`, `workflow_dispatch`), com tags
  `ghcr.io/mdmv-laboratorio-de-solucoes-digitais/flora-web:production` e
  `:${{ github.sha }}`, sem adicionar trigger automático de push/PR/merge e sem
  alterar required checks ou gates.
- **FR-003**: O FQDN `flora.mdmv.qzz.io` DEVE apontar para o serviço web
  (`/`, `/tasks`); a API DEVE ser exposta em `flora-api.mdmv.qzz.io`
  (`/health`, `/metrics`, `/llms.txt`), separando as duas origens.
- **FR-004**: O `config.js` servido em runtime DEVE ser gerado a partir de
  `FLORA_API_BASE_URL`, sobrescrevendo
  `globalThis.__FLORA_CONFIG__.apiBaseUrl` (e derivando a base do websocket da
  mesma origem quando aplicável), sem modificar os defaults comitados em
  `apps/frontend/static/config.js`.
- **FR-005**: A allowlist `FLORA_APP__CORS_ORIGINS` da API DEVE incluir a origem
  exata `https://flora.mdmv.qzz.io`, sem `*`, preservando as origens locais e
  mantendo coerência com a checagem de origem do websocket.
- **FR-006**: O ciclo DEVE verificar rotas, header CORS e conteúdo do
  `config.js`, registrar o resultado remoto como **[NV]** quando não observado,
  e manter o diff nos artefatos de deploy/ciclo — sem tocar em `validate/`,
  `release/`, `.specify/memory/STATE.md`, gates, API, runtime, schema ou
  migration.

### Key Entities

- **flora-web**: imagem do frontend SvelteKit (`apps/frontend/Dockerfile`),
  distinta da imagem `flora` da API.
- **FQDNs**: `flora.mdmv.qzz.io` (web) e `flora-api.mdmv.qzz.io` (API), com
  conjuntos de rotas distintos.
- **config.js**: contrato de runtime entre container web e bundle, via
  `globalThis.__FLORA_CONFIG__` (`apiBaseUrl`, `wsBaseUrl`).
- **FLORA_APP__CORS_ORIGINS**: allowlist exata de origens do navegador,
  compartilhada por CORS e pelo origin check do websocket.
- **Workflow `docker-frontend.yml`**: publicação manual no GHCR e disparo do deploy
  Coolify; permanece fora do gate automático.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Em `flora.mdmv.qzz.io`, `/` e `/tasks` retornam 200 com HTML do
  SvelteKit e `/health`, `/metrics`, `/llms.txt` não são mais servidos por essa
  origem; em `flora-api.mdmv.qzz.io`, as rotas de API respondem como antes.
- **SC-002**: O `config.js` entregue ao browser contém `apiBaseUrl` igual a
  `FLORA_API_BASE_URL` e nenhuma ocorrência de `localhost:3000` no runtime; uma
  requisição com `Origin: https://flora.mdmv.qzz.io` recebe
  `Access-Control-Allow-Origin: https://flora.mdmv.qzz.io`.
- **SC-003**: A imagem `flora-web` é publicada somente por `workflow_dispatch`
  com as tags `:production` e `:sha`; nenhum required check, workflow de gate, API,
  runtime, schema, migration, `validate/`, `release/` ou `STATE` foi alterado, e
  o resultado do deploy remoto permanece **[NV]** neste snapshot.

## Assumptions

- O diagnóstico é o registrado em
  `.specify/assessments/095-frontend-deploy/decision.md`: o 404 em `/` e
  `/tasks` decorre de o FQDN servir a imagem da API, não de falha intermitente.
- `apps/frontend/Dockerfile` e o adapter-node já produzem `build/index.js`
  executável; o ciclo não reescreve o frontend.
- `bun.lock` continua sendo o lockfile canónico da CI; nenhuma troca de
  gerenciador é feita neste ciclo.
- As verificações e o estado remoto são os registrados no handoff; este
  artefato não reexecutou gates, build, testes, formatters nem consultou o
  deploy ao vivo.
