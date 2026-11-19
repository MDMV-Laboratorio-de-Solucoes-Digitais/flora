# Define: limpeza das chaves OIDC em `.env.example`

- **Slug**: 081-env-oidc-cleanup · **Definido**: 2026-11-19T22:06Z

## Problema

`.env.example:62-63` documenta `FLORA_OIDC__REDIRECT_URI` como se configurasse o callback OIDC, mas a chave não tem campo em `OidcConfig` (`flora-core/src/config/oidc.rs:7-22`) e é silenciosamente descartada pelo Figment (`config/mod.rs:17-31,43-49`). O callback real deriva de `FLORA_APP__BASE_URL` (`flora-api/src/routes/auth.rs:101`) e URIs extras usam `FLORA_OIDC__ALLOWED_REDIRECT_URIS` (`oidc.rs:20-21`, allowlist exata em `auth.rs:105-117`) — nenhum dos dois aparece no template. `.env.example:65` repete `# OIDC_ISSUER_URL`, lida apenas em contrato histórico (`specs/001-flora-seed-v0-1/contracts/OIDC.md:10,17`), nunca em Rust. O resultado é drift de config (registrado em `docs/research/2026-11-16-016-macro-files-ops-parity.md:32`) que induz operadores a configurarem uma chave inerte e a não encontrarem a chave que funciona. A correção é documental e localizada em um arquivo.

## Afetados / metas / não-metas

Afetados: maintainers, operadores e agentes que copiam `.env.example`; nenhum runtime (o template não é lido em execução — `.env.example:33-34`). Metas: em `.env.example`, substituir `:62-63` por comentário que explique `FLORA_OIDC__ALLOWED_REDIRECT_URIS` e inclua o exemplo comentado `# FLORA_OIDC__ALLOWED_REDIRECT_URIS=["http://127.0.0.1:4173/callback"]`; remover `:65`; opcionalmente realinhar `:64` para "Legacy fallbacks read by `OidcConfig::default()` via `std::env::var:`". Não-metas: `flora-core/src/config/{mod,oidc}.rs`, `flora-api/src/routes/auth.rs`, as demais linhas do bloco OIDC (`:58-61,66-68`), o exemplo de CORS (`:31-39`), docs/specs de research, contrato `specs/001-.../contracts/OIDC.md`, worktree espelho `state-closure-072`, gate/`scripts/ci/check_gate_integrity.py`, `.github/workflows/`, ADRs, constituição, STATE, specs/081-* (exceto os 5 artefatos), DB/rede, segredos, `.tokens/`, leg RBAC e lockfiles.

## Requisitos funcionais

- **FR-001** — substituir, em `.env.example`, as linhas atuais `:62` (`# Redirect/callback URI (verify config key support in your flow):`) e `:63` (`FLORA_OIDC__REDIRECT_URI=http://localhost:3000/auth/callback`) por comentário explicando que o callback padrão é derivado de `FLORA_APP__BASE_URL` (`flora-api/src/routes/auth.rs:101`, `{base_url}/api/v1/auth/callback`) e que URIs extras exigem `FLORA_OIDC__ALLOWED_REDIRECT_URIS` (`oidc.rs:20-21`, `auth.rs:105-117`), seguido do exemplo **comentado** `# FLORA_OIDC__ALLOWED_REDIRECT_URIS=["http://127.0.0.1:4173/callback"]` (aspas duplas internas, sintaxe Figment 0.10.19 / `Cargo.lock:941-942`).
- **FR-002** — remover a linha atual `:65` (`# OIDC_ISSUER_URL=http://localhost:8080`), que nunca é lida em Rust (apenas `specs/001-flora-seed-v0-1/contracts/OIDC.md:10,17` e docs históricos).
- **FR-003** — manter byte-idênticas as linhas `:58-61` (`FLORA_OIDC__ISSUER_URL/CLIENT_ID/CLIENT_SECRET`, com leitores Figment via `config/mod.rs:43-46` → `oidc.rs:9,11,13`) e `:66-68` (`ZITADEL_*`, lidas por `std::env::var` em `oidc.rs:27,29,31`), além de `:64` salvo pela alteração opcional de redação. Não introduzir `deny_unknown_fields`, alias, variável nova nem qualquer mudança em arquivo Rust: uma chave legada `FLORA_OIDC__REDIRECT_URI` exportada por algum deploy continua sendo ignorada sem erro de startup (`config/mod.rs:17-31,43-49`).
- **FR-004** — limitar o diff rastreado a `.env.example` (2 linhas substituídas, 1 removida, no máximo 1 redação ajustada em `:64`), além dos 5 artefatos `.specify/assessments/081-env-oidc-cleanup/`; nenhum outro arquivo de código, config, doc, gate, CI ou lockfile é modificado; base `35b8d3142` preservada em todo o resto.

## Métricas

- **SC-001** — `grep -n "FLORA_OIDC__\|OIDC_ISSUER_URL\|ZITADEL_" .env.example` mostra `FLORA_OIDC__ISSUER_URL`, `__CLIENT_ID`, `__CLIENT_SECRET` e `FLORA_OIDC__ALLOWED_REDIRECT_URIS` (comentado), e **zero** ocorrências de `FLORA_OIDC__REDIRECT_URI` e de `OIDC_ISSUER_URL` fora da sequência `ZITADEL_ISSUER_URL`. `[NV]` depende da edição.
- **SC-002** — a linha de exemplo permanece **comentada** e usa aspas duplas internas idênticas a `.env.example:35`; descomentá-la produz um array Figment válido (`figment 0.10.19`) que `Vec<String>` aceita. O callback derivado de `FLORA_APP__BASE_URL` continua válido e a allowlist exata de `auth.rs:105-117` não é duplicada com o callback padrão.
- **SC-003** — `git diff --name-only` lista somente `.env.example` e os 5 artefatos 081; `python3 scripts/ci/check_gate_integrity.py` passa e o gate segue PASS; `cargo check -p flora-core` termina sem erro — nenhum desses comandos lê `.env` em runtime (`.env.example:33-34`). `[NV]` não executado nesta avaliação.
- **SC-004** — nenhuma mudança de comportamento de auth: `flora-core/src/config/oidc.rs`, `flora-core/src/config/mod.rs` e `flora-api/src/routes/auth.rs` permanecem byte-idênticos; zero mutação de docs de research, contrato OIDC, worktree, gate, CI, DB ou segredos; consumidores externos do template e CI `[NV]` — não verificáveis localmente, mas sem quebra de leitura de config.

## Baseline

Branch `feature/081-env-oidc-cleanup`, HEAD `35b8d3142`, gate PASS, zero PRs. `.env.example:58-68` com `__REDIRECT_URI` e `# OIDC_ISSUER_URL` vivas e sem menção a `ALLOWED_REDIRECT_URIS`; `flora-core/src/config/oidc.rs:7-22,24-41` e `flora-api/src/routes/auth.rs:101,105-117` lidos e citados, não editados; `Cargo.lock:941-942` figment 0.10.19; research/registro de drift presente.
