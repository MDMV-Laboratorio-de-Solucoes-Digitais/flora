# Analyze: 095 — Deploy do frontend Flora (READ-ONLY)

**Data**: 2026-11-20 · **Modo**: READ-ONLY · **Branch**: `feature/095-frontend-deploy` @ `29b34aefe` · **Baseline**: `29b34aefe`
**Artefatos revisados**: `.specify/feature.json` (= `specs/095-frontend-deploy`), `.specify/assessments/095-frontend-deploy/decision.md` (verdict `go`), `spec.md` (FR-001..FR-006, SC-001..SC-003), `plan.md`, `tasks.md` (T001..T008, todos `[X]`) e a cadeia de deploy no repositório (`Dockerfile` da raiz, `.github/workflows/docker-staging.yml`, `apps/frontend/Dockerfile`, `apps/frontend/docker-entrypoint.sh`, `.github/workflows/docker-frontend.yml`, allowlist `FLORA_APP__CORS_ORIGINS`).
**Escopo**: corrigir a causa raiz determinística do 404 em `/` e `/tasks` — o FQDN `flora.mdmv.qzz.io` servia a imagem da API Rust (`Dockerfile` da raiz, `context: .` no workflow de staging), cujas rotas são `/health`, `/metrics` e `/llms.txt`. A decisão publica o frontend numa imagem `flora-web` própria, mantém a API em origem separada (`flora-api.mdmv.qzz.io`), gera `config.js` em runtime a partir de `FLORA_API_BASE_URL`, inclui a origem web na allowlist CORS e mantém a publicação manual (`workflow_dispatch`). Fora de escopo: required checks, gate, constituição/ADRs, código da API, runtime, schema, migration, PR/merge/tag e observação ao vivo do deploy.

**Estado final confirmado**: no estado final do branch, `apps/frontend/Dockerfile` usa `node:22-alpine` (exigência do SvelteKit 3) e `.github/workflows/docker-frontend.yml` publica `flora-web:production` e `flora-web:${{ github.sha }}` no GHCR. Tag e versão do Node foram confirmados pela leitura direta dos arquivos no alvo.

## Checagem determinística

```
artifacts=9  critical=0  requirements_covered=6/6  fr=6/6  sc=3/3  tasks=8/8  go=True  errors=[]
FR match= True ['FR-001', 'FR-002', 'FR-003', 'FR-004', 'FR-005', 'FR-006']
SC match= True ['SC-001', 'SC-002', 'SC-003']
tasks unique= True ['T001', 'T002', 'T003', 'T004', 'T005', 'T006', 'T007', 'T008']
```

`requirements_covered=6/6` contabiliza os 6 requisitos funcionais (FR-001..FR-006); SC-001..SC-003 são rastreados nas mesmas linhas. `critical=0`: nenhuma divergência de contrato entre `decision.md`, `spec.md`, `plan.md`, `tasks.md` e a cadeia de deploy revisada — o diagnóstico do 404 e as quatro frentes de correção (imagem, origens, config de runtime, CORS) são coerentes. `go=True` registrado em `decision.md`. `analyze` não alterou código, workflow, gate, ADR, constituição, `STATE.md`, `validate/` nem `release/`, não reexecutou gates, build, testes, formatters nem comandos git de mutação.

## Rastreabilidade — 6/6 requisitos cobertos, 8/8 tarefas

