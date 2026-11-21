# Converge: 097 — Primeira configuração condicionada ao estado da instalação (append-only)

> Append-only. Novas entradas devem ser acrescentadas ao final; nunca reescrever,
> renumerar ou remover entradas anteriores.

## 2026-11-20 — entrada 1

- **Decisão do ciclo**: eliminar a oferta incondicional do fluxo de primeira configuração — "É uma instalação nova? Fazer primeira configuração" ("Is this a fresh install? Start First Seed Journey") é renderizado sempre no branch `{:else}` da tela de login (`apps/frontend/src/routes/(auth)/+page.svelte:238-243`), embora a garantia de bootstrap único exista somente no backend, dentro de `POST /api/v1/bootstrap` (`BOOTSTRAP_LOCK_KEY` + contagem de organizações não-sistema, `flora-api/src/routes/bootstrap.rs:28-29`, `:238-244`, `:287-291`, 409). A correção expõe o estado real da instalação por uma rota de leitura, condiciona o link a esse estado com fail-closed e mantém o guardião 409 como última linha de defesa — chamadas diretas e concorrentes continuam fail-closed com 409.
- **Evidência determinística consolidada** (branch `feature/097-first-seed-once`, baseline `696849934`), atribuída à nota de governança do ciclo, ao handoff do main agent e à leitura direta do baseline:
  - o rodapé da tela de login exibe o link sem consultar estado; em instalação configurada, `handleBootstrap()` recebe 409 mapeado para "Este workspace já foi configurado." (FR-004, FR-005, SC-001; T005, T006, T007);
  - `create_bootstrap_router()` registra somente `POST /bootstrap` — não há endpoint/flag que informe "instalação vazia" ao cliente (FR-001; T001, T004);
  - a contagem de organizações não-sistema (`id <> '00000000-0000-0000-0000-000000000000'`) é extraída para um helper compartilhado e usada tanto na rota de status quanto no guardião atômico do `POST`, sem remover `pg_advisory_xact_lock` nem o `Error::DuplicateEntry` (FR-002, SC-002; T002, T004);
  - a rota read-only responde `{ "is_fresh": boolean }`, sem dados de tenant/PII; falha de banco devolve `500` sem detalhes e o cliente trata erro como "não fresh" — fail-closed (FR-003, SC-002, SC-003; T001, T003);
  - o frontend carrega o status ao montar e renderiza o link somente com `is_fresh === true`; em `false`, `undefined` ou erro o link permanece oculto e a tela segue com Entrar/SSO, sem regressão de login local/SSO/bootstrap (FR-004, FR-005, SC-001, SC-003; T005, T006, T007, T008);
  - o contrato JSON entre endpoint e frontend é verificado, e `toggleSetup()`/`handleBootstrap()` permanecem inalterados (FR-004, SC-001; T008);
  - verificação esperada do handoff: `python3 scripts/ci/check_gate_integrity.py` **PASS** (registrado no diagnóstico, reexecutar no checkout final), `git diff --check` limpo, vitest da tela cobrindo vazio/configurado/erro e o fluxo bootstrap → sessão → `/tasks`, `npm --prefix apps/frontend run check` 0 erros e build Docker com `/`, `/config.js`, `/health` → 200; não reexecutados por este agente (FR-001..FR-006, SC-001..SC-003; T003, T007, T008, T010);
  - fronteira: `validate/` e `release/` são escritos apenas como registros documentais das fases R4/R7 deste ciclo; a implementação não tocou gate, required checks, constituição, ADRs, schema, PR/merge/tag, ambientes de produção nem `STATE.md` (SC-003; T009, T010).
- **Analyze** (`specs/097-first-seed-once/analyze.md`, READ-ONLY, 2026-11-20): `critical=0`, `requirements_covered=6/6`, `fr=6/6`, `sc=3/3`, `tasks=10/10`, `errors=[]` — nenhuma correção de fonte necessária.
- **Fronteira preservada**: `.github/workflows/ci.yml`, gate `check_gate_integrity.py`, required checks, constituição, ADRs, guardião 409 do bootstrap, PR/merge/tag e worktrees alheios intocados pela implementação. Fora de escopo: deploy, estado real da instalação em produção e o fluxo E2E autenticado.
- **[NV]**: estado real da instalação em produção, comportamento do link após deploy, E2E autenticado (bootstrap → sessão → `/tasks`, com 409 preservado), CI/PR/produção/Coolify/Forgejo ao vivo e as verificações esperadas do handoff não observados nesta passagem.
- **Sem novas tarefas**: nenhuma divergência spec/tasks a corrigir; T001..T010 permanecem no escopo de rota de status, helper de contagem compartilhado, resposta mínima/fail-closed, frontend condicional, testes dos três estados, contrato de integração, artefatos do ciclo e fecho de gates/lockfile/supply-chain/revisão.

## Resultado

**Converged — a implementação satisfaz a spec e o analyzer.** Nenhuma tarefa nova, nenhuma correção de fonte e nenhuma reexecução de gates/formatters por este agente; registro criado apenas para rastreabilidade.

converge written: 1 file; gates/formatters skipped
