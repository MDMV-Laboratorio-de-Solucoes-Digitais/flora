# Idea Intake: Lega bootstrap no CI

- **Slug**: 087-bootstrap-ci-leg
- **Created**: 2026-11-20
- **Source**: repositório local — `.specify/assessments/087-bootstrap-ci-leg/decision.md` (+ `specs/087-bootstrap-ci-leg/spec.md`)
- **Type**: improvement

## Idea (as captured)

Quatro contratos DB em `flora-tests/src/contract/bootstrap.rs` —
`db_bootstrap_creates_org_admin_role_membership_workspace_and_session` (:138),
`db_bootstrap_rejects_second_call_without_writes` (:188),
`db_bootstrap_rejects_when_organization_exists` (:211) e
`db_bootstrap_concurrent_calls_create_single_organization` (:229) — estão
`#[ignore]` e só rodam com `FLORA_DATABASE__POSTGRES_URL`/`DATABASE_URL`
(`decision.md:11-21`). O job `db-tests` não tinha lega `bootstrap` e, como um
`cargo test` com filtro vazio sai 0 (`decision.md:21-27`; `rust.yml:83-84`),
provisionamento atômico de organização/admin/papel/membership/workspace/sessão,
rejeição de segunda chamada e concorrência podiam regredir sem sinal. O artefato
registra como direção incluir a lega com filtro `db_` — o filtro longo
`...::db_bootstrap_` não casava a regex do guarda — e um novo piso no mapa
`FLOORS` (`decision.md:29-39`; `specs/087-bootstrap-ci-leg/spec.md:9,13-17`).

## Restated

Propõe-se exercitar no CI os quatro contratos de bootstrap e proteger o piso
dessa lega no guarda de integridade, preservando o piso `rbac` do ciclo anterior,
para que falhas de provisionamento atômico, idempotência e concorrência no
bootstrap falhem o merge automático.

## Origin & Context

- **Raised by**: Engenheiro de Software / maintainer da MDMV (`@lfgranja`), conforme registrado nos artefatos do ciclo.
- **Trigger**: repetição do padrão visto em 086 — quatro contratos `#[ignore]` em `bootstrap.rs` fora do CI, descobertos manualmente (`decision.md:11-27`) — após o mesmo *confidence gap* ter sido identificado em RBAC.

## First-Glance Unknowns

- [NEEDS CLARIFICATION: o piso 4 acompanha exatamente os quatro testes `db_bootstrap_*` — quem decide e em qual ciclo ele muda se um novo contrato de bootstrap aparecer?]
- [NEEDS CLARIFICATION: a escolha do filtro `contract::bootstrap::tests::db_` em vez de `...::db_bootstrap_` foi ditada pela regex do guarda (linha 118); essa regex e seus limites estão documentados fora do próprio script?]
- [NEEDS CLARIFICATION: os quatro cenários, inclusive o de chamadas concorrentes, são determinísticos no Postgres descartável do CI?]
- [NEEDS CLARIFICATION: a adição do décimo piso (10 entradas, 58 no total) exige atualização de documentação de estado ou de outros artefatos além do diff de seis arquivos?]
- [NEEDS CLARIFICATION: se o job `db-tests` rodar em outra branch que não `dev`, a lega `bootstrap` deve manter o mesmo piso ou pode divergir?]
