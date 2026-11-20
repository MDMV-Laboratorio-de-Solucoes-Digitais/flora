# Converge: 056 — Docs research index

> Append-only. Novas entradas devem ser acrescentadas ao final; nunca reescrever,
> renumerar ou remover entradas anteriores.

## 2026-11-18T02:29Z — entrada 1

- Decisão 80/20 registrada em `STATE.md` antes do BLOCO 6: `(2×5)/(3×1)=3.33`,
  maior candidato autônomo; ADR-0003 drift (5.0) escalado **[NV]** por exigir
  aprovação de maintainer. R1 `go (scoped)`; assess/spec/clarify/plan/checklist/
  tasks/analyze criados; `analyze` PASS na primeira execução.
- Implementação: 2 arquivos novos, 206 linhas (documentação, zero código):
  - `docs/research/index.md` — 135 entradas geradas deterministicamente dos
    arquivos atuais, agrupadas por data, com título do primeiro `# ` e links
    relativos; metodologia e comando de regeneração no cabeçalho.
  - `docs/PROGRESSO.md` — nota datada de estado verificado: âncora, gate,
    contagens por `find`, funcionamento verificado, dívidas e **[NV]**.
- Verificador isolado (1ª passagem) encontrou divergências reais: contagens de
  `specs/`/assessments desatualizadas pelos próprios artefatos do ciclo, redação
  da fonte que incluiria `index.md`, e afirmações remotas sem `[NV]` inline.
  Corrigido na fonte em dois commits (`docs: corrige metodologia e contagens do
  indice`, `docs: corrige contagens e marca NV inline`) e reverificado:
  **PASS** em todos os itens — 135 entradas, 135 links válidos, 0 duplicatas,
  0 ausentes; contagens 202/175/43/42/12/135/6 conferem com `find`.
- Idempotência: duas gerações consecutivas produzem o mesmo MD5.
- Gates locais (02:21–02:28Z): guardião PASS (8 pisos/50); fmt/clippy/deny/
  build/test exit 0; frontend check 0/0, lint ok, 42/42, build ok.
- Nenhum código/workflow/ADR alterado; DBs temporárias desta sessão: nenhuma.
- Factory fonte `046e1f2a8ce96a42a04b7ea3735da0251e7b1cef` (mutou desde
  `d2bd958`); nada transplantado para o alvo.
