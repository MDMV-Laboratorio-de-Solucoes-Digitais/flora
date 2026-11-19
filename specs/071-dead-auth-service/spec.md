# Feature Specification: Remove dead AuthService stub — 071

**Feature Branch**: `feature/071-dead-auth-service`
**Created**: 2026-11-19
**Status**: Approved (`decision.md` go scoped)
**Input**: "Delete the always-failing AuthService stub and all of its wiring"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Fewer misleading concepts (P1)

Como leitor/agente do código, quero que `AppState` não exponha um serviço que sempre falha, para não inferir uma capacidade de autenticação inexistente.

**Independent Test**: `grep -rn 'AuthService\|auth_service' --include='*.rs'` → 0 ocorrências; `cargo test --workspace` permanece verde.

### User Story 2 — No behavior change (P1)

Como maintainer, quero que a autenticação real (`routes/auth.rs`, extractor de sessão) continue intacta e que `ServiceUnavailable` permaneça disponível para OIDC.

**Acceptance Scenarios**: handlers de auth inalterados; `Error::ServiceUnavailable` ainda referenciado em `oidc_transactions.rs`; gates verdes.

## Requirements

- **FR-001** excluir arquivo do stub e do `services/mod.rs`; **FR-002** limpar `AppState`; **FR-003** limpar 7 construtores de teste; **FR-004** preservar trait/variante; **FR-005** rastreabilidade.
- **SC-001** zero ocorrências de `AuthService`/`auth_service`; **SC-002** gates verdes, pisos intactos; **SC-003** diff restrito aos 11 arquivos + artefatos.

## Scope / Out of scope

Dentro: 11 arquivos Rust + artefatos. Fora: `AuthProvider`/`UserInfo`, `ServiceUnavailable`, handlers, CI, ADRs, constituição, produção.
