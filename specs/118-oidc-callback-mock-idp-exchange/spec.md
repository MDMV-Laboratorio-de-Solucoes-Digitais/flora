# Feature Specification: Servidor IdP OIDC HTTP mock e endurecimento do callback

**Feature ID**: 118 · **Status**: specified · **Data**: 2026-11-23

## User Scenario

Como engenheiro/mantenedor do Flora, quero um servidor IdP OIDC HTTP local determinístico que exercite todo o fluxo real de callback (`/auth/login` → `POST /oauth/v2/token` com PKCE S256 → Discovery → JWKS `EdDSA` → `verify_id_token` → criação de usuário/sessão em PostgreSQL), substituindo os testes placeholder em `registration.rs` e garantindo bloqueio fail-closed de contas desativadas ou com conflito de `oidc_subject`.

## Acceptance Scenarios

1. **SC-001 (Extração de troca OIDC + testes unitários assimétricos/JWKS em `flora-api`)**: **Given** `OidcConfig`, authorization `code` e `OidcTransaction`, **when** `exchange_code_and_verify_id_token` executa em `flora-api/src/oidc.rs`, **then** realiza `POST {issuer}/oauth/v2/token`, obtém Discovery + JWKS via `fetch_jwks`, valida assinatura assimétrica `EdDSA` via `verify_id_token` (`allow_symmetric = false`), rejeita chaves simétricas `oct` e `jwks_uri` HTTP não-local, e normaliza `email` (`trim` + `to_lowercase`) e `name`.
2. **SC-002 (Endurecimento fail-closed do `/auth/callback`)**: **Given** uma identidade OIDC verificada em `flora-api/src/routes/auth.rs`, **when** existe usuário com o mesmo e-mail porém `oidc_subject` vinculado a outro sujeito, ou quando `user.is_active == false`, **then** `/auth/callback` rejeita com `401 Unauthorized` sem emitir JWT nem gravar `Session`; caso ativo, assina `FloraClaims` com `&user.email`.
3. **SC-003 (`MockOidcIdpServer` HTTP + cobertura de `registration.rs`)**: **Given** `MockOidcIdpServer` escutando em `http://127.0.0.1:0` com endpoints `/.well-known/openid-configuration`, `/oauth/v2/keys` (`OKP`/`Ed25519`) e `/oauth/v2/token` (PKCE S256, `client_id`, `client_secret`, `redirect_uri`, uso único), **when** `test_registration_flow_with_mock_oidc` e `test_registration_flow` rodam em `cargo test -p flora-tests`, **then** validam tanto o fluxo feliz quanto falhas HTTP reais (PKCE inválido, estado reutilizado pós-erro, assinatura adulterada, nonce divergente, JWK simétrico e `email` ausente) — zero placeholders.
4. **SC-004 (Contratos DB fim a fim `/auth/login` → `/auth/callback` e trava lockstep `82`)**: **Given** PostgreSQL 18 com migrações aplicadas, **when** `db_oidc_callback_exchanges_code_registers_new_user_and_creates_session` e `db_oidc_callback_existing_user_login_and_rejects_deactivated_or_conflicting_subject` executam, **then** comprovam registro inicial, segundo login reutilizando `user.id`, emissão de `session_token` + linha ativa em `sessions`, rejeição `401` cross-subject e rejeição `401` para conta desativada; `FLOORS["auth"]` sobe em lockstep `23 -> 25` (total `82`).
5. **SC-005 (Briefing `auth`, R4/R7, docs append-only e ACR)**: **Given** o ciclo implementado e verificado, **then** entrega Briefing de Compreensão (`auth`), roteiro/evidências R4, release R7 (`0.10.0 → 0.10.1` PATCH), atualização append-only em `STATE.md`/`PROGRESSO.md`/`OBJETIVO.md`/`ROADMAP.md`/`research/index.md` e revisão agêntica independente.

## Requirements

- **FR-001**: Criar `VerifiedOidcIdentity` e `exchange_code_and_verify_id_token` em `flora-api/src/oidc.rs`, com testes unitários cobrindo `verify_id_token` `EdDSA` (`allow_symmetric = false`), rejeição de JWK `oct` e rejeição de `jwks_uri` HTTP externa em `fetch_jwks`.
- **FR-002**: Refatorar `callback` em `flora-api/src/routes/auth.rs` para usar `exchange_code_and_verify_id_token`, rejeitar `401 Unauthorized` para usuário desativado ou e-mail já vinculado a `oidc_subject` distinto, e emitir `FloraClaims` com `&user.email`.
- **FR-003**: Expor `base64`, `jsonwebtoken` e `serde_urlencoded` em `flora-tests/Cargo.toml` e implementar `flora-tests/src/oidc_mock.rs` (`MockOidcIdpServer`), exportando `pub mod oidc_mock` em `flora-tests/src/lib.rs`.
- **FR-004**: Substituir os dois stubs em `flora-tests/src/integration/registration.rs` por testes HTTP reais contra `MockOidcIdpServer`, adicionar `2` contratos DB `db_oidc_callback_*` em `flora-tests/src/contract/auth.rs` e atualizar `scripts/ci/check_gate_integrity.py` + `.github/workflows/rust.yml` (`auth: 23 -> 25`, total `82`).
- **FR-005**: Preencher Briefing de Compreensão (`auth`), governança, roteiro/evidências, R4/R7 (`0.10.1`), `converge.md`, docs append-only e ACR independente.
