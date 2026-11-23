# Assess — research (Bloco 4a): 118

**Data**: 2026-11-23 · **Decisão**: Pesquisa externa dispensada (nesta entrega não há mudança de algoritmo criptográfico nem de provedor OIDC externo — toda a base RFC 6749/RFC 7636 PKCE S256, OIDC Core 1.0 §3.1.3 e APIs locais de `jsonwebtoken 11.0.0` já existe no código e no `Cargo.lock`).

## Achados locais verificados

1. `flora-tests/Cargo.toml` já declara `wiremock`, `axum`, `tokio`, `reqwest`, `tower`, `http`, `url`; falta apenas expor `base64.workspace = true`, `jsonwebtoken.workspace = true` e `serde_urlencoded.workspace = true` (todas já declaradas em `[workspace.dependencies]` do `Cargo.toml` raiz — zero crates novos no `Cargo.lock` e zero impacto de `cargo deny`).
2. `jsonwebtoken 11.0.0` (verificado diretamente no fonte local do registry):
   - Suporta `Algorithm::EdDSA`, `EncodingKey::from_ed_der(48 bytes PKCS#8 v1)`, `Jwk::from_encoding_key(&key, Algorithm::EdDSA)` (produzindo `kty = "OKP"`, `crv = "Ed25519"`, `alg = "EdDSA"`, `x = ...`) e `DecodingKey::from_jwk(&jwk)`.
   - `flora_api::oidc::fetch_jwks` (`flora-api/src/oidc.rs:116-163`) busca `GET {issuer}/.well-known/openid-configuration`, exige `jwks_uri` começando por `https://`, `http://localhost` ou `http://127.0.0.1`, e então busca `GET {jwks_uri}` — logo um servidor `axum` local em `http://127.0.0.1:<porta>` é aceito em testes mantendo a proteção contra SSRF/HTTP inseguro para hosts externos.
3. `flora_core::utils::pkce::derive_s256_challenge` é `pub` em `flora-core`, permitindo que o `MockOidcIdpServer` valide de fato `derive_s256_challenge(&code_verifier) == stored.code_challenge` no `POST /oauth/v2/token` segundo RFC 7636.
