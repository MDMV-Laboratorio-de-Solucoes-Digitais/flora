# Evidence — ciclo 125 revalidação de sessão no WebSocket

**Data/hora**: 2026-11-25T08:20Z · **Base**: `github/dev@4f619cb28e8d` · **Worktree**: `.specify/queue/worktrees/lightsout-125` · **Branch**: `fix/125-ws-session-revalidation`

Roteiro: `.specify/validations/125-ws-session-revalidation/test-roadmap.md`. Nenhuma credencial real em artefato.

| Passo | Ação | Saída observada | Resultado | Bloco |
|---|---|---|---|---|
| 1 | Estrutura do fix | token retido, TTL 30 s, close `1008`, serviço indisponível preserva socket | PASS | A |
| 2 | Contrato DB | `db_ws_revoked_after_connect_rejected` 1 passed / 0 failed com PG18 fresco | PASS | B |
| 3 | `cargo fmt --all -- --check` | RC 0 | PASS | C |
| 3 | `cargo check -p flora-api` | 0 erros, 22 warnings de manifesto | PASS | C |
| 3 | `cargo clippy -p flora-api -p flora-tests --all-targets -- -D warnings` | RC 0 (warnings de dependências não usadas no manifesto) | PASS | C |
| 4 | `python3 scripts/ci/check_gate_integrity.py` | PASS, 88 pisos, `GATE_RC=0` | PASS | D |
| 5 | Required checks remotos | sem observação ao vivo neste head | `[NV]` | E |
| 6 | `git diff --check` / escopo | RC 0; 8 paths do ciclo | PASS | F |

## Bloco A — handler

```text
flora-api/src/routes/websocket.rs:117  SESSION_REVALIDATION_TTL = 30 s
flora-api/src/routes/websocket.rs:130  pub verify_ws_session(token, state, user_id, org_id) -> Result<(), StatusCode>
flora-api/src/routes/websocket.rs:164  extract_org_context devolve token próprio + user_id + org_id
flora-api/src/routes/websocket.rs:185  on_upgrade passa o token retido a handle_socket
flora-api/src/routes/websocket.rs:447  handle_client_message (ping tratado antes da revalidação)
flora-api/src/routes/websocket.rs:514  close_invalid_session: Error{SESSION_INVALID} + Close(1008, "SESSION_INVALID")
flora-api/src/routes/websocket.rs:534  revalidate_before_action: TTL, 401 encerra, 500 apenas loga
flora-api/src/routes/websocket.rs:562  handle_socket revalida antes de ações e antes do fanout do hub
```

O token nunca aparece em mensagens `tracing`; apenas `user_id`, `org_id` e status. Nenhuma configuração nova foi introduzida.

## Bloco B — contrato DB (PG18 descartável em `:15435`)

```text
TEST_START=2026-11-25T08:17:49Z
running 1 test
test contract::websocket::db_websocket::db_ws_revoked_after_connect_rejected ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 123 filtered out; finished in 0.87s
TEST_RC=0
TEST_END=2026-11-25T08:17:51Z
```

O teste semeia sessão ativa, chama `verify_ws_session` (Ok), revoga via `state.session_service.revoke_session(&jti)` e exige `StatusCode::UNAUTHORIZED` na segunda chamada — sem novo handshake. O banco foi um container `postgres:18-alpine` temporário, removido ao final; o Postgres compartilhado em `:5432` não foi tocado.

## Bloco C — formatação, check e clippy

```text
$ cargo fmt --all -- --check
FMT_RC=0

$ cargo check -p flora-api
cargo check: 0 errors, 22 warnings (275 crates)
CHECK_RC=0

$ cargo clippy -p flora-api -p flora-tests --all-targets -- -D warnings
CLIPPY_RC=0
```

Warnings restantes são do manifesto (dependências não usadas) e não foram introduzidas neste ciclo.

## Bloco D — gate determinístico

```text
2026-11-25T08:16:34Z
gate-integrity: PASS
  ok  job frontend-build present and non-vacuous
  ok  job rust-build present and non-vacuous
  ok  job db-tests present and non-vacuous
  ok  job dependency-review present and non-vacuous
  ok  rust.yml contains 'cargo build --workspace'
  ok  rust.yml contains 'cargo test --workspace'
  ok  all 11 db contract floors intact (88 total)
GATE_RC=0
2026-11-25T08:17:16Z
```

Piso `websocket` 8→9 em `check_gate_integrity.py` e `.github/workflows/rust.yml`, em lockstep. Baseline de 87 pisos passou antes da mudança (07:52:52Z→08:00:24Z).

## Bloco E — CI remoto

`[NV]`: required checks no head exato do PR serão observados após a abertura; nenhum PASS remoto é afirmado antes disso.

## Bloco F — higiene

```text
=== STATUS ===
M .github/workflows/rust.yml
M .specify/memory/STATE.md
M docs/PROGRESSO.md
M flora-api/src/routes/websocket.rs
M flora-tests/src/contract/websocket.rs
M scripts/ci/check_gate_integrity.py
?? .specify/validations/125-ws-session-revalidation/
=== DIFF CHECK ===
DIFFCHECK_RC=0
```

Nenhum path de outra sessão foi tocado; nenhum commit, push, merge ou deploy foi executado nesta worktree.
