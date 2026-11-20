# Feature Specification: Bootstrap CI leg — fechar o confidence gap dos contratos DB

**Feature Branch**: `feature/087-bootstrap-ci-leg`

**Created**: 2026-11-20

**Status**: Implemented (evidência local registrada em `tasks.md`)

**Input**: 4 contratos `#[ignore]` de bootstrap em `flora-tests/src/contract/bootstrap.rs` não eram executados pelo job `db-tests` do CI.

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Toda mudança de bootstrap é verificada no CI (Priority: P1)

Como mantenedor, quero que os contratos DB de bootstrap rodem no job
obrigatório `db-tests`, para que uma regressão de provisionamento atômico,
idempotência ou concorrência falhe o merge automático.

**Why this priority**: é o próprio confidence gap; sem isso os quatro testes
existem mas nunca são exercitados (`cargo test` com filtro vazio sai 0).

**Independent Test**: rodar apenas a lega `contract::bootstrap::tests::db_`
com Postgres local e observar 4 passed; remover a linha da lega e observar 0.

**Acceptance Scenarios**:

1. **Given** o job `db-tests` com o piso `bootstrap = 4`, **When** os quatro
   contratos DB rodam com `--ignored`, **Then** o contador `N passed` é 4 e a
   lega passa.
2. **Given** `FLORA_DATABASE__POSTGRES_URL` ausente, **When** a lega inicia,
   **Then** falha (fail-closed) em vez de reportar 0 testes como sucesso.

---

### User Story 2 — O piso não pode desaparecer silenciosamente (Priority: P2)

Como guardião do gate ADR-0003, quero que `check_gate_integrity.py` valide a
lega `bootstrap` com o mesmo piso, para que uma edição futura do workflow não
reabra o gap.

**Why this priority**: o script é a defesa determinística contra drift; sem a
entrada `bootstrap` o guarda não notaria a remoção da lega.

**Independent Test**: executar o script e conferir a linha
`all 10 db contract floors intact (58 total)`.

**Acceptance Scenarios**:

1. **Given** `FLOORS` com `"bootstrap": 4` (e `"rbac": 3` preservado) e a lega
   presente, **When** o script roda, **Then** exit 0 e nenhuma falha
   `db floor for bootstrap`.
2. **Given** a lega removida ou com piso diferente de 4, **When** o script
   roda, **Then** falha com
   `rust.yml: db floor for bootstrap (4) missing or changed`.

---

### User Story 3 — Escopo Cirúrgico (Priority: P3)

Como revisor, quero o diff limitado a duas linhas de CI, para que o ciclo não
toque `validate/`, `release/`, `STATE` nem lógica de aplicação.

**Why this priority**: reduz risco de regressão e mantém o handoff auditável.

**Independent Test**: `git diff --name-only 367a50e79`.

**Acceptance Scenarios**:

1. **Given** a branch 087 sobre `367a50e79`, **When** o diff é listado,
   **Then** aparecem apenas `rust.yml`, `check_gate_integrity.py` e os 4
   artefatos do ciclo.

---

### Edge Cases

- `cargo test` com filtro que casa zero testes: exit 0 — mitigado pelo
  contador `N passed` + piso do helper `run_leg` (`rust.yml:90-105`).
- Variáveis de banco ausentes: helper usa `: "${VAR:?...}"` e falha cedo.
- Filtro `...::db_bootstrap_` não casa a regex do guarda (linha 118): usa-se
  `contract::bootstrap::tests::db_`, que cobre os mesmos 4 testes e é aceito
  pelo guarda sem relaxar a regex.
- Piso maior que 4 (novo teste não registrado): exigiria nova avaliação de
  escopo; neste ciclo o piso acompanha exatamente os 4 testes descobertos.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: O job `db-tests` DEVE executar a lega `bootstrap` com filtro
  `contract::bootstrap::tests::db_` e piso mínimo de 4 testes aprovados,
  usando o helper `run_leg` existente.
- **FR-002**: `scripts/ci/check_gate_integrity.py` DEVE incluir
  `"bootstrap": 4` no mapa `FLOORS` (mantendo `"rbac": 3`) e validar a
  presença/valor da lega no workflow via regex existente.
- **FR-003**: A lega DEVE exigir `FLORA_DATABASE__POSTGRES_URL` e `DATABASE_URL`
  e falhar quando ausentes (fail-closed), sem alterar o comportamento das
  outras 9 legs.
- **FR-004**: O diff DEVE ficar limitado a `.github/workflows/rust.yml`,
  `scripts/ci/check_gate_integrity.py` e aos 4 artefatos do ciclo 087;
  `validate/`, `release/` e `.specify/memory/STATE.md` NÃO DEVEM ser tocados.

### Key Entities

- **db-tests leg**: invocação `run_leg <label> <filter> <floor>` que soma os
  contadores `N passed` e exige `>= floor`.
- **FLOORS**: mapa determinístico no guarda de integridade; a soma passa de
  54 para 58 com a entrada `bootstrap` (10ª entrada).
- **Bootstrap DB contract**: teste `#[ignore]` em `contract/bootstrap.rs`,
  habilitado somente na lega `db_`.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: `cargo test -p flora-tests contract::bootstrap::tests::db_ --
  --ignored --test-threads=1` com Postgres local reporta exatamente
  **4 passed**; ausência da lega produz 0 (regressão detectada).
- **SC-002**: `python3 scripts/ci/check_gate_integrity.py` retorna PASS com
  `all 10 db contract floors intact (58 total)` e nenhuma falha de `db floor`.
- **SC-003**: `git diff --name-only 367a50e79` contém apenas os 6 arquivos
  autorizados (2 de CI + 4 artefatos 087); zero arquivos em `validate/`,
  `release/` ou `STATE.md`.

## Assumptions

- Postgres descartável já é provisionado pelo job `db-tests` (as 9 legs
  existentes dependem do mesmo env).
- O piso 4 corresponde aos 4 testes `db_bootstrap_*` descobertos em
  `contract/bootstrap.rs`; novos testes exigirão nova avaliação.
- Evidência de execução local e do gate é a registrada no handoff do ciclo;
  subagentes de documentação não reexecutam gates/testes.
