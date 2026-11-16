# Seed — "Users & Roles (OIDC)": minimal verifiable identity slice (2026-11-16)

**Topic:** what a minimal, *verifiable* OIDC-based identity slice requires for a self-hosted SMB app (Flora Seed / VPS profile): Authorization Code + PKCE S256, discovery/JWKS/key rotation, ID-token validation, session binding and logout, role/permission model, and whether Keycloak/Zitadel is mandatory. Compared against lightweight self-hosted options.

**Method:** read-only inspection of Flora (`flora-api/src/routes/auth.rs`, `flora-api/src/extractors/auth.rs`, `flora-api/src/state/oidc_transactions.rs`, `flora-core/src/config/{oidc,app}.rs`, `flora-core/src/utils/{jwt,pkce}.rs`, `flora-core/src/services/{rbac_service,auth_service}.rs`, `flora-api/src/routes/rbac.rs`, migrations, workspace `Cargo.toml`) plus primary sources (OpenID Foundation specs, IETF RFCs, OWASP) and provider documentation. Multilingual pass across EN, pt-BR, es, de, fr, zh-CN, ja using authoritative EN specs as the normative layer and localized official/secondary material for corroboration. **No application code was changed.**

## Executive summary

1. Flora already has the *hard* part of a stateful design right: the PKCE transaction store, per-request server-side session authority via `jti`, org binding with membership check, and `jti` rotation on org switch. These are the expensive pieces to retrofit and they exist.
2. Flora does **not** currently perform OIDC identity verification: the ID token is base64-decoded and trusted (`auth.rs:201-203`, `auth.rs:398-414`). No signature, `iss`, `aud`, `exp`, or `nonce` check exists. PKCE is present but `nonce` — the replay defence for the ID token — is absent entirely.
3. The authorization and token endpoints are **hardcoded Zitadel v2 paths** (`auth.rs:121`, `auth.rs:161-164`); there is no discovery, no `jwks_uri`, no `kid` key selection, no rotation. The `openidconnect 4.0` crate is declared in the workspace (`Cargo.toml:171`) but unused in code (only a comment at `auth.rs:400`).
4. The "Users & Roles" half is weaker than the "OIDC" half: all five RBAC HTTP endpoints are literal stubs (`rbac.rs:25-48`), `AuthService` is a `ServiceUnavailable` stub (`auth_service.rs`), and workspace-level permissions delegate to org-level (`rbac_service.rs:132-141`).
5. **Keycloak/Zitadel is not mandatory.** For a single self-hosted app the normative requirement is that Flora be a correct RP + keep its own session/role store. An IdP is a deployment choice. Seed can be *verifiably* ready with a mock/local OP in tests and an optional external OP in production — no JVM IdP on the VPS.
6. The highest-value 80/20 slice is: (a) verified ID token via the already-declared `openidconnect` crate with `nonce`, `iss`, `aud`, `exp`, `kid` allowlist; (b) identity by `sub` instead of email; (c) exact redirect-URI allowlist; (d) implement the 5 RBAC endpoints against existing repositories with negative tests; (e) local logout scoped to the current `jti`. All of it is testable deterministically without a live IdP.

---

## 1. What a minimal verifiable OIDC identity slice requires (primary sources)

### 1.1 Authorization Code + PKCE S256 — already correctly shaped in Flora

