# Problem Definition: Cobertura dos contratos de bootstrap no job de banco

- **Slug**: 087-bootstrap-ci-leg
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md

## Problem Statement

Quatro contratos de banco em `flora-tests/src/contract/bootstrap.rs` — `db_bootstrap_creates_org_admin_role_membership_workspace_and_session`, `db_bootstrap_rejects_second_call_without_writes`, `db_bootstrap_rejects_when_organization_exists` e `db_bootstrap_concurrent_calls_create_single_organization` (`decision.md:11-18`) — estão marcados com `#[ignore]` e só rodam com `FLORA_DATABASE__POSTGRES_URL`/`DATABASE_URL`. Até o ciclo 087, o job obrigatório `db-tests` não exercitava nenhum deles, e um `cargo test` com filtro que casa zero testes termina com exit 0 (`decision.md:21-27`; `.github/workflows/rust.yml:83-84`), tornando a lacuna silenciosa. O provisionamento atômico de organização, admin, papel, membership, workspace e sessão, a rejeição de segunda chamada e a concorrência podiam regredir sem sinal vermelho. A escolha de prefixo era fragilidade adicional: o filtro longo `...::db_bootstrap_` casava zero testes e não satisfazia a regex do guarda (`decision.md:33-36`). No checkout corrente o piso é 13, não os 4 do ciclo (`research.md:34-35`).

## Affected Users & Stakeholders

- **Users**: mantenedor/guardião do gate ADR-0003, que precisa que falhas de provisionamento, idempotência e concorrência no bootstrap falhem o merge automático (`research.md:9-10`). [ASSUMPTION] pessoas que dependem da criação inicial de organização e acesso — não entrevistadas diretamente nos artefatos.
- **Stakeholders**: dono do processo de merge autônomo; autor dos contratos de bootstrap; equipe responsável pelo job `db-tests` e pelo guarda de integridade.

## Goals

- Fazer com que regressões de provisionamento atômico, rejeição de segunda chamada e concorrência sejam detectáveis no job obrigatório de banco.
- Proteger a contagem de contratos de bootstrap contra remoção ou drift silencioso no guarda determinístico.
- Preservar a cobertura já registrada para RBAC, tratada como piso anterior no mesmo mecanismo (`decision.md:29-39`).
- Distinguir o que foi verificado no ciclo do que mudou depois — o piso corrente (13) inclui testes `db_startup_password_*`/`db_bootstrap_status_*` não descritos no artefato (`research.md:35,60`).

## Non-Goals

- Alterar a lógica de provisionamento atômico, idempotência ou concorrência do bootstrap.
- Criar required check adicional ou promover a cobertura a gate separado (`decision.md:41`).
- Modificar os contratos de teste em si, o código de aplicação, ADRs, constituição, `validate/`, `release/` ou `STATE.md`.
- Afirmar determinismo do cenário concorrente no Postgres do CI sem medição — o artefato apenas o listou como risco coberto (`research.md:43`); estado remoto **[NV]**.

## Success Metrics

- Contratos `#[ignore]` de bootstrap exercitados no job obrigatório, com contagem observável (baseline: 4 contratos fora do CI no ciclo 087; piso corrente 13 em `.github/workflows/rust.yml:120`).
- Mudança ou remoção da contagem produz falha determinística (baseline: ausência silenciosa, filtro vazio com exit 0 — `decision.md:21-24`).
- Guarda de integridade PASS com a contagem de pisos coerente com o checkout (baseline: `all 10 db contract floors intact (58 total)` registrado; checkout com 11 pisos/87 — `research.md:36`).
- Cobertura anterior de RBAC preservada no mesmo mapa (baseline: `"rbac": 3` no snapshot do ciclo — `decision.md:29-39`).

## Cost of Inaction

O provisionamento de organização/admin/papel/membership/workspace/sessão, a idempotência e a concorrência do bootstrap podem regredir sem sinal; a repetição do padrão de 086 mostrou que a descoberta depende de inspeção humana (`intake.md:34`). Registros com números antigos (4/10/58) criam expectativa inconsistente com o checkout (13/11/87) e podem mascarar quantos contratos estão realmente cobertos (`research.md:47`).

## Open Questions

- [NEEDS CLARIFICATION: o piso 4 acompanhava exatamente os quatro testes `db_bootstrap_*`; quem decide e em qual ciclo ele muda quando novos contratos aparecem — como os `db_startup_password_*`/`db_bootstrap_status_*` hoje contados no piso 13?] (`intake.md:38`; `research.md:60`)
- [NEEDS CLARIFICATION: a escolha do filtro `contract::bootstrap::tests::db_` em vez de `...::db_bootstrap_` foi ditada pela regex do guarda; essa regex e seus limites estão documentados fora do próprio script?] (`intake.md:39`)
- [NEEDS CLARIFICATION: os quatro cenários originais, inclusive chamadas concorrentes, são determinísticos no Postgres descartável do CI?] (`intake.md:40`)
- [NEEDS CLARIFICATION: a adição do décimo piso exige atualização de documentação de estado ou outros artefatos além do diff registrado?] (`intake.md:41`)
- [NEEDS CLARIFICATION: se o job `db-tests` rodar em outra branch que não `dev`, o piso deve ser o mesmo ou pode divergir?] (`intake.md:42`)
- [NEEDS CLARIFICATION: como reconciliar as linhas citadas no artefato (`:138`, `:188`, `:211`, `:229`) com as linhas correntes (`:210`, `:272`, `:295`, `:313`) sem reescrever o registro histórico?] (`research.md:61`)
