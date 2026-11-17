# Release: 042 — Real RBAC role listing

**Ciclo:** 042 · **Data:** 2026-11-17 · **Baseline:** `dev@411b379a` ·
**Tipo:** primeira funcionalidade RBAC real, sem deploy/tag.

## Entregue

- `GET /api/v1/roles` real: tenancy da sessão, gate de membership + `OrgRead|OrgAdmin`,
  DTO sem vazamento de tenancy, `[]` para org vazia.
- Helper puro `can_list_roles` + 3 testes no gate `cargo test --workspace`.
- F-1 do ciclo 041 resolvido como falso positivo por concorrência (fresh check 330 crates OK).
- Decisão 80/20, assessment `go (scoped)`, spec/tasks, validação, research doc.

## Evidência

`.specify/validations/042-real-rbac-list/validate.md` ·
`docs/research/2026-11-17-042-real-rbac.md`.

## Não feito (autonomia preservada)

Nenhum assign/revoke/create/get-by-id, PermissionService no AppState, RLS/role change,
ADR/constituição, required checks, deploy/tag/release/segredos. RLS segue `[NV]`.

## Follow-ups

1. Decisão do dono: semântica de assign/revoke (revoke não implementado), proteção do último Owner, DTO de criação.
2. Role least-privilege + policies RLS + FORCE — autorização do SE.
3. Teste de contrato DB-gated de isolamento A/B (exige helper de estado compartilhado).
