# Bug test — wcag-axe-2026-11-22 (ciclo 112)

**Data**: 2026-11-22

| Bug | Verificação | Autoridade |
|---|---|---|
| BUG-111-01 | varredura axe em `/tasks` e `/channels` sem `button-name` | job `e2e-browser` (CI) |
| BUG-111-02 | varredura axe sem `color-contrast`; razão calculada 5,84:1 no tema claro | CI + cálculo HSL |
| BUG-111-03 | varredura axe sem `document-title` | job `e2e-browser` (CI) |


Na segunda rodada apareceram dois nós de `color-contrast` adicionais (marca da sidebar e canal selecionado); aguardando reexecução com resumo "sem achados" no head final — **[NV]**.

Nenhuma regra `disabledRules`/tag foi removida. O resumo "sem achados" precisa aparecer no log do job no head exato do PR — **[NV]** até execução.
