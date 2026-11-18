# Release: 066 — RBAC RoleWrite

**Ciclo:** 066 · **Data:** 2026-11-18T14:50Z · **Baseline:** `github/dev@8d0cb64b0`
**Tipo:** API; **nenhuma tag, release ou deploy executado.**

## Entregue

- CRUD real de papéis (POST/GET/PATCH/DELETE) e assign/revoke, com
  permissão `RoleWrite` dedicada e `OrgAdmin` mantida por padrão.
- Guarda de imutabilidade para papéis `is_builtin`.
- DTOs e validação de permissões; tenant sempre da sessão.
- Notas datadas em `STATE.md`/`OBJETIVO.md`/`ROADMAP.md`.

## SemVer

**Proposta:** minor `0.1.0` → `0.2.0` (superfície de API antes no-op passa a
mutar dados; propostas 0.1.1–0.1.18 permanecem não aplicadas). Nenhuma tag criada.

## Changelog (IDs)

- `FR-001` — RoleWrite || OrgAdmin.
- `FR-002` — create real com permissões validadas.
- `FR-003` — get escopado por org.
- `FR-004` — assign/revoke via membership.
- `FR-005` — builtins imutáveis.
- `FR-006` — autenticação/tenant da sessão.
- `SC-001`–`SC-004` — verificados (testes, gates, CI, escopo).
- `T001`–`T004` — concluídas; evidência em `validate.md`.
- Bug: nenhum em `.specify/bugs/`; C5 veio do registro de escalonamento.

## Evidência

`.specify/validations/066-rbac-role-write/validate.md` ·
`specs/066-rbac-role-write/converge.md` ·
`specs/066-rbac-role-write/analyze.md`.

## Rollback

`git revert` dos commits `feat(api): implementa CRUD de papeis com RoleWrite` e
`fix(api): corrige lints do CRUD de papeis`. Nenhuma migration/deploy envolvido.

## Não feito / NV

RLS (C6) segue como canário **sem produção**; ADRs/constituição inalterados.
Estado remoto/CI: **[NV]**. Sem merge/auto-merge/force-push nesta fase.
