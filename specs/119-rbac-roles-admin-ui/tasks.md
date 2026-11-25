# Tasks: 119 — UI CRUD de administração de papéis RBAC em `/settings`

**Data**: 2026-11-23 · **Spec**: `specs/119-rbac-roles-admin-ui/spec.md`

- T-001 (Issue #405): Criar `RoleItem` (`models.ts`), utilitários puros (`utils/roles.ts`), cliente API `GET/POST/PATCH/DELETE` (`api/roles.ts`), `RoleStore.svelte.ts` com deduplicação e `syncContext` e suíte unitária em `tests/unit/roles.test.ts` [FR-001 / SC-001]
- T-002 (Issue #406): Implementar `RoleManager.svelte` (listagem, atualização, criação, edição, exclusão, bloqueio visual em nativos, prevenção de loop `$effect`, mapeamento `403`/`409` em `pt-BR`), chaves `pt-BR.ts` e integração em `routes/(app)/settings/+page.svelte` [FR-002 / SC-002]
- T-003 (Issue #407): Expandir `apps/frontend/tests/e2e/seed-flow.e2e.ts` com Passo 9 testando `/settings` em tema escuro e claro, persistência pós-`reload()`, criação/edição/exclusão real de papel e auditoria WCAG 2.2 AA [FR-003 / SC-003]
- T-004 (Issue #408): Preencher Briefing de Compreensão (`auth`/`rbac`), governança, docs append-only do ciclo 119, R4 (`test-roadmap.md`, `evidence.md` com capturas Gist, `validate.md`), R7 (`0.11.0` MINOR), ACR independente e promoção `dev -> main` [FR-004 / SC-004]
