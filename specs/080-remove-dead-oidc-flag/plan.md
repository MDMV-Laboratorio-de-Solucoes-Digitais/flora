# Implementation Plan: 080 — remoção da flag OIDC morta `local_fallback_enabled`

**Entrada**: `spec.md`, `clarify.md` e `.specify/assessments/080-remove-dead-oidc-flag/{intake,research,define,concept,decision}.md` (go scoped, opção A, E=15,0).
**Ordem**: paths explícitos, um arquivo por commit, nunca `git add -A`/`.`. Convenção: Conventional Commits em pt-BR, um arquivo por commit; os artefatos do ciclo são documentação e podem ser commitados em bloco à parte.
**Base do diff**: `e3944e962` (HEAD da branch `feature/080-remove-dead-oidc-flag` no início do ciclo).

## Rastreabilidade

| Passo | Requisito funcional | Critério de sucesso |
|---|---|---|
| T001 | FR-001 | SC-001 |
| T002 | FR-002 | SC-001, SC-002 |
| T003 | FR-003 | SC-003 |
| T004 | FR-004 | SC-004 |

Cada célula referencia identificadores individuais — não intervalos. Cobertura: FR-001, FR-002, FR-003, FR-004 e SC-001, SC-002, SC-003, SC-004 aparecem ao menos uma vez. SC-001 aparece em T001 e T002 porque o `grep` só retorna zero após as duas remoções (`:18-19` em T001 e `:40` em T002).

## Passos

1. **T001** Em `flora-core/src/config/oidc.rs`, remover a doc `/// Local fallback enabled when Zitadel is unavailable (default: true in dev).` e a linha `pub local_fallback_enabled: bool,` (atual `:18-19`). Conferir que `code_challenge_method: String` (`:16-17`) passa a ser seguido diretamente pelo bloco de `allowed_redirect_uris` (`:20-23`), com seu `#[serde(default)]` e os dois comentários intactos. O `grep` de SC-001 ainda não zera neste ponto (`:40` permanece); não commitar isoladamente — T001 e T002 incidem sobre o mesmo arquivo e formam um único commit de 3 linhas removidas ao final de T002. *(FR-001, SC-001)*
2. **T002** Remover a linha `local_fallback_enabled: true,` do `impl Default for OidcConfig` (atual `:40`), mantendo as demais 5 inicializações (`:29-41`) e o `Ok(())` de `validate()` (`:73`). Confirmar que `code_challenge_method: "S256".to_owned(),` e `allowed_redirect_uris: Vec::new(),` ficam adjacentes. Rodar o `grep` de SC-001 (zero) e os gates de compilação/estilo de SC-002 (`cargo check -p flora-core`, `cargo fmt --check`, `cargo clippy -p flora-core --all-targets -- -D warnings`). Commitar somente `flora-core/src/config/oidc.rs` com as 3 linhas removidas (`git add flora-core/src/config/oidc.rs`; nunca `git add -A`/`.`). *(FR-002, SC-001, SC-002)*
3. **T003** Verificar, sem editar, que nenhum substituto entrou: `Config` (`flora-core/src/config/mod.rs:17-31`) continua sem `deny_unknown_fields`, `Config::load` (`:43-49`) mantém `Serialized::defaults(Self::default())` + `Env::prefixed("FLORA_").split("__")` e nenhum `#[serde(default)]` novo/alias de env/shim foi adicionado (o `#[serde(default)]` de `allowed_redirect_uris:22` é pré-existente e preservado). Rodar `python3 scripts/ci/check_gate_integrity.py` e exigir gate do alvo PASS. Qualquer diff em `config/mod.rs`, auth, `.env.example`, gate ou workflows invalida o passo. *(FR-003, SC-003)*
4. **T004** Auditar o escopo com `git diff --stat e3944e962..HEAD` e `git diff --name-only e3944e962..HEAD`: somente `flora-core/src/config/oidc.rs` (3 linhas a menos, sem reindentação) e os artefatos do ciclo 080. Confirmar byte-idênticos os demais paths (auth, `.env.example`, docs, worktree, gate/scripts/workflows, ADRs, constituição, STATE, DB, segredos, lockfiles). Registrar os `[NV]` em `spec.md`/`clarify.md`, confirmar zero mutação de DB/segredos e validar a reversão do único path conforme a seção Rollback. *(FR-004, SC-004)*

## Gates determinísticos

```
# 0. base de comparação (raiz do repo)
BASE=e3944e962

# 1. zero leitores da flag em Rust (raiz) — falha se imprimir algo
grep -rn "local_fallback" --include='*.rs' flora-core/src apps

# 2. compilação, formatação e lint do crate alvo (raiz)
cargo check -p flora-core
cargo fmt --check
cargo clippy -p flora-core --all-targets -- -D warnings

# 3. integridade de gate (raiz, sem DB/rede)
python3 scripts/ci/check_gate_integrity.py

# 4. nenhum substituto introduzido (raiz) — falha se imprimir algo
grep -rn "deny_unknown_fields\|local_fallback" flora-core/src/config/mod.rs

# 5. artefatos sensíveis byte-idênticos ao base (raiz)
git diff --exit-code "$BASE" -- \
  flora-core/src/config/mod.rs \
  flora-core/src/auth.rs \
  flora-core/src/services/auth_service.rs \
  .env.example \
  scripts/ci/check_gate_integrity.py \
  .github/workflows

# 6. auditoria de escopo do diff (raiz)
git diff --stat "$BASE"..HEAD
git diff --name-only "$BASE"..HEAD
```

A auditoria de escopo espera somente `flora-core/src/config/oidc.rs` (3 linhas removidas) e `specs/080-remove-dead-oidc-flag/` (mais os artefatos de assessment já registrados em `.specify/assessments/080-remove-dead-oidc-flag/`); qualquer outro path invalida SC-004. O gate 2 também é sensível à remoção correta da inicialização em `:40`: deixá-la gera erro de campo desconhecido no struct. Todos os comandos acima são determinísticos e não dependem de DB, segredos ou rede; sua execução real fica com o main agent após a edição e é `[NV]` nesta spec (marcadores em SC-002 e SC-003 de `spec.md`).

## Rollback

Reverter o commit que introduziu T001/T002 ou restaurar diretamente o único path ao base:

```
git checkout e3944e962 -- flora-core/src/config/oidc.rs
```

Isso devolve as 3 linhas (doc+campo em `:18-19` e `local_fallback_enabled: true,` em `:40`), restaurando `oidc.rs` byte-idêntico ao base. Nenhum `config/mod.rs`, auth, `.env.example`, doc, worktree, gate/script, workflow, ADR, constituição, STATE, DB, rede ou segredo é modificado. Os artefatos do ciclo 080 são documentação e podem ser revertidos em bloco. Sem force-push ou reescrita de histórico; após a reversão, repetir os gates 1, 2, 3, 5 e 6 para confirmar o retorno ao estado base.
