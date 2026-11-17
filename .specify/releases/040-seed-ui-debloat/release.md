# Release: 040 — Seed UI debloat + real board

**Ciclo:** 040 · **Data:** 2026-11-17 · **Baseline:** `dev@c177caa46` (+ 039 `5050911`) ·
**Tipo:** mudança estrutural frontend, sem deploy/tag.

## Entregue

- `apps/web` (5.844 arquivos / 43 MB) + 2 e2e mortos removidos — debloat real do checkout
  para VPS barata, com non-regression provado nos 4 gates frontend.
- `KanbanBoard` real (controlado por props, dados da API, PATCH de status) e módulo puro
  `kanban.svelte.ts` com 6 testes no gate `server`.
- Decisão 80/20, assessment `go (scoped)`, spec/tasks, validação e auditoria author-blind.

## Evidência

`.specify/validations/040-seed-ui-debloat/validate.md` ·
`docs/research/2026-11-17-040-seed-ui-debloat.md` ·
`/tmp/opencode/dark-factory-80-20/c040-11-author-blind-audit.md`.

## Não feito (autonomia preservada)

Nenhum backend/migration/RBAC/RLS, ADR/constituição, required checks/protections,
deploy/tag/release/segredos. `apps/web` removido porque a tarefa explícita do usuário
sanciona o debloat do cliente Macro original — não como "modernização isolada".

## Follow-ups

1. H4: remover crates dead (argon2/bcrypt/dotenv/rust-i18n) — ~40–60 crates no grafo.
2. Ligar `RbacService` às rotas (decisão de escopo de autorização).
3. Converter o board em visão de time (exige visibilidade/membership) e persistir ordem
   (migration) — fora do Seed.
4. Drift documental residual: menções históricas a `apps/web` em `.specify/` e docs.
