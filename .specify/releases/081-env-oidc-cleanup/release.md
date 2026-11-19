# Release: 081 — limpeza das chaves OIDC em `.env.example`

**Ciclo**: 081 · **Data**: 2026-11-19T22:20Z · **Branch**: `feature/081-env-oidc-cleanup` · **Baseline**: `35b8d3142`
**Tipo**: higiene de documentação de configuração (remoção de chave inerte `__REDIRECT_URI` e do alias isolado `OIDC_ISSUER_URL`, com explicação do callback derivado de `FLORA_APP__BASE_URL` e do uso da allowlist), sem mudança de comportamento, API ou contrato de leitura. **Nenhuma tag, release, deploy, merge ou produção.**

## Entregue

- `.env.example` — `+4/−4` em 1 path: `:62-63` substituídas por 3 linhas de comentário (callback padrão derivado de `FLORA_APP__BASE_URL` via `auth.rs:101`; URIs extras exigem `FLORA_OIDC__ALLOWED_REDIRECT_URIS` via `oidc.rs:20-21` e `auth.rs:105-117`), com o exemplo `# FLORA_OIDC__ALLOWED_REDIRECT_URIS=["http://127.0.0.1:4173/callback"]` mantido **comentado** (aspas duplas internas, sintaxe Figment `0.10.19` para `Vec<String>`); `:64` reescrita para `# Legacy fallbacks read by OidcConfig::default() via std::env::var:` (`:65`); `:65` original (`# OIDC_ISSUER_URL=http://localhost:8080`) removida. `:59-61` (`FLORA_OIDC__*`) e `:66-68` (`ZITADEL_*`) byte-idênticos.
- `Config` (`config/mod.rs:17-31`) permanece sem `deny_unknown_fields` e `Config::load` (`:43-49`) continua ignorando a chave legada `FLORA_OIDC__REDIRECT_URI` sem erro de startup — compatibilidade por ausência, sem alias, shim ou substituto. Nenhum leitor Rust mudou: `flora-core/src/config/{mod,oidc}.rs` e `flora-api/src/routes/auth.rs` byte-idênticos ao base.
- Artefatos completos: assessment (5), spec, clarify, plan, checklists/tasks, analyze, converge append-only, validate R4, release R7.

## SemVer

**Proposta:** patch `0.3.1` → `0.3.2` (mudança puramente documental em template de configuração, sem mudança de API, comportamento de runtime ou dependência ativa; a branch anterior 080 entregou `0.3.0` → `0.3.1`). Registro opcional a critério do maintainer. Nenhuma tag criada.

## Changelog (FR / SC / T)

- `FR-001` comentário novo em `:62-64` nomeia `FLORA_APP__BASE_URL` como origem do callback padrão e `FLORA_OIDC__ALLOWED_REDIRECT_URIS` como caminho para URIs extras, com exemplo comentado (Nit 1 do `EnvVerifier` corrigido por `EnvNitFix` em `:63`: `# inside). Exact match; the callback from FLORA_APP__BASE_URL is always allowed.`); `FR-002` `:65` (`# OIDC_ISSUER_URL=http://localhost:8080`) removida — zero `FLORA_OIDC__REDIRECT_URI` e zero `OIDC_ISSUER_URL` isolado; `FR-003` leitores preservados, nenhum `deny_unknown_fields`, alias, variável nova ou shim, chave legada ignorada sem erro de startup; `FR-004` diff rastreado limitado a `.env.example` (`+4/−4`) + artefatos 081, zero segredos, credenciais, dependências novas ou mudanças em Rust/auth/gate/workflows.
- `SC-001` `grep -n "FLORA_OIDC__\|OIDC_ISSUER_URL\|ZITADEL_" .env.example` → só `__ISSUER_URL`/`__CLIENT_ID`/`__CLIENT_SECRET` (`:59-61`), `__ALLOWED_REDIRECT_URIS` comentado (`:64`) e `ZITADEL_*` (`:66-68`); `SC-002` exemplo comentado com aspas duplas válidas, URI extra distinta do callback padrão e allowlist de `auth.rs:105-117` não duplicada; `SC-003` `check_gate_integrity.py` PASS e byte-identidade dos três arquivos de código contra `35b8d3142` (Nit 2 — 12 arquivos não rastreados vs 11 nomeados, informativo, registrado no `converge.md`, sem edição de spec); `SC-004` auditoria de escopo mostra só `.env.example` + artefatos 081, zero mutação de Rust, auth, docs, worktree, gate, CI, DB ou segredos.
- `T001`..`T004` executados; verificador isolado `EnvVerifier` **PASS-WITH-NITS** (ambos os nits tratados: 1 corrigido, 2 informativo); `analyze` `{"artifacts": 11, "errors": [], "fr": 4, "go": true, "sc": 4, "tasks": 4}`; `cargo check -p flora-core` `exit 0`. Nenhum bug de `.specify/bugs/` acionado; leg RBAC permanece bloqueada por maintainer (fora do ciclo).

## Evidência

`.specify/validations/081-env-oidc-cleanup/validate.md` · `specs/081-env-oidc-cleanup/converge.md` · `specs/081-env-oidc-cleanup/analyze.md`.

## Rollback

Reverter, em `dev/`, exclusivamente `.env.example` ao estado do base `35b8d3142` — restaurando `# Redirect/callback URI (verify config key support in your flow):` (`:62`), `FLORA_OIDC__REDIRECT_URI=http://localhost:3000/auth/callback` (`:63`), a redação original de `:64` e `# OIDC_ISSUER_URL=http://localhost:8080` — com:

```
git checkout 35b8d3142 -- .env.example
```

Novo commit de reversão, sem force-push ou reescrita de histórico. É um único arquivo de template; nenhum outro path é tocado: `flora-core/src/config/{mod,oidc}.rs`, `flora-api/src/routes/auth.rs`, docs de research, contrato OIDC, worktree `state-closure-072`, gate/scripts, `.github/workflows/`, ADRs, constituição, STATE, specs alheias, DB, rede e segredos permanecem intactos. Os artefatos do ciclo 081 são documentação e podem ser revertidos em bloco. Após a reversão, repetir os gates 1 a 5 de `plan.md` para confirmar o retorno ao estado base.

## [NV]

Consumidores externos do template e comportamento de deploy fora do repo que ainda exporta a chave legada; CI ao vivo, Forgejo, aprovações, merge, tag e produção; leg RBAC/maintainer. Nenhuma inferência além da evidência local consolidada no ciclo.
