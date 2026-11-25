# Roteiro de testes — ciclo 125 revalidação de sessão no WebSocket

**Slug**: `125-ws-session-revalidation` · **Data**: 2026-11-25 · **Base**: `github/dev@4f619cb28e8d`

## Escopo / IDs

Candidato escolhido em `.specify/memory/STATE.md:1377-1383`: revalidação de sessão pós-handshake no WS (`432/3125`). Consumidor = membro autenticado cuja sessão é revogada (logout/rebaixamento) com `/api/v1/ws` aberto. Superfícies: handler Rust (token retido, TTL de 30 s, close `1008`), contrato DB, piso do gate e job `db-tests` do CI. Fora de escopo: frontend, migrations, Valkey, contrato REST, deploy/promoção.

## Pré-condições

- Worktree isolada `.specify/queue/worktrees/lightsout-125`, branch `fix/125-ws-session-revalidation`, HEAD `4f619cb28e8d`.
- `verify_bearer_token` (`flora-api/src/extractors/auth.rs:33`) já é a autoridade DB-backed de sessão; nenhuma credencial real em artefato.
- Gate autoritativo: `python3 scripts/ci/check_gate_integrity.py`; piso `websocket` 8→9 em lockstep com `.github/workflows/rust.yml`.
- Este worker não executa gate agregado nem `cargo test` completo: baseline do gate já está em execução na janela principal.

## Passos

1. **Estrutura do fix** — `verify_ws_session` público compara user/org do token verificado; `websocket_handler` retém o token e o passa a `handle_socket`; revalidação com `Instant`/`Duration` de 30 s antes de ações do cliente (exceto Ping) e antes de encaminhar eventos do hub; `401` envia `Error{SESSION_INVALID}` + `Close(1008)` e encerra o loop; `500` apenas loga. Automatizável: sim (inspeção/grep). Captura: terminal/evidence.md.
2. **Contrato DB** — novo `db_ws_revoked_after_connect_rejected` com `#[ignore = "requires Postgres; ..."]`: sessão ativa → `verify_ws_session` Ok → `state.session_service.revoke_session(&jti)` → `StatusCode::UNAUTHORIZED`. Comando: `cargo test -p flora-tests contract::websocket::db_ws_revoked_after_connect_rejected -- --ignored --test-threads=1` com `FLORA_DATABASE__POSTGRES_URL` apontando para Postgres com migrations aplicadas. Automatizável: sim (depende de PG). Captura: terminal; se PG local estiver contaminado, registrar `blocked` e usar `db-tests` remoto como autoridade.
3. **Formatação e lints** — `rustfmt --edition 2024 --check` nos dois arquivos Rust alterados; clippy pedantic é responsabilidade do job `rust-build` remoto. Automatizável: sim. Captura: terminal.
4. **Floors em lockstep** — `FLOORS["websocket"] = 9` e `run_leg websocket contract::websocket::db_ 9`; conferir que o total passa de 87 para 88 pisos. Comando do orquestrador: `python3 scripts/ci/check_gate_integrity.py` (com `TMPDIR` gravável). Automatizável: sim. Captura: terminal.
5. **CI remoto** — required checks no head exato do PR (`rust-build`, `db-tests` em especial). Marcar `[NV]` até observação ao vivo; nenhum PASS local é afirmado sem execução. Automatizável: não por este worker. Captura: URL do run.
6. **Higiene** — `git diff --check`, verificação de que apenas os 8 paths do ciclo foram tocados, nenhum commit/push/merge/deploy. Automatizável: sim. Captura: terminal.

## Rollback

Reverter os 8 paths do ciclo (handler, contrato, floors, workflow e artefatos/notas) via commit reverso explícito; nenhuma migration, frontend, ADR, constituição ou credencial é alterada. Em caso de falha do job `db-tests`, reverificar o lockstep floor/teste antes de qualquer promoção.
