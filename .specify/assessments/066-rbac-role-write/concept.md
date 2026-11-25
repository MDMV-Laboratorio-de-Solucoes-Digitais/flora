# Concept: Autoridade real para mutar papéis RBAC

- **Slug**: 066-rbac-role-write
- **Created**: 2026-11-25
- **Recommended option**: CRUD real com permissão RoleWrite dedicada

## Options

### Option A — CRUD real com permissão RoleWrite dedicada
- **Sketch**: Os quatro handlers que hoje só devolvem strings passam a criar, consultar, atribuir e revogar papéis de verdade, sempre no contexto da organização da sessão. A autoridade é uma permissão específica para escrita de papéis, que administradores da organização já possuem por padrão; papéis embutidos não podem ser alterados nem excluídos. Nenhuma rota nova aparece fora desse desenho autenticado e com tenant.
- **Appetite**: medium — quatro operações, testes e ajustes no modelo de permissão; evidência de assinaturas ainda em mapeamento no research.md.
- **Trade-offs**: Ganha autoridade definida e operações que realmente persistem, fechando o vazio deixado pelos placeholders. Sacrifica a inocuidade dos endpoints: passam a mutar dados, então o risco é médio. Reversão é possível por `git revert`.
- **Rabbit holes**: Habilitar RLS em produção (C6); adicionar convites, e-mail ou tokens; permitir edição de papéis embutidos; redesenhar a hierarquia de erros.

### Option B — Manter OrgAdmin como única autoridade
- **Sketch**: Os handlers passam a funcionar, mas continuam exigindo apenas o papel de administrador da organização, sem criar uma permissão dedicada.
- **Appetite**: small.
- **Trade-offs**: Menos mudança no modelo de permissão e compatibilidade com o admin legado; mas contraria a decisão do SE, que escolheu uma permissão própria para escrita de papéis.
- **Rabbit holes**: Reintroduzir a permissão depois; criar exceções para casos que a permissão dedicada resolveria.

### Option C — Não fazer
- **Sketch**: Manter os placeholders que retornam strings após checar o administrador.
- **Appetite**: small (nenhuma mudança de arquivo).
- **Trade-offs**: Zero risco de mutação, mas a autoridade para mudar papéis continua indefinida e os handlers seguem inúteis.
- **Rabbit holes**: — (nenhum; a lacuna permanece).

## Recommendation

**Option A**. A decisão do SE registrada no intake é explícita: permissão `RoleWrite` dedicada, `OrgAdmin` habilitada por padrão e papéis `is_builtin` imutáveis. A opção atende FR-001 a FR-006 e ao critério de testes cobrindo `RoleWrite` e builtins (SC-001), preservando RLS, ADRs e constituição (SC-004). É a única que transforma os placeholders em autoridade definida sem inventar rotas autônomas.

## Out of Scope (for the recommended option)

- Habilitar RLS em produção (C6) ou alterar ADRs, constituição e deploy.
- Adicionar convites, e-mail ou tokens; mudar a lógica de autorização existente.
- Execução real em DB/PR remoto — permanece **[NV]**, observada no CI antes do merge.

## Assumptions to Validate

- A variante `RoleWrite` existe (ou precisa ser acrescentada) no modelo de permissões — o research.md ainda marca esse mapeamento como pendente.
- `revoke_role` remove a membership do usuário na organização e `assign_role` a recria, pois não há método de atualização — assunção registrada no shape.md.
- As assinaturas de `RoleRepository` e `MembershipRepository` permitem esse uso sem mudanças amplas.
