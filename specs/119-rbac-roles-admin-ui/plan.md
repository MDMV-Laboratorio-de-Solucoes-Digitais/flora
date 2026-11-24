# Plan: 119 — UI CRUD de administração de papéis RBAC em `/settings`

**Data**: 2026-11-23 · **Baseline**: `github/dev@df60db4b8`

## Mapeamento FR/SC → Arquivos e Verificação

| FR / SC | Arquivos alvo | Verificação |
|---|---|---|
| FR-001 / SC-001 | `apps/frontend/src/lib/types/models.ts`, `src/lib/utils/roles.ts`, `src/lib/api/roles.ts`, `src/lib/state/RoleStore.svelte.ts`, `tests/unit/roles.test.ts` | `bun run test:unit -- --run --project server` (`9` casos em `roles.test.ts`; `80` totais) |
| FR-002 / SC-002 | `apps/frontend/src/lib/components/rbac/RoleManager.svelte`, `src/routes/(app)/settings/+page.svelte`, `src/routes/layout.css`, `src/lib/i18n/pt-BR.ts` | `bun run check && bun run lint && bun run build` |
| FR-003 / SC-003 | `apps/frontend/tests/e2e/seed-flow.e2e.ts` | `bun run test:e2e` contra PG 18 (`4 passed`, `14/14` WCAG 2.2 AA `sem achados` em `.dark` e claro + PNGs) |
| FR-004 / SC-004 | `specs/119-*`, `.specify/{assessments,validations,releases}/119-*`, `docs/governance/2026-11-23-119-*.md`, `STATE.md`, `PROGRESSO.md`, `OBJETIVO.md`, `ROADMAP.md` | `python3 scripts/ci/check_gate_integrity.py`, `git diff --check`, ACR independente e CI remoto verde |

## Ordem de Execução

1. Especificar e planejar em `specs/119-*` e `.specify/assessments/119-*`; criar issues R6 para `T-001`..`T-004` e registrar `tasks.md` antes de `analyze.md` (read-only).
2. Validar utilitários e `RoleStore` via TDD em `tests/unit/roles.test.ts`, corrigindo deduplicação/contexto e cobertura de edição/validação.
3. Completar `RoleManager.svelte` (listagem, criação, edição, exclusão, `403`/`409`, carga única por contexto), ligar em `/settings` e adicionar Passo 9 em `seed-flow.e2e.ts`.
4. Executar todos os gates locais e Playwright real com capturas via Gist, registrar R4 (`validate.md` + `evidence.md`), R7 (`0.11.0`) e Briefing `auth`/`rbac`, abrir PR para `dev`, passar ACR + CI GREEN, mesclar em `dev` e abrir PR de promoção `release/119-promotion -> main`.
