# Research: 009 — DB-backed Select-Org Tests, Integrated Smoke & Minimal Frontend Slice

**Date:** 2026-11-15  
**Inputs:** cycle 008 handoff, codebase audit, subagents `ses_e1e8f0341ffeS9O37ZKM946HT1`, `ses_e1e8f0341ffdZw46QTHDDb5BhM`, `ses_e1e8f0341ffcqTrfH71HTjgehq`, `ses_e1e8f0340ffe4m9OxyO9VZASaF`, `ses_e1e8f0340ffdtK4QZLcUFWjhpp`.

## Dark-factory / SDD gates (synthesis)
- Hub exige SDD ordenado e artefatos por ciclo: assessment/decision (R1) antes de specify/execução comprometida, `specs/<NNN>-<slug>/{spec,plan,tasks,checklists}` (+ converge append-only quando usado), validação R4 antes de release R7; evidência determinística autoritativa, claims remotos/NV explícitos, sem auto-merge cego. (mdmv-dark-factory `AGENTS.md`, `.specify/memory/constitution.md`, `scripts/ci/check_sdd_completeness.py`, `docs/governance/SATELLITE-ADOPTION-CHECKLIST.md`)

## DB-backed tests & CI (audit + research)
- `select-organization` depende de sessão DB por `jti` (ativa, mesmo usuário) e membership; `AuthExtractor` valida JWT apenas. Casos DB: sucesso (rotaciona/revoga/novo jti+org), 403 não-membro, 401 sessão ausente/inativa/mismatch.
- Padrão viável no repo: testes `#[ignore]` + env-gated (`FLORA_DATABASE__POSTGRES_URL`/`DATABASE_URL`), `PgPool::connect` + `run_migrations`, fixtures únicas (user/org/role/membership/session/jti + JWT assinado com `config.app.jwt_secret`), `axum::Router::oneshot`, `--test-threads=1` em CI.
- CI: manter job `build` existente e adicionar job separado `db-tests` com `services.postgres` (healthcheck), env de DB e execução explícita de testes ignorados; evitar quebrar pipeline atual.

## Integrated smoke feasibility/execution
- Probe inicial: `docker compose up -d` do projeto **no-go as-is** no ambiente por porta host `5432` ocupada (5432/6379/7700 mapeados pelo compose). Daemon/compose disponíveis.
- Execução alternativa realizada com containers efêmeros em portas não-conflitantes (`postgres:18` em 15432, valkey 16379, meilisearch 17700) + env overrides para `flora-api` (migrations no startup), seed, `scripts/dev-smoke-session.sql` e `test-flora.sh`.
- **Resultado:** smoke `select org → workspace → task` **passou (rc=0)**: select-org rotacionou sessão (200), workspaces listados, task criada (`201`/retorno com id/status). Lista pós-criação vazia esperada por semântica assignee-scoped (task criada sem assignee).
- **Compatibilidade observada:** `postgres:16-alpine` (compose) falhou na migração por `uuidv7()` inexistente; `postgres:18` funcionou (função nativa). Documentação/compose do projeto usa pg16 — risco/nuance a tratar em ciclo futuro.

## Frontend minimal slice (audit)
- Gaps: login não persiste `state`; callback não valida/envia `state` nem trata `select-organization`/rotação de token; `AuthState` sem org/workspace/persistência/guards; UI org/workspace/tasks desalinhada (payloads, listagem assignee-scoped, tipos). Menor fatia: persistir state+redirect_uri, callback com state→sessão nil-org→select-org (token rotacionado)→workspace/task mínimos com guards e mapeamento de contrato.

## Multilingual research (highlights)
- `#[sqlx::test]`/per-test DB e isolamento forte são ideais, mas exigem privilégios/contexto; env-gated/ignored + `connect`+migrate + fixtures únicas é pragmático para `sqlx 0.7`/harness atual. `connect_lazy` mascara falhas de DB em testes; preferir conexão real em jobs DB. Serializar (`--test-threads=1`) reduz contenção.
- GitHub Actions: service Postgres com healthcheck, env explícito, job separado para testes DB ignorados, cache/toolchain alinhados; manter jobs existentes intactos.
- SvelteKit: tratar servidor/BFF como guardião de tokens quando possível; para bearer client-side mínimo, preservar/validar `state`, trocar e **rotacionar/substituir** token após seleção de org, guards server/client coerentes e evitar vazar segredos. (Fontes oficiais/primárias EN/zh-CN/ja e guias reputáveis conforme subagente.)
