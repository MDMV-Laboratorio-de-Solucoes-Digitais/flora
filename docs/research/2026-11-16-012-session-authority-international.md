# Session authority & contract confidence — international research (cycle 012)

**Date:** 2026-11-16  
**Scope:** server-side session authority for JWTs, per-request validation, revocation, frontend error contract, Macro debloat context.  
**Method:** authoritative primary sources (OWASP, RFCs, Axum 0.8, sqlx/Postgres docs, Macro official docs/repo) plus prior multilingual 011 research. Non-English primary material for Axum+sqlx session authority is sparse; where it exists it is treated as secondary to official EN docs.

## 1. Key findings

| Topic | Evidence | Implication for Flora |
|---|---|---|
| JWT signature is not session authority | OWASP JWT Cheat Sheet — *JWT revocation / Token Status List / denylist by `jti`*; RFC 7519 §4.1.7 (`jti`) | A valid JWT remains valid until `exp` unless server state is checked. Flora must consult the `sessions` row. |
| Per-request session lookup is an accepted tradeoff | Axum 0.8 docs/`FromRequestParts`; codeloopback Axum auth guide: extractor does one DB lookup and roles/tenant come from DB; WorkOS/Own-Auth/JWT-revocation analyses | Extractor with an indexed `jti` lookup is the minimal correct pattern for a single-app Postgres deployment. |
| Revocation must be immediate for logout/org switch | OWASP; Jsonic revocation guide (allowlist = delete row, next request fails) | Revoked `jti` must fail on the next protected request; no TTL-based delay. |
| Caching trades revocation latency | Jsonic; Redis JWT blog | No cache in 012. A short-TTL cache would delay revocation; only consider with measured load and explicit invalidation. |
| Error leakage | OWASP ASVS auth guidance; Axum error handling docs | Return generic 401 for token/session failures; keep detailed reason in `tracing` only. DB failure is a generic 500 (fail-closed). |
| Token/TTL hygiene | WorkOS JWT best practices; RFC 8725 | Keep access JWT short-lived; refresh rotation already present. Session row expiry is checked independently of JWT `exp`. |
| Frontend error contract | MDN Fetch (fetch does not reject on 4xx/5xx; body stream once); SvelteKit errors docs | Parse body before branching on 401; preserve status/code/field; centralize `lockSession()`. |

## 2. Authoritative sources

- OWASP JSON Web Token Cheat Sheet — JWT revocation, Token Status List, `jti`/iss denylist: https://cheatsheetseries.owasp.org/cheatsheets/JSON_Web_Token_Cheat_Sheet.html
- RFC 7519 JSON Web Token (§4.1.7 `jti`, §10 security considerations): https://www.rfc-editor.org/rfc/rfc7519
- RFC 8725 JSON Web Token Best Current Practice: https://www.rfc-editor.org/rfc/rfc8725
- OWASP Session Management Cheat Sheet: https://cheatsheetseries.owasp.org/cheatsheets/Session_Management_Cheat_Sheet.html
- OWASP ASVS — Authentication / Session Management: https://owasp.org/www-project-application-security-verification-standard/
- Axum 0.8 middleware / `FromRequestParts` docs: https://docs.rs/axum/0.8.9/axum/extract/trait.FromRequestParts.html
- Axum middleware guide: https://docs.rs/axum/0.8.9/axum/middleware/index.html
- codeloopback — Authentication and Authorization in Axum (server-side session lookup in extractor): https://codeloopback.dev/articles/axum-authentication-authorization
- sqlx Postgres docs (pool/query patterns): https://docs.rs/sqlx/latest/sqlx/
- PostgreSQL 18 `uuidv7()` / indexing docs: https://www.postgresql.org/docs/18/functions-uuid.html
- Jsonic — JWT revocation strategies (allowlist/denylist/versioning tradeoffs): https://jsonic.io/guides/jwt-revocation-strategies
- WorkOS — JWT best practices (revocation options): https://workos.com/blog/jwt-best-practices
- MDN Fetch API (localized variants PT-BR/ES/DE/FR/zh-CN/ja/ko available for base fetch semantics): https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API/Using_Fetch
- Macro official docs/repo (feature scope and AGPL self-host notes): https://github.com/macro-inc/macro , https://docs.macro.com/

### Localized secondary sources (base web platform semantics)
- PT-BR: MDN *Usando Fetch*: https://developer.mozilla.org/pt-BR/docs/Web/API/Fetch_API/Using_Fetch
- ES: MDN *Uso de Fetch*: https://developer.mozilla.org/es/docs/Web/API/Fetch_API/Using_Fetch
- DE: MDN *Verwenden der Fetch API*: https://developer.mozilla.org/de/docs/Web/API/Fetch_API/Using_Fetch
- FR: MDN *Utiliser l’API Fetch*: https://developer.mozilla.org/fr/docs/Web/API/Fetch_API/Using_Fetch
- zh-CN: MDN *使用 Fetch*: https://developer.mozilla.org/zh-CN/docs/Web/API/Fetch_API/Using_Fetch
- ja: MDN *フェッチ API の使用*: https://developer.mozilla.org/ja/docs/Web/API/Fetch_API/Using_Fetch
- ko: MDN *Fetch API 사용하기*: https://developer.mozilla.org/ko/docs/Web/API/Fetch_API/Using_Fetch

## 3. Synthesis for 012

1. Use one async Axum extractor/helper that decodes the JWT **and** looks up `jti` in Postgres.
2. Accept only when: row exists, `is_active`, `expires_at > now()`, `session.user_id == claims.sub`, `session.organization_id == claims.organization_id` (nil-org pre-selection included).
3. Fail closed: generic 401 for invalid/revoked/expired/mismatched; generic 500 when DB unavailable. No detailed reason to clients.
4. No cache initially. One indexed read per protected request is acceptable for SMB/VPS; revisit only with measurements and explicit invalidation/TTL policy.
5. Frontend parses the body before 401 handling so `ApiError` preserves `code/message/body` while still locking the session.
6. Deterministic DB tests must prove valid vs revoked vs expired vs missing vs user/org mismatch and nil-org pass-through.

## 4. Uncertainties / NV

- Real OIDC/JWKS callback e2e is out of scope (deterministic bootstrap used in smoke).
- WebSocket upgrade session authority is not covered by the HTTP extractor and is a follow-up.
- Host port `5432` may be occupied in this environment; DB tests use disposable Postgres 18 on an alternate port.
- Non-EN implementation-grade primary sources for “Axum + DB session revocation” were limited; official EN docs plus RFC/OWASP were treated as authoritative.
