# Idea Research: Lega RBAC no CI

- **Slug**: 086-rbac-ci-leg
- **Created**: 2026-11-25
- **Evidence confidence (overall)**: high
- **Cobertura idiomática real**: pt-BR_en (artefatos e specs em pt-BR; nomes de testes, workflow e scripts em inglês técnico)

Observação de método: research coleta e cita; não decide, não desenha solução e não atribui veredito. Números de linha citados são do checkout da worktree isolada (base `github/dev@e88dc8e4d`); estados remotos ao vivo não foram reconsultados e são marcados `[NV]`.

## Users & Demand

- O demandante é o mantenedor/guardião do gate ADR-0003: a necessidade registrada é que os contratos DB de RBAC passem a rodar no job obrigatório `db-tests`, sem alterar a lógica de autorização em si — "fechar um *confidence gap*, não alterar a lógica de autorização" [source: .specify/assessments/086-rbac-ci-leg/intake.md:25-29] (confidence: high).
- A demanda concreta são três contratos `#[ignore]`: `db_rbac_list_scoped_to_session_org`, `db_rbac_create_requires_role_write_and_uses_session_org` e `db_rbac_builtin_immutable_custom_patchable` — identificados no artefato e localizados no código em `flora-tests/src/contract/rbac.rs:149-150,177-178,228-229` [source: .specify/assessments/086-rbac-ci-leg/decision.md:11-16; flora-tests/src/contract/rbac.rs:149-229] (confidence: high).
- A spec traduz a dor em critério observável: rodar a lega com Postgres local e observar `3 passed`; remover a linha da lega e observar zero testes [source: specs/086-rbac-ci-leg/spec.md:22-23,109-111] (confidence: high).
- O gatilho foi descoberta manual no próprio ciclo 086, não alerta automático — padrão que se repetiu em 087 e motivou 088 [source: .specify/assessments/086-rbac-ci-leg/intake.md:34; .specify/memory/STATE.md:721-747] (confidence: high).
- A ausência da lega era silenciosa porque um `cargo test` filtrado que casa zero testes termina com exit 0 [source: .specify/assessments/086-rbac-ci-leg/decision.md:17-21; .github/workflows/rust.yml:87-89] (confidence: high).

## Prior Art

- O helper `run_leg` já existia no job `db-tests` e soma os contadores `N passed` do libtest, falhando abaixo do piso; a decisão 086 registrou reuso desse helper, sem lógica nova [source: .github/workflows/rust.yml:94-108; .specify/assessments/086-rbac-ci-leg/decision.md:39; specs/086-rbac-ci-leg/plan.md:19-22] (confidence: high).
- A armadilha do filtro vazio já estava documentada no workflow desde o ciclo 037, com comment explícito e a convenção "Floors = número de testes `#[ignore]` descobertos por filtro" [source: .github/workflows/rust.yml:87-89] (confidence: high).
- O mapa `FLOORS` em `scripts/ci/check_gate_integrity.py` é o mecanismo anterior de proteção contra drift; a direção registrada foi acrescentar uma entrada, não criar outro check [source: scripts/ci/check_gate_integrity.py:42-54; .specify/assessments/086-rbac-ci-leg/decision.md:23-30] (confidence: high).
- Durante a verificação do ciclo o filtro inicial `contract::rbac::db_` casou zero testes e foi substituído por `contract::rbac::db::db_` após `cargo test -- --list` [source: .specify/memory/STATE.md:725] (confidence: high).
- Contratos DB de RBAC já tinham histórico de aprovação em R4 e release em ciclos anteriores (070/085), então a lega é uma extensão de cobertura, não criação de teste [source: docs/PROGRESSO.md:137,156,199-202] (confidence: high).

## Market & Context

