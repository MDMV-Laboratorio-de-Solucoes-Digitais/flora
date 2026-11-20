# Release: 095 — Deploy do frontend Flora

**Ciclo**: 095 · **Data**: 2026-11-20 · **Branch**: `feature/095-frontend-deploy` @ `29b34aefe` · **Baseline**: `29b34aefe`
**Tipo**: correção de deploy — publicação da imagem `flora-web` do frontend em origem própria, API em `flora-api.mdmv.qzz.io`, `config.js` gerado em runtime e allowlist CORS com a origem web. **Nenhuma alteração de gate, required check, constituição, ADR, API, runtime, schema ou migration.** O deploy remoto segue **[NV]** até a validação final.

## Entregue

- Imagem `flora-web` construída de `apps/frontend/Dockerfile` com `context: apps/frontend` (build `adapter-node`, `build/index.js`), em lugar da imagem `flora-api` do `Dockerfile` da raiz que servia `/health`, `/metrics` e `/llms.txt` no FQDN web e deixava `/` e `/tasks` em 404 (FR-001, SC-001).
- Workflow `.github/workflows/docker-frontend.yml` de publicação **manual** (`workflow_dispatch`), com tags no GHCR e webhook Coolify condicionado a `HAS_HOOK`; nenhum trigger automático de push/PR/merge e nenhum required check novo (FR-002, SC-003).
- Mapa de origens registrado: `flora.mdmv.qzz.io` → frontend (`/`, `/tasks`); `flora-api.mdmv.qzz.io` → API (`/health`, `/metrics`, `/llms.txt`), com a configuração remota de DNS/Coolify fora do repositório e **[NV]** até o deploy final (FR-003, SC-001).
- `config.js` renderizado no start do container web a partir de `FLORA_API_BASE_URL` por `apps/frontend/docker-entrypoint.sh`, sobrescrevendo `globalThis.__FLORA_CONFIG__.apiBaseUrl` e escrevendo também `build/client/config.js`, sem alterar os defaults comitados em `apps/frontend/static/config.js` (FR-004, SC-002).
- Origem exata `https://flora.mdmv.qzz.io` incluída em `FLORA_APP__CORS_ORIGINS`, sem `*`, com as origens locais preservadas e coerência com o origin check do websocket (FR-005, SC-002).
- Verificação documental: build do frontend, Dockerfile/workflow lidos no branch, `check_gate_integrity.py` **PASS** (`exit=0`) e `/health` **HTTP 200** como evidência esperada do handoff; detalhe literal em `.specify/validations/095-frontend-deploy/validate.md` (R4 **APROVADO**), com **[NV]** para deploy/rotas/CORS ao vivo até a validação final (FR-006, SC-003; T007).

## SemVer

**Proposta:** MINOR `0.4.9` → `0.5.0`. Motivo: muda o contrato de deploy e de origens do produto — nova imagem `flora-web`, FQDN web passa a servir a aplicação, API ganha origem dedicada, `config.js` passa a ser configurável em runtime e a allowlist CORS passa a aceitar a origem pública. Não é patch: o comportamento observable do FQDN e a topologia de origens mudam; não é major: nenhuma API, schema, migration ou contrato de dados existente é quebrado. **Nenhuma tag criada**; registro a critério do maintainer.

## Changelog (FR / T)

- `FR-001` imagem `flora-web` a partir de `apps/frontend/Dockerfile` (`context: apps/frontend`, adapter-node), sem reaproveitar a imagem da API (T001, T002, T003, T007).
- `FR-002` publicação manual por `workflow_dispatch`, tags `flora-web:<tag>` e `:${{ github.sha }}` no GHCR, sem trigger automático e sem mexer em required checks/gates (T003, T008).
- `FR-003` `flora.mdmv.qzz.io` serve `/` e `/tasks`; `flora-api.mdmv.qzz.io` serve `/health`, `/metrics` e `/llms.txt` (T001, T004, T007).
- `FR-004` `config.js` gerado em runtime de `FLORA_API_BASE_URL`, com base WS derivada da mesma origem quando aplicável e defaults comitados preservados (T002, T005, T007).
- `FR-005` origem exata `https://flora.mdmv.qzz.io` em `FLORA_APP__CORS_ORIGINS`, sem `*`, coerente com o websocket (T006, T007).
- `FR-006` smoke de rotas/CORS/`config.js`, deploy remoto marcado **[NV]** e diff confinado ao deploy/ciclo — gate, API, runtime, schema, migration, `STATE.md`, `validate/` e `release/` fora da mudança de implementação (T007, T008).
- `T001` baseline `29b34aefe` e causa raiz · `T002` Dockerfile do frontend não publicado + defaults locais · `T003` workflow manual de `flora-web` · `T004` FQDNs e mapa de rotas · `T005` `config.js` de runtime · `T006` CORS · `T007` verificação (build, rotas, CORS, config; remoto **[NV]**) · `T008` fecho T001..T008 e fronteira.

