# Research: 096 — login local pt-BR para conta bootstrap

**Data:** 2026-11-21 · **Evidência local determinística** · IdP externo **[NV]** · branch `feature/096-auth-login-ptbr@1910d1c7d`

## Primária (local)

1. `flora-core/src/config/oidc.rs:27-31` — `OidcConfig::default()` lê
   `ZITADEL_ISSUER_URL`, `ZITADEL_CLIENT_ID` e `ZITADEL_CLIENT_SECRET`, com
   defaults `http://localhost:8080`, `flora@localhost` e string vazia.
   `.env.example:66-68` mantém as três variáveis comentadas.
2. `flora-core/src/config/oidc.rs:38` — `allowed_redirect_uris: Vec::new()`;
   o campo existe (`:18-21`, `#[serde(default)]`) mas não é populado.
3. `flora-api/src/routes/auth.rs:99-116` — `issuer` vem de `oidc.issuer_url`;
   `default_redirect = format!("{}/api/v1/auth/callback", base_url)` (`:101`);
   qualquer outro `redirect_uri` fora da allowlist exata retorna
   `Error::OidcProvider("redirect_uri is not allowed")` (`:114-116`).
   A URL de autorização é montada em `:137` sobre `{issuer}/oauth/v2/authorize`.
4. `apps/frontend/src/routes/(auth)/+page.svelte:43-45` — `handleLogin()`
   envia `redirect_uri=${location.origin}/callback` e só trata o fluxo OIDC
   (`:38-72`); mensagens de erro em inglês (`:51-52,64-68`).
5. `flora-api/src/routes/bootstrap.rs:32-37` — `BootstrapRequest` tem
   `organization_name` e `admin_email`, sem senha. O `INSERT INTO users`
   (`:146-158`) grava id, email, oidc_subject, display_name, avatar_url,
   profile, is_active e timestamps — sem credencial. O token de sessão já é
   emitido dentro da transação (`:207-233,256-302`).
6. `flora-core/src/models/user.rs:13-33` — `User` não possui `password_hash`
   nem `password_updated_at`. `flora-core/src/repositories/user_repository.rs`
   localiza por e-mail/`oidc_subject`, sem consulta de senha.
7. `flora-core/src/traits/auth_provider.rs:9-12` — fallback local é citado como
   extensão futura; não existe implementação. `grep` por `argon2` /
   `password_hash` em `flora-core` + `flora-api` → apenas
   `flora-core/src/utils/validation.rs:59-105` (`validate_password`), que pode
   ser reaproveitado para complexidade.
8. `flora-core/Cargo.toml:23-52` e `flora-api/Cargo.toml:27-61` — não há crate
   de hash de senha; `argon2` + `password-hash` (PHC) precisam entrar via T001,
   com `Cargo.lock` atualizado.
9. `flora-api/src/middleware/rate_limit.rs:138-193` — já existem classes de rate
   limit (inclui `Bootstrap`); a nova classe de login local deve seguir o mesmo
   padrão (FR-006).
10. `flora-core/src/config/mod.rs:43-64` — `Config::load()` usa Figment com
    prefixo `FLORA_` e `split("__")`; `validate()` é chamado no startup. Isso
    habilita `FLORA_BOOTSTRAP__ADMIN_EMAIL` / `ADMIN_PASSWORD` sem novo
    provider. `flora-core/src/migrations/sql/` contém as migrations SQL
    versionadas executadas por `sqlx::migrate!` (`migrations/mod.rs`).
11. `apps/frontend/src/app.html:2` — `lang="en"`; a tela de login e as mensagens
    de erro estão em inglês (`+page.svelte:118-119,51-52,64-68`).
12. `flora-api/src/routes/auth.rs:247-291` — o fluxo OIDC já cria/encontra
    usuário e emite `session_token` JWT; é o contrato a replicar no login local
    (escopo organizacional, `session_token`, `expires_in`).

## Diagnóstico

Cadeia fechada: defaults OIDC sem IdP + allowlist vazia + redirect do frontend
fora do callback + conta bootstrap sem senha + ausência de rota local = usuário
existente não entra. Não há indício de falha de rede, DNS ou build intermitente.
A correção é um fallback local verificável por inspeção e testes; o IdP externo
não precisa ser configurado para que o login funcione.

## [NV]

- IdP externo (Zitadel), seus redirect URIs registrados, segredo-cliente e
  disponibilidade de rede: **[NV]**. Nenhuma chamada a provedor externo foi feita.
- Ambiente remoto/produção, valor real de `FLORA_APP__BASE_URL` e logs de runtime:
  **[NV]**. Verificação remota e gates são evidência consolidada do handoff;
  este artefato não os reexecutou.
- Cobertura idiomática: pt-BR nos artefatos e na UI; inglês em nomes de código,
  seguindo a convenção do repositório.
