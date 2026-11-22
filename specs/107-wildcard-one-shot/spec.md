# Feature Specification: Wildcard de recuperação com papel e one-shot

**Feature Branch**: `feature/107-wildcard-one-shot`
**Created**: 2026-11-22 · **Status**: Specified — go

## User Scenarios

### US1 — Seleção administrativa (P1)

Com `FLORA_BOOTSTRAP__ADMIN_EMAIL=*`, o startup e o login consideram apenas usuários ativos com papel built-in `Admin` ou `Owner`, no tenants não-sistema, em ordem determinística.

**Acceptance**: usuário sem papel administrativo não é selecionado; nenhuma enumeração é vazada em log/resposta.

### US2 — Consumo one-shot (P1)

Após o primeiro login wildcard bem-sucedido, o marcador persistente é consumido atomicamente; tentativas posteriores falham com 401 genérico mesmo com a env ainda presente.

**Acceptance**: duas tentativas concorrentes produzem no máximo uma sessão; restart não reativa o seletor.

### US3 — ADR de expiração (P2)

ADR registra a data de expiração (2026-11-29) e os quatro gates de desligamento/rotação.

## Requirements

- **FR-001** helper SQL único com JOIN `memberships`/`roles`, `is_builtin` e `name IN ('Admin','Owner')`.
- **FR-002** tabela singleton com `consumed_at`/`consumed_user_id`, `pg_advisory_xact_lock` e `UPDATE ... WHERE consumed_at IS NULL RETURNING`.
- **FR-003** startup não reaplica hash quando o marcador estiver consumido.
- **FR-004** login consome na mesma transação da sessão; falha → 401 genérico (`invalid_credentials`).
- **FR-005** ADR `0007` com expiração e testes sem novos `#[ignore]`.

## Success Criteria

- **SC-001** usuário não-admin não é alvo do wildcard.
- **SC-002** segundo login com `*` falha; gate PASS (10 legs / 77 pisos).
- **SC-003** nenhuma credencial ou e-mail real em log/resposta/artefato.

## [NV]

Staging real, credenciais, restart multiréplica e rotação operacional das variáveis.
