# Analyze: 119 (READ-ONLY)

**Data**: 2026-11-23 · **Baseline**: `github/dev@df60db4b8`

## Checagem cruzada (`spec.md` ↔ `plan.md` ↔ `tasks.md` ↔ GitHub `#405`..`#408`)

- **Rastreabilidade FR → Tasks → Issues R6**:
  - `FR-001` / `SC-001` → `T-001` → [#405](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/405) (`models.ts`, `utils/roles.ts`, `api/roles.ts`, `RoleStore.svelte.ts`, `tests/unit/roles.test.ts`)
  - `FR-002` / `SC-002` → `T-002` → [#406](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/406) (`RoleManager.svelte`, `routes/(app)/settings/+page.svelte`, `pt-BR.ts`)
  - `FR-003` / `SC-003` → `T-003` → [#407](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/407) (`apps/frontend/tests/e2e/seed-flow.e2e.ts` Passo 9)
  - `FR-004` / `SC-004` → `T-004` → [#408](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/408) (Briefing `auth`/`rbac`, governança, docs append-only, R4/R7 `0.11.0`, ACR e promoção)
- **Consistência de domínio (`auth`/`rbac`)**:
  - Strings de permissão em `ROLE_PERMISSION_OPTIONS` correspondem 1:1 às 27 permissões canônicas de `Permission::from_str` em `flora-core/src/models/role.rs`, excluindo `*` no formulário.
  - Nenhum segredo ou `organization_id` vem do cliente: todos os chamados passam apenas o `Bearer` da sessão ativa e o backend deriva o tenant de `auth.require_org()`.
- **Órfãos / Conflitos**: `0` requisitos órfãos; `0` tarefas órfãs; zero mudanças em schema SQL ou nos `82` pisos DB (`check_gate_integrity.py` PASS).
- **Ordem normativa**: `tasks.md` e issues `#405`..`#408` registrados com `analyze.md` somente leitura antes de fechar PR/evidência.
- **Resumo**: `fr=4/4 sc=4/4 tasks=4/4 critical=0 go=True`.
