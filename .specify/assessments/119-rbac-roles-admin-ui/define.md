# Assess — define: 119

**Data**: 2026-11-23

## Escopo fechado

- Entregar UI completa em `/settings` para gerenciar **papéis e permissões RBAC** da organização ativa na sessão (`GET /api/v1/roles`, `POST /api/v1/roles`, `PATCH /api/v1/roles/{id}`, `DELETE /api/v1/roles/{id}`).
- Garantir isolamento reativo no cliente (`RoleStore` com `syncContext`, `loadSequence` e deduplicação de requisições concorrentes), exibição honesta de papéis nativos como imutáveis (`isBuiltin = true` sem ações destrutivas), e mensagens de erro/status inteiramente em `pt-BR` (mapeando `403` e `409` sem vazar texto bruto ou mensagem de bootstrap).
- Fora de escopo neste ciclo (mantido para próximo PR focado de `BF-02`): convite de usuários por e-mail (`/users/invite` não existe no backend) e matriz de atribuição/revogação de papéis por membro (`/roles/{id}/assign` e `revoke`, pois não há endpoint de listagem de membros com filtro explícito conectado).
