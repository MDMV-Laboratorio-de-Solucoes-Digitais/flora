# Intake: 059 — DB tests fail locally

**Data:** 2026-11-18T03:25Z · **Âncora:** `github/dev@2730fd2dd` · branch `feature/059-db-tests-fail-local`

## Sinal

Sem `FLORA_DATABASE__POSTGRES_URL`/`DATABASE_URL`, os 9 helpers `db_state()`
devolvem `Ok(None)` e os 50 testes `#[ignore]` retornam `Ok(())` — o libtest
conta `passed` sem nenhuma asserção. O CI já é fail-closed (ciclo 054), mas
a execução local continua enganosa.

## Evidência (verificada)

- `grep -rn 'return Ok(None);' flora-tests/src/contract/*.rs` → **9**
  ocorrências em 8 arquivos; `return Ok(())` → **50**.
- 7 dos 9 têm `eprintln!("skipping DB-backed …")`; `tasks.rs:42` e
  `workspace.rs:44` não têm mensagem.
- Comando executado na worktree (03:24Z):
  `env -u FLORA_DATABASE__POSTGRES_URL -u DATABASE_URL cargo test -q -p flora-tests 'contract::tasks::db::db_' -- --ignored`
  → `test result: ok. 4 passed; 0 failed` em **0.00s**, exit 0 — nenhum DB tocado.
- `anyhow.workspace = true` em `flora-tests/Cargo.toml:31` — `anyhow::bail!`
  disponível sem nova dependência.

## 80/20

`(I×C)/(E×R)` = (3×5)/(4×2) = **1.875**, maior candidato autônomo restante.
ADR drift (5.0) escalado **[NV]**; N3 1.0; dívidas de fase 0.67; C5/C6 não autônomos.

## Fronteiras

Escopo: 9 blocos `else` em 8 arquivos `flora-tests/src/contract/*.rs`,
artefatos do ciclo e notas datadas. Fora: CI, gate, ADRs, constituição,
C5/C6, deploy. Factory fonte `1083d321…`, nada transplantado.
