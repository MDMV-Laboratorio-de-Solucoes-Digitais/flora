# Assess — shape: 111

**Data**: 2026-11-22

## Abordagem escolhida

Estender a suíte Playwright existente com `accessibility.e2e.ts`. O teste usa a sessão já injetada pelo `seed.setup.ts`, navega em duas rotas representativas e chama `new AxeBuilder({ page }).withTags([...]).analyze()`. Achados são formatados em pt-BR na mensagem de asserção.

## Alternativas rejeitadas

- **Vitest + jsdom**: não executa layout real; `axe-core` perde regras de contraste e foco.
- **CI required check**: mudaria governança e exigiria autorização separada do SE.

## Riscos

- Falso positivo de contraste em tema escuro → mitigado ao rodar localmente antes do PR; se persistir, registrar como dívida **[NV]** em vez de relaxar regra.
- Lockfiles divergentes → sincronizar `bun.lock` e `package-lock.json` no mesmo commit.
