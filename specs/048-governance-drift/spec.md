# Feature Specification: Governance drift 048

**Feature Branch**: `feature/048-governance-drift`

**Created**: 2026-11-17

**Status**: Approved (decision `go (scoped)`)

**Input**: "Corrige drift de governanca em AGENTS.md"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Instrução governante confiável (Priority: P1)

Como agente ou maintainer, quero que `AGENTS.md` descreva o workspace real, para
que decisões tomadas a partir dele não pressuponham crates ou toolchain inexistentes.

**Why this priority**: `AGENTS.md` é lido no início de toda tarefa no repositório.

**Independent Test**: `grep` por `flora-tasks|flora-files|1.98.0` em `AGENTS.md` → 0;
comparar lista de membros com `cargo metadata --no-deps`.

**Acceptance Scenarios**:

1. **Given** o checkout com 5 membros + raiz, **When** leio `AGENTS.md`, **Then** a lista coincide.
2. **Given** `Cargo.toml:19` com `1.98.1`, **When** leio `AGENTS.md`, **Then** a toolchain declarada é `1.98.1`.

### Edge Cases

- `flora-tasks`/`flora-files` existem como módulos de rota; não devem ser confundidos com crates.
- `rustc --version` do host (1.99.1) não é o pin do CI; a fonte é `Cargo.toml`/workflows.

## Requirements *(mandatory)*

- **FR-001**: lista de membros em `AGENTS.md` = 5 membros + raiz.
- **FR-002**: toolchain declarada = `1.98.1`.
- **FR-003**: nenhuma regra/gate/permissão enfraquecida.
- **FR-004**: notas datadas e append-only em `OBJETIVO.md`/`ROADMAP.md`.

## Success Criteria *(mandatory)*

- **SC-001**: 0 ocorrências dos termos obsoletos em `AGENTS.md`.
- **SC-002**: `check_gate_integrity.py` → PASS.
- **SC-003**: diff limitado a `AGENTS.md`, `OBJETIVO.md`, `ROADMAP.md` + artefatos SDD.

## Assumptions

- Código/workflows são autoridade sobre prosa.
- Nenhuma reintrodução de crates neste ciclo.
- PR #262 permanece intacto.

## Open Questions

Nenhuma; itens remotos **[NV]**.
