# Clarify: 051 — Dead invite UI

**Data:** 2026-11-17T17:30Z

## Resoluções

1. **Há falso sucesso?** Não: `client.ts` lança em não-2xx e `successMessage` só é
   setado após 2xx. O defeito é falha silenciosa + affordance morta.
2. **Implementar convite?** Não: exigiria e-mail/token e decisão de autoridade.
3. **Apagar o cenário da spec 002?** Não: marcar como não implementado, preservando o texto.
4. **Identidade 051?** `feature.json` apontava 050; `--number 051` por inventário,
   nunca pelo nome do branch.

## Pendências

Nenhum `NEEDS CLARIFICATION`.
