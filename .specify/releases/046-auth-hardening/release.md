# Release: 046 — Endurecimento de autenticação

**Ciclo:** 046 · **Data:** 2026-11-17 · **Baseline:** `dev@226e54f`.

## Entregue

- R1: 4 handlers RBAC autenticados e tenant-scoped (helper `can_manage_roles` + `require_org_admin`).
- C2: logout revoga só a sessão atual (`revoke_session(jti)`); frontend chama a API antes de limpar storage.
- Contrato DB de logout (replay → 401, outra sessão intacta); suíte auth 12→13.
- Decisão 80/20 + 6 relatórios + validação.

## Evidência

`.specify/validations/046-auth-hardening/validate.md` ·
`docs/research/2026-11-17-046-auth-hardening.md`.

## Não feito (autonomia/Autorização)

Nenhum required check, deploy, tag, segredo, RLS enforcement, ADR ou trabalho de outros agentes alterado.

## Follow-ups

1. R2: revalidar membership em tasks/messages/WS/search (helper `ensure_member` de files.rs → tenant.rs).
2. R3: validar `thread_id` contra o canal.
3. Remover chamadas mortas `/users/invite` e `/telemetry/logs` (ou abrir issues para implementar).
4. Converter `contract/rbac.rs` (stub) em contrato DB real.
5. Wrapper de transação para RLS (2 ciclos, ADR-0006) — autorização SE.
