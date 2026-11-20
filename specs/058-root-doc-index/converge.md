# Converge: 058 — Root doc index

> Append-only. Novas entradas devem ser acrescentadas ao final; nunca reescrever,
> renumerar ou remover entradas anteriores.

## 2026-11-18T03:08Z — entrada 1

- Decisão 80/20 registrada em `STATE.md` antes do BLOCO 6: `(1×5)/(2×1)=2.5`.
  ADR-0003/0006 (5.0) escalados **[NV]** por aprovação de maintainer.
  R1 `go (scoped)`; analyze PASS na primeira execução.
- Implementação: 1 arquivo novo (`index.md`, 26 linhas) com cabeçalho datado,
  âncora `github/dev@58921a337`, 8 links relativos agrupados em Direção /
  Estado verificado / Referência e nota **[NV]**. Nenhum código/workflow alterado.
- Verificador isolado encontrou duplicação de conteúdo (contagem 135 e dívida
  dos ADRs, já em `PROGRESSO.md`) — corrigido na fonte em commit explícito e
  **reverificado: PASS** (duplicação removida, 8/8 links, 26 linhas).
- Gates locais (03:06Z): guardião PASS (8 pisos/50, total derivado);
  fmt/clippy/deny/build/test exit 0; frontend check 0/0, lint ok, 42/42, build ok.
- Factory fonte `38f682663f0121ec178bac592216501ad9fce8c3`; nada transplantado.
