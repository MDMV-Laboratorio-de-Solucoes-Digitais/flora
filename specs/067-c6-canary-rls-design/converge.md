# Converge: 067 — C6 canário RLS

> Append-only. Novas entradas devem ser acrescentadas ao final; nunca reescrever,
> renumerar ou remover entradas anteriores.

## 2026-11-18T15:54Z — entrada 1

- Autorização SE prévia em `STATE.md` (13:04Z / 14:38Z / 14:50Z); decisão 80/20
  `E = (5×5)/(3×1) = 8.33` registrada às 15:49:43Z, antes do BLOCO 6.
- R1 `decision.md` com `go (scoped)`; R2/R3 `clarify.md` PASS; R4/R7 pendentes
  até o Bloco 7 (serão registrados em `validate.md`/`release.md`).
- Entregável: `docs/research/2026-11-18-067-c6-canary-rls-design.md` — estado
  verificado (11 tabelas ENABLE, 0 policies/GUC/papéis/wrapper), pré-condição
  ADR-0006(a) descrita e não implementada, 5 fases de canário, SQL de referência,
  matriz (a)–(d) com `[NV]`, rollback em 6 passos sem dado destrutivo.
- Refresh append-only: `docs/PROGRESSO.md` (contagens por `find`: 58 specs,
  55 assessments, 53 validations, 53 releases, 4 bugs, 138 research, 6 ADRs;
  8 pisos / 51; PR #285 merged observado ao vivo) e `docs/research/index.md`
  (seção 2026-11-18 com 2 entradas + dívida de indexação retroativa registrada).
- **Fronteira preservada:** nenhum código, workflow, migration, ADR, constituição,
  required check, credencial, deploy, tag ou produção alterado. Nenhum `FORCE`/
  `CREATE POLICY` aplicado em runtime.
- Scripts SDD da factory (HEAD `c81f0f8191d9dfa1285001bdd45f2a8fddd8dac3`)
  materializados apenas em runtime para leitura; sem baseline transplantado.
- Factory lida, nunca modificada.