- OIDC Core defines the Authorization Code Flow: the client sends an authorization request, receives a code, exchanges it server-to-server at the token endpoint, receives an ID token, then **validates the ID token and retrieves the subject** ([OIDC Core §3.1.1](https://openid.net/specs/openid-connect-core-1_0.html#AuthorizationCodeFlowSteps)). Tokens not revealed to the user agent and client authentication are the stated advantages of this flow ([§3](https://openid.net/specs/openid-connect-core-1_0.html#Authentication)).
- PKCE (RFC 7636) exists to stop authorization-code interception. The verifier must be a cryptographically random string of 43–128 unreserved characters, **recommended 32 octets → base64url → 43 chars**, with ≥256 bits entropy ([§4.1](https://www.rfc-editor.org/rfc/rfc7636#section-4.1), [§7.1](https://www.rfc-editor.org/rfc/rfc7636#section-7.1)); `S256` is `BASE64URL(SHA256(ASCII(verifier)))` ([§4.2](https://www.rfc-editor.org/rfc/rfc7636#section-4.2)) and is **mandatory to implement** on servers; clients capable of S256 **MUST** use it and **MUST NOT** downgrade to `plain` ([§7.2](https://www.rfc-editor.org/rfc/rfc7636#section-7.2)). The token endpoint recomputes the challenge and returns `invalid_grant` on mismatch ([§4.6](https://www.rfc-editor.org/rfc/rfc7636#section-4.6)).
- Flora matches the crypto shape: 32 random bytes → 43-char base64url verifier/state and S256 challenge (`pkce.rs:12-31`, `pkce.rs:37-41`), S256 enforced (`config/oidc.rs:63-67`, `auth.rs:89-98`), short-lived one-time transaction with `SET NX` / `GETDEL` (`oidc_transactions.rs:14, 67-101, 110-139`).
- **Gap against the RFC:** Flora has no unit test using the RFC 7636 Appendix B vector (`dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk` → `E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM`); its tests only assert determinism and URL-safety (`pkce.rs:47-77`). This is a cheap, high-confidence verification win.

### 1.2 Discovery, JWKS and key rotation

- OIDC Core assumes endpoint locations are normally obtained via Discovery ([§1](https://openid.net/specs/openid-connect-core-1_0.html#Introduction)).
- Discovery requires `GET {issuer}/.well-known/openid-configuration` with `application/json`, and mandates metadata including `issuer`, `authorization_endpoint`, `token_endpoint`, `jwks_uri`, `response_types_supported`, `subject_types_supported`, `id_token_signing_alg_values_supported` ([Discovery §3, §4.1](https://openid.net/specs/openid-connect-discovery-1_0.html#ProviderMetadata)). Critically, the RP **MUST ensure the issuer URL used for the configuration request exactly matches the `issuer` metadata value and the `iss` claim in ID tokens** ([§4.3](https://openid.net/specs/openid-connect-discovery-1_0.html#ProviderConfigValidation), [§7.2](https://openid.net/specs/openid-connect-discovery-1_0.html#Impersonation)) — this is the anti-impersonation anchor.
- `jwks_uri` publishes the OP's signing public keys; JWK sets allow multiple keys and `kid` is the normal selector. Rotation is a first-class concern: OIDC Core §10.1 explicitly covers rotation of asymmetric signing keys, and OWASP warns that `jwk`/`jku`/`x5u` values **from the token header** are unauthenticated attacker input — trust must be anchored to the issuer's configured/pinned `jwks_uri`, with `kid` used only as a selector *inside* that set ([OWASP JWT — Trusting key material named in the token header](https://cheatsheetseries.owasp.org/cheatsheets/JSON_Web_Token_Cheat_Sheet.html)).
- **Flora:** no discovery, no JWKS fetch, no `kid` handling, no key cache/rotation. Endpoints are string-built from `issuer_url` (`auth.rs:100-103, 121, 161-164`). Config validation does not require HTTPS or reject an empty client secret (`config/oidc.rs:47-68`); the default issuer is `http://localhost:8080` with an empty secret (`config/oidc.rs:25-29`).

### 1.3 ID-token validation (issuer / audience / exp / nonce / signature)

OIDC Core §3.1.3.7 is the normative checklist for the code flow ([full text](https://openid.net/specs/openid-connect-core-1_0.html#IDTokenValidation)):

| Check | Requirement | Flora |
|---|---|---|
| Signature | Verify per JWS using the algorithm in `alg`, with keys provided by the Issuer. **Exception:** when the ID token arrives via direct client↔token-endpoint communication, TLS server validation MAY be used in place of the signature check. Default expected `alg` is RS256. | ❌ No signature check. Note the exception exists, but Flora's default issuer is plain HTTP (`oidc.rs:25-27`), so the TLS substitute is not available in the default profile. |
| `iss` | Issuer Identifier **MUST exactly match** the `iss` claim. | ❌ |
| `aud` | **MUST** contain the RP's `client_id`; reject if the RP is not listed. | ❌ |
| `exp` | Current time **MUST** be before `exp`. | ❌ |
| `nonce` | If a nonce was sent, the claim **MUST** be present and equal. Nonce is RECOMMENDED in the request and binds client session ↔ ID token, mitigating replay ([§2](https://openid.net/specs/openid-connect-core-1_0.html#IDToken), [§15.5.2](https://openid.net/specs/openid-connect-core-1_0.html#NonceNotes)). | ❌ No nonce generated, stored, sent, or compared. |
| `azp`, `iat` | Conditional/optional; `iat` can bound nonce storage lifetime. | ❌ / n/a |

The `id_token` is decoded by splitting on `.` and base64url-decoding segment 2 (`auth.rs:398-414`), with the in-code comment literally "unverified … In production, verify the signature against Zitadel's JWKS" (`auth.rs:201-203`). Email/name/sub are then read with `unwrap_or("")`, `unwrap_or("User")`, `unwrap_or("")` (`auth.rs:204-215`), so an empty `email` can be persisted.

Supporting guidance: OWASP requires rejecting `alg:none`, hardcoding an algorithm allowlist, binding the key to the validated issuer's `jwks_uri`, validating `iss`/`aud` as whole-string comparisons, requiring `exp`/`iss`/`aud`, and using explicit `typ` or mutually exclusive validation rules to avoid cross-JWT/token-type confusion ([OWASP JWT](https://cheatsheetseries.owasp.org/cheatsheets/JSON_Web_Token_Cheat_Sheet.html); [RFC 8725](https://www.rfc-editor.org/rfc/rfc8725)). Flora's *own* session JWTs use `Validation::default()` with HS256 (`jwt.rs:96-104`) — appropriate for a self-issued, self-consumed token under a dedicated secret, but note the `ver` claim (`jwt.rs:30-32`) is never used for invalidation.

### 1.4 Session binding and logout

- Session authority must be server-side for revocability: a valid JWT remains valid until `exp` unless server state is consulted; `jti` denylists/allowlists are the standard mechanism ([OWASP JWT — JWT revocation](https://cheatsheetseries.owasp.org/cheatsheets/JSON_Web_Token_Cheat_Sheet.html); [RFC 7519 §4.1.7](https://www.rfc-editor.org/rfc/rfc7519#section-4.1.7); [RFC 8725](https://www.rfc-editor.org/rfc/rfc8725)). Flora already does this correctly: `verify_bearer_token` decodes then checks row existence, `is_active`, `expires_at`, and user/org equality against claims (`extractors/auth.rs:33-97`), failing closed with generic 401/500.
- Tenant context must come from the verified session, never from client-supplied identity headers — consistent with prior Flora research ([016 auth/RBAC/OIDC](016-auth-rbac-oidc-minimal-vps.md#3-recomendações-8020-priorizadas)) and the session-authority cycle ([012](2026-11-16-012-session-authority-international.md)).
- RP-initiated logout is a separate OIDC spec: the RP redirects the user agent to the OP's `end_session_endpoint` ([§2](https://openid.net/specs/openid-connect-rpinitiated-1_0.html#RPLogout)), discovered via metadata ([§2.1](https://openid.net/specs/openid-connect-rpinitiated-1_0.html#OpenIDProviderDiscoveryMetadata)); `id_token_hint` is RECOMMENDED and `post_logout_redirect_uri` MUST exactly match a registered URI, with redirection forbidden without a valid hint or other confirmation ([§2, §3](https://openid.net/specs/openid-connect-rpinitiated-1_0.html#RedirectionToRPAfterLogout)). Local RP logout may happen before redirecting, but unreliable notifications are explicitly acknowledged by the spec.
- **Flora:** logout revokes **all sessions for the user** (`auth.rs:268-281`, specifically `revoke_all_for_user` at `:277`), returns 204 even when no/invalid token is presented (`:269-280`), and has no `end_session_endpoint`, no stored `id_token`, and no `id_token_hint`. `refresh` reuses the same `jti` (`auth.rs:303-312`) and `refresh_token_id` is always `None` (`auth.rs:245, 364`) — no rotation or reuse detection. The `state` is returned to the caller in JSON (`auth.rs:135-139`) rather than bound to a browser cookie, so the caller must round-trip it; a prior assessment records the frontend not doing so (`.specify/assessments/006-secure-vertical-slice/research.md:19` — re-verify before relying on it).

### 1.5 Role / permission model

- The minimum for a multi-tenant SMB app is: verified identity → membership in an organization → role → permissions, with every check performed server-side and tenant scope taken from the session. OWASP's multi-tenant guidance and the prior Flora research converge on "roles→permissions, enforce server-side, negative cross-tenant tests" ([016 §3](016-auth-rbac-oidc-minimal-vps.md#3-recomendações-8020-priorizadas)).
- `roles`/`groups`/`entitlements` claims in an *external* ID token are authorization-relevant but must be treated as untrusted unless the issuer/audience/key binding is airtight; OWASP flags "spoofed cross-issuer authorization" as the risk ([OWASP JWT — Claims](https://cheatsheetseries.owasp.org/cheatsheets/JSON_Web_Token_Cheat_Sheet.html)). For Seed, **keep roles/permissions authoritative in Flora's own DB**, not in the IdP token. This also keeps the app portable across IdPs.
- Flora has a reasonable domain layer: owner role seeded with 26 permissions (`rbac_service.rs:49-89`), `has_permission` with admin bypass and membership check (`:97-121`), and negative tests exist in `flora-tests/src/contract/auth.rs`. But `has_permission_in_workspace` just delegates to org scope (`:132-141`), the role cache has no automatic invalidation on writes (`:27-28, 163-192`), and the HTTP surface is entirely stubbed (`rbac.rs:25-48`).

### 1.6 Is Keycloak/Zitadel mandatory? No.

Normatively, OIDC places no requirement on the *product* used as OP — only that the RP discovers/validates correctly. Operational evidence:

- **Zitadel** self-host docs: "For test environments, ZITADEL does not need many resources, 1 CPU and 512MB memory are more than enough", but it requires a PostgreSQL instance and points to a separate production setup and HA/Kubernetes guidance ([Zitadel deploy overview](https://zitadel.com/docs/self-hosting/deploy/overview)). Viable, but a second stateful service with its own DB/schema/upgrade path.
- **Keycloak** primary docs show the operational tax explicitly: production requires an explicit `hostname` (security measure to prevent forged discovery URLs/token issuer), front/back-channel and admin URLs, reverse-proxy header handling (`proxy-headers`, `hostname-strict`), and warnings about Host-header injection when dynamic URLs are enabled ([Keycloak hostname v2](https://www.keycloak.org/server/hostname)). That is real configuration surface on a small VPS.
- **Casdoor** shows the lightweight end of the spectrum: one Go binary + a SQL database, built-in OIDC/OAuth/SAML/CAS, RP-initiated logout, and Casbin-backed ACL/RBAC/ABAC; docs are localized in EN/es/fr/de/ja/zh/pt ([Casdoor overview](https://casdoor.org/docs/overview)).
- Other credible lightweight/forward-auth options (Authentik, Authelia, Kanidm, Pocket ID) cover single-app login with far less moving parts; they are *alternatives*, not requirements. Authentik/Authelia-style forward-auth protects a reverse proxy but does not replace Flora's own session/role authority.
- **Conclusion:** for Flora Seed, an external IdP is **optional**. The mandatory work is on the RP side (verification, session, roles). A local/mock OP in tests verifies the contract; a real OP (Zitadel/Casdoor/Authentik/Keycloak) is a deployment profile chosen by the operator. Avoid making JVM IdP footprint a Seed dependency — this matches the prior 80/20 recommendation ([016](016-auth-rbac-oidc-minimal-vps.md#2-síntese-da-pesquisa-multilíngue-fontes-primárias)).

---

## 2. Síntese multilíngue (corroboração, não substitui as specs)

| Idioma | Fonte representativa | Consenso útil ao Seed |
|---|---|---|
| EN | [OIDC Core](https://openid.net/specs/openid-connect-core-1_0.html), [Discovery](https://openid.net/specs/openid-connect-discovery-1_0.html), [RP-Initiated Logout](https://openid.net/specs/openid-connect-rpinitiated-1_0.html), [RFC 7636](https://www.rfc-editor.org/rfc/rfc7636), [OWASP JWT](https://cheatsheetseries.owasp.org/cheatsheets/JSON_Web_Token_Cheat_Sheet.html) | Norma: validar ID token (iss/aud/exp/nonce/assinatura), usar `jwks_uri` do issuer, nonce/state, sessão com autoridade server-side. |
| pt-BR | [016 — Auth, RBAC e OIDC mínimos e seguros para VPS](016-auth-rbac-oidc-minimal-vps.md), [seed 80/20](2026-11-15-flora-seed-80-20-research.md) | "OIDC opcional, um provider, feature/profile"; evitar Keycloak/Zitadel como dependência obrigatória no Seed; Postgres como fonte de verdade de sessão. |
| es | [OpenID (es), Wikipedia](https://es.wikipedia.org/wiki/OpenID) + [Casdoor es](https://casdoor.org/docs/overview) | OIDC = capa de identidad sobre OAuth 2.0; RP verifica el ID token; IdPs ligeros ofrecen OIDC estándar sin infraestructura pesada. |
| de | [OpenID (de), Wikipedia](https://de.wikipedia.org/wiki/OpenID) + [Keycloak docs](https://www.keycloak.org/server/hostname) | "ID-Token muss vom RP verifiziert werden"; Hostname/Discovery korrekt konfigurieren — Betriebsaufwand real. |
| fr | [OpenID (fr), Wikipedia](https://fr.wikipedia.org/wiki/OpenID) + [Casdoor fr](https://casdoor.org/docs/overview) | Vérifier signature et audience; les fournisseurs légers exposent les mêmes endpoints standard. |
| zh-CN | [OpenID (zh), Wikipedia](https://zh.wikipedia.org/wiki/OpenID) + [Casdoor 中文文档](https://casdoor.org/docs/overview) | 校验 ID Token 的 iss/aud/exp/nonce；轻量 IdP 足以覆盖单应用场景。 |
| ja | [OpenID (ja), Wikipedia](https://ja.wikipedia.org/wiki/OpenID) | 「RPはOPの公開鍵で署名検証しなければならない」「nonce/state でリプレイ・CSRF を防ぐ」— corrobora §1.3/§1.4 (secondary; specs normativas). |

> Localized material is corroborating only. Where a localized summary and a spec disagree, the spec wins (e.g., the TLS-substitute for signature in code flow is a real `MAY` in §3.1.3.7 that short summaries omit).

---

## 3. Flora baseline — estado exato (file:line)

### 3.1 What exists and works

| Capability | Evidence |
|---|---|
| PKCE S256 verifier/challenge/state, 32 bytes entropy | `flora-core/src/utils/pkce.rs:12-41`; enforced at `flora-core/src/config/oidc.rs:63-67` and `flora-api/src/routes/auth.rs:89-98` |
| One-time, expiring, atomic state/verifier store (Valkey + in-memory fallback) | `flora-api/src/state/oidc_transactions.rs:14, 19-25, 67-101, 110-139` |
| Token exchange at a token endpoint with `code_verifier` | `flora-api/src/routes/auth.rs:160-195` |
| Self-issued HS256 session JWT + DB session row with `jti` | `flora-core/src/utils/jwt.rs:15-104`; `flora-api/src/routes/auth.rs:230-257`; migration `20240818000001_init_schema.sql:128-141` |
| Per-request server-side session authority (active/expiry/user/org match) | `flora-api/src/extractors/auth.rs:21-97` |
| Org selection with membership check + `jti` rotation | `flora-api/src/routes/auth.rs:321-385` (`348-351`, `353-356`, `373-376`) |
| Nil/"System" org seeded for pre-selection sessions | migration `20240818000001_init_schema.sql:27`; `auth.rs:236` — FK is satisfied, not a bug |
| Role/permission domain service + owner bootstrap + negative contract tests | `flora-core/src/services/rbac_service.rs:49-89, 97-121`; `flora-tests/src/contract/auth.rs` |
| `openidconnect 4.0` dependency declared | `Cargo.toml:171` (not used in any `.rs`; only a comment at `auth.rs:400`) |

### 3.2 Gap list (precise, with spec reference)

| # | Severity | Gap | Location | Spec / reason |
|---|---|---|---|---|
| G1 | **P0** | ID token is base64-decoded only; no signature, `iss`, `aud`, `exp`, `nonce` validation. Empty claims fall back to `""`/`"User"`. | `flora-api/src/routes/auth.rs:201-215`, `:398-414` | OIDC Core §3.1.3.7; OWASP JWT; RFC 8725 |
| G2 | **P0** | No `nonce` generated, stored in `OidcTransaction`, sent in authorize request, or checked. | `auth.rs:105-131`, `:387-396`; `oidc_transactions.rs:19-25` | OIDC Core §2/§3.1.2.1/§3.1.3.7 — replay defence |
| G3 | **P0** | Endpoints hardcoded to Zitadel v2 (`/oauth/v2/authorize`, `/oauth/v2/token`); no discovery, no `jwks_uri`, no `kid`, no rotation, no issuer metadata cross-check. | `auth.rs:100-103, 121, 161-164`; `config/oidc.rs:1-20` | Discovery §3/§4.3/§7.2; OIDC Core §10.1 |
| G4 | **P0** | `redirect_uri` is taken from caller input and echoed into the request/transaction with no server-side allowlist or exact match. | `auth.rs:29-34, 103, 128, 170`; `oidc_transactions.rs:24` | OIDC Core §3.1.2.1 — MUST exactly match a pre-registered URI |
| G5 | **P0** | Identity resolved by email (`find_by_email`), not by `sub`; `find_by_oidc_subject` exists but is unused. `oidc_subject` is indexed but **not UNIQUE**; existing users' subjects are never backfilled or verified. | `auth.rs:219-228`; `flora-core/src/repositories/user_repository.rs:41`, `flora-core/src/traits/repository.rs:21`; migration `:8, :146` | OIDC Core §2 (`sub` locally unique, never reassigned) — email is mutable and issuer-scoped identity should be `iss+sub` |
| G6 | **P0** | All RBAC HTTP endpoints are placeholders returning empty/fake data; no permission check. | `flora-api/src/routes/rbac.rs:25-48` | "Users & Roles" milestone; OWASP server-side enforcement |
| G7 | **P1** | Logout revokes **all** user sessions and always returns 204; no current-`jti`-only path, no OIDC RP-initiated logout, no `id_token` stored for `id_token_hint`. | `auth.rs:267-281`; migration `:128-141` (no id_token column) | RP-Initiated Logout §2/§3 |
| G8 | **P1** | Refresh reuses the same `jti`; `refresh_token_id` always `None`; no rotation/reuse detection. | `auth.rs:245, 283-319, 364` | OWASP JWT revocation; session hygiene |
| G9 | **P1** | Default config permits insecure operation: HTTP issuer `http://localhost:8080`, empty client secret, `local_fallback_enabled` declared but never consulted. | `config/oidc.rs:18, 25-29, 47-68`; `auth.rs` (no use of fallback flag) | Discovery TLS requirements; fail-closed config |
| G10 | **P1** | `AuthService` is entirely stubbed (`ServiceUnavailable`, `is_available() == false`); dual auth code paths (route-level vs trait) with no single authority. | `flora-core/src/services/auth_service.rs:1-60` | Maintainability / verifiability of "OIDC" claim |
| G11 | **P2** | Workspace-level permission checks delegate to org scope; role cache has no write-triggered invalidation. | `rbac_service.rs:27-28, 132-141, 163-192` | Least privilege / propagation (config has `permission_propagation_grace_secs`, `app.rs:24-25`) |
| G12 | **P2** | PKCE tests lack the RFC 7636 Appendix B vector; no test pins `state` round-trip or replay of the same state. | `pkce.rs:43-77`; `oidc_transactions.rs` (no tests) | RFC 7636 Appendix B; deterministic verification |
| G13 | **P1** | `openidconnect 4.0` declared but unused; `Cargo.toml` also declares a second `jwt = "0.16"`/`jsonwebtoken 11` — token library surface is duplicated. | `Cargo.toml:171, 205, 211`; `auth.rs:400` (comment only) | Avoid writing JOSE verification by hand |
| G14 | **NV** | Frontend `state` round-trip and token/cookie storage not verified here; prior assessment reports `state` not round-tripped and `select-organization` not called. | `.specify/assessments/006-secure-vertical-slice/research.md:19`; `auth.rs:135-139` | Browser e2e remains NV |

### 3.3 Explicitly stubbed / not implemented

- `flora-api/src/routes/rbac.rs:25-48` — all five endpoints.
- `flora-core/src/services/auth_service.rs:30-60` — every `AuthProvider` method.
- ID-token verification — `auth.rs:398-414` with the production TODO at `:200-203`.
- Local fallback (`config/oidc.rs:18`) — no code path.
- `refresh_token_id` / `ver` claims — fields exist, unused.

---

## 4. Proposta 80/20 — tornar "Users & Roles (OIDC)" verificavelmente Seed-ready

Princípio: **o Seed não precisa de um IdP próprio; precisa de um RP correto + sessão/roles com autoridade no Postgres, provados por testes determinísticos.** Manter o modelo stateful atual (não migrar para JWT 100% stateless).

### Slice A (P0) — Callback OIDC verificável, sem infraestrutura nova

1. **Usar `openidconnect 4.0` (já declarado)** para discovery + `id_token` verification: `IssuerUrl → discover()` (ou metadata explícita em testes offline), `aud = client_id`, `iss` exatamente igual ao issuer configurado/descoberto, `exp`, `nonce`, e `alg` allowlist (RS256/ES256) com chave selecionada pelo `kid` **dentro do `jwks_uri` do issuer**. Rejeitar `jwk`/`jku`/`x5u` do header. *(G1, G2, G3, G13)*
2. **Adicionar `nonce` a `OidcTransaction`** e ao authorize request; validar no callback. TTL de 600 s já existe.*(G2)*
3. **Allowlist exata de `redirect_uri`** em config (remover aceitação irrestrita de `AuthQuery.redirect_uri`). *(G4)*
4. **Identidade por `sub`:** `find_by_oidc_subject` primeiro; fallback para email apenas se `email_verified == true` e com backfill de `oidc_subject` + log; nova migração com `UNIQUE (oidc_subject)` (não editar migração já aplicada, convenção do repo). *(G5)*
5. **Config fail-closed em produção:** exigir `https` no issuer, segredo não vazio e `client_secret` apenas para confidential client; remover/implementar `local_fallback_enabled`. *(G9, G10)*

### Slice B (P0) — "Roles" deixa de ser stub

6. Implementar os 5 endpoints de `rbac.rs` sobre `PgRoleRepository`/`PgMembershipRepository` + `RbacService`, com `AuthExtractor::require_org()` e checagem `has_permission(..., "RoleWrite")`; authorities de tenant **sempre** da sessão, nunca de header. *(G6)*
7. Testes negativos: member ≠ owner → 403; role de outra org → 403; org nil → 400; revogação efetiva no request seguinte (cache invalidado na escrita). *(G6, G11)*

### Slice C (P1) — Sessão/logout com semântica explícita

8. `logout` revoga **apenas o `jti` corrente** (manter `revoke_all_for_user` como admin action explícita, não default) e responde 401 quando não há sessão válida. *(G7)*
9. Documentar que o Seed faz **logout local**; RP-initiated logout (com `end_session_endpoint` + `id_token_hint`) fica P1 e exige coluna `id_token` na sessão. *(G7)*
10. `refresh` com rotação de `jti` e detecção de reuso usando `refresh_token_id`; ou, para debloat, remover o endpoint do contrato Seed e adotar TTL curto + re-login. *(G8)*
11. Pinar vetor RFC 7636 Appendix B nos testes de PKCE e testar replay do mesmo `state` (deve falhar). *(G12)*

### Critérios de verificação (determinísticos, sem IdP real)

| ID | Teste | Evidência esperada |
|---|---|---|
| V1 | Vetor PKCE RFC 7636 App. B | `derive_s256_challenge("dBjft…")` == `E9Melhoa…` |
| V2 | ID token com chave RSA local + JWKS fixture | passa com `iss`/`aud`/`exp`/`nonce` corretos; falha com assinatura corrompida, `iss` trocado, `aud` trocado, expirado, nonce ausente/diferente, `kid` desconhecido, `alg=HS256` assinado com chave pública (key confusion) |
| V3 | Callback com token endpoint mockado (fixture HTTP) | cria/encontra usuário por `sub`, grava sessão com `jti`; `redirect_uri` fora da allowlist → 400 |
| V4 | Org binding DB-gated | `select-organization` com membership → 200 + novo `jti`; `jti` antigo rejeitado no próximo request; sem membership → 403 |
| V5 | RBAC endpoints | owner 200; member 403; cross-org 403; revogação imediata |
| V6 | Logout/refresh | logout revoga só o dispositivo atual; refresh com `jti` reutilizado → 401 |
| V7 | Gate repo | `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test -p flora-core -p flora-api -p flora-tests` |

> O caminho de smoke determinístico já existente (`scripts/dev-smoke-session.sql` + `dev-seed.sql` + `test-flora.sh`, com bootstrap de JWT) continua válido e **não depende de OIDC real** — é a base para V3–V6 sem provisionar Zitadel.

### Fora de escopo do slice

Keycloak/Zitadel/Authentik em produção, SAML/LDAP/SCIM, MFA/passkeys, RLS em tabelas core, permissões em escopo de workspace/canal/arquivo, e e2e browser real.

---

## 5. NVs (não verificados / not verified)

- **NV-1** — Round-trip real de `state`/`nonce` pelo frontend `apps/frontend` e armazenamento de token/cookie (prior assessment indica que `state` não é persistido; re-verificar).
- **NV-2** — Fluxo OIDC/JWKS e2e contra um OP real (Zitadel/Casdoor/Authentik): descoberta, TLS, rotação de `kid`, `end_session_endpoint`. Nenhum OP foi executado nesta pesquisa.
- **NV-3** — Comportamento do callback com token endpoint mockado ainda não existe como teste; V2/V3 são propostas, não evidência executada.
- **NV-4** — Unicidade de `oidc_subject` em produção: migração atual só tem índice não-único; backfill de usuários pré-existentes precisa de plano de dados.
- **NV-5** — Semântica de logout/refresh em produção-like (multi-aba, dispositivos, `revoke_all_for_user` vs por-dispositivo, reuse detection).
- **NV-6** — Footprint real do perfil Seed em VPS 2 vCPU/4 GB com/sem OP externo; Zitadel declara 1 CPU/512 MB em teste, mas exige Postgres e tem guia de produção/HA separado ([source](https://zitadel.com/docs/self-hosting/deploy/overview)).
- **NV-7** — Efetividade de RBAC em escopo workspace/canal/arquivo (`has_permission_in_workspace` delega à org) e invalidação do cache de papéis após escrita.
- **NV-8** — Config de produção (`jwt_secret`, CORS, HTTPS atrás de proxy) e o impacto de `http://localhost:8080` como issuer default, especialmente porque a exceção de assinatura do §3.1.3.7 depende de TLS validado.
- **NV-9** — CI remota e testes DB-gated com Postgres real não executados nesta sessão (research-only).

---

## 6. Fontes

**Primary — specs / RFCs**
- [OpenID Connect Core 1.0 (errata 2)](https://openid.net/specs/openid-connect-core-1_0.html) — §2 ID Token, §3.1.1 code flow, §3.1.2.1 auth request, §3.1.3.7 ID Token Validation, §10.1 key rotation, §15.5.2 nonce.
- [OpenID Connect Discovery 1.0 (errata 2)](https://openid.net/specs/openid-connect-discovery-1_0.html) — §3 metadata, §4 well-known, §4.3 validation, §7.2 impersonation.
- [OpenID Connect RP-Initiated Logout 1.0](https://openid.net/specs/openid-connect-rpinitiated-1_0.html) — §2, §2.1, §3.
- [RFC 7636 — PKCE](https://www.rfc-editor.org/rfc/rfc7636) — §4.1/4.2/4.6, §7.1/7.2, Appendix B.
- [RFC 7519 — JWT](https://www.rfc-editor.org/rfc/rfc7519) · [RFC 7517 — JWK](https://www.rfc-editor.org/rfc/rfc7517) · [RFC 8725 — JWT BCP](https://www.rfc-editor.org/rfc/rfc8725).
- [OWASP JSON Web Token Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/JSON_Web_Token_Cheat_Sheet.html) — alg allowlist, issuer/audience confusion, header key material, revocation, token-type confusion.

**Primary — provider docs**
- [Zitadel — Deploy overview](https://zitadel.com/docs/self-hosting/deploy/overview) (test footprint, Postgres requirement, production/HA).
- [Keycloak — Configuring the hostname](https://www.keycloak.org/server/hostname) (discovery URLs, proxy headers, strict hostname).
- [Casdoor — Overview](https://casdoor.org/docs/overview) (single Go binary + SQL DB, OIDC code flow, RP-initiated logout, Casbin RBAC, localized docs).

**Local (read-only inspection + prior research)**
- `flora-api/src/routes/auth.rs`, `flora-api/src/extractors/auth.rs`, `flora-api/src/state/oidc_transactions.rs`, `flora-api/src/routes/rbac.rs`.
- `flora-core/src/config/{oidc,app}.rs`, `flora-core/src/utils/{jwt,pkce}.rs`, `flora-core/src/services/{rbac_service,auth_service}.rs`, `flora-core/src/repositories/user_repository.rs`, `flora-core/src/traits/repository.rs`.
- `flora-core/src/migrations/sql/20240818000001_init_schema.sql`, `Cargo.toml`, `scripts/dev-smoke-session.sql`, `dev-seed.sql`.
- `docs/research/2026-11-16-016-auth-rbac-oidc-minimal-vps.md`, `docs/research/2026-11-16-012-session-authority-international.md`, `docs/research/2026-11-15-flora-seed-80-20-research.md`, `.specify/assessments/006-secure-vertical-slice/research.md`.

**Localized secondary (corroboration only)**
- [OpenID — ja](https://ja.wikipedia.org/wiki/OpenID), [es](https://es.wikipedia.org/wiki/OpenID), [de](https://de.wikipedia.org/wiki/OpenID), [fr](https://fr.wikipedia.org/wiki/OpenID), [zh](https://zh.wikipedia.org/wiki/OpenID), plus Casdoor localized docs referenced above.

> All recommendations are conditional on the V1–V7 evidence being produced. Nothing in this document claims runtime compliance; see NVs.
