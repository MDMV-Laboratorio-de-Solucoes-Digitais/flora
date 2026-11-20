# Feature Specification: Rate limit paths 050

**Feature Branch**: `feature/050-rate-limit-paths`

**Created**: 2026-11-17

**Status**: Approved (decision `go (scoped)`)

**Input**: "Corrige correspondencia de paths do rate limiter"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Limites específicos voltam a valer (Priority: P1)

Como operador, quero que os limites de login/refresh/invite sejam aplicados às
rotas reais, para que a proteção contra força bruta não seja apenas nominal.

**Why this priority**: o limite específico de login (5/60) hoje nunca dispara.

**Independent Test**: teste unitário puro com os paths reais do router.

**Acceptance Scenarios**:

1. **Given** `/api/v1/auth/login`, **When** classificado, **Then** retorna `Login`.
2. **Given** `/api/v1/users/invite`, **When** classificado, **Then** retorna `Invite`.
3. **Given** `/health`, **When** classificado, **Then** retorna `General`.

### Edge Cases

- Formas legadas sem prefixo (`/auth/login`) continuam classificadas corretamente.
- `path.contains("/invites")` legado é preservado.

## Requirements *(mandatory)*

- **FR-001**: `classify_endpoint` reconhece login.
- **FR-002**: reconhece refresh.
- **FR-003**: reconhece invite.
- **FR-004**: demais paths → `General`.
- **FR-005**: 4 testes unitários puros.

## Success Criteria *(mandatory)*

- **SC-001**: `cargo test -p flora-api` verde com os novos testes.
- **SC-002**: limites e janelas inalterados no código.
- **SC-003**: gates completos verdes.

## Assumptions

- O middleware vê o path completo (`req.uri().path()`).
- Nenhum state distribuído é introduzido.

## Open Questions

Nenhumas; remoto **[NV]**.
