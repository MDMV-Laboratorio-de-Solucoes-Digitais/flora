# Converge: 057 — Dynamic floor total

> Append-only. Novas entradas devem ser acrescentadas ao final; nunca reescrever,
> renumerar ou remover entradas anteriores.

## 2026-11-18T02:48Z — entrada 1

- Decisão 80/20 registrada em `STATE.md` antes do BLOCO 6: `(3×5)/(2×1)=7.5`.
  ADR-0003/0006 (5.0) escalados **[NV]** por exigirem aprovação de maintainer.
  R1 `go (scoped)`; analyze PASS na primeira execução.
- Implementação: 1 linha em `scripts/ci/check_gate_integrity.py:120` — literal
  `"all eight … (50 total)"` substituído por f-string derivada de
  `len(FLOORS)` e `sum(FLOORS.values())`. Regex, mensagens de falha, `FLOORS`
  e demais lógica intactos.
- A saída real passou a ser `all 8 db contract floors intact (50 total)`
  (numeral em vez da palavra `eight`); correção registrada na fonte em
  `define.md`/`spec.md` antes do fechamento.
- Verificações: guardião PASS; mutação em `/tmp` com 9º piso + leg →
  `all 9 db contract floors intact (54 total)` (prova da derivação);
  verificador isolado PASS 5/5 (diff de 1 linha, saída, consistência
  `FLOORS`↔`run_leg`, risco de regressão, conteúdo não-confiável).
- Gates locais (02:46Z): guardião PASS, fmt/clippy/deny/build/test exit 0;
  frontend check 0/0, lint ok, 42/42, build ok.
- ADRs seguem com o texto antigo (43) — dívida de governança escalada, não editada.
- Factory fonte `38f682663f0121ec178bac592216501ad9fce8c3`; nada transplantado.
