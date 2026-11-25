# Evidence — ciclo 126 quota agregada transacional

**Data/hora**: 2026-11-25T15:42Z · **Base**: `github/dev@cbf39dbb22c0` · **Worktree**: `.specify/queue/worktrees/lightsout-126` · **Branch**: `fix/126-aggregate-quota`

Roteiro: `.specify/validations/126-aggregate-quota/test-roadmap.md`. Superfície: Rust core/API + contrato DB + gate/CI. Tokens de teste sintéticos; nenhuma credencial real.

| Passo | Ação | Saída observada | Resultado | Bloco |
|---|---|---|---|---|
| 1 | Inspeção/implementação `create_with_quota` | traço; repos trait/service/floors alterados | PASS | A |
| 1 | `cargo check -p flora-core -p flora-api -p flora-tests` | 0 erros, 28 warnings de manifesto | PASS | A |
| 2 | Contrato `db_aggregate_quota_rejects_second_upload` | 1 passed / 0 failed em PG18 descartável | PASS | B |
| 2 | Suíte `contract::files::db_` | 8 passed / 0 failed | PASS | B |
| 3 | `cargo fmt --all -- --check` | RC 0 | PASS | C |
| 3 | `cargo clippy -p flora-core -p flora-api -p flora-tests --all-targets -- -D warnings` | RC 0 | PASS | C |
| 4 | `python3 scripts/ci/check_gate_integrity.py` | PASS, 89 pisos, `GATE_RC=0` | PASS | D |
| 5 | Required checks remotos | sem observação no head do PR | `[NV]` | E |
| 6 | `git diff --check` / status | RC 0; 9 paths do ciclo | PASS | F |

## Bloco A — superfície de código

```text
flora-core/src/traits/repository.rs:305-323      create_with_quota(file, org_limit_bytes, workspace_limit_bytes)
flora-core/src/repositories/file_repository.rs:114-201  transação + pg_advisory_xact_lock + SUM ativo + INSERT + commit
flora-core/src/services/file_service.rs:197-205 / 269-276  upload_file e create_file usam create_with_quota
scripts/ci/check_gate_integrity.py:48            "files": 8
.github/workflows/rust.yml:116                   run_leg files contract::files::db_ 8
```

Nenhuma migration, rota ou required check foi alterado. `Error::FileTooLarge` continua mapeando para `413` com `{error:"file_too_large"}`.

## Bloco B — contrato DB (PG18 descartável em `:15436`)

```text
TEST_START=2026-11-25T15:39:15Z
OPENERS=1
FMT_RC=0
running 1 test
test contract::files::db_files::db_aggregate_quota_rejects_second_upload ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 124 filtered out; finished in 1.07s
TEST_RC=0
TEST_END=2026-11-25T15:39:29Z

=== FILES CONTRACTS ===
cargo test: 8 passed, 117 filtered out (1 suite, 2.68s)
FILES_RC=0
```

O teste semeia quota 10 bytes, envia 6 bytes (201) e depois outros 6 bytes (413), confirma 1 linha ativa, nenhum blob `.part` em `.tmp` e apenas 1 blob final. A primeira compilação do teste falhou por `Config` não importado após o refactor `db_state_with_config`; o import foi restaurado conforme convenção dos demais contratos (`use flora_core::config::Config`) e o re-run passou.

## Bloco C — formatação, check e clippy

```text
$ cargo fmt --all -- --check
FMT_RC=0

$ cargo check -p flora-core -p flora-api -p flora-tests
cargo check: 0 errors, 28 warnings (281 crates)
CHECK_RC=0

$ cargo clippy -p flora-core -p flora-api -p flora-tests --all-targets -- -D warnings
CLIPPY_RC=0
```

Warnings restantes são do manifesto (dependências não usadas) e não foram introduzidos neste ciclo.

## Bloco D — gate determinístico

```text
2026-11-25T15:40:59Z
gate-integrity: PASS
  ok  job frontend-build present and non-vacuous
  ok  job rust-build present and non-vacuous
  ok  job db-tests present and non-vacuous
  ok  job dependency-review present and non-vacuous
  ok  rust.yml contains 'cargo build --workspace'
  ok  rust.yml contains 'cargo test --workspace'
  ok  all 11 db contract floors intact (89 total)
GATE_RC=0
2026-11-25T15:41:01Z
```

Baseline de 88 pisos passou antes da mudança (15:27:16Z→15:32:19Z).

## Bloco E — CI remoto

`[NV]`: required checks no head exato do PR serão observados após a abertura; nenhum PASS remoto é afirmado antes disso.

## Bloco F — higiene

```text
=== STATUS ===
M .github/workflows/rust.yml
M .specify/memory/STATE.md
M flora-core/src/repositories/file_repository.rs
M flora-core/src/services/file_service.rs
M flora-core/src/traits/repository.rs
M flora-tests/src/contract/files.rs
M scripts/ci/check_gate_integrity.py
?? .specify/validations/126-aggregate-quota/
=== DIFF CHECK ===
DIFFCHECK_RC=0
```

Nenhum path de outra sessão foi tocado; nenhum commit, push, merge ou deploy executado nesta worktree.
