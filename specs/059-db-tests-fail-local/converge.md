# Converge: 059 — DB tests fail locally

> Append-only. Novas entradas devem ser acrescentadas ao final; nunca reescrever,
> renumerar ou remover entradas anteriores.

## 2026-11-18T03:31Z — entrada 1

- Decisão 80/20 registrada em `STATE.md` antes do BLOCO 6: `(3×5)/(4×2)=1.875`.
  ADR-0003/0006 escalados **[NV]**. R1 `go (scoped)`; analyze PASS.
- Vermelho determinístico: sem env, `cargo test -p flora-tests 'contract::tasks::db::db_' -- --ignored`
  reportava **4 passed em 0.00s**, exit 0 — nenhuma asserção executada.
- Implementação: 9 blocos `else` em 8 arquivos `flora-tests/src/contract/*.rs`
  (auth ×2; files, messaging, notifications, search, tasks, websocket,
  workspace ×1) — `Ok(None)`/`eprintln!("skipping…")` substituídos por
  `anyhow::bail!("FLORA_DATABASE__POSTGRES_URL/DATABASE_URL required for DB contract tests")`.
  Os 50 chamadores `else { return Ok(()) }` permanecem intactos.
- Verificação: sem env → tasks 0 passed/4 failed e auth 0 passed/13 failed,
  exit 101; `--ignored --list` = 50; com Postgres 18.6 isolado
  (`flora_059_verify`) os 8 legs passaram (13/4/8/8/5/6/4/2 = 50). DB
  temporário removido ao final; DB `flora` alheio não tocado.
- Verificador isolado: PASS 5/5 (escopo do diff, 0 resíduos `Ok(None)`/`skipping`,
  50 chamadores intactos, lints — `bail!` não usa `panic`/`unwrap`/`expect`,
  nenhuma injeção). Encontrou divergência textual "is required" vs mensagem
  real sem "is" — corrigida em 4 artefatos com notas append-only.
- `cargo fmt --all` aplicado (bail quebrado em 2 linhas) e commitado por arquivo;
  gates finais: guardião, fmt, clippy, deny, build, test e frontend
  check/lint/42/42/build — todos exit 0.
- Factory fonte `1083d3212b595d56b15bee1186708ba34b7ea7e4`; nada transplantado.
