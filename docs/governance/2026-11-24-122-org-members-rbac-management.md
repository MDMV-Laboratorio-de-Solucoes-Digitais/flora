# Governança 122 — Gestão de membros e papéis RBAC em `/settings`

**Data**: 2026-11-24 · **Ciclo**: 122 · **Fechamento**: `BF-02 -> fechado`

## Decisões Arquiteturais e de Segurança

1. **`MembershipRepository::update_role` atômico**: substitui a remoção+reinserção em `assign_role`, preservando `joined_at` e `metadata` e garantindo atualização exclusiva dentro de `(user_id, organization_id)`.
2. **Endpoints `/api/v1/roles/members` + hardening `assign_role`/`revoke_role`**: autoridade granular (`OrgRead|OrgAdmin` para leitura; `RoleWrite|OrgAdmin` para mutação), bloqueio de auto-rebaixamento/auto-revogação (`403`) e verificação de pertencimento do papel alvo na organização ativa.
3. **Frontend Svelte 5 (`MemberStore.svelte.ts` + `MemberManager.svelte`)**: isolamento multi-contexto por `syncContext`/`loadSequence`, `encodeURIComponent(roleId)` nas chamadas HTTP e acessibilidade WCAG 2.2 AA (`pt-BR`).
4. **Lockstep Gate DB (`85 -> 87`)**: incremento da perna `rbac` de `3` para `5` em `scripts/ci/check_gate_integrity.py` e `.github/workflows/rust.yml`.
