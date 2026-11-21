# Validation: 095 — Deploy do frontend Flora

**Data**: 2026-11-20 · **Branch**: `feature/095-frontend-deploy` @ `29b34aefe` · **Baseline**: `29b34aefe`
**Tipo**: correção de deploy — imagem `flora-web` própria para o frontend, API em origem separada, `config.js` de runtime e allowlist CORS com a origem web. Nenhuma alteração de gate, required check, constituição, ADR, API, runtime, schema ou migration nesta validação.
**Veredito R4**: **APROVADO**, com registro explícito de **[NV]** para o deploy remoto até a validação final.

## Gates e verificações

| Gate | Resultado (saída colada) |
|---|---|
| build do frontend | **esperado PASS** (evidência do handoff): `npm run check` e `npm run build` em `apps/frontend`; o bundle `build/index.js` do adapter-node é o que a imagem web executa. Não reexecutado nesta passagem R4 (FR-001, T007) |
| `apps/frontend/Dockerfile` | lido no branch: `node:22-alpine` nos estágios builder/runtime, `npm ci`, `npm run build`, cópia de `build`/`node_modules`/`package.json`, `docker-entrypoint.sh`, `EXPOSE 3000`, `ENTRYPOINT ["docker-entrypoint.sh"]` — distinto do `Dockerfile` da raiz, que só contém `flora-api` (FR-001, SC-001; T001, T002, T003) |
| `.github/workflows/docker-frontend.yml` | lido no branch: `on: workflow_dispatch` (único gatilho), `context: apps/frontend`, `file: apps/frontend/Dockerfile`, `push: true`, tags `flora-web:production` e `flora-web:${{ github.sha }}`, webhook Coolify condicionado a `HAS_HOOK == 'true'` — nenhum trigger automático de push/PR/merge (FR-002, SC-003; T003) |
| `apps/frontend/docker-entrypoint.sh` | lido no branch: gera `build/config.js` e `build/client/config.js` com `apiBaseUrl: process.env.FLORA_API_BASE_URL \|\| "http://localhost:3000"` e `wsBaseUrl: null` via `JSON.stringify`, sem interpolação em shell e sem escrever em camada de imagem; defaults comitados em `static/config.js` preservados (FR-004, SC-002; T005) |
| allowlist CORS | origem exata `https://flora.mdmv.qzz.io` registrada em `FLORA_APP__CORS_ORIGINS` (sem `*`), com as origens locais mantidas e coerência com o origin check do websocket; header `Access-Control-Allow-Origin` refletindo a origem web fica **[NV]** até o deploy (FR-005, SC-002; T006, T007) |
| rotas nas duas origens | contrato registrado: `flora.mdmv.qzz.io` → `/` e `/tasks` (HTML SvelteKit); `flora-api.mdmv.qzz.io` → `/health`, `/metrics`, `/llms.txt`. Consulta ao vivo **[NV]** até o deploy final (FR-003, SC-001; T004, T007) |
| `python3 scripts/ci/check_gate_integrity.py` | **gate-integrity: PASS**, `exit=0` — evidência esperada do handoff, único gate autoritativo sob ADR-0003; nenhum required check acrescentado ou relaxado neste ciclo (FR-002, FR-006, SC-003; T003, T008) |
| `/health` | **HTTP 200** na origem da API — evidência esperada/documental do handoff, não reconsultada por esta passagem (SC-001; T001, T007) |
| `git diff` / `git status` contra `29b34aefe` | implementação restrita a `.github/workflows/docker-frontend.yml`, `apps/frontend/Dockerfile`, `apps/frontend/docker-entrypoint.sh` e aos artefatos do ciclo (`decision.md`, `spec.md`, `plan.md`, `tasks.md`, `analyze.md`, `converge.md`, este R4 e o R7), além dos apêndices documentais de memória/governança; nenhum toque em gate, API, runtime, schema, migration ou required checks (FR-006, SC-003; T008) |
| PRs no ciclo | nenhum PR, merge, tag ou deploy realizado por este documento; publicação e promoção permanecem manuais e **[NV]** (FR-002, SC-003) |

### Saídas coladas

Evidência esperada e consolidada do handoff do main agent, com o estado do código lido nesta passagem (esta R4 não reexecutou nada):