- A governança local (ADR-0003) favorece reforçar um required check existente em vez de adicionar outro; o próprio scorecard enquadra a lega como "reforço de um required check" [source: .specify/assessments/086-rbac-ci-leg/decision.md:40; specs/086-rbac-ci-leg/spec.md:34-41] (confidence: high).
- O ambiente do job `db-tests` já provisiona Postgres 18 como serviço e exporta `FLORA_DATABASE__POSTGRES_URL`/`DATABASE_URL` no `env` do job [source: .github/workflows/rust.yml:51-68] (confidence: high).
- O escopo registrado é cirúrgico: duas linhas de configuração de CI mais quatro artefatos do ciclo, sem tocar `validate/`, `release/`, `STATE.md`, código de aplicação ou testes de contrato [source: .specify/assessments/086-rbac-ci-leg/decision.md:43-50; specs/086-rbac-ci-leg/spec.md:92-94] (confidence: high).
- A sequência 086 → 087 → 088 mostra o tema como reforço incremental do CI/CD, com contagens de pisos evoluindo de 9/54 para 10/58 e depois para o estado atual do checkout [source: .specify/memory/STATE.md:721-747] (confidence: high).
- A evidência de sucesso registrada no handoff (3 passed; gate PASS com 9 pisos/54) foi produzida localmente e não reexecutada pelos agentes de documentação [source: .specify/assessments/086-rbac-ci-leg/decision.md:52-58] (confidence: high).

## Data & Constraints

- Estado atual do checkout: `.github/workflows/rust.yml:119` contém `run_leg rbac contract::rbac::db::db_ 5` e `scripts/ci/check_gate_integrity.py:51` contém `"rbac": 5` — ou seja, o piso hoje é 5, não os 3 registrados no ciclo 086 [source: .github/workflows/rust.yml:119; scripts/ci/check_gate_integrity.py:51] (confidence: high).
- O `FLOORS` atual tem 11 entradas (`auth 25, messaging 5, websocket 8, search 8, notifications 6, files 7, tasks 4, workspace 3, rbac 5, bootstrap 13, hitl 3`), soma 87; a mensagem de sucesso derivada seria `all 11 db contract floors intact (87 total)`, não `9/54` [source: scripts/ci/check_gate_integrity.py:42-54,176] (confidence: high).
- Além dos três contratos originais, `rbac.rs` contém dois contratos `#[ignore]` adicionados no ciclo 122 (`db_rbac_members_list_add_assign_and_revoke_lifecycle` e `db_rbac_members_prevents_self_demotion_and_self_revocation`), o que explica a evolução do piso para 5 [source: flora-tests/src/contract/rbac.rs:505-506,611-612] (confidence: high).
- O helper falha cedo se as variáveis de banco estiverem ausentes (`: "${VAR:?...}"`), e a regex do guarda exige linhas `run_leg <label> <filter> <floor>` ancoradas [source: .github/workflows/rust.yml:96-97; scripts/ci/check_gate_integrity.py:60] (confidence: high).
- A contagem da lega depende do prefixo escolhido: `contract::rbac::db::db_` cobre os contratos `db_*`, e um contrato `#[ignore]` com outro nome ficaria fora da lega mesmo com o piso presente [source: .github/workflows/rust.yml:119; flora-tests/src/contract/rbac.rs:149-612] (confidence: high).
- Limite operacional: `db-tests` tem `timeout-minutes: 45` e executa `--ignored --test-threads=1` [source: .github/workflows/rust.yml:52,100] (confidence: high).
- Estado ao vivo de CI/PR/merge e contagens remotas: **[NV]** — não reconsultado nesta avaliação documental [source: .specify/memory/STATE.md:727] (confidence: high).

## Evidence Against the Idea

- Os números de evidência do ciclo 086 (3 passed, `all 9 db contract floors intact (54 total)`) estão defasados em relação ao checkout atual (piso 5, 11 pisos/87). Qualquer citação do artefato como estado corrente induziria a erro; a divergência é verificável nos dois arquivos de configuração [source: .specify/assessments/086-rbac-ci-leg/decision.md:52-58; .github/workflows/rust.yml:119; scripts/ci/check_gate_integrity.py:42-54] (confidence: high).
- O mecanismo original (espelho entre `FLOORS` e `run_leg`) não teria detectado o aumento de 3 para 5 contratos sem edição humana: os dois lados são mantidos à mão, como registrado no ciclo 088 [source: .specify/assessments/088-dynamic-gate-oracle/decision.md:11-16; flora-tests/src/contract/rbac.rs:505-612] (confidence: high).
- Um filtro que casa zero testes retorna exit 0; a proteção não vinha do `cargo test` em si, mas do contador `N passed` somado pelo helper — se o helper for alterado, a lega pode voltar a ser silenciosa [source: .github/workflows/rust.yml:87-89,100-107] (confidence: high).
- Não foi possível verificar de forma independente, nesta avaliação, se a execução local de 3 passed e o gate PASS ocorreram em `dev` ou em branch de feature, nem se precisam ser revalidados antes de merge [source: .specify/assessments/086-rbac-ci-leg/intake.md:42] (confidence: medium).
- Não há evidência de que a lega tenha sido promovida a required check adicional; pelo contrário, o artefato afirma explicitamente que `rbac` não entra em `REQUIRED`/`ALLOWED_JOB_IF` — ausência de promoção é coerente com ADR-0003, mas também significa que a proteção depende inteiramente do job `db-tests` permanecer required [source: .specify/assessments/086-rbac-ci-leg/decision.md:41] (confidence: high).

