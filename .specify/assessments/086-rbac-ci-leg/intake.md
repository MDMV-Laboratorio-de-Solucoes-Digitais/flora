# Idea Intake: Lega RBAC no CI

- **Slug**: 086-rbac-ci-leg
- **Created**: 2026-11-20
- **Source**: repositório local — `.specify/assessments/086-rbac-ci-leg/decision.md` (+ `specs/086-rbac-ci-leg/spec.md`)
- **Type**: improvement

## Idea (as captured)

Três contratos DB de RBAC em `flora-tests/src/contract/rbac.rs` —
`db_rbac_list_scoped_to_session_org` (:144),
`db_rbac_create_requires_role_write_and_uses_session_org` (:172) e
`db_rbac_builtin_immutable_custom_patchable` (:223) — estão marcados com
`#[ignore]` e só rodam quando `FLORA_DATABASE__POSTGRES_URL`/`DATABASE_URL`
estão definidos (`decision.md:11-16`). Até este ciclo o job `db-tests` de
`.github/workflows/rust.yml` não tinha lega `rbac`; um `cargo test` filtrado
que casa zero testes termina com exit 0 (`decision.md:17-21`; `rust.yml:87-88`,
cycle 037), de modo que autorização por papel/organização e imutabilidade de
papéis embutidos podiam regredir sem sinal vermelho. O artefato registra como
direção acrescentar a lega `rbac` ao job e o piso correspondente ao guarda de
integridade (`decision.md:23-30`; `specs/086-rbac-ci-leg/spec.md:9,13-17`).

## Restated

Propõe-se executar no job obrigatório `db-tests` os contratos de banco de RBAC
que hoje ficam fora do CI e registrar o piso dessa lega no guarda determinístico,
para que regressões de escopo por organização e de imutabilidade de papéis
deixem de passar despercebidas. Trata-se de fechar um *confidence gap*, não de
alterar a lógica de autorização em si.

## Origin & Context

- **Raised by**: Engenheiro de Software / maintainer da MDMV (`@lfgranja`), conforme registrado nos artefatos do ciclo.
- **Trigger**: descoberta manual, no ciclo 086, de que os três contratos `#[ignore]` de `rbac.rs` nunca eram exercitados no CI, somada à armadilha do filtro vazio já documentada em `.github/workflows/rust.yml:87-88` (cycle 037) (`decision.md:16-21`).

## First-Glance Unknowns

- [NEEDS CLARIFICATION: o piso 3 acompanha exatamente os três testes descobertos — quem decide e em qual ciclo ele deve ser revisto se surgir um novo contrato `#[ignore]` de RBAC?]
- [NEEDS CLARIFICATION: o Postgres descartável e as variáveis `FLORA_DATABASE__POSTGRES_URL`/`DATABASE_URL` estão garantidos em todos os ambientes do job `db-tests`?]
- [NEEDS CLARIFICATION: existem testes de RBAC fora do prefixo `contract::rbac::db::db_` que também deveriam entrar na lega, ou esse filtro é exaustivo?]
- [NEEDS CLARIFICATION: a remoção ou alteração da lega em branches futuras deve ser tratada como regressão de gate em todos os ramos ou apenas em `dev`?]
- [NEEDS CLARIFICATION: a evidência local registrada (3 passed, gate PASS) foi produzida em qual ambiente/branch e precisa ser revalidada antes de qualquer merge?]