| # | Requisito (spec) | Tarefa(s) | Evidência |
|---|---|---|---|
| R1 | FR-001 — servir o frontend por imagem `flora-web` construída de `apps/frontend/Dockerfile` (`context: apps/frontend`, build `adapter-node`, `build/index.js`), sem reaproveitar a imagem `flora-api` da raiz (SC-001) | T001, T002, T003, T007 | cadeia determinística registrada em `decision.md`: `docker-staging.yml` publica `context: .` (Dockerfile raiz, `cargo build -p flora-api`), logo `/` e `/tasks` 404; `apps/frontend/Dockerfile` compila o bundle e o `docker-frontend.yml` usa `context: apps/frontend`/`file: apps/frontend/Dockerfile` |
| R2 | FR-002 — publicação de `flora-web` somente por `workflow_dispatch`, sem trigger automático de push/PR/merge e sem alterar required checks/gates (SC-003) | T003, T008 | `.github/workflows/docker-frontend.yml` declara apenas `on: workflow_dispatch`, `permissions: contents: read`/`packages: write`, webhook condicionado a `HAS_HOOK`; nenhum required check acrescentado, gate `check_gate_integrity.py` sob ADR-0003 preservado |
| R3 | FR-003 — `flora.mdmv.qzz.io` serve o web (`/`, `/tasks`) e `flora-api.mdmv.qzz.io` serve a API (`/health`, `/metrics`, `/llms.txt`) (SC-001) | T001, T004, T007 | mapa de rotas em `spec.md`/`plan.md` e `decision.md`; T004 registra a reatribuição de FQDNs e o disparo manual do Coolify; observação das rotas ao vivo permanece **[NV]** até o deploy final |
| R4 | FR-004 — `config.js` gerado em runtime de `FLORA_API_BASE_URL`, sobrescrevendo `globalThis.__FLORA_CONFIG__.apiBaseUrl` e derivando a base WS da mesma origem, sem editar os defaults comitados (SC-002) | T002, T005, T007 | `apps/frontend/docker-entrypoint.sh` renderiza `build/config.js` e `build/client/config.js` com `process.env.FLORA_API_BASE_URL` (fallback local), sem interpolação em shell e sem gravar em camada de imagem; `apps/frontend/static/config.js` segue como default versionado |
| R5 | FR-005 — `FLORA_APP__CORS_ORIGINS` inclui a origem exata `https://flora.mdmv.qzz.io`, sem `*`, preserva origens locais e é coerente com o origin check do websocket (SC-002) | T006, T007 | T006 registra a inclusão da origem web na allowlist de `flora-core/src/config/app.rs` e a coerência com `flora-api/src/routes/websocket.rs`; verificação do header `Access-Control-Allow-Origin` ao vivo fica **[NV]** até o deploy |
| R6 | FR-006 — verificar rotas, CORS e `config.js`, registrar remoto como **[NV]** quando não observado e manter o diff confinado ao deploy/ciclo (SC-003) | T007, T008 | T007 consolida o smoke esperado; T008 fecha T001..T008 e reconfirma a fronteira — `validate/` e `release/` são escritos apenas como registros documentais das fases R4/R7, não como mudança de deploy; gate, API, runtime, schema, migration e `STATE.md` fora do diff de implementação |

SC-001 (rotas web/API separadas, `/` e `/tasks` 200 na origem web) é coberta por R1/R3; SC-002 (`config.js` com `apiBaseUrl == FLORA_API_BASE_URL` e zero `localhost:3000`, ACAO refletindo a origem web) por R4/R5; SC-003 (publicação manual, nenhum required check/gate/API/runtime/schema/migration/`validate/`/`release/`/`STATE` alterado na implementação, deploy remoto **[NV]**) por R2/R6. T001 fixa baseline e causa raiz; T002 confirma que o Dockerfile do frontend não era publicado; T003 cria a publicação manual da imagem web; T004 registra os FQDNs; T005 a config de runtime; T006 o CORS; T007 a verificação; T008 o fecho documental e a fronteira. O detalhe literal está consolidado em `.specify/validations/095-frontend-deploy/validate.md`.

## [NV]

- **Deploy remoto**: configuração de DNS/FQDN, app Coolify, webhook e o resultado do deploy de `flora-web` não foram observados ao vivo nesta passagem; as rotas ainda podem servir a API até que o deploy final seja executado e confirmado.
- **Verificações esperadas**: build do frontend, `gate-integrity: PASS` e `/health 200` são evidências esperadas/documentais do handoff — não reexecutadas por este agente, conforme o modo READ-ONLY.
- **CI/PR/produção**: nenhum PR, merge, tag, required check ou promoção foi executado ou inspecionado ao vivo; gate local não substitui CI remota.
- **Reexecução nesta passagem**: gates, build, testes, formatters e comandos git não foram executados por este agente; a análise é somente leitura e a evidência vem do handoff do main agent e do estado atual do repositório.

## Resultado

**PASS — 6/6 requisitos cobertos, 3/3 critérios de sucesso, 8/8 tarefas, 0 críticos.** FR-001..FR-006, SC-001..SC-003 e T001..T008 (únicos e sequenciais) estão alinhados ao `go` de `decision.md` e ao escopo determinístico de separar a origem web da origem da API. Nenhuma tarefa nova, nenhuma divergência de contrato e nenhuma edição de fonte nesta fase; `[NV]` preservado nos artefatos do ciclo. Analyze somente leitura — gates, build, testes e formatters não foram reexecutados aqui.