## Gaps & Open Questions

- [NEEDS CLARIFICATION: o piso 3 acompanhava exatamente três testes descobertos; quem decide e em qual ciclo ele deve ser revisto quando surgir um novo contrato `#[ignore]` de RBAC — como ocorreu com os dois testes do ciclo 122?] [source: .specify/assessments/086-rbac-ci-leg/intake.md:38; flora-tests/src/contract/rbac.rs:505-612] (confidence: high).
- [NEEDS CLARIFICATION: o Postgres descartável e as variáveis `FLORA_DATABASE__POSTGRES_URL`/`DATABASE_URL` estão garantidos em todos os ambientes do job `db-tests`?] [source: .specify/assessments/086-rbac-ci-leg/intake.md:39] (confidence: high).
- [NEEDS CLARIFICATION: existem testes de RBAC fora do prefixo `contract::rbac::db::db_` que deveriam entrar na lega, ou esse filtro é exaustivo?] [source: .specify/assessments/086-rbac-ci-leg/intake.md:40] (confidence: high).
- [NEEDS CLARIFICATION: remoção ou alteração da lega em branches futuras deve ser tratada como regressão de gate em todos os ramos ou apenas em `dev`?] [source: .specify/assessments/086-rbac-ci-leg/intake.md:41] (confidence: high).
- [NEEDS CLARIFICATION: a evidência local registrada (3 passed, gate PASS) foi produzida em qual ambiente/branch e precisa ser revalidada antes de qualquer merge?] [source: .specify/assessments/086-rbac-ci-leg/intake.md:42] (confidence: high).
- [NEEDS CLARIFICATION: qual é o processo para reconciliar os números defasados do artefato 086 (3/9/54) com o estado atual do checkout (5/11/87) sem reescrever o registro histórico?] [source: .specify/assessments/086-rbac-ci-leg/decision.md:52-58; scripts/ci/check_gate_integrity.py:42-54] (confidence: high).
- [ASSUMPTION] A intenção original de "piso igual ao número de testes descobertos" permanece válida após a subida para 5; não há fonte no artefato 086 que cubra os contratos adicionados depois. (confidence: low).

## Sources

- `system` | `.specify/assessments/086-rbac-ci-leg/intake.md:1-42` | lido diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `.specify/assessments/086-rbac-ci-leg/decision.md:1-58` | lido diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `specs/086-rbac-ci-leg/spec.md:9-125` e `specs/086-rbac-ci-leg/plan.md:14-60` | lidos diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `.github/workflows/rust.yml:51-121` (job `db-tests`, helper `run_leg`, leg `rbac` com piso 5) | lido diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `scripts/ci/check_gate_integrity.py:42-60,150-176` (`FLOORS`, `LEG_RE`, mensagem derivada) | lido diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `flora-tests/src/contract/rbac.rs:149-229,505-612` (contratos `#[ignore]`) | lido diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `.specify/memory/STATE.md:721-727` e `docs/PROGRESSO.md:199-202` (registro do ciclo 086) | lidos diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `.specify/assessments/088-dynamic-gate-oracle/decision.md:11-16` (natureza de espelho do guarda) | lido diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- Web: nenhum host buscado. Não há URL externa necessária para os fatos locais acima; nenhum redirect seguido, nenhuma credencial incluída.