## Evidência

`.specify/validations/095-frontend-deploy/validate.md` (R4 **APROVADO**; evidência esperada do handoff: build do frontend, Dockerfile/workflow, gate PASS, `/health 200`, com **[NV]** para deploy até validação final) · `specs/095-frontend-deploy/analyze.md` (READ-ONLY, `critical=0`, `requirements_covered=6/6`, `tasks=8/8`) · `specs/095-frontend-deploy/converge.md` (**Converged**, sem novas tarefas) · baseline `29b34aefe`. Nenhum gate, teste, formatter, build ou comando git foi reexecutado ao escrever este release.

**Estado final confirmado**: `apps/frontend/Dockerfile` em `node:22-alpine` e `.github/workflows/docker-frontend.yml` publicando `flora-web:production` e `flora-web:${{ github.sha }}`; valores confirmados no branch e usados como base deste release. A imagem API anterior do rollback permanece `flora:staging`.

## Rollback

Restaurar o estado anterior de deploy — **`Dockerfile`/workflow antigos e redeploy da imagem anterior da API**, sem force-push e sem reescrita de histórico:

1. no Coolify, apontar o app do FQDN `flora.mdmv.qzz.io` de volta para a configuração anterior (imagem `ghcr.io/mdmv-laboratorio-de-solucoes-digitais/flora:staging`, ou a tag `:${{ github.sha }}` anterior) e executar o **redeploy da imagem API anterior**; AGUARDAR o deploy terminar antes de declarar a reversão concluída;
2. desabilitar/remover o workflow manual `.github/workflows/docker-frontend.yml` e restaurar `apps/frontend/Dockerfile` e `apps/frontend/docker-entrypoint.sh` ao estado anterior; se `docker-staging.yml` ou o `Dockerfile` da raiz tiverem sido ajustados neste ciclo, restaurá-los ao conteúdo pré-095;
3. restaurar os valores de ambiente anteriores usados no redeploy (`FLORA_API_BASE_URL` e `FLORA_APP__CORS_ORIGINS` sem a origem web, se tiverem sido injetados) para que a imagem API volte a operar como antes;
4. **não reverter** gate, `check_gate_integrity.py`, required checks, constituição, ADR-0003, API, runtime, schema, migration, PRs, merge ou tags — nenhum deles foi alterado por este ciclo;
5. após o redeploy, repetir `python3 scripts/ci/check_gate_integrity.py` (esperado permanecer **PASS**) e confirmar `/health` → 200 na origem restaurada; o retorno do 404 em `/` e `/tasks` é o comportamento anterior esperado, não uma regressão do rollback;
6. se as alterações ainda estiverem não commitadas, descartar apenas os arquivos do ciclo 095; se commitadas, usar novo commit de reversão ou `git revert` — nunca `--force`. Os documentos do ciclo (decision/spec/plan/tasks/analyze/converge, R4 e este release) são registros e podem permanecer como histórico.

## [NV]

- **Deploy remoto até validação final**: DNS/FQDN, app Coolify, webhook e publicação de `flora-web` não observados; o FQDN pode seguir servindo a API até a promoção final.
- **Rotas/CORS/config ao vivo**: `/` e `/tasks` 200 com HTML SvelteKit, separação de rotas de API, `Access-Control-Allow-Origin` refletindo a origem web e `config.js` sem `localhost:3000` permanecem **[NV]** até a validação pós-deploy.
- **CI/PR/produção**: nenhum PR, merge, tag, aprovação, proteção de branch ou ambiente de produção verificado; gate local esperado PASS não substitui CI remota para promoção.
- **Reexecução nesta passagem**: nenhum gate, teste, formatter, build ou comando git foi reexecutado ao escrever este release; evidência literal consolidada no R4/handoff do main agent.
