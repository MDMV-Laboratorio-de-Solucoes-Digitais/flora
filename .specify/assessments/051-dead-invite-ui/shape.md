# Shape: 051 — Dead invite UI

## Forma escolhida

Excluir `UserManager.svelte`, remover import e bloco de renderização em
`settings/+page.svelte` (mantendo o grid com as duas colunas restantes), e anotar
em `specs/002-.../spec.md:34` que o cenário não foi implementado nesta baseline.

## Alternativas descartadas

- Implementar rota real: exige e-mail/token e decisão de autoridade — fora de escopo.
- Manter o formulário desabilitado: continua sendo uma affordance enganosa.
- Apenas esconder o erro: falha silenciosa persistente.

## Risco e reversão

Risco baixo: componente isolado, sem estado compartilhado. Reversão: `git revert`.

## Verificação

`rg` de ausência + gates completos.
