# Analyze: 067 — C6 canário RLS

**Data:** 2026-11-18T15:54Z · **Modo:** READ-ONLY

## Saída executada (primeira passagem, antes de `analyze.md`/`converge.md`)

```
missing artifacts: ['analyze.md', 'converge.md']
FR/SC ids: 11 ['FR-001', ..., 'FR-007', 'SC-001', ..., 'SC-004']
tasks ids: 9 ['T001', ..., 'T009']
tasks unique: True
go=True: True
NV marked: True
```

## Correção na fonte e reexecução

Conforme a regra ("se apontar conflito, corrija na fonte e rode de novo"), os
dois artefatos apontados foram criados — `converge.md` (append-only) e este
`analyze.md` — e a verificação foi repetida:

```
missing artifacts: none
FR/SC ids: 11 (FR-001..FR-007, SC-001..SC-004)
tasks ids: 9 (T001..T009)
tasks unique: True
go=True: True
NV marked: True
policy/FORCE applied in runtime: 0 (grep)
code/workflow/migration/ADR/constitution changed: 0
```

## Resultado

**PASS — sem conflitos.** R1 satisfeito pela autorização SE e pelo `go` registrados
antes do BLOCO 6; nenhuma fonte de código alterada nesta fase. Nenhuma "correção"
consistiu em reescrever artefato alheio ou renumerar converge.
