# Implementation Plan: 097 — gating do link "primeira configuração" no bootstrap one-time

**Branch**: `feature/097-first-seed-once` @ `696849934` | **Date**: 2026-11-21 |
**Spec**: [spec.md](./spec.md)

**Input**: `.specify/assessments/097-first-seed-once/decision.md` (go) e diagnóstico `FirstSeedScout`

## Summary

Tornar a visibilidade do link de "primeira configuração" reativa ao estado real
da instalação: extrair o predicado `installation_is_empty(db)` do `POST
/bootstrap`, expô-lo como `GET /api/v1/bootstrap/status` público e booleano
(somente UX, `no-store`, sem dados), e condicionar o render no frontend com
fail-closed e tratamento reativo do 409. O `POST` mantém o advisory lock como
guarda autoritativa; o fluxo OIDC permanece intacto. IdP externo e deploy remoto
**[NV]**.

## Technical Context

**Root cause**: `apps/frontend/src/routes/(auth)/+page.svelte:17,174-175,236-242`
— `isSetup` é estado simulado e o link é renderizado incondicionalmente;
`flora-api/src/routes/bootstrap.rs:58-59` registra somente `POST /bootstrap`, e
`:276-290` concentra a única checagem de "fresh install" (lock + contagem de
organizações não-sistema → 409). Não há endpoint de status: `GET /bootstrap` →
405, `GET /organizations` → 401, `health` sem estado.

**Target Path**: `flora-api/src/routes/bootstrap.rs` (predicado + rota GET),
`flora-api/src/routes/mod.rs` (mount já público), `flora-tests/src/contract/bootstrap.rs`
(novos contratos de status), `apps/frontend/src/routes/(auth)/+page.svelte`
(estado/fetch/render/409) e `login-error.svelte.spec.ts` (specs).

**Testing**: contratos de status (vazio, pós-201, org pré-existente, usuário
órfão, sem `Authorization`, `no-store`, concorrência) e specs frontend (status
true/false, 409, erro de rede, testes existentes ajustados); gates do alvo (PASS
no handoff). Ambiente remoto/IdP **[NV]**.

**Constraints**: status booleano sem contagens/e-mails/nomes; GET sem lock e sem
autorização; `POST` com lock, 409 e contrato inalterados; fail-closed no frontend;
sem alterar OIDC/gates/workflows/ADRs; sem toques em `analyze/`, `converge/`,
`validate/`, `release/`, `STATE.md` ou `PROGRESSO*`.

## Steps

1. **Predicado** — extrair `installation_is_empty(db)` de `bootstrap.rs:276-290`,
   mantendo o mesmo critério e o `POST` com advisory lock (T001).
2. **Endpoint** — registrar `GET /bootstrap/status` em
   `create_bootstrap_router()` → `{ "available": bool }`, `Cache-Control: no-store`,
   público (T002).
3. **Contratos backend** — cobrir vazio / pós-201 / org pré-existente / usuário
   órfão / sem `Authorization` / `no-store` / concorrência existente (T003).
4. **Estado e fetch** — `canBootstrap = $state(false)` por default e consulta de
   status no mount do frontend (T004).
5. **Render e fail-closed** — condicionar o bloco de `:236-242` a
   `canBootstrap === true`; erro de rede/404/405/503/payload inválido → oculto
   (T005).
6. **409 reativo** — no `handleBootstrap()`, 409 → `canBootstrap = false` +
   "Este workspace já foi configurado.", sem `goto` (T006).
7. **Specs frontend** — mocks de status e 409, erro de rede, ajuste dos testes
   existentes (T007).
8. **Regressão e fecho** — OIDC e `POST` inalterados, gates do alvo PASS e
   rastreabilidade T001..T010 sem tocar na fronteira proibida (T008–T010).

## Constitution Check

Sem violações: nenhum required check, gate, contrato do `POST`, lock, critério de
organização, workflow, ADR ou constituição é alterado; nenhum dado de usuário é
exposto; o diff fica restrito ao gating e aos artefatos do ciclo 097.

## Project Structure

```text
.specify/assessments/097-first-seed-once/{intake,research,define,shape,decision}.md
specs/097-first-seed-once/
├── spec.md      # FR-001..FR-006 / SC-001..SC-003
├── plan.md      # T001..T010
└── tasks.md     # todas [X]
flora-api/src/routes/bootstrap.rs       # installation_is_empty + GET /bootstrap/status
flora-api/src/routes/mod.rs             # mount público /api/v1 (inalterado)
flora-tests/src/contract/bootstrap.rs   # contratos de status
apps/frontend/src/routes/(auth)/+page.svelte
apps/frontend/src/routes/(auth)/login-error.svelte.spec.ts
```

## Complexity Tracking

Nenhuma. São uma função extraída, um GET booleano somente leitura, uma condição
de render, um ajuste no tratamento de 409 e testes — todos verificáveis por
contrato, spec e inspeção, sem nova dependência nem migração.
