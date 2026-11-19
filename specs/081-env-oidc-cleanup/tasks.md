# Tasks: 081 — limpeza das chaves OIDC em `.env.example`

- **T001** Em `.env.example`, substituir as linhas `:62` (`# Redirect/callback URI (verify config key support in your flow):`) e `:63` (`FLORA_OIDC__REDIRECT_URI=http://localhost:3000/auth/callback`) por 3 linhas de comentário: explicar que o callback padrão é derivado de `FLORA_APP__BASE_URL` (`flora-api/src/routes/auth.rs:101`, `{base_url}/api/v1/auth/callback`) e que URIs extras exigem `FLORA_OIDC__ALLOWED_REDIRECT_URIS` (`flora-core/src/config/oidc.rs:20-21`, allowlist exata em `auth.rs:105-117`), seguidas do exemplo **comentado** `# FLORA_OIDC__ALLOWED_REDIRECT_URIS=["http://127.0.0.1:4173/callback"]` (aspas duplas internas, sintaxe Figment 0.10.19 / `Cargo.lock:941-942`). Manter o exemplo comentado: o template não é lido em runtime (`.env.example:33-34`). *(FR-001, SC-001, SC-002)*
- **T002** Remover a linha `:65` (`# OIDC_ISSUER_URL=http://localhost:8080`), nunca lida em Rust — apenas em `specs/001-flora-seed-v0-1/contracts/OIDC.md:10,17` e docs históricos. Com T001 e T002 concluídos, `grep -n "FLORA_OIDC__\|OIDC_ISSUER_URL\|ZITADEL_" .env.example` mostra `__ISSUER_URL`, `__CLIENT_ID`, `__CLIENT_SECRET` e `__ALLOWED_REDIRECT_URIS` (comentado), com zero `FLORA_OIDC__REDIRECT_URI` e zero `OIDC_ISSUER_URL` isolado. *(FR-002, SC-001)*
- **T003** Preservar a fronteira: manter byte-idênticas as linhas `:58-61` (`FLORA_OIDC__ISSUER_URL/CLIENT_ID/CLIENT_SECRET`, leitores Figment via `config/mod.rs:43-46` → `oidc.rs:9,11,13`) e `:66-68` (`ZITADEL_*`, lidos por `std::env::var` em `oidc.rs:27,29,31`); alinhar `:64` para "Legacy fallbacks read by `OidcConfig::default()` via `std::env::var:`". Não introduzir `deny_unknown_fields`, alias, variável nova nem qualquer mudança em arquivo Rust — `Config` (`config/mod.rs:17-31`) e `Config::load` (`:43-49`) continuam ignorando a chave legada `FLORA_OIDC__REDIRECT_URI` sem erro de startup. *(FR-003, SC-004)*
- **T004** Auditar o diff e preparar o rollback: `git diff --name-only` e `git diff --stat` devem mostrar somente `.env.example` (2 linhas substituídas, 1 removida, no máximo 1 redação ajustada) e os artefatos do ciclo 081; `python3 scripts/ci/check_gate_integrity.py` passa e `cargo check -p flora-core` termina sem erro. Zero mutação de `flora-core/src/config/{mod,oidc}.rs`, `flora-api/src/routes/auth.rs`, docs/specs de research, contrato OIDC, worktree `state-closure-072`, gate/scripts, `.github/workflows/`, ADRs, constituição, STATE, DB/rede, segredos, `.tokens/`, leg RBAC e lockfiles; base `35b8d3142` preservada. Registrar `[NV]` para consumidores externos do template e CI. *(FR-004, SC-003, SC-004)*

## Rastreabilidade

| Passo | Requisito funcional | Critério de sucesso |
|---|---|---|
| T001 | FR-001 | SC-001, SC-002 |
| T002 | FR-002 | SC-001 |
| T003 | FR-003 | SC-004 |
| T004 | FR-004 | SC-003, SC-004 |

SC-001 aparece em T001 e T002 porque o `grep` só zera após a substituição em `:62-63` (remove `FLORA_OIDC__REDIRECT_URI`) **e** a remoção em `:65` (remove `OIDC_ISSUER_URL` isolado). IDs sempre individuais, sem intervalos (R4/R7).

## Gates determinísticos (locais/offline; `[NV]` até a implementação)

```
# 1. chaves OIDC no template — esperado: __ISSUER_URL, __CLIENT_ID,
#    __CLIENT_SECRET, __ALLOWED_REDIRECT_URIS (comentado); zero
#    FLORA_OIDC__REDIRECT_URI e zero OIDC_ISSUER_URL isolado
grep -n "FLORA_OIDC__\|OIDC_ISSUER_URL\|ZITADEL_" .env.example

# 2. integridade de gate e compilação (nenhum lê .env em runtime)
python3 scripts/ci/check_gate_integrity.py
cargo check -p flora-core

# 3. auditoria de escopo (base 35b8d3142)
git diff --name-only
git diff --stat
```

A auditoria espera somente `.env.example` e os artefatos do ciclo 081 (`.specify/assessments/081-env-oidc-cleanup/` e `specs/081-env-oidc-cleanup/`); qualquer outro path invalida SC-004. Os gates são executados pelo main agent; `[NV]` nesta avaliação.

Rollback: reverter `.env.example` ao estado do base `35b8d3142` (restaurando as linhas `:62-63`, `:65` e a redação original de `:64`); nenhum outro path é tocado — `flora-core/src/config/{mod,oidc}.rs`, `flora-api/src/routes/auth.rs`, docs, worktree, gate/scripts, workflows, ADRs, constituição, STATE, specs alheias, DB, rede e segredos permanecem intactos. Os artefatos 081 são documentação do próprio ciclo e podem ser revertidos em bloco. Sem force-push ou reescrita de histórico.

Convenção: um arquivo por commit; Conventional Commits em pt-BR; paths explícitos nos comandos; nunca `git add -A`/`.`. Sem `/speckit.validate` nem `/speckit.release`.
