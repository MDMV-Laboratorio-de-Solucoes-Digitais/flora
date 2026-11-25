# Decision: Callback OIDC com IdP mock HTTP e troca real

- **Slug**: 118-oidc-callback-mock-idp-exchange
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | define.md | shape.md | decision.md (original 2026-11-23) | specs/118-oidc-callback-mock-idp-exchange/{spec,plan}.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `intake.md:9-14` mostra placeholders em `flora-tests/src/integration/registration.rs:10-23` e dois riscos latentes em `flora-api/src/routes/auth.rs:179-307` (conflito de `oidc_subject` e conta inativa). |
| Evidence strength | adequate | `research.md:7-18` verifica `jsonwebtoken 11.0.0` (EdDSA/JWK OKP), `fetch_jwks` com localhost aceito e `derive_s256_challenge` público; provedor Zitadel real e E2E externo são `[NV]`. |
| Value vs. inaction | strong | Fecha `BF-01` com consumidor real: sem o mock HTTP, o callback nunca exercita Discovery/JWKS/PKCE e falhas fail-closed passam despercebidas (`intake.md:5`). |
| Feasibility / appetite | adequate | Opção A reusa `axum`, `jsonwebtoken`, `serde_urlencoded` e `wiremock` já presentes; zero crates novos no `Cargo.lock` e zero impacto de `cargo deny` (`research.md:7`, `shape.md:5-13`). |
| Strategic fit | strong | Domínio `auth` crítico e alinhado à base RFC 6749/RFC 7636/OIDC Core já adotada (`research.md:3`) e ao encerramento de `BF-01`. |
| Risk posture | adequate | `allow_symmetric = false`, PKCE S256, código de uso único, `jwks_uri` restrito a localhost/HTTPS e `401` sem emissão de JWT mitigam o risco; segredos e IdP real não foram lidos (`research.md:11-14`, `define.md:7-10`). |

## Verdict & Rationale

Veredito histórico **`go` preservado**: a decisão de 2026-11-23 não é contradita por nenhum artefato. O scorecard sustenta o `go` — problema de segurança observável, evidência local suficiente e reuso integral de dependências já aprovadas — com o score original **2000 / 3125** (`impacto=5 × confiança=5 × fechamento=5 × (6−esforço=2)=4 × (6−risco=2)=4`). As restrições originais permanecem: Briefing de Compreensão (`auth`) documentando invariantes de `id_token` assimétrico, PKCE S256, vinculação de `oidc_subject` e bloqueio de contas inativas; lockstep de `FLOORS["auth"] = 25` em `scripts/ci/check_gate_integrity.py` e `.github/workflows/rust.yml` (total `82`); zero novas dependências no `Cargo.lock`; sem `.unwrap()`/`.expect()` em produção; `missing_docs = "deny"`; SemVer `PATCH` (`0.10.0 → 0.10.1`).

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; nenhuma incógnita bloqueante nos artefatos).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: Os testes de registro são placeholders e o `callback` mistura troca de código com persistência sem checar conflito de `oidc_subject` nem `is_active = false` (`intake.md:9-14`).
- **Chosen approach**: Opção A — `MockOidcIdpServer` Axum efêmero em `127.0.0.1:0` com chaves `Ed25519` em memória, validando PKCE S256 e uso único, em vez da Opção B (HS256/stubs), que não exercitaria HTTP real, JWKS `EdDSA` ou `PgUserRepository`/`PgSessionRepository` (`shape.md:5-13`).
- **In scope / out of scope**: **In** — extrair `VerifiedOidcIdentity` + `exchange_code_and_verify_id_token`, endurecer `callback` com `401` fail-closed, mock HTTP com modos de falha, substituir os 2 placeholders, 2 contratos DB (`auth: 23 → 25`, total `82`) e Briefing `auth` (`define.md:5-13`). **Out** — provedor OIDC externo real (Zitadel), fluxo passkey/WebAuthn (`ADR-0008`) e endpoint `/auth/refresh` em browser E2E (`define.md:15`).
- **Success metrics**: SC-001..SC-005 em `specs/118-oidc-callback-mock-idp-exchange/spec.md` — `verify_id_token` EdDSA com rejeição de JWK `oct` e `jwks_uri` HTTP externo, `401` cross-subject/conta desativada sem JWT ou `Session`, fluxo feliz e falhas HTTP reais sem placeholders, contratos `db_oidc_callback_*` e piso `auth: 25` (total `82`).
- **Carried-forward open questions**: `[NV]` execução contra IdP real, staging/produção e credenciais — nenhuma credencial foi lida ou registrada; `[NV]` comportamento de `nonce` e e-mail ausente é coberto como modo de falha do mock, não contra Zitadel (`spec.md:SC-003`); `[NEEDS CLARIFICATION: o reúso do `user.id` no segundo login deve atualizar `oidc_subject` quando o provedor rotaciona o subject, ou permanecer estritamente vinculado ao subject original?]` (`spec.md:SC-004`).
