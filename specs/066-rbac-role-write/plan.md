# Implementation Plan: 066 — RBAC RoleWrite

**Data:** 2026-11-18T14:41Z · **Baseline:** `github/dev@8d0cb64b0`

## Abordagem

1. DTOs serde + imports (StatusCode, Deserialize, FromStr, Membership).
2. `can_manage_roles` → `RoleWrite || OrgAdmin`; ajustar testes.
3. Implementar create/get/assign/revoke + PATCH/DELETE com guarda builtin.
4. Validar permission strings com `Permission::from_str`.
5. Gates + CI.

## Riscos

Endpoints antes no-op agora mutam dados — comportamento desejado e autorizado.
Sem mudança de contrato público fora do módulo.

## Verificação

`cargo test`, fmt/clippy/deny/build, frontend, CI.

## Fora do plano

RLS/produção, ADRs, constituição, convites.
