# Decision: 063 — Validate OSV run

**Verdict:** go (scoped)

## Decisão

Disparar uma execução manual do workflow OSV já declarado no repositório e
registrar o resultado determinístico, fechando o `[NV]` de 052. Nenhuma
autorização adicional é necessária: o gatilho `workflow_dispatch` é parte do
próprio workflow e a ação não é merge/force-push/deleção.

## Delimitadores

- 1 apêndice em `STATE.md`, artefatos do ciclo.
- PR independente; nenhum trabalho alheio tocado.

## NV

Resultado da execução antes de obtido: **[NV]** — preenchido com evidência real.
