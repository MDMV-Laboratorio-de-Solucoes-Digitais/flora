# Intake: 066 — RBAC RoleWrite

**Data:** 2026-11-18T14:39Z · **Âncora:** `github/dev@8d0cb64b0` · branch `feature/066-rbac-role-write`

## Sinal

`flora-api/src/routes/rbac.rs:131-164` tem quatro handlers placeholder
(`create_role`, `get_role`, `assign_role`, `revoke_role`) que retornam strings
após checar `require_org_admin`. `GET /roles` já é real desde o ciclo 042.
A autoridade para criar/atribuir papéis não estava definida.

## Decisão SE

- Permissão **`RoleWrite`** dedicada;
- `OrgAdmin` deve tê-la **habilitada por padrão**;
- papéis `is_builtin` são **imutáveis** (premissa registrada, não contradita).

## Evidência

- `grep -rn 'RoleWrite'` em `flora-core/src/models` — resultado do verificador pendente.
- Gate PASS (8 pisos/51); nenhum PR/worktree alheio.

## 80/20

`(I×C)/(E×R)` = (4×5)/(4×2) = **2.5**, último item de código autorizado.
C6 (canário) = 3.33 mas é documento, sem código/produção.

## Fronteiras

Escopo: `rbac.rs`, modelo de permissão se necessário, serviços/repositórios,
testes e artefatos. Fora: RLS (C6), ADRs, constituição, deploy, produção.
Factory `c81f0f81…`, nada transplantado.
