# Shape: 061 — Dead circuit breaker

## Forma escolhida

Três edições: excluir `flora-core/src/utils/circuit_breaker.rs`; remover
os dois campos e defaults em `config/messaging.rs`; remover a variante e o
braço de match em `error.rs`. Nenhum outro arquivo é tocado.

## Alternativas descartadas

- Implementar o placeholder: sem chamadores, seria código novo inútil.
- Manter config "futura": env ignorada por Figment, sem consumidor.
- Refatorar `Error` amplamente: fora de escopo.
- ADRs: aprovação pendente.

## Risco e reversão

Risco baixo: o módulo nunca compilava; remover campos de config com Figment
sem `deny_unknown_fields` não afeta env extras. Reversão: `git revert` dos
commits (restaura o arquivo e as linhas).

## Verificação

`grep` de 0 ocorrências, fmt/clippy/deny/build/test, gates, CI ao vivo.
