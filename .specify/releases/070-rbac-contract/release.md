# Release: 070 — contrato RBAC real

**Ciclo:** 070 · **Data:** 2026-11-19T03:09Z · **Baseline:** `github/dev@0c7890ef4`
**Tipo:** testes DB-gated. **Nenhuma tag, release, deploy ou produção.**

## Entregue

- `flora-tests/src/contract/rbac.rs`: stub `Ok(())` substituído por 3 contratos DB-gated —
  `db_rbac_list_scoped_to_session_org` (tenant), `db_rbac_create_requires_role_write_and_uses_session_org` (`RoleWrite` + org da sessão), `db_rbac_builtin_immutable_custom_patchable`.
- Fail-closed sem env; sensibilidade provada por teste de mutação; padrão idêntico a `workspace.rs`/`auth.rs`.
- Artefatos completos: assess (5), spec (8), validate (R4), release (R7), `STATE.md`, correções append-only do nit do verificador.

## SemVer

**Proposta:** patch `0.2.2` → `0.2.3` (cobertura de teste, sem mudança de API externa). Propostas anteriores permanecem não aplicadas. **Nenhuma tag criada.**

## Changelog (IDs)

- `FR-001` — módulo `db` fail-closed; `FR-002` — listescopo por sessão; `FR-003` — create exige `RoleWrite`/`OrgAdmin`; `FR-004` — built-ins imutáveis; `FR-005` — rastreabilidade.
- `SC-001` — 3 contratos passam com DB real e falham sem env; `SC-002` — gates verdes, pisos intactos; `SC-003` — diff restrito aos paths declarados.
- `T001`–`T006` — executados; `T006` (PR/merge/higiene) conduzido no BLOCO 7.
- Bug: nenhum em `.specify/bugs/`; origem é decisão autônoma 80/20 em `STATE.md`.

## Evidência

`.specify/validations/070-rbac-contract/validate.md` ·
`specs/070-rbac-contract/converge.md` ·
`specs/070-rbac-contract/analyze.md` ·
verificador isolado `PASS-WITH-NITS` (nit corrigido).

## Rollback

Reverter os commits do PR em `dev/` com um novo commit de reversão (sem force-push e sem reescrita de histórico). Nenhum dado, schema, required check, deploy ou estado de produção a restaurar. O banco descartável `flora_070_rbac_verify` será removido ao final desta sessão.
