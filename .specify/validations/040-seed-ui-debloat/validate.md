# Validation: 040 — Seed UI debloat + real board

**Data:** 2026-11-17 · **Baseline:** `github/dev@c177caa46` (com 039 local `5050911`) ·
**Tipo:** mudança estrutural frontend, sem backend/deploy.

## Gates determinísticos

| Gate | Antes | Depois | Resultado |
|---|---|---|---|
| `bun run check` (apps/frontend) | 0 erros / 0 warnings | 0 erros / 0 warnings | PASS |
| `bun run lint` | green | green (após Prettier nos 4 arquivos novos) | PASS |
| `bun run test:unit -- --run --project server` | 10 files / 30 tests | **11 files / 36 tests** (+6 kanban) | PASS |
| `bun run build` | ok | ok | PASS |
| `cargo fmt --all -- --check` | ok | ok | PASS |
| `python3 scripts/ci/check_gate_integrity.py` | ok | ok (4 required checks + piso 43 intactos) | PASS |

## Remoção auditável

- `git diff --cached --name-status -- apps/web` → somente `D` (5.844 arquivos).
- `git diff --cached --name-status -- apps/frontend/tests/e2e` → `D files.spec.ts`,
  `D tasks.spec.ts` (arquivos mortos: `testMatch '**/*.e2e.ts'` não os executava).
- `grep -rn "apps/web\|@coparse" --include='*.rs' --include='*.toml' --include='*.yml'
  --include='*.json' .` → zero hits (só docs históricas, fora do escopo do gate).
- Prova de non-regression: os 4 gates frontend rodados antes e depois da remoção
  produziram contagens idênticas (check/lint/build ok, 30→36 testes só pelo novo teste).

## Comportamento do board

- `KanbanBoard` é controlado: `tasks` e `onStatusChange` via `$props()`; `grouped`
  derivado de `groupByStatus(tasks)` (módulo puro testável no gate `server`).
- `changeStatus()` na rota faz `PATCH /api/v1/tasks/{id}` com `{ status }` e recarrega
  via `loadTasks()`. Status canônico `Todo|InProgress|Done` (Archived omitido do board).
- Sem duplicação de estado: a rota é a única fonte; `taskStore` continua órfão (não
  importado) — decisão consciente para evitar 3 cópias com drift.

## Auditoria author-blind

Ver `/tmp/opencode/dark-factory-80-20/c040-11-author-blind-audit.md` (verificador em
contexto fresco, contrato congelado em `c040-10`, gates re-executados de forma
independente). **Veredito: NO-GO para merge "dark" pleno, sem falha dura de gate.**
Gates re-executados pelo auditor: check 0/0, lint limpo, 11 files/36 testes, build
exit 0, `check_gate_integrity.py` PASS; remoção = só `D` (5.846 entradas), zero refs.

Defeitos encontrados e disposição:

| # | Defeito | Severidade | Disposição |
|---|---|---|---|
| 1 | Caminho `changeStatus`→PATCH sem teste (só helpers puros) | MEDIUM | **Corrigido**: extraído `status-patch.svelte.ts` (helper puro) usado pela rota; +3 testes cobrindo o body exato `{"status":"Todo"\|...}`, rejeição de não-canônicos e encoding do id → **39 testes** no gate `server`. |
| 2 | Tarefas `Archived` invisíveis no board | LOW–MED | Design documentado no spec (non-goal); mantido, sinalizado ao usuário como decisão de produto. |
| 3 | DnD sem `onkeydown` (a11y) | LOW | §2 julgamento humano; fora do escopo Seed. |
| 4 | `taskStore` órfão com vocabulário legado | LOW | Decisão consciente (evita 3 cópias); documentado. |
| 5 | Drift README/AGENTS ("apps/web presente") | LOW | **Corrigido**: README:66, AGENTS:7/15 atualizados para "removido no 040". |

Gates [NV] na passagem do auditor (Rust completo/DB remoto/e2e/janela 20 PRs) não
foram alterados por este ciclo (diff só frontend + docs). Com CI remoto verde e a
decisão de produto #2, converte para GO.

## NV / fora do escopo

- Mudança de status depende de PATCH sem 400 para status inválido (mitigado pela união
  canônica no módulo puro). Ordem dos cards não persistida (created_at DESC).
- GET /tasks é assignee-scoped — board de time exigiria nova decisão de visibilidade.
- Lanes `client` e browser e2e não são gates required neste ciclo.
- Forgejo/[NV] proteções e resultado remoto do CI [NV]; merge via ADR-0003 sob 4 checks.
