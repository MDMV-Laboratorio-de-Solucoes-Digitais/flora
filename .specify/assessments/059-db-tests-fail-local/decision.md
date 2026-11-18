# Decision: 059 — DB tests fail locally

**Verdict:** go (scoped)

## Decisão

Fazer os contratos DB falharem localmente quando a env de Postgres estiver
ausente, eliminando o "passed" vazio. CI, gate e names de check intactos.

## Delimitadores

- 8 arquivos existentes em `flora-tests/src/contract/`, 9 blocos `else`.
- Artefatos do ciclo + notas datadas (append-only).
- PR independente; nenhum trabalho alheio tocado.

## NV

Runner/CI ao vivo: **[NV]** — observar no PR antes do merge (ADR-0003).
