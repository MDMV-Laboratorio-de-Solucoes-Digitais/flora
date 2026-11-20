# Shape: 070 — contrato RBAC real no lugar do stub

## Forma escolhida

Substituir os 7 linhas do stub por um módulo `db` com:

1. `db_state()` — lê `FLORA_DATABASE__POSTGRES_URL`/`DATABASE_URL`, conecta, roda migrações, constrói `AppState` via `make_test_state`. Sem env: `anyhow::bail!` (fail-closed, ciclo 059).
2. `seed_member(state, config, permissions)` — cria usuário, organização, role com as permissões dadas, membership e sessão; retorna `(org, token, role)`.
3. Helpers `authed_request`/`json_body` usando `tower::ServiceExt::oneshot` sobre `create_rbac_router().with_state(state)`.
4. Três testes `#[tokio::test] #[ignore = "requires Postgres; ..."]`, nomeados `db_rbac_*` para casar com o filtro do leg `db-tests` em `rust.yml`.

Nenhuma linha de produção é alterada. A suíte padrão (`cargo test --workspace`) continua sem executar DB; o leg `db-tests` executa com `--ignored`.

## Riscos e mitigação

- **Risco:** identificadores/tenant errados no seed → mitigado pelo padrão idêntico a `workspace.rs` e execução real no DB descartável.
- **Risco:** lint (`unwrap_used`, `expect_used`, `panic`) → mitigado por `anyhow::bail!`/`ensure!` e `unwrap_or_default` apenas onde o padrão já é usado.
- **Risco:** alterar número de testes do leg `db-tests` e quebrar piso → mitigado sem tocar em `rust.yml`/guardian; apenas um arquivo de teste.

## Entrega

Um PR para `dev/`, título conventional em inglês, evidências coladas (incluindo execução real com DB), riscos e rollback; merge deliberado após CI verde ao vivo (ADR-0003).
