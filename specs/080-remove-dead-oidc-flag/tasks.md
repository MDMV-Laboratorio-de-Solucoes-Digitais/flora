# Tasks: 080 — remoção da flag OIDC morta `local_fallback_enabled`

- **T001** Em `flora-core/src/config/oidc.rs`, remover a doc `/// Local fallback enabled when Zitadel is unavailable (default: true in dev).` e a linha `pub local_fallback_enabled: bool,` (`:18-19`), deixando `code_challenge_method` seguido diretamente de `allowed_redirect_uris` (`:20-23`, com seu `#[serde(default)]` intacto). O `grep` de SC-001 só zera após T001 **e** T002, pois a inicialização em `:40` ainda menciona o campo. *(FR-001, SC-001)*
- **T002** No `impl Default for OidcConfig` (`flora-core/src/config/oidc.rs:26-44`), remover `local_fallback_enabled: true,` (`:40`), mantendo as demais 5 inicializações (`:29-41`) e o `Ok(())` final de `validate()` (`:73`) inalterados. Com T001 concluída, `grep -rn "local_fallback" --include='*.rs' flora-core/src apps` retorna zero; verificar `cargo check -p flora-core`, `cargo fmt --check` e `cargo clippy -p flora-core --all-targets -- -D warnings`. *(FR-002, SC-001, SC-002)*
- **T003** Manter a fronteira intacta: não introduzir `deny_unknown_fields`, `#[serde(default)]`, alias de env ou substituto; `Config` (`flora-core/src/config/mod.rs:17-31`) e `Config::load` (`:43-49`) continuam ignorando a chave legada `FLORA_OIDC__LOCAL_FALLBACK_ENABLED` sem erro de startup. Verificar `python3 scripts/ci/check_gate_integrity.py` e a aceitação de ambiente com a chave legada (`[NV]` até a execução). *(FR-003, SC-003)*
- **T004** Auditar o diff e preparar o rollback: `git diff --name-only e3944e962` e `git diff --stat e3944e962` devem mostrar somente `flora-core/src/config/oidc.rs` (3 linhas removidas) e os artefatos do ciclo 080; zero mutação de `.env.example`, auth, docs de research, worktree `state-closure-072`, gate/scripts, `.github/workflows/`, ADRs, constituição, STATE, DB/rede, segredos, `.tokens/`, issues, leg RBAC e lockfiles. Registrar `[NV]` para gates pós-edição e consumidores externos de `OidcConfig`. *(FR-004, SC-004)*

## Rastreabilidade

| Passo | Requisito funcional | Critério de sucesso |
|---|---|---|
| T001 | FR-001 | SC-001 |
| T002 | FR-002 | SC-001, SC-002 |
| T003 | FR-003 | SC-003 |
| T004 | FR-004 | SC-004 |

SC-001 aparece em T001 e T002 porque o `grep "local_fallback"` só zera após as duas remoções (`:18-19` em T001 e `:40` em T002). IDs sempre individuais, sem intervalos (R4/R7).

## Gates determinísticos (locais/offline; `[NV]` até a implementação)

```
# 1. zero leitores em Rust
grep -rn "local_fallback" --include='*.rs' flora-core/src apps

# 2. compilação, formatação e lint
cargo check -p flora-core
cargo fmt --check
cargo clippy -p flora-core --all-targets -- -D warnings

# 3. integridade de gate (sem DB/rede)
python3 scripts/ci/check_gate_integrity.py

# 4. auditoria de escopo (base e3944e962)
git diff --name-only e3944e962
git diff --stat e3944e962
```

A auditoria espera somente `flora-core/src/config/oidc.rs` (3 linhas removidas) e os artefatos do ciclo 080 (`.specify/assessments/080-remove-dead-oidc-flag/` e `specs/080-remove-dead-oidc-flag/`); qualquer outro path invalida SC-004. As únicas menções restantes a `local_fallback` no repo são 3 docs de research históricos e a cópia em worktree `state-closure-072`, fora de escopo. Os gates são executados pelo main agent; `[NV]` nesta avaliação.

Rollback: reverter `flora-core/src/config/oidc.rs` ao estado do base `e3944e962` (restaurando a doc `:18`, o campo `:19` e a inicialização `:40`); nenhum outro path é tocado — `config/mod.rs`, auth, `.env.example`, docs, worktree, gate/scripts, workflows, ADRs, constituição, STATE, specs alheias, DB, rede e segredos permanecem intactos. Os artefatos 080 são documentação do próprio ciclo e podem ser revertidos em bloco. Sem force-push ou reescrita de histórico.

Convenção: um arquivo por commit; Conventional Commits em pt-BR; paths explícitos nos comandos; nunca `git add -A`/`.`. Sem `/speckit.validate` nem `/speckit.release`.
