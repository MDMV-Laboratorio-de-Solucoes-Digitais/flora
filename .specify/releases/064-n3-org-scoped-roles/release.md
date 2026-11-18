# Release: 064 — N3 org-scoped roles

**Ciclo:** 064 · **Data:** 2026-11-18T11:29Z · **Baseline:** `github/dev@448090327`
**Tipo:** isolamento multi-tenant + migrations; **nenhum deploy executado.**

## Entregue

- Resolução de papéis escopada por organização em repositório/serviços/rotas;
  cache de papéis valida a org.
- `UNIQUE(organization_id, id)` em `roles` e FK composta com
  `ON DELETE RESTRICT` + pré-voo fail-closed.
- Novo contrato DB cross-org; piso `workspace` 2→3 (51 testes no total).
- Notas datadas em `STATE.md`/`OBJETIVO.md`/`ROADMAP.md` (append-only).

## SemVer

**Proposta:** minor `0.1.0` → `0.2.0` (mudança de fronteira de segurança e
schema; propostas de patch 0.1.1–0.1.17 permanecem não aplicadas).
Nenhuma tag criada.

## Changelog (IDs)

- `FR-001` — 9 call sites escopados por org.
- `FR-002` — UNIQUE aditivo.
- `FR-003` — FK com `RESTRICT` e pré-voo fail-closed.
- `FR-004` — sem deploy/limpeza/ADR/C5/C6.
- `SC-001`–`SC-004` — verificados (teste, migrations, gates, auditoria local).
- `T001`–`T005` — concluídas; evidência em `validate.md`.
- Bug: nenhum reportado em `.specify/bugs/`; N3 veio da fila autorizada pelo SE.

## Evidência

`.specify/validations/064-n3-org-scoped-roles/validate.md` ·
`specs/064-n3-org-scoped-roles/converge.md` ·
`specs/064-n3-org-scoped-roles/analyze.md`.

## Rollback

`git revert` dos commits `fix(core)`/`migrate`/`ci`/`test` do ciclo. As
migrations são aditivas; reverter o código restaura o comportamento anterior.
Nenhum efeito externo (sem deploy).

## Não feito / NV

Produção não auditada nem alterada; limpeza de dados sujos depende de decisão
do SE; ADR-0003/0006, C5 (`RoleWrite`) e C6 (canário RLS) autorizados em
ciclos separados. Estado remoto/CI: **[NV]**. Sem merge/auto-merge/force-push.
