# Decision: 057 — Dynamic floor total

**Verdict:** go (scoped)

## Decisão

Fazer o guardião determinístico derivar de `FLOORS` a contagem e o total que
imprime, eliminando a duplicação de verdade que gerou drift em ciclos anteriores.
Nenhuma decisão de autorização, ADR ou CI é alterada.

## Delimitadores

- 1 arquivo existente, 1 linha (`scripts/ci/check_gate_integrity.py`).
- Artefatos do ciclo + notas datadas (append-only).
- PR independente; nenhum trabalho alheio tocado.

## NV

CI/PR ao vivo e aprovação para corrigir ADRs: **[NV]**.
