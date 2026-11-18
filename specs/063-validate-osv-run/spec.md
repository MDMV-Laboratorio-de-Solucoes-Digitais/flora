# Feature Specification: Validate OSV run 063

**Feature Branch**: `feature/063-validate-osv-run`

**Created**: 2026-11-18

**Status**: Approved (decision `go (scoped)`)

**Input**: "Fechar [NV] do pin OSV com execução real"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Pin validado por execução (Priority: P1)

Como mantenedor, quero evidência de que o workflow OSV com `@v2.7.1` executa,
para confiar no sinal advisory de supply-chain.

**Why this priority**: é o único [NV] que pode ser fechado sem aprovação externa.

**Independent Test**: `gh run list --workflow osv-scan.yml --limit 1` mostra conclusão.

**Acceptance Scenarios**:

1. **Given** o workflow `active`, **When** disparado via `workflow_dispatch`, **Then** cria um run.
2. **Given** o run, **When** concluído, **Then** `status`/`conclusion` são registrados com timestamp.
3. **Given** `STATE.md`, **When** lido, **Then** o [NV] anterior está corrigido por apêndice.

### Edge Cases

- `fail-on-vuln: false` — vulnerabilidades encontradas não falham o job; isso
  é esperado e não invalida a validação do pin.
- Upload SARIF é efeito normal do workflow; nenhuma credencial é copiada.
- Se falhar, a falha é registrada — não disfarçada.

## Requirements *(mandatory)*

- **FR-001**: disparar 1 execução.
- **FR-002**: registrar resultado em `STATE.md` (apêndice datado).
- **FR-003**: sem código/workflow/ADR/gate.
- **FR-004**: falha registrada sem maquiagem.

## Success Criteria *(mandatory)*

- **SC-001**: run observado até conclusão.
- **SC-002**: resultado colado com timestamp.
- **SC-003**: gates locais verdes.
- **SC-004**: diff sem código.

## Assumptions

- `gh` autenticado no github.com (verificado em 0.a).
- Nenhuma aprovação externa é necessária para `workflow_dispatch`.

## Open Questions

Nenhuma.
