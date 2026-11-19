# Converge: 075 — E2E de criação de tarefa

**Data**: 2026-11-19T10:17:04Z · **Tipo**: append-only · **Baseline**: `dev@d8f647798`

## Estado convergido

- Decisão 80/20 `E=(3×5)/(2×1)=7,5` — issue #193 é lacuna real: `tests/e2e/` não tinha teste de tasks; demais T026–T032 já implementados no código (não fechados sem autorização).
- Assess (5), spec, clarify (0 perguntas), plan, checklist revisado, tasks, analyze READ-ONLY: 1ª execução apontou FR-002/003/004 em notação de intervalo; corrigido na fonte; 2ª execução `errors=[]`, `go=true`.
- Entregável: `apps/frontend/tests/e2e/tasks.e2e.ts` — seleção de org Acme com escopo por linha, guard do botão, criação com descrição, card na região `To Do column`.
- Divergência registrada: a issue pede `tasks.spec.ts`, mas `playwright.config.ts:9` exige `*.e2e.ts`; arquivo entregue com sufixo correto.
- Verificação estática: `playwright test --list` descobre o caso; `svelte-check` 0 erros; Prettier/ESLint ok; formatação corrigida na fonte.
- Gate autoritativo PASS; diagnósticos SDD da factory (`a6722a0e`) N/A, `converge` PASS.

## Intenção preservada

Nenhuma alteração em código de produção, workflows, ADRs, constituição, issues ou worktree 073. DB compartilhado em 5432 não tocado; cluster isolado em 5434 usado para a execução E2E.

## Pendente de apêndice

Resultado da execução E2E no cluster isolado (job em andamento na data deste converge). Nenhum veredito R4 será emitido sem essa saída.

## Correção de escopo e resultado E2E — 2026-11-19T10:39Z (append-only)

- O verificador isolado (`E2EChecker`) emitiu **FAIL** para o arquivo dedicado: um segundo `select-org` revoga o jti compartilhado do `storageState` (`auth.rs:387,390`), quebrando deterministicamente o smoke existente em suíte com `retries: 0`/`workers: 1`. Linhas 9–10 acima permanecem como registro da intenção original; **foram superadas por esta correção**.
- Entregável real: `apps/frontend/tests/e2e/seed-flow.e2e.ts:41-55` reforçado com guard do botão, preenchimento e verificação da descrição, e card/título/descrição na região `To Do column`. Nenhum arquivo novo de teste permanece; a suíte continua com um único consumidor da sessão.
- `spec.md`, `plan.md`, `tasks.md` e `checklists/requirements.md` atualizados na fonte; `analyze.md` reexecutado: `artifacts=13, errors=[], go=true` na 3ª execução.
- Execução E2E completa: `npx playwright test tests/e2e/tasks.e2e.ts --project=chromium` (quando o arquivo existia) atingiu `exit=124` em 600s; o setup falhou no minter de sessão (`seed.setup.ts:36`), e a reprodução isolada do minter também excedeu 180s. Cluster Postgres isolado em 5434 foi desligado; DB compartilhado em 5432 intacto; artefatos `test-results/`/`playwright-report/` removidos.
- Verificação determinística disponível: `playwright test --list` descobre o smoke; `svelte-check` 0 erros; Prettier/ESLint ok; gate do alvo PASS. A execução runtime do smoke permanece **[NV]** neste ambiente.
