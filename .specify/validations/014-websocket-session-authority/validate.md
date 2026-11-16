# Validation: 014 — WebSocket Session Authority & Protocol Alignment (80/20)

**Branch:** `feat/014-websocket-session-authority`  
**Base:** `dev@994f8be3f` (post 013/014-R1 scaffolding)  
**Date:** 2026-11-16

## Scope (delivered)
- **Backend (`flora-api`)**: `extractors::auth::verify_bearer_token`/`VerifiedSession` compartilhados (JWT + `validate_session(jti)` + ativo/expiração/user/org, 401/500 genéricos) reutilizados por extractors HTTP; `routes/websocket.rs` resolve token de `Authorization: Bearer` (preferido) ou `?token=` (fallback browser), valida sessão DB-backed **antes** do upgrade, rejeita nil-org, mantém checagens de org no canal e envelopes/handlers existentes; rotas `/ws` e `/api/v1/ws`.
- **Frontend (`apps/frontend`)**: `WsClient` conecta em `/api/v1/ws` (ws/wss), suporta token via query, parse seguro de envelopes tagueados (`Subscribed/Unsubscribed/NewMessage/MessageSent/Pong/Error`), sem dependência de `/messages/sync`, falha de auth/policy interrompe reconnect e aciona `lockSession()`, backoff limitado com jitter para desconexões retentáveis, ciclo de vida em `(app)/+layout` com auth/org/token (root sem conexão duplicada); testes unit WS adicionados.
- **Testes/CI**: `flora-tests/src/contract/websocket.rs` (`db_ws_*`: válido, revogado, expirado, ausente, mismatch user/org, nil-org, sem token; handshake 101/401 — válido aceita 101 ou 426 quando upgrade não extraível no harness oneshot após auth bem-sucedida), `contract/mod.rs` e `.github/workflows/rust.yml` `db-tests` inclui `contract::websocket::db_`.

## Deterministic verification (local)
- `cargo fmt --all -- --check` — pass
- `cargo check -p flora-api -p flora-core -p flora-tests` — pass
- `cargo clippy -p flora-api -p flora-core -p flora-tests --all-targets -- -D warnings` — pass
- `cargo test -p flora-core --lib` — **8 passed**
- `cargo test -p flora-api` — **11 passed, 1 ignored**
- `cargo test -p flora-tests --lib` — **51 passed, 23 ignored**
- DB-gated (Postgres 18 descartável, `127.0.0.1:15435`, 5432 ocupado externamente): `contract::websocket::db_` — **8 passed**; `contract::auth::db_` — **12 passed** (regressão); `contract::messaging::db_` — **3 passed** (regressão)
- Frontend: `bun run check` — 0 erros/avisos; `bun run lint` — pass; `bun run test:unit -- --run --project server` — **8 arquivos, 22 testes**; `bun run build` — pass

## NV / limitations
- CI remoto/merge pendentes (PR).
- Evidência DB local em PG18 descartável (porta alternativa).
- **Não-goals mantidos:** fanout/registry/broadcast multi-cliente, `/messages/sync`, replay/catch-up, eventos realtime expandidos, Valkey/NOTIFY push, anexos, DMs, busca/notificações, OIDC/JWKS e browser e2e, `apps/web`.
- Janela residual de revogação pós-handshake (sem revalidação periódica nesta fatia) documentada como tradeoff do slice A.
