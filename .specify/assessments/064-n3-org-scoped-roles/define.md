# Define: 064 — N3 org-scoped roles

## Problema

Resolução de papel por id puro permite que uma membership da org A use um
papel da org B; o banco não impede o ponteiro inválido.

## Requisitos funcionais

- **FR-001**: buscas de papel em rotas/serviços DEVEM ser escopadas por `organization_id`.
- **FR-002**: `roles` DEVE ter `UNIQUE(organization_id, id)`.
- **FR-003**: `memberships` DEVE ter FK composto `(organization_id, role_id)` com `ON DELETE RESTRICT` e pré-voo fail-closed.
- **FR-004**: nenhum deploy em produção e nenhuma limpeza de dados DEVE ocorrer.

## Critérios de sucesso

- **SC-001**: cross-org rejeitado em nível de aplicação (teste).
- **SC-002**: migrations aplicam em DB limpo; FK rejeita insert cross-org e órfão.
- **SC-003**: fmt/clippy/deny/build/test + frontend verdes.
- **SC-004**: auditoria local registrada; produção marcada **[NV]**.

## Não-objetivos

- Não auditar/alterar produção (sem acesso).
- Não limpar dados sujos sem volta ao SE.
- Não habilitar RLS (C6), não implementar RoleWrite (C5), não editar ADRs sem ciclos próprios.
