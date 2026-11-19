# Validation: 075 — E2E de criação de tarefa

**Data**: 2026-11-19T12:10:18Z · **Baseline**: `dev@d8f647798`
**Veredito R4**: **APROVADO** (com execução runtime E2E marcada **[NV]**).

## Gates e verificações

| Gate | Resultado |
|---|---|
| `python3 scripts/ci/check_gate_integrity.py` | PASS — 4 required checks, 8 pisos/51, exit 0 |
| `cargo fmt/clippy/deny/build/test` (executado antes da edição do teste; código Rust inalterado) | exit 0; 105 passed / 0 failed |
| `bun run check` | `COMPLETED 4572 FILES 0 ERRORS 0 WARNINGS`; exit 0 |
| `bun run lint` (Prettier + ESLint) | `All matched files use Prettier code style!`; exit 0 |
| `bun run test:unit -- --run --project server` | `Tests 42 passed (42)`; exit 0 |
| `npx playwright test --list` | smoke `seed-flow.e2e.ts:5` descoberto; `Total: 4 tests in 3 files` |
| Verificador isolado `E2EChecker` | **FAIL** no arquivo dedicado → corrigido na fonte (ver `converge.md`); seletores e FR-002..FR-005 confirmados |
| `analyze.md` | 1ª: conflito FR-002/003/004 em intervalo; 2ª: `errors=[]`; 3ª após mudança de escopo: `artifacts=13, errors=[], go=true` |
| Diff final | 1 arquivo de teste alterado + artefatos 075; nenhum arquivo novo de teste |

## Escopo verificado

`apps/frontend/tests/e2e/seed-flow.e2e.ts:41-55` — guard do botão (FR-003), descrição preenchida e verificada (FR-004), card e descrição na região `To Do column` (FR-005), sessão one-shot preservada (FR-001/FR-002). Nenhuma mudança em produção, CI, ADRs, constituição, issues ou worktree 073. DB compartilhado 5432 intacto; cluster isolado 5434 desligado.

## [NV]

Execução runtime do smoke neste ambiente: setup falhou no minter (`seed.setup.ts:36`) e a reprodução isolada excedeu 180s; `npx playwright test` atingiu `exit=124`. CI do PR, Forgejo, aprovações e merge: **[NV]**.
