# Release: 080 — remoção da flag OIDC morta `local_fallback_enabled`

**Ciclo**: 080 · **Data**: 2026-11-19T21:39Z · **Branch**: `feature/080-remove-dead-oidc-flag` · **Baseline**: `e3944e962`
**Tipo**: higiene de configuração (remoção de declaração write-only que prometia um fallback de segurança nunca implementado), sem mudança de comportamento, API ou contrato de leitura. **Nenhuma tag, release, deploy, merge ou produção.**

## Entregue

- `flora-core/src/config/oidc.rs` — 3 linhas removidas, sem reindentação: a doc `/// Local fallback enabled when Zitadel is unavailable (default: true in dev).` (`:18`), o campo `pub local_fallback_enabled: bool,` (`:19`) e a inicialização `local_fallback_enabled: true,` no `impl Default` (`:40`). Restam as 5 inicializações e os demais campos do struct; `allowed_redirect_uris` mantém seu `#[serde(default)]` e comentários; `validate()` e seu `Ok(())` finais inalterados.
- `Config` (`config/mod.rs:17-31`) permanece sem `deny_unknown_fields` e `Config::load` (`:43-49`) continua ignorando a chave legada `FLORA_OIDC__LOCAL_FALLBACK_ENABLED` sem erro de startup — compatibilidade por ausência, sem alias, shim ou substituto.
- Artefatos completos: assess (5), spec, clarify, plan, checklist/tasks, analyze, converge append-only, validate R4, release R7.

## SemVer

**Proposta:** patch `0.3.0` → `0.3.1` (remoção de superfície inerte em configuração, sem mudança de API, comportamento de runtime ou dependência ativa; a branch anterior 079 entregou a minor `0.2.9` → `0.3.0`). Nenhuma tag criada.

## Changelog (FR / SC / T)

- `FR-001` doc e campo `:18-19` removidos, com `code_challenge_method` seguido diretamente de `allowed_redirect_uris` (`:20-23`, `#[serde(default)]` intacto); `FR-002` inicialização `:40` removida, 5 inicializações restantes e `Ok(())` de `validate()` preservados; `FR-003` nenhum `deny_unknown_fields`, `#[serde(default)]` novo, alias de env ou substituto — chave legada ignorada sem erro; `FR-004` diff rastreado limitado a `flora-core/src/config/oidc.rs` (3 linhas) + artefatos 080, zero segredos, credenciais, dependências novas ou mudanças em gate/scripts/workflows.
- `SC-001` `grep -rn "local_fallback" --include='*.rs' flora-core/src apps` → zero; `SC-002` `cargo fmt --check`, `cargo check -p flora-core` e `cargo clippy -p flora-core --all-targets -- -D warnings` limpos; `SC-003` `check_gate_integrity.py` PASS e compatibilidade da chave legada por ausência de `deny_unknown_fields`; `SC-004` `git diff --stat` mostra só `oidc.rs` (3 linhas a menos) + artefatos 080, zero mutação de `.env.example`, auth, docs, worktree, gate, CI, DB ou segredos.
- `T001`..`T004` executados; verificador isolado `OidcFlagVerifier` **PASS** (remoção exata, sem leitores, serde compatível, diff só oidc.rs); `analyze` `{"artifacts": 11, "errors": [], "fr": 4, "go": true, "sc": 4, "tasks": 4}`; `cargo deny`, `cargo build --workspace` e `cargo test --workspace` (105/0) com `exit 0`. Nenhum bug de `.specify/bugs/` acionado; leg RBAC permanece bloqueada por maintainer (fora do ciclo).

## Evidência

`.specify/validations/080-remove-dead-oidc-flag/validate.md` · `specs/080-remove-dead-oidc-flag/converge.md` · `specs/080-remove-dead-oidc-flag/analyze.md`.

## Rollback

Reverter, em `dev/`, exclusivamente `flora-core/src/config/oidc.rs` ao estado do base `e3944e962` — restaurando a doc `/// Local fallback enabled...` (`:18`), o campo `pub local_fallback_enabled: bool,` (`:19`) e a inicialização `local_fallback_enabled: true,` (`:40`) — com novo commit de reversão, sem force-push ou reescrita de histórico. É um único arquivo de código; nenhum outro path é tocado: `config/mod.rs`, auth, `.env.example`, docs de research, worktree `state-closure-072`, gate/scripts, `.github/workflows/`, ADRs, constituição, STATE, specs alheias, DB, rede e segredos permanecem intactos. Os artefatos do ciclo 080 são documentação e podem ser revertidos em bloco.

## [NV]

Consumidores externos de `OidcConfig` e comportamento da chave legada em deploy fora do repo; CI ao vivo, Forgejo, aprovações, merge e produção; leg RBAC/maintainer. Nenhuma inferência além da evidência local consolidada no ciclo.
