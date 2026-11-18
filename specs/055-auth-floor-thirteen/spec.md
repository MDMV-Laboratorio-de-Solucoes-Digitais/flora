# Feature Specification: Auth floor thirteen 055

**Feature Branch**: `feature/055-auth-floor-thirteen`

**Created**: 2026-11-18

**Status**: Approved (decision `go (scoped)`)

**Input**: "Fechar folga de 1 no leg auth do db-tests"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Piso reflete testes descobertos (Priority: P1)

Como mantenedor, quero que o piso do leg `auth` seja igual ao número de testes
descobertos, para que nenhum teste possa desaparecer sem ficar vermelho.

**Why this priority**: é o único leg com folga (12 exigidos, 13 descobertos).

**Independent Test**: `cargo test --list` → 13; guardião PASS com 13/13.

**Acceptance Scenarios**:

1. **Given** `rust.yml`, **When** inspecionado, **Then** `run_leg auth … 13`.
2. **Given** o guardião, **When** executado, **Then** valida 13 e imprime 50 total.
3. **Given** Postgres real, **When** o leg `auth` roda, **Then** 13 testes passam.

### Edge Cases

- Os 13 testes são `#[ignore]` + env-guarded; o job `db-tests` já falha fechado
  sem env (ciclo 054), portanto os 13 não são vacuos na CI.
- `run_leg` usa `--test-threads=1`, mitigando corridas de seed.
- ADR-0003 permanece com o texto antigo (43) — dívida de governança, fora de escopo.

## Requirements *(mandatory)*

- **FR-001**: piso 13 em `rust.yml`.
- **FR-002**: piso 13 em `FLOORS["auth"]`.
- **FR-003**: texto "49 total" → "50 total".
- **FR-004**: nada mais muda (outros pisos, checks, testes, proteção).

## Success Criteria *(mandatory)*

- **SC-001**: `--list` = 13 e guardião PASS 13/50.
- **SC-002**: leg auth com DB isolado = 13 passed.
- **SC-003**: gates locais permanecem verdes.
- **SC-004**: diff de código ≤ 3 linhas em 2 arquivos.

## Assumptions

- Os 13 testes passam de forma estável no runner (evidência local + CI **[NV]**).
- Nenhum teste será adicionado/removido no mesmo ciclo.

## Open Questions

Nenhuma. Runner ao vivo: **[NV]**.
