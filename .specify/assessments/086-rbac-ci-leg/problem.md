# Problem Definition: Cobertura dos contratos RBAC no job de banco

- **Slug**: 086-rbac-ci-leg
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md

## Problem Statement

Três contratos de banco de RBAC marcados com `#[ignore]` em `flora-tests/src/contract/rbac.rs` — `db_rbac_list_scoped_to_session_org`, `db_rbac_create_requires_role_write_and_uses_session_org` e `db_rbac_builtin_immutable_custom_patchable` (`decision.md:11-16`) — só rodam quando `FLORA_DATABASE__POSTGRES_URL`/`DATABASE_URL` estão definidos e, até o ciclo 086, não eram exercitados pelo job obrigatório `db-tests`. Como um `cargo test` com filtro que casa zero testes termina com exit 0 (`decision.md:17-21`; `.github/workflows/rust.yml:87-88`), a ausência de cobertura era silenciosa: autorização por papel/organização e imutabilidade de papéis embutidos podiam regredir sem sinal vermelho. Trata-se de um *confidence gap* na verificação, não de um defeito na lógica de autorização (`intake.md:27-29`). Os números registrados no ciclo são datados: o checkout corrente mostra piso 5 para `rbac` e 11 pisos/87 no total (`research.md:34-36`), contra 3 e 9/54 no artefato original.

## Affected Users & Stakeholders

- **Users**: mantenedor/guardião do gate ADR-0003, que precisa confiar que uma regressão de escopo por organização ou de papéis embutidos apareça antes do merge automático (`research.md:9-10`). [ASSUMPTION] pessoas cujas sessões dependem da autorização por papel/organização — não entrevistadas diretamente nos artefatos.
- **Stakeholders**: dono do processo de merge autônomo sob ADR-0003; autor dos contratos de teste; equipe responsável pelo job `db-tests` e pelo guarda determinístico.

## Goals

- Tornar observável, no job obrigatório de banco, qualquer regressão nos contratos de RBAC que hoje dependem de execução manual.
- Fazer com que a quantidade de contratos RBAC exercitados não possa cair sem produzir falha no gate determinístico.
- Preservar a semântica de autorização existente: o problema é de cobertura, não de comportamento (`intake.md:27-29`).
- Manter coerência entre os números registrados nos artefatos e o estado corrente do checkout (`research.md:34-36,54`), sem reescrever o registro histórico.

## Non-Goals

- Alterar a lógica de autorização por papel/organização ou a imutabilidade de papéis embutidos.
- Criar novo required check ou promover a cobertura RBAC a gate separado (`decision.md:41`).
- Modificar os próprios contratos de teste, a aplicação, ADRs, constituição, `validate/`, `release/` ou `.specify/memory/STATE.md`.
- Afirmar estado remoto de CI, PR ou merge: permanece **[NV]** nesta avaliação documental (`research.md:39`).

## Success Metrics

- Contratos `#[ignore]` de RBAC efetivamente exercitados no job obrigatório, com contagem observável e não-zero (baseline: 3 contratos fora do CI no ciclo 086; piso corrente 5 em `.github/workflows/rust.yml:119`).
- Remoção da cobertura ou queda da contagem gera falha determinística (baseline: silencioso — filtro vazio retorna exit 0, `.github/workflows/rust.yml:87-88`).
- Guarda de integridade PASS com contagem de pisos coerente com o checkout (baseline: `all 9 db contract floors intact (54 total)` registrado; checkout com 11 pisos/87 — `research.md:35`).

## Cost of Inaction

Autorização por papel e escopo por organização, além da imutabilidade de papéis embutidos, podem regredir sem sinal vermelho; a descoberta continua dependente de inspeção manual — padrão que motivou o ciclo 088 (`decision.md` 088:18-24). Registros defasados (3/9/54) induzem expectativa errada sobre a cobertura real (`research.md:47`).

## Open Questions

- [NEEDS CLARIFICATION: o piso 3 acompanhava exatamente três testes; quem decide e em qual ciclo ele é revisto quando surgir um novo contrato `#[ignore]` de RBAC — como os dois testes adicionados depois, hoje contados no piso 5?] (`intake.md:38`; `research.md:51`)
- [NEEDS CLARIFICATION: o Postgres descartável e `FLORA_DATABASE__POSTGRES_URL`/`DATABASE_URL` estão garantidos em todos os ambientes do job `db-tests`?] (`intake.md:39`)
- [NEEDS CLARIFICATION: existem testes de RBAC fora do prefixo usado na lega ou esse filtro é exaustivo?] (`intake.md:40`)
- [NEEDS CLARIFICATION: remoção ou alteração da cobertura em branches futuras deve ser tratada como regressão de gate em todos os ramos ou apenas em `dev`?] (`intake.md:41`)
- [NEEDS CLARIFICATION: a evidência local registrada (3 passed, gate PASS) foi produzida em qual ambiente/branch e precisa ser revalidada antes de qualquer merge?] (`intake.md:42`)
- [NEEDS CLARIFICATION: como reconciliar os números defasados do artefato (3/9/54) com o estado atual do checkout (5/11/87) sem reescrever o registro histórico?] (`research.md:54`)
