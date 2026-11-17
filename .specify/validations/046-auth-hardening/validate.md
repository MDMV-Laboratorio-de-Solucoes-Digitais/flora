# Validation: 046 — Endurecimento de autenticação

**Data:** 2026-11-17 · **Baseline:** `dev@226e54f` · **Tipo:** segurança (authn/authz).

## Gates

| Gate | Resultado |
|---|---|
| `cargo fmt --all -- --check` | limpo |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit 0 |
| `cargo deny check bans licenses sources` | ok |
| `cargo test --workspace` | verde (13 suítes) |
| `python3 scripts/ci/check_gate_integrity.py` | PASS (piso 43 intacto) |
| frontend `check` / `lint` / `test:unit server` / `build` | 0/0 · ok · 39/39 · ok |

## R1 — superfície RBAC autenticada

`create/get/assign/revoke role` passaram a exigir `AuthExtractor` + `require_org_admin()`
(membership + role `OrgAdmin`). Helper puro `can_manage_roles` testado (aceita `org:admin`/`*`;
rejeita `org:read`/outros). 4 testes rbac PASS.

## C2 — logout real

Backend: `revoke_session(jti)` em vez de `revoke_all_for_user` + no-op removido.
Frontend: POST `/auth/logout` best-effort + cleanup síncrono (compatível com callers).
Contrato DB `db_logout_revokes_only_current_session` PASS: token A → 401; token B íntegro.
Suíte auth **13 passed** (era 12).

## NV / follow-ups

R2 (revalidar membership em 7 handlers — sem exploit vivo), R3 (`thread_id`), 404s
frontend, FASE 7, RLS wrapper. Nenhum required check/deploy/RLS enforcement tocado.
