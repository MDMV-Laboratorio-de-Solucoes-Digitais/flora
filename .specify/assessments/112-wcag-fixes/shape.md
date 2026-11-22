# Assess — shape: 112

**Data**: 2026-11-22

## Abordagem

- `aria-label` dinâmico nos três botões ícone do layout, com chaves `nav.openSidebar`/`nav.closeSidebar` e reuso de `nav.*AiPanel`.
- Substituir `disabled:opacity-50` por `disabled:bg-muted disabled:text-muted-foreground` na base do componente `Button`; ajustar `--muted-foreground` para 40% de luminosidade (contraste calculado ≥5,8:1 no tema claro).
- `<title>Flora</title>` em `app.html`.
- `console.info` no helper de auditoria para tornar o resumo legível no log do CI, sem transformar o job advisory em gate.

## Riscos

- Regressão visual em botões desabilitados → verificar no `e2e-browser` e, se possível, captura de tela local.
- Contraste em modo escuro não medido neste ciclo → **[NV]**; variáveis do tema escuro permanecem inalteradas.
