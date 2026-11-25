# Decision: Contrato RBAC real no lugar do stub

- **Slug**: 070-rbac-contract
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | define.md | shape.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `flora-tests/src/contract/rbac.rs:3-7` é um teste que só retorna `Ok(())`; uma regressão de autorização multi-tenant passaria despercebida (`intake.md:7`, `define.md:5`). |
| Evidence strength | strong | Handlers reais em `flora-api/src/routes/rbac.rs:168-320`, regras em `:86-123`, padrão canônico em `workspace.rs:5-7,38-52,134-215` e DB descartável executado nesta sessão (`research.md:5-14`). |
| Value vs. inaction | strong | Stub vazio não detecta vazamento entre tenants, criação sem `RoleWrite` nem mutação de role built-in (`research.md:15-21`). |
| Feasibility / appetite | strong | Substituição de 7 linhas por três contratos DB-gated seguindo padrão existente; apetite small (`shape.md:5-12`). |
| Strategic fit | adequate | Mudança restrita a testes, sem comportamento de produção; emenda constitucional segue escalada `[NV]` (`intake.md:20`). |
| Risk posture | strong | Zero alteração em `flora-api/`, `flora-core/`, migrations ou gates; piso de `db-tests` e nomes de jobs intactos (`shape.md:18`, `decision anterior:11-13`). |

## Verdict & Rationale

Veredito histórico `go (scoped)` de 2026-11-19 preservado como `go`: nenhum critério é `weak`/`unknown`, não há contradição nos artefatos e a evidência inclui execução real com Postgres descartável (`flora_070_rbac_verify`). A lacuna é de cobertura de contrato, não de implementação — os handlers existem desde o ciclo 066. CI remota, Forgejo, produção e aprovações permanecem `[NV]`.

## If needs-clarification

- **Blocking questions**: N/A — nenhuma.
- **Revisit stage**: N/A

## If go — Handoff to `/speckit.specify`

- **Problem**: o único contrato de `/roles` é uma asserção vazia, incapaz de detectar regressões de autorização multi-tenant.
- **Chosen approach**: substituir o stub por módulo `#[cfg(test)] mod db` com `db_state()` fail-closed e três contratos `db_rbac_*` (`shape.md:5-12`).
- **In scope / out of scope**: dentro = `flora-tests/src/contract/rbac.rs`, artefatos do ciclo e `STATE.md`; fora = handlers, `flora-api/`, `flora-core/`, migrations, config, workflows, gates, required checks, ADRs, constituição, contratos assign/revoke e execução em produção (`intake.md:22-26`, `define.md:21-23`).
- **Success metrics**: SC-001 `cargo test -p flora-tests contract::rbac::db_ -- --ignored --test-threads=1` com Postgres descartável passa e sem env falha fechada; SC-002 gates do alvo verdes e contagem de legs `db-tests` inalterada; SC-003 diff restrito ao contrato + artefatos + `STATE.md` (`define.md:15-19`).
- **Carried-forward open questions**: nenhuma bloqueante; `(assign/revoke)` ficam para ciclo seguinte; CI futura/Forgejo/produção/aprovações `[NV]`.
