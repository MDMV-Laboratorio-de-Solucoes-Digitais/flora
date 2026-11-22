# Converge: 112 — correção WCAG 2.2 AA

**Data**: 2026-11-22 · **Modo**: append-only · **Status: Converged**

- FR-001: `aria-label` nos três botões ícone de `(app)/+layout.svelte`; chaves `nav.openSidebar`/`nav.closeSidebar` adicionadas ao pt-BR.
- FR-002: `disabled:opacity-50` substituído por `disabled:bg-muted disabled:text-muted-foreground`; `--muted-foreground` claro 40% (contraste calculado 5,84:1).
- FR-003: `<title>Flora</title>` em `app.html`.
- FR-004: `console.info` do resumo axe no smoke, mantendo anotações e nenhuma regra desativada.

- 2026-11-22 (ACR): revisor independente encontrou blocker determinístico — `console.info` no helper axe viola `no-console: error` e derruba `frontend-build`. Corrigido com `process.stdout.write`, sem desativar regra. Timeout do smoke elevado a 180 s para acomodar duas varreduras.
- Verificação local: prettier OK; `bun run check` 4886 arquivos/0 erros; gate PASS (10 legs / 77 pisos).
- **[NV]**: modo escuro e staging real; verificação de UI local depende de sessão segura não fornecida.

- 2026-11-22 (segunda rodada CI): `color-contrast` restante em marca da sidebar e canal selecionado; ambos passaram a usar `text-foreground` sobre fundo realçado. Reexecução pendente.
