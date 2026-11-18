# Clarify: 064 — N3 org-scoped roles

**Data:** 2026-11-18T11:16Z

## Resoluções

1. **É multi-tenancy?** Sim: o tenant é `organizations`; papéis são por org e
   memberships ligam usuário+org+papel.
2. **Deploy em produção?** Não autorizado — apenas código e migrations.
3. **Dados sujos?** Não limpar; pré-voo fail-closed e retorno ao SE.
4. **`ON DELETE`?** `RESTRICT`: excluir papel em uso falha em vez de remover
   memberships silenciosamente.
5. **Identidade 064?** `feature.json` aponta 051; NNN por inventário/`--dry-run`.

## Pendências

Nenhum `NEEDS CLARIFICATION`.
