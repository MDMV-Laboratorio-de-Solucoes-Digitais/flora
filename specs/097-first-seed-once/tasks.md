# Tasks: 097 — gating do link "primeira configuração" no bootstrap one-time

**Input**: `specs/097-first-seed-once/spec.md`, `plan.md`,
`.specify/assessments/097-first-seed-once/decision.md`, diagnóstico `FirstSeedScout`

**Date**: 2026-11-21

**Format**: `[ID] Description` — T001..T010 concluídas; gate do alvo PASS
(registrado no handoff consolidado).

## Phase 1 — Predicado e status endpoint

- [X] T001 Extrair de `flora-api/src/routes/bootstrap.rs:276-290` o predicado
  reutilizável `installation_is_empty(db)`, baseado em
  `COUNT(*) FROM organizations WHERE id <> nil`, preservando exatamente o critério
  do `POST` (usuários órfãos e workspaces não alteram o resultado) e mantendo o
  `pg_advisory_xact_lock` e o 409 `bootstrap already completed` como guarda
  autoritativa (FR-001, SC-002).
- [X] T002 Registrar `GET /api/v1/bootstrap/status` em
  `create_bootstrap_router()` (`bootstrap.rs:58-59`, montado em `/api/v1` por
  `routes/mod.rs:32-33`), público e sem `Authorization`, respondendo
  `{ "available": bool }` com `Cache-Control: no-store`, sem contagens, e-mails,
  nomes ou detalhes de usuários; o GET não adquire lock nem cria dados — é somente
  UX (FR-002, SC-002).

## Phase 2 — Contratos backend

- [X] T003 Estender `flora-tests/src/contract/bootstrap.rs` com os contratos de
  status: `available: true` em banco vazio, `false` após POST 201, `false` com
  organização pré-existente, `true` com usuário órfão e zero organizações, sem
  header `Authorization`, header `Cache-Control: no-store`, preservando os testes
  de concorrência existentes (`:229`) sem alteração de semântica (FR-005, SC-002).

## Phase 3 — Frontend reativo

- [X] T004 Substituir em `apps/frontend/src/routes/(auth)/+page.svelte` o estado
  simulado `isSetup` (`:17`) por `canBootstrap = $state(false)` e buscar o status
  no mount da tela de login, sem persistir a decisão localmente (FR-003, SC-001).
- [X] T005 Condicionar o bloco "É uma instalação nova? / Fazer primeira
  configuração" (`:236-242`) a `canBootstrap === true`; erro de rede, 404, 405,
  503 ou payload inválido deve manter o link oculto (fail-closed), com
  `handleLogin()`, SSO e `isSubmitting` intactos (FR-003, FR-004, SC-001, SC-003).
- [X] T006 Ajustar `handleBootstrap()` (`:123-171`) para que o `case 409` já
  existente também defina `canBootstrap = false` e exiba "Este workspace já foi
  configurado.", sem chamar `goto('/tasks')` — tornando o gating idempotente
  também quando outra sessão configurou a instalação (FR-004, SC-003).

## Phase 4 — Specs frontend e regressão

- [X] T007 Estender
  `apps/frontend/src/routes/(auth)/login-error.svelte.spec.ts` com: status
  `false` → link ausente; status `true` → link presente; 409 no submit → link
  removido, erro visível e `goto` não chamado; erro de rede no status → link
  oculto; ajustando os testes existentes (`:101,138`) para incluírem a chamada de
  status (FR-006, SC-001, SC-003).
- [X] T008 Confirmar a não regressão: fluxo OIDC (`handleLogin`, SSO), contrato do
  `POST /api/v1/bootstrap` (201/409), advisory lock e critério de organizações
  não-sistema permanecem inalterados; o predicado extraído é a única fonte do GET
  e do POST (FR-001, FR-002, SC-002).

## Phase 5 — Fecho

- [X] T009 Executar/consolidar os gates do alvo (PASS no handoff), conferindo que
  contratos e specs frontend passam e que nenhuma credencial ou dado de usuário
  aparece em log, resposta de status ou artefato do ciclo (FR-006, SC-002, SC-003).
- [X] T010 Fechar a rastreabilidade T001..T010 em decision/spec/plan/tasks e
  confirmar que nenhum arquivo de `analyze/`, `converge/`, `validate/`,
  `release/`, `STATE.md` ou `PROGRESSO*` foi tocado; IdP externo, allowlist OIDC,
  deploy remoto e o estado real do DB de produção permanecem **[NV]** (SC-003).

## Dependencies & Execution Order

T001 → T002 → T003 → T004 → T005 → T006 → T007 → T008 → T009 → T010. T002
depende de T001; T003 de T002; T005 de T004; T006 de T005; T007 depende de T005 e
T006; T008 verifica T001–T007 em conjunto; T009 e T010 consolidam gates e
rastro documental. O `POST /bootstrap` e o fluxo OIDC não são alterados em nenhum
passo. IdP externo, configuração de OIDC e deploy remoto permanecem **[NV]**.
