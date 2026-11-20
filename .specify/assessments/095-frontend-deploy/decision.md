# Decision: 095 — Deploy do frontend Flora

- **Slug**: 095-frontend-deploy
- **Decidido**: 2026-11-20
- **Verdict**: **go**
- **Branch**: `feature/095-frontend-deploy` @ `29b34aefe`
- **Âncora**: `.specify/feature.json` → `specs/095-frontend-deploy`
- **Artefatos revisados**: spec | plan | tasks

## Contexto / Problema — causa raiz determinística

O FQDN `flora.mdmv.qzz.io` **não serve o frontend**; ele serve a API Rust. A cadeia
de evidência é determinística, não intermitente:

1. `.github/workflows/docker-staging.yml` (gatilho apenas `workflow_dispatch`)
   publica a imagem construída com `context: .`, ou seja, pelo `Dockerfile` da
   raiz, sob as tags `ghcr.io/mdmv-laboratorio-de-solucoes-digitais/flora:staging`
   e `:${{ github.sha }}`.
2. O `Dockerfile` da raiz compila `-p flora-api` e a imagem final contém somente
   `/usr/local/bin/flora-api` com `EXPOSE 3000` — nenhum asset do SvelteKit.
3. Por isso `/health`, `/metrics` e `/llms.txt` respondem como rotas da API no
   FQDN, enquanto `/` e `/tasks` retornam 404: as rotas da aplicação nunca foram
   publicadas nessa origem.
4. `apps/frontend/Dockerfile` existe (`node:22-alpine`, build
   `adapter-node` → `build/index.js`), mas nenhum workflow publica essa imagem;
   o frontend não é servido em nenhum FQDN.
5. Mesmo que o frontend fosse servido, `apps/frontend/static/config.js` e
   `DEFAULT_API_BASE_URL` apontam para `http://localhost:3000`; sem runtime
   config gerada no deploy, o browser chamaria localhost. A allowlist
   `FLORA_APP__CORS_ORIGINS` (em `flora-core/src/config/app.rs`) contém apenas
   origens de loopback, sem `https://flora.mdmv.qzz.io`.

Não há indício de falha de rede, DNS intermitente ou build quebrado: o served
content e o código de deploy explicam integralmente o 404 nas rotas da app.

## Decisão

Registrar o ciclo 095 como **deploy do frontend em origem própria**, com seis
compromissos rastreáveis (FR-001..FR-006) e oito tarefas (T001..T008):

1. **Imagem web (FR-001 / SC-001)**: criar a imagem `flora-web` a partir de
   `apps/frontend/Dockerfile` (`context: apps/frontend`); não reaproveitar a
   imagem da API para o FQDN do frontend.
2. **Publicação manual (FR-002 / SC-003)**: publicar `flora-web` no GHCR por `.github/workflows/docker-frontend.yml`
   `workflow_dispatch`, com tags `:production` e `:${{ github.sha }}`, sem criar
   trigger automático de push/PR/merge e sem mexer em required checks.
3. **FQDNs (FR-003 / SC-001)**: `flora.mdmv.qzz.io` passa a servir o frontend
   (`/`, `/tasks`); a API passa a `flora-api.mdmv.qzz.io`
   (`/health`, `/metrics`, `/llms.txt`).
4. **Config de runtime (FR-004 / SC-002)**: gerar `config.js` no container a partir de
   `FLORA_API_BASE_URL`, sobrescrevendo
   `globalThis.__FLORA_CONFIG__.apiBaseUrl`; os defaults comitados permanecem
   seguros para dev local.
5. **CORS (FR-005 / SC-002)**: incluir a origem exata `https://flora.mdmv.qzz.io` em
   `FLORA_APP__CORS_ORIGINS`, sem curinga, preservando as origens locais e a
   consistência com a checagem de origem do websocket.
6. **Verificação e fronteira (FR-006 / SC-003)**: smoke de rotas, headers CORS e
   `config.js`; diff restrito aos artefatos do ciclo e ao deploy, sem tocar em
   `validate/`, `release/` ou `.specify/memory/STATE.md`.

## Evidência de entrada

- Baseline `29b34aefe`, branch `feature/095-frontend-deploy`.
- `.github/workflows/docker-staging.yml`: `context: .`, `workflow_dispatch`,
  tags `flora:staging` / `flora:<sha>` e webhook de deploy Coolify condicionado
  a `COOLIFY_DEPLOY_WEBHOOK`.
- `Dockerfile` da raiz: `cargo build --release -p flora-api`, imagem final só
  `flora-api` (porta 3000).
- `apps/frontend/Dockerfile`: estágio builder `node:22-alpine` + `npm ci` +
  `npm run build`; runtime `node:22-alpine` com `build/`, `node_modules/` e
  `CMD node build/index.js`.
- `apps/frontend/static/config.js`: `apiBaseUrl: "http://localhost:3000"`,
  `wsBaseUrl: null`; `src/app.html` carrega `%sveltekit.assets%/config.js` e
  `src/lib/api/client.ts` resolve a base a partir de `__FLORA_CONFIG__`.
- `.env.example`: `FLORA_APP__CORS_ORIGINS` com apenas origens `localhost` /
  `127.0.0.1` (portas 5173/4173); `AppConfig::validate()` rejeita `*`.

## Racional

**go.** O defeito observado (404 em `/` e `/tasks`, respostas de API em
`/health`, `/metrics`, `/llms.txt`) é a consequência exata do que o workflow
publica e do que o `Dockerfile` da raiz contém. A correção é uma separação de
imagens e de origens, com config de runtime e allowlist CORS explícitas — todas
verificáveis por inspeção de arquivos e por smoke de rotas. Nenhuma mudança de
gate, schema ou contrato da API é necessária; o diff é reversível.

## [NV]

- A publicação no GHCR, o deploy no Coolify, a configuração de DNS/FQDN e as
  respostas HTTP remotas **não foram executadas nem observadas neste snapshot**:
  permanecem **[NV]**. O veredito `go` autoriza o ciclo; não afirma deploy pronto.
- As verificações relatadas no handoff são evidência consolidada; este artefato
  não reexecutou gates, build, testes ou formatters.

## Fora de âmbito

Mudanças de required checks, gates de CI, API de aplicação, runtime, schema,
migration, credenciais e produção, bem como qualquer alteração em `validate/`,
`release/` ou `.specify/memory/STATE.md`.

## Handoff

`/speckit.specify` — especificar o deploy como requisitos verificáveis
(FR-001..FR-006 / SC-001..SC-003) e tarefas T001..T008, com a imagem `flora-web`
publicada por workflow manual, os FQDNs separados, o `config.js` gerado de
`FLORA_API_BASE_URL`, a origem web na allowlist CORS e a fronteira preservada.
