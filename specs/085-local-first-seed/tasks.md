# Tasks: First Seed Journey local one-time bootstrap

**Input**: Design documents from `/specs/085-local-first-seed/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/bootstrap.md

**Tests**: incluídos porque a spec exige cobertura automatizada (SC-004) e a constituição exige TDD.

**Organization**: tarefas agrupadas por user story; cada fase é um incremento independente.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: pode rodar em paralelo (arquivos distintos, sem dependência incompleta)
- **[Story]**: US1, US2 ou US3 conforme a spec

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: confirmar âncora e ambiente descartável sem tocar em dados de outras sessões

- [X] T001 Confirmar branch `feature/085-local-first-seed` e `.specify/feature.json` apontando para `specs/085-local-first-seed` em `.specify/feature.json`
- [X] T002 [P] Criar banco descartável `flora_demo_085` e documentar env de execução em `specs/085-local-first-seed/quickstart.md`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: infraestrutura comum antes de qualquer história; nenhuma história começa antes deste checkpoint

- [X] T003 Adicionar classe de rate limit `Bootstrap` (5/60s) em `flora-api/src/middleware/rate_limit.rs` conforme `research.md` Decision 5
- [X] T004 Registrar módulo `bootstrap` no router público em `flora-api/src/routes/mod.rs` seguindo o contrato `specs/085-local-first-seed/contracts/bootstrap.md`
- [X] T005 [P] Criar módulo de teste de contrato `flora-tests/src/contract/bootstrap.rs` e registrá-lo em `flora-tests/src/contract/mod.rs` com `#[ignore = "requires Postgres"]`

**Checkpoint**: foundation ready — implementação das histórias pode começar

---

## Phase 3: User Story 1 - Criar a primeira organização e entrar (Priority: P1) 🎯 MVP

**Goal**: instalação vazia provisiona org/admin/role/membership/workspace/sessão e entra em `/tasks`

**Independent Test**: banco vazio + formulário preenchido → `/tasks` com organização já selecionada, sem token injetado

### Tests for User Story 1 (write first; must fail before implementation)

- [X] T006 [P] [US1] Escrever teste de contrato `db_bootstrap_creates_org_admin_role_membership_workspace_and_session` em `flora-tests/src/contract/bootstrap.rs`, incluindo decodificação do JWT e sessão ativa
- [X] T007 [P] [US1] Escrever teste de componente para o clique "Sprout Workspace" em `apps/frontend/src/routes/(auth)/login-error.svelte.spec.ts`, cobrindo chamada ao bootstrap e persistência via `AuthState`

### Implementation for User Story 1

- [X] T008 [US1] Implementar `POST /api/v1/bootstrap` com payload validado em `flora-api/src/routes/bootstrap.rs` (organization_name 1–255; admin_email válido e lowercase)
- [X] T009 [US1] Implementar transação atômica com `pg_advisory_xact_lock`, contagem de organizações não-sistema (excluir placeholder `system`) e inserts em `flora-api/src/routes/bootstrap.rs` conforme `data-model.md`
- [X] T010 [US1] Emitir `Session` + `FloraClaims`/JWT na mesma transação e responder 201 com `{session_token, expires_in, user_id, organization_id, workspace_id}` em `flora-api/src/routes/bootstrap.rs`
- [X] T011 [US1] Ligar o botão "Sprout Workspace" ao bootstrap e chamar `authState.login(token, user, organizationId)` antes de navegar para `/tasks` em `apps/frontend/src/routes/(auth)/+page.svelte`
- [X] T012 [US1] Registrar log de auditoria sem token em `flora-api/src/routes/bootstrap.rs`

**Checkpoint**: US1 funcional e testável isoladamente

---

## Phase 4: User Story 2 - Impedir reexecução (Priority: P1)

**Goal**: nenhuma segunda inicialização cria objetos nem substitui a primeira identidade

**Independent Test**: segunda chamada e chamada com banco não vazio retornam 409 com contagens inalteradas; duas chamadas concorrentes produzem uma única org

### Tests for User Story 2

- [X] T013 [P] [US2] Escrever teste `db_bootstrap_rejects_second_call_without_writes` em `flora-tests/src/contract/bootstrap.rs`
- [X] T014 [P] [US2] Escrever teste `db_bootstrap_rejects_when_organization_exists` em `flora-tests/src/contract/bootstrap.rs`
- [X] T015 [US2] Escrever teste `db_bootstrap_concurrent_calls_create_single_organization` em `flora-tests/src/contract/bootstrap.rs`

### Implementation for User Story 2

- [X] T016 [US2] Garantir rollback e mapeamento 409 para conflito de organização/e-mail/slug em `flora-api/src/routes/bootstrap.rs`

**Checkpoint**: US2 funcional; superfície não autenticada limitada a uma inicialização

---

## Phase 5: User Story 3 - Falha explícita e recuperável na interface (Priority: P2)

**Goal**: recusa mostra mensagem legível, mantém dados informados e evita duplo clique

**Independent Test**: forçar 409/422/500 e observar mensagem na tela sem mudança de estado de autenticação

### Tests for User Story 3

- [X] T017 [P] [US3] Estender `apps/frontend/src/routes/(auth)/login-error.svelte.spec.ts` com casos de bootstrap recusado, submissão duplicada e preservação dos campos

### Implementation for User Story 3

- [X] T018 [US3] Reutilizar/ajustar `errorMessage`, `isSubmitting` e `readErrorDetail` em `apps/frontend/src/routes/(auth)/+page.svelte` para o caminho de bootstrap sem duplicar lógica de erro

**Checkpoint**: US3 funcional sem regressão no "Sign In" OIDC

---

## Phase 6: Polish & Cross-Cutting Concerns

- [X] T019 [P] Registrar decisão de exceção Seed (bootstrap sem verificação de e-mail) em `docs/governance/` com ADR ou nota datada, conforme FR-012
- [X] T020 [P] Atualizar `docs/PROGRESSO.md` com nota datada de estado verificado e links para artefatos do ciclo 085
- [X] T021 Rodar gates: `cargo fmt`, `cargo clippy -- -D warnings`, `cargo test -p flora-tests`, `npm run check`, `npm run lint`, teste de browser e verificação real em `http://127.0.0.1:5173`, colando saídas no PR
- [X] T022 [P] Registrar teste de regressão do fluxo OIDC existente (login/callback/select-org) em `flora-tests/src/contract/auth.rs` para cobrir FR-007/SC-005
- [X] T023 [P] Atualizar `docs/architecture/` com a nova rota pública `POST /api/v1/bootstrap` quando o diagrama de API for regenerado neste ciclo

---

## Dependencies

- US1 → US2 → US3 (US2 depende da transação implementada em US1; US3 depende do contrato de US1)
- T003/T004/T005 bloqueiam T006–T018
- T019/T020 podem rodar em paralelo após T012
- T021 depende de todas as fases

## Parallel Execution Examples

- Paralelo seguro: T002 + T005 (banco e arquivo de teste distintos)
- Paralelo seguro: T006 + T007 (contrato Rust e teste de componente Svelte)
- Paralelo seguro: T013 + T014 (mesmo arquivo de teste, funções distintas — serializar edição se conflito de escrita)
- Paralelo seguro: T019 + T020 (docs distintos)

## Implementation Strategy

**MVP**: Phase 1 + Phase 2 + US1 (T001–T012) já entrega o fluxo ponta a ponta. US2 fecha a segurança; US3 cobre a UX de erro. Nenhuma fase depende de Zitadel, Valkey, Meilisearch ou RustFS.
