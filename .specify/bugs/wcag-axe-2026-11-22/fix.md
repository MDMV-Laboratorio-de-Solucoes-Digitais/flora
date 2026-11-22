# Bug fix — wcag-axe-2026-11-22 (ciclo 112)

**Data**: 2026-11-22

- **BUG-111-01**: `aria-label` nos três botões ícone de `(app)/+layout.svelte`, usando `nav.openSidebar`, `nav.closeSidebar`, `nav.openAiPanel`, `nav.closeAiPanel` adicionadas ao dicionário pt-BR.
- **BUG-111-02**: `disabled:opacity-50` substituído por `disabled:bg-muted disabled:text-muted-foreground disabled:cursor-not-allowed` em `ui/button/button.svelte`; `--muted-foreground` claro de 46,9% → 40% (contraste calculado 5,84:1 sobre `--background`).

- **BUG-111-03**: `<title>Flora</title>` em `src/app.html`.
- **FR-004**: resumo axe via `process.stdout.write` (após correção do blocker `no-console`), mantendo anotações e nenhuma regra desativada.

## Segunda rodada (CI head `546b875`)

A varredura ainda encontrou `color-contrast` em `.font-bold.truncate.text-primary > span` (marca da sidebar) e em `.bg-primary/10.py-2.text-left` (canal selecionado). Corrigido localmente: marca usa `text-foreground` e canal selecionado usa `bg-primary/10 text-foreground`, preservando o realce sem depender do verde primário de baixo contraste.
