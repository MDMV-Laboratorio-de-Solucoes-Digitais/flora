# Bug test — wcag-axe-2026-11-22 (ciclo 112)

**Data**: 2026-11-22

| Bug | Verificação | Autoridade |
|---|---|---|
| BUG-111-01 | varredura axe em `/tasks` e `/channels` sem `button-name` | job `e2e-browser` (CI) |
| BUG-111-02 | varredura axe sem `color-contrast`; razão calculada 5,84:1 no tema claro | CI + cálculo HSL |
| BUG-111-03 | varredura axe sem `document-title` | job `e2e-browser` (CI) |


Segunda rodada: dois nós de `color-contrast` adicionais (marca da sidebar e canal selecionado) corrigidos; reexecução final no head `30d90f286` sem achados.

Nenhuma regra `disabledRules`/tag foi removida. Resumo "sem achados" observado no log do job `45048150000` no head exato do PR.
