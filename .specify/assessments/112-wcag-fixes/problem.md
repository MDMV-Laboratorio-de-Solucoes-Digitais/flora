# Problem Definition: Violações WCAG reais reparadas pela varredura

- **Slug**: 112-wcag-fixes
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md | define.md | shape.md

## Problem Statement

A varredura automatizada do ciclo 111 (`e2e-browser`, run 45038205571 / job 137256545911) encontrou três violações reais e ficou em modo report-only; os problemas permanecem no frontend canônico (intake.md:5-8; research.md:11). São eles: `button-name` (critical) — botão `size="icon"` de alternância da sidebar e dois botões do painel de IA em `routes/(app)/+layout.svelte` sem nome acessível (BUG-111-01; research.md:8); `color-contrast` (serious) — `disabled:opacity-50` em `ui/button/button.svelte:7` produz 3.73:1 sobre `#f8fafc`, com `--muted-foreground` em 46,9% de luminosidade em `routes/layout.css:52` (BUG-111-02; research.md:9-10); e `document-title` (critical) — `app.html` sem elemento `<title>` (BUG-111-03; research.md:7).

O pedido registrado em `.specify/bugs/wcag-axe-2026-11-22/assessment.md` (R5) é corrigir BUG-111-01/02/03 sem desativar regras axe nem alterar required checks, mantendo a varredura informativa e registrando evidência (intake.md:3,9-11; decision.md:9). A autoridade local é a constituição v1.3 e ADR-0003; o roteiro de testes, a evidência e a revisão agêntica independente são condições para tratar o PR como mesclável (decision.md:9-15). Staging real e execução local da UI não foram verificados — credenciais de staging foram solicitadas na PR Forgejo #133 — **[NV]** (research.md:19; shape.md:16).

## Affected Users & Stakeholders

- **Users**: usuários do frontend canônico, em especial quem usa leitor de tela (botões ícone sem nome, documento sem título) e quem tem baixa visão (contraste insuficiente em botões desabilitados) (intake.md:5-8; research.md:7-10).
- **Stakeholders**: o guardião de required checks e do gate, que exige que nenhuma regra axe seja desativada e nenhum piso alterado; o revisor agêntico independente antes do merge em `dev`; quem opera o job informativo e precisa de resumo legível no log do CI (decision.md:13-15; define.md:10).

## Goals

- Garantir nome acessível em pt-BR para todo botão ícone das rotas `(app)`, incluindo alternância da sidebar e painel de IA (define.md:7; research.md:8).
- elevar o contraste de botões desabilitados ao mínimo WCAG (≥4,5:1), hoje em 3,73:1 sobre `#f8fafc` (define.md:8; research.md:9).
- Fazer `app.html` declarar `<title>` não vazio (define.md:9; research.md:7).
- Manter a varredura informativa com resumo visível no log do CI, sem desativar regras nem criar required check (define.md:10; decision.md:13).
- Registrar roteiro e evidência de testes, com commits em pt-BR, PR em inglês e revisão independente antes do merge (define.md:15; decision.md:15).

## Non-Goals

- Não alterar required checks, gate floor, migration, backend, credenciais, deploy nem realizar auditoria humana completa WCAG 2.2 AA (intake.md:13; define.md:14).
- Não desativar regras axe para eliminar achados, nem relaxar o gate (intake.md:11; decision.md:13).
- Não medir ou afirmar contraste em tema escuro neste ciclo: variáveis do tema escuro permanecem inalteradas e o resultado é **[NV]** (shape.md:16; research.md:19).
- Não citar o status pós-correção como conformidade AA plena: a automação tem cobertura parcial (113-bug-status/research.md:47).

## Success Metrics

- Botões ícone sem nome acessível nas rotas `(app)`: alvo 0; baseline: 3 (1 sidebar + 2 painel de IA) (research.md:8).
- Contraste de botões desabilitados no tema claro: alvo ≥4,5:1 (registro de ≈5,84:1 após ajuste); baseline: 3,73:1 sobre `#f8fafc`, com `--muted-foreground` a 46,9% de luminosidade (research.md:9-10; 113-bug-status/research.md:23).
- Elemento `<title>` não vazio em `app.html`: alvo presente; baseline: ausente (research.md:7).
- Achados axe em `/tasks` e `/channels`: alvo "sem achados" mantido na varredura informativa; baseline: 3 violações (BUG-111-01/02/03) (intake.md:5-8; 113-bug-status/research.md:22-23).
- Regras axe desativadas e required checks alterados: alvo 0; baseline: 0 — nenhum registro de relaxamento no ciclo 111 (intake.md:11; decision.md:13).

## Cost of Inaction

Botões ícone sem nome e documento sem título impedem navegação compreensível por leitores de tela; contraste de 3,73:1 em botões desabilitados abaixo do mínimo WCAG 2.2 (critério 1.4.3) mantém barreira para usuários com baixa visão (research.md:7-10,15). Como a varredura é report-only, o CI continua gerando achados sem que a interface melhore, e a dívida WCAG bloqueia o fechamento documental do bug pelo ciclo 113 (intake.md:7; 113-bug-status/intake.md:5-8). Manter o job informativo sem correção também corrói a confiança no sinal automatizado (shape.md:12-16).

## Open Questions

- [NEEDS CLARIFICATION: como medir contraste em modo escuro sem alterar as variáveis do tema neste ciclo?] — não medido, **[NV]** (shape.md:16).
- [NEEDS CLARIFICATION: as capturas de tela locais estão disponíveis para verificar a regressão visual em botões desabilitados?] (shape.md:12).
- [NEEDS CLARIFICATION: as chaves pt-BR do dicionário para `nav.openSidebar`/`nav.closeSidebar` e `nav.*AiPanel` já existem e estão consistentes?] (shape.md:7).
- [NEEDS CLARIFICATION: se aparecer falso positivo de contraste, como registrá-lo como dívida sem relaxar a regra?] (111-wcag-axe/shape.md:16).
- [NEEDS CLARIFICATION: quem realizará a auditoria humana completa WCAG 2.2 AA, fora de escopo aqui?] (intake.md:13).
