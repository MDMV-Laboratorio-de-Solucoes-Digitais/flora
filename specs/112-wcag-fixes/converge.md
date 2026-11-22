# Converge: 112 — correção WCAG 2.2 AA

**Data**: 2026-11-22 · **Modo**: append-only · **Status: Converged**

- FR-001: `aria-label` nos três botões ícone de `(app)/+layout.svelte`; chaves `nav.openSidebar`/`nav.closeSidebar` adicionadas ao pt-BR.
- FR-002: `disabled:opacity-50` substituído por `disabled:bg-muted disabled:text-muted-foreground`; `--muted-foreground` claro 40% (contraste calculado 5,84:1).
- FR-003: `<title>Flora</title>` em `app.html`.
- FR-004: `console.info` do resumo axe no smoke, mantendo anotações e nenhuma regra desativada.
- Verificação local: prettier OK; `bun run check` 4886 arquivos/0 erros; gate PASS (10 legs / 77 pisos).
- **[NV]**: modo escuro e staging real; verificação de UI local depende de sessão segura não fornecida.
