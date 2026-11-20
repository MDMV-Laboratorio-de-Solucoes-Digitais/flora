# Converge: 095 — Deploy do frontend Flora (append-only)

> Append-only. Novas entradas devem ser acrescentadas ao final; nunca reescrever,
> renumerar ou remover entradas anteriores.

## 2026-11-20 — entrada 1

- **Decisão do ciclo**: corrigir a causa raiz determinística do 404 em `/` e `/tasks`: o FQDN `flora.mdmv.qzz.io` servia a imagem da API Rust construída pelo `Dockerfile` da raiz (`context: .` em `docker-staging.yml`), cujas rotas são `/health`, `/metrics` e `/llms.txt`. A correção publica o frontend como imagem `flora-web` própria a partir de `apps/frontend/Dockerfile`, mantém a API em `flora-api.mdmv.qzz.io`, gera `config.js` em runtime de `FLORA_API_BASE_URL`, inclui a origem web exata em `FLORA_APP__CORS_ORIGINS` e mantém a publicação manual por `workflow_dispatch`, fora dos gates de merge (ADR-0003). Verdict `go` e cadeia de evidência em `.specify/assessments/095-frontend-deploy/decision.md`.
- **Evidência determinística consolidada** (branch `feature/095-frontend-deploy`, baseline `29b34aefe`), atribuída ao handoff do main agent e ao estado do repositório:
  - o workflow `.github/workflows/docker-staging.yml` publica `context: .` com o `Dockerfile` da raiz (`flora-api`): por isso as rotas da API apareciam no FQDN e `/`/`/tasks` retornavam 404; `apps/frontend/Dockerfile` não era publicado por nenhum workflow (FR-001, SC-001; T001, T002);
  - `.github/workflows/docker-frontend.yml` usa `context: apps/frontend` e `file: apps/frontend/Dockerfile`, tem apenas `on: workflow_dispatch`, publica `flora-web` no GHCR nas tags `flora-web:production` e `flora-web:${{ github.sha }}` e dispara o webhook Coolify somente quando `HAS_HOOK == 'true'`; `apps/frontend/Dockerfile` usa `node:22-alpine` (exigência do SvelteKit 3) — tag e runtime **confirmados no estado final do branch**; nenhum trigger automático de push/PR/merge e nenhum required check novo (FR-002, SC-003; T003);
  - mapa de origens registrado: `flora.mdmv.qzz.io` → web (`/`, `/tasks`), `flora-api.mdmv.qzz.io` → API (`/health`, `/metrics`, `/llms.txt`), com deploy/FQDN remotos fora do repositório e **[NV]** até validação final (FR-003, SC-001; T004, T007);
  - `apps/frontend/docker-entrypoint.sh` renderiza `build/config.js` e `build/client/config.js` no start do container a partir de `process.env.FLORA_API_BASE_URL`, sem interpolação em shell, sem gravar em camada de imagem e sem alterar os defaults comitados em `apps/frontend/static/config.js` (FR-004, SC-002; T002, T005);
  - origem exata `https://flora.mdmv.qzz.io` registrada em `FLORA_APP__CORS_ORIGINS`, sem `*`, preservando as origens locais e a coerência com o origin check do websocket (FR-005, SC-002; T006);
  - verificação esperada do handoff: build do frontend, Dockerfile/workflow de `flora-web`, `python3 scripts/ci/check_gate_integrity.py` → **PASS** (`exit=0`) e `/health` → **HTTP 200** na origem da API; não reexecutados por este agente (FR-001, FR-006, SC-003; T007);
  - fronteira: `validate/` e `release/` são escritos apenas como registros documentais das fases R4/R7 deste ciclo; a implementação não tocou gate, required checks, constituição, ADRs, API, runtime, schema, migration, `STATE.md`, PR/merge/tag/deploy (FR-006, SC-003; T008).
- **Analyze** (`specs/095-frontend-deploy/analyze.md`, READ-ONLY, 2026-11-20): `critical=0`, `requirements_covered=6/6`, `fr=6/6`, `sc=3/3`, `tasks=8/8`, `errors=[]` — nenhuma correção de fonte necessária.
- **Fronteira preservada**: `.github/workflows/ci.yml`, `rust.yml`, gate, required checks, constituição, ADRs, código da API, schema, migration, `STATE.md` e worktrees alheiros intocados pela implementação. Fora de escopo: PR/merge/tag/promoção e a configuração ao vivo do Coolify/DNS.
- **[NV]**: deploy remoto, FQDN/DNS, rotas ao vivo, header CORS e `config.js` servido não observados nesta passagem; build/gate/`/health` são evidências esperadas do handoff, não reexecuções.
- **Sem novas tarefas**: nenhuma divergência spec/tasks a corrigir; T001..T008 permanecem no escopo de diagnóstico, imagem web, publicação manual, origens, config de runtime, CORS, verificação e fecho documental.

## Resultado

**Converged — a implementação satisfaz a spec e o analyzer.** Nenhuma tarefa nova, nenhuma correção de fonte e nenhuma reexecução de gates/formatters por este agente; registro criado apenas para rastreabilidade.

converge written: 1 file; gates/formatters skipped
