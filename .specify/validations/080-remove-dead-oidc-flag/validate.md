# Validation: 080 — remoção da flag OIDC morta `local_fallback_enabled`

**Data**: 2026-11-19T21:39Z · **Branch**: `feature/080-remove-dead-oidc-flag` · **Baseline**: `e3944e962`
**Veredito R4**: **APROVADO**.

## Gates e verificações

| Gate | Resultado (saída colada) |
|---|---|
| `git diff --name-only e3944e962` | `flora-core/src/config/oidc.rs` — exatamente 1 path de código (restante: artefatos do ciclo 080) |
| `git diff --stat e3944e962` | `flora-core/src/config/oidc.rs \| 3 ---` — `1 file changed, 3 deletions(-)` |
| `git diff e3944e962 -- flora-core/src/config/oidc.rs` | 3 remoções: doc `/// Local fallback enabled...` (`:18`), campo `pub local_fallback_enabled: bool,` (`:19`) e inicialização `local_fallback_enabled: true,` (`:40`); sem reindentação |
| `grep -rn "local_fallback" --include='*.rs' flora-core/src apps` | zero correspondências (SC-001) |
| `cargo fmt --check` | `exit 0` — sem reindentação; edição é somente remoção de linhas (SC-002) |
| `cargo check -p flora-core` | `exit 0` (SC-002) |
| `cargo clippy -p flora-core --all-targets -- -D warnings` | `exit 0` — nenhum warning novo (SC-002) |
| `cargo deny check bans licenses sources` | `bans ok, licenses ok, sources ok`; `exit 0` |
| `cargo build --workspace` | `exit 0` |
| `cargo test --workspace` | `passed=105 failed=0`; `exit 0` |
| `python3 scripts/ci/check_gate_integrity.py` | `PASS` — gate do alvo íntegro; `exit 0` (SC-003) |
| `specs/080-remove-dead-oidc-flag/analyze.md` | `{"artifacts": 11, "errors": [], "fr": 4, "go": true, "sc": 4, "tasks": 4}` · `exit=0` |
| Verificador isolado `OidcFlagVerifier` | **PASS** — remoção exata (doc+campo+default), zero leitores em Rust, serde compatível (sem `deny_unknown_fields`; chave legada `FLORA_OIDC__LOCAL_FALLBACK_ENABLED` ignorada sem erro de startup), diff restrito a `oidc.rs` |

### Saídas coladas

Auditoria de escopo observada contra a base `e3944e962`:

```
$ git diff --name-only e3944e962
flora-core/src/config/oidc.rs

$ git diff --stat e3944e962
 flora-core/src/config/oidc.rs | 3 ---
 1 file changed, 3 deletions(-)
```

Remoção exata em `flora-core/src/config/oidc.rs`:

```
@@ -15,8 +15,6 @@ pub struct OidcConfig {
     pub scopes: Vec<String>,
     /// PKCE code challenge method (default: "S256").
     pub code_challenge_method: String,
-    /// Local fallback enabled when Zitadel is unavailable (default: true in dev).
-    pub local_fallback_enabled: bool,
     /// Additional exact-match redirect URIs accepted by `/auth/login`.
     /// The configured application callback is always allowed without listing it here.
     #[serde(default)]
@@ -37,7 +35,6 @@ impl Default for OidcConfig {
                 "email".to_owned(),
             ],
             code_challenge_method: "S256".to_owned(),
-            local_fallback_enabled: true,
             allowed_redirect_uris: Vec::new(),
         }
     }
  +0 -3
```

Analisador (`specs/080-remove-dead-oidc-flag/analyze.md`):

```
{"artifacts": 11, "errors": [], "fr": 4, "go": true, "sc": 4, "tasks": 4}
exit=0
```

Nota de proveniência: esta passagem de documentação não reexecutou gates, build, testes ou formatters — as saídas acima são a evidência consolidada do ciclo (agente implementador + verificador isolado `OidcFlagVerifier`, **PASS**). Nenhuma inferência além dela.

## Fronteira

Somente `flora-core/src/config/oidc.rs` mudou no código: 3 linhas removidas (doc `:18`, campo `:19` e inicialização `:40`), sem reindentação; `code_challenge_method` passa a ser seguido diretamente de `allowed_redirect_uris`, com seu `#[serde(default)]` e comentários preservados; `validate()` segue com as mesmas verificações e o `Ok(())` final. Todo o restante são artefatos do ciclo 080. `config/mod.rs`, código de auth (`auth.rs`, `services/auth_service.rs`), `.env.example`, docs de research históricos, worktree `state-closure-072`, gate/`scripts/ci/check_gate_integrity.py`, `.github/workflows/`, ADRs, constituição, `.specify/memory/STATE.md`, DB/rede, segredos, `.tokens/` e lockfiles permanecem byte-idênticos e intocados. Nenhum `deny_unknown_fields`, alias de env, novo `#[serde(default)]` ou substituto foi introduzido (FR-003).

## [NV]

Consumidores externos de `OidcConfig` (serialização/leitura fora do repo, sem consumidor de `Serialize` sobre config no repo e sem shim de compatibilidade); comportamento da chave legada `FLORA_OIDC__LOCAL_FALLBACK_ENABLED` em deploy fora do repo; CI ao vivo, Forgejo, aprovações, merge, produção e fechamento de issues; leg RBAC/maintainer e a cópia em worktree `state-closure-072` com os 3 docs de research históricos (fora do escopo por SC-001/SC-004). Nenhuma inferência além da evidência local.
