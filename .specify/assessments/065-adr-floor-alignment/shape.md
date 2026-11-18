# Shape: 065 — ADR floor alignment

## Forma escolhida

Três edições literais em 2 ADRs: a lista de pisos e o total em 0003:19, e
as duas citações de "piso 43" em 0006:10,52. Autorização e divergência
50→51 registradas em `STATE.md` (apêndice).

## Alternativas descartadas

- Mudare gate para 50: contrariaria `FLOORS` real (51).
- Reescrever notas históricas: append-only.
- Tocar constituição: fora da autorização.

## Risco e reversão

Risco nulo: documentação. Reversão: `git revert` dos commits.
Nenhuma credencial envolvida.

## Verificação

`grep`, guardião, gates, verificador.
