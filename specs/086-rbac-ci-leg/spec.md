# Feature Specification: RBAC CI leg — fechar o confidence gap dos contratos DB

**Feature Branch**: `feature/086-rbac-ci-leg`

**Created**: 2026-11-20

**Status**: Implemented (evidência local registrada em `tasks.md`)

**Input**: 3 contratos `#[ignore]` de RBAC em `flora-tests/src/contract/rbac.rs` não eram executados pelo job `db-tests` do CI.

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Toda mudança de RBAC é verificada no CI (Priority: P1)

Como mantenedor, quero que os contratos DB de RBAC rodem no job obrigatório
`db-tests`, para que uma regressão de escopo por organização ou de
imutabilidade de papéis embutidos falhe o merge automático.

**Why this priority**: é o próprio confidence gap; sem isso os três testes
existem mas nunca são exercitados (`cargo test` com filtro vazio sai 0).

**Independent Test**: rodar apenas a lega `contract::rbac::db::db_` com Postgres
local e observar 3 passed; remover a linha da lega e observar o teste zero.

**Acceptance Scenarios**:

1. **Given** o job `db-tests` com o piso `rbac = 3`, **When** os três contratos
   DB rodam com `--ignored`, **Then** o contador `N passed` é 3 e a lega passa.
2. **Given** `FLORA_DATABASE__POSTGRES_URL` ausente, **When** a lega inicia,
   **Then** falha (fail-closed) em vez de reportar 0 testes como sucesso.

---

### User Story 2 — O piso não pode desaparecer silenciosamente (Priority: P2)

Como guardião do gate ADR-0003, quero que `check_gate_integrity.py` valide a
lega `rbac` com o mesmo piso, para que uma edição futura do workflow não
reabra o gap.

**Why this priority**: o script é a defesa determinística contra drift; sem a
entrada `rbac` o guarda não notaria a remoção da lega.

**Independent Test**: executar o script e conferir a linha
`all 9 db contract floors intact (54 total)`.

**Acceptance Scenarios**:

1. **Given** `FLOORS` com `"rbac": 3` e a lega presente, **When** o script roda,
   **Then** exit 0 e nenhuma falha `db floor for rbac`.
2. **Given** a lega removida ou com piso diferente de 3, **When** o script roda,
   **Then** falha com `rust.yml: db floor for rbac (3) missing or changed`.

---

### User Story 3 — Escopo Cirúrgico (Priority: P3)

Como revisor, quero o diff limitado a duas linhas de CI, para que o ciclo não
toque `validate/`, `release/`, `STATE` nem lógica de aplicação.

**Why this priority**: reduz risco de regressão e mantém o handoff auditável.

**Independent Test**: `git diff --name-only 9b05abb9d`.

**Acceptance Scenarios**:

1. **Given** a branch 086 sobre `9b05abb9d`, **When** o diff é listado,
   **Then** aparecem apenas `rust.yml`, `check_gate_integrity.py` e os 4
   artefatos do ciclo.

---

### Edge Cases

- `cargo test` com filtro que casa zero testes: exit 0 — mitigado pelo
  contador `N passed` + piso do helper `run_leg` (`rust.yml:90-104`).
- Variáveis de banco ausentes: helper usa `: "${VAR:?...}"` e falha cedo.
- Piso maior que 3 (novo teste não registrado): exigiria nova avaliação de
  escopo; neste ciclo o piso acompanha exatamente os 3 testes descobertos.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: O job `db-tests` DEVE executar a lega `rbac` com filtro
  `contract::rbac::db::db_` e piso mínimo de 3 testes aprovados, usando o
  helper `run_leg` existente.
- **FR-002**: `scripts/ci/check_gate_integrity.py` DEVE incluir `"rbac": 3` no
  mapa `FLOORS` e validar a presença/valor da lega no workflow via regex.
- **FR-003**: A lega DEVE exigir `FLORA_DATABASE__POSTGRES_URL` e `DATABASE_URL`
  e falhar quando ausentes (fail-closed), sem alterar o comportamento das
  outras 8 legs.
- **FR-004**: O diff DEVE ficar limitado a `.github/workflows/rust.yml`,
  `scripts/ci/check_gate_integrity.py` e aos 4 artefatos do ciclo 086;
  `validate/`, `release/` e `.specify/memory/STATE.md` NÃO DEVEM ser tocados.

### Key Entities

- **db-tests leg**: invocação `run_leg <label> <filter> <floor>` que soma os
  contadores `N passed` e exige `>= floor`.
- **FLOORS**: mapa determinístico no guarda de integridade; a soma passa de
  51 para 54 com a entrada `rbac`.
- **RBAC DB contract**: teste `#[ignore]` em `contract/rbac.rs`, habilitado
  somente na lega `db_`.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: `cargo test -p flora-tests contract::rbac::db::db_ -- --ignored
  --test-threads=1` com Postgres local reporta exatamente **3 passed**;
  ausência da lega produz 0 (regressão detectada).
- **SC-002**: `python3 scripts/ci/check_gate_integrity.py` retorna PASS com
  `all 9 db contract floors intact (54 total)` e nenhuma falha de `db floor`.
- **SC-003**: `git diff --name-only 9b05abb9d` contém apenas os 6 arquivos
  autorizados (2 de CI + 4 artefatos 086); zero arquivos em `validate/`,
  `release/` ou `STATE.md`.

## Assumptions

- Postgres descartável já é provisionado pelo job `db-tests` (as 8 legs
  existentes dependem do mesmo env).
- O piso 3 corresponde aos 3 testes `db_` descobertos em `contract/rbac.rs`;
  novos testes exigirão nova avaliação.
- Evidência de execução local e do gate é a registrada no handoff do ciclo;
  subagentes de documentação não reexecutam gates/testes.
