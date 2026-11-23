# Tasks: 118 — servidor IdP OIDC HTTP mock e endurecimento do callback

**Data**: 2026-11-23 · **Spec**: `specs/118-oidc-callback-mock-idp-exchange/spec.md`

- T-001 (Issue [#399](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/399)): Extrair `VerifiedOidcIdentity` / `exchange_code_and_verify_id_token` com testes unitários assimétricos (`EdDSA`) e SSRF JWKS em `flora-api/src/oidc.rs`, e endurecer `callback` em `flora-api/src/routes/auth.rs` contra contas inativas e conflito de `oidc_subject` [FR-001 / FR-002 / SC-001 / SC-002]
- T-002 (Issue [#400](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/400)): Implementar servidor HTTP local `MockOidcIdpServer` (`/.well-known/openid-configuration`, `/oauth/v2/keys` `Ed25519`/`OKP`, `POST /oauth/v2/token` PKCE S256) em `flora-tests/src/oidc_mock.rs` expondo crates já aprovados em `flora-tests/Cargo.toml` [FR-003 / SC-003]
- T-003 (Issue [#401](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/401)): Substituir os dois stubs de `flora-tests/src/integration/registration.rs` por testes HTTP reais (happy path + falhas fail-closed) e adicionar `2` contratos DB `db_oidc_callback_*` em `flora-tests/src/contract/auth.rs` com trava lockstep `auth: 23 -> 25` (`82` total) [FR-004 / SC-003 / SC-004]
- T-004 (Issue [#402](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/402)): Preencher Briefing de Compreensão `auth`, roteiro/validação R4, release R7 (`0.10.1`), `converge.md`, docs append-only e conduzir ACR independente + CI verde [FR-005 / SC-005]
