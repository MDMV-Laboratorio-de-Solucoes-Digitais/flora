# Assess — research: 112

**Data**: 2026-11-22 · **Cobertura idiomática**: pt-BR (primária) + en (WCAG/axe)

## Evidência local

- `app.html` sem elemento `<title>` — BUG-111-03.
- `routes/(app)/+layout.svelte`: botão `size="icon"` de alternância da sidebar sem nome acessível — BUG-111-01; dois botões do painel de IA também sem `aria-label`.
- `ui/button/button.svelte:7`: `disabled:opacity-50` produz contraste 3.73:1 sobre `#f8fafc` — BUG-111-02.
- `routes/layout.css:52`: `--muted-foreground` com 46,9% de luminosidade.
- Log CI: run 45038205571, job 137256545911 (2026-11-22).

## Fontes primárias

- WCAG 2.2, critérios 1.4.3, 2.4.2, 4.1.2 — https://www.w3.org/TR/WCAG22/ (W3C, 2023-10-05).
- axe-core rule descriptions `button-name`, `color-contrast`, `document-title` — https://github.com/dequelabs/axe-core/blob/develop/doc/rule-descriptions.md (acesso 2026-11-22).

## [NV]

Staging real; execução local da UI (credenciais de staging solicitadas na PR Forgejo #133).
