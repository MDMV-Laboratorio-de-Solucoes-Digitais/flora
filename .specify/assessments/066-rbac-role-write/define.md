# Define: 066 — RBAC RoleWrite

## Problema

Quatro handlers de RBAC são placeholders que apenas retornam strings; a
autoridade para mutar papéis nunca foi definida.

## Requisitos funcionais

- **FR-001**: `can_manage_roles` DEVE aceitar `RoleWrite` (e `OrgAdmin` por padrão).
- **FR-002**: `create_role` DEVE criar papel real com permissões validadas.
- **FR-003**: `get_role` DEVE retornar papel da org da sessão.
- **FR-004**: `assign_role`/`revoke_role` DEVEM alterar membership real.
- **FR-005**: papéis `is_builtin` NÃO DEVEM ser alterados/excluídos.
- **FR-006**: nenhuma rota nova sem autenticação/tenant da sessão.

## Critérios de sucesso

- **SC-001**: testes unitários cobrem `RoleWrite` e builtin.
- **SC-002**: fmt/clippy/deny/build/test + frontend verdes.
- **SC-003**: CI verde; diff reviewável.
- **SC-004**: nenhuma mudança em RLS/ADRs/constituição.

## Não-objetivos

- Não habilitar RLS em produção (C6).
- Não adicionar convites/email/token (fora de escopo).