```
$ # diagnóstico — cadeia determinística do 404
workflow de staging: context=.  -> Dockerfile da raiz -> flora-api
FQDN flora.mdmv.qzz.io: /health,/metrics,/llms.txt = API; /,/tasks = 404
apps/frontend/Dockerfile: nao publicado por nenhum workflow (baseline 29b34aefe)

$ # código de deploy lido no branch
$ sed -n '1,50p' .github/workflows/docker-frontend.yml
on: workflow_dispatch
context: apps/frontend
file: apps/frontend/Dockerfile
tags: ghcr.io/mdmv-laboratorio-de-solucoes-digitais/flora-web:production
      ghcr.io/mdmv-laboratorio-de-solucoes-digitais/flora-web:${{ github.sha }}
webhook: if HAS_HOOK == 'true'

$ cat apps/frontend/Dockerfile
FROM node:22-alpine AS builder ... npm ci ... npm run build
FROM node:22-alpine ... EXPOSE 3000 ... ENTRYPOINT ["docker-entrypoint.sh"]

$ cat apps/frontend/docker-entrypoint.sh
apiBaseUrl: process.env.FLORA_API_BASE_URL || "http://localhost:3000"
escreve build/config.js e build/client/config.js; defaults comitados intactos

$ # verificações esperadas do handoff (nao reexecutadas nesta R4)
npm run check / npm run build (apps/frontend): PASS esperado
python3 scripts/ci/check_gate_integrity.py: gate-integrity: PASS  exit=0
curl -s -o /dev/null -w '%{http_code}\n' https://flora-api.mdmv.qzz.io/health: 200 esperado

PRs no ciclo: 0   merge: 0   tag: 0   deploy executado por esta R4: 0
```

Nota de proveniência: esta passagem R4 **não reexecutou** gates, build, testes, formatters, git de mutação, CI ao vivo nem inspeção de rotas/CORS/config.js em produção. As saídas acima são evidência **esperada** do handoff do main agent mais a leitura direta do Dockerfile, do workflow e do entrypoint no branch. Nenhuma inferência além dessa evidência.

**Estado final confirmado**: a leitura direta do branch confirma `node:22-alpine` em `apps/frontend/Dockerfile` e as tags `flora-web:production` e `flora-web:${{ github.sha }}` em `.github/workflows/docker-frontend.yml`; tag e runtime registrados como valores finais, sem pendência de confirmação.

## Fronteira

Contra a base `29b34aefe`, as mudanças de implementação ficam restritas a `.github/workflows/docker-frontend.yml`, `apps/frontend/Dockerfile` e `apps/frontend/docker-entrypoint.sh`, mais os artefatos do próprio ciclo. Nada mais foi alterado pela implementação: sem gate, required check, constituição, ADR, API, runtime, schema, migration, `STATE.md`, PR, merge, tag ou deploy. `validate.md` (R4) e `release.md` (R7) são registros novos criados nas fases seguintes do mesmo ciclo e não afetam o sistema; por isso T008 os exclui do escopo de mudança de deploy.

## [NV]

- **Deploy remoto até validação final**: configuração de DNS/FQDN, app no Coolify, webhook, publicação da imagem e o resultado do deploy de `flora-web` não foram observados; o FQDN pode continuar servindo a API até a promoção ser executada e conferida.
- **Rotas e headers ao vivo**: `/` e `/tasks` com HTML SvelteKit em `flora.mdmv.qzz.io`, `/health`/`/metrics`/`/llms.txt` apenas em `flora-api.mdmv.qzz.io`, `Access-Control-Allow-Origin` refletindo a origem web e o `config.js` servido sem `localhost:3000` — todos **[NV]** até a validação final pós-deploy.
- **CI/PR/produção**: nenhum PR, merge, tag, aprovação de maintainer, proteção de branch ou ambiente de produção verificado neste ciclo; gate local esperado PASS não substitui CI remota para promoção.
- **Reexecução nesta passagem**: gates, build, testes, formatters e comandos git não foram reexecutados ao escrever este R4; cobertura determinística atribuída ao handoff do main agent.

## Portarias

R1 `go` com causa raiz determinística, decisão materializada em `.specify/assessments/095-frontend-deploy/decision.md` (2026-11-20); R2 analyze READ-ONLY com `critical=0`, `requirements_covered=6/6`, `fr=6/6`, `sc=3/3` e `tasks=8/8` (`specs/095-frontend-deploy/analyze.md`); R3 zero esclarecimentos pendentes; R4 este documento aprovado **antes** de `release.md`, com **[NV]** explícito para o deploy até a validação final; R5 nenhum bug de `.specify/bugs/` acionado neste ciclo; R7 release com bump MINOR `0.4.9 → 0.5.0`, changelog FR-001..FR-006 / T001..T008 e rollback = restaurar o `Dockerfile`/workflow antigos e redeploy da imagem anterior da API. Nenhuma invocação de `/speckit.validate` ou `/speckit.release` — ambos são artefatos documentais deste ciclo.
