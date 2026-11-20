# Decision: 067 — C6 canário RLS

**Verdict:** go (scoped — autorização SE prévia em `STATE.md` 13:04Z / 14:38Z / 14:50Z)

## Decisão

Produzir o desenho de canário RLS com rollback, restrito a DB descartável e a
documentação, sem habilitar enforcement em produção, sem alterar required checks,
credenciais, deploy ou ADRs. Nenhuma autorização adicional é necessária para
**documentar**; qualquer execução futura (Fases 2–4) exige autorização explícita
do SE conforme ADR-0006 §3.

## Delimitadores

- 1 documento de pesquisa + artefatos do ciclo + refresh de 2 índices de docs.
- Zero arquivos de código, workflow, migration, ADR ou constituição.
- `STATE.md` recebe apenas apêndice datado (decisão 80/20 + fechamento).

## NV

Execução real do canário, A/B em app, produção e CI remota pós-merge: **[NV]** —
CI observada ao vivo no Bloco 7 antes do merge.
