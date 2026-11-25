# Decision: Correção das violações WCAG encontradas

- **Slug**: 112-wcag-fixes
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (anterior)

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | A varredura do ciclo 111 encontrou três violações reais e ficou em report-only: `button-name` (critical), `color-contrast` (serious) e `document-title` (critical) (intake.md:5-8; research.md:7-10). |
| Evidence strength | strong | BUG-111-01/02/03 rastreados a arquivos e linhas, contraste 3,73:1 em `ui/button/button.svelte:7`, `--muted-foreground` a 46,9% e log do run 45038205571 / job 137256545911 (research.md:7-11). |
| Value vs. inaction | strong | Botões ícone sem nome e documento sem título bloqueiam leitores de tela; contraste abaixo de 4,5:1 mantém barreira para baixa visão, e o log vira ruído recorrente (problem.md:41-43). |
| Feasibility / appetite | strong | Appetite small e correção localizada: nome acessível em pt-BR, ajuste de contraste no tema claro e `<title>` em `app.html` (concept.md:11; problem.md:19-24). |
| Strategic fit | strong | R5 aplicável a partir do bug reportado em `.specify/bugs/wcag-axe-2026-11-22/`; autoridade local constituição v1.3 e ADR-0003, sem autorização nova (decision.md anterior:9). |
| Risk posture | adequate | Há risco de regressão visual em botões desabilitados e o tema escuro não foi medido **[NV]**; nenhuma regra axe é desativada e nenhum required check é alterado (concept.md:11; decision.md anterior:13). |

## Verdict & Rationale

O veredito histórico **`go`** é preservado: os três achados são reais, localizados e objeto de pedido explícito no bug, e nada nos artefatos sugere correção relaxada. Mantêm-se os fatos e restrições anteriores — R5 aplicável, score **300/625** (critério e denominador em `STATE.md`), autoridade local constituição v1.3 e ADR-0003 sem autorização nova; não desativar regras axe, não relaxar gate, não tocar required checks; roteiro de testes e evidência antes de tratar o PR como mesclável (0.3) e Code Review agêntico independente antes do merge em `dev` (0.4) (decision.md anterior:9-15). Staging real e execução local da UI não foram verificados **[NV]** (research.md:19); "sem achados" não equivale a conformidade AA plena.

## If needs-clarification

- **Blocking questions**: N/A — veredito `go` preservado; as perguntas sobre tema escuro e capturas não bloqueiam o handoff.
- **Revisit stage**: N/A

## If go — Handoff to `/speckit.specify`

- **Problem**: Corrigir BUG-111-01/02/03 sem desativar regras axe nem alterar required checks, mantendo a varredura informativa e registrando evidência.
- **Chosen approach**: Opção A — nome acessível em pt-BR para os botões ícone, contraste ≥4,5:1 em botões desabilitados do tema claro e `<title>` não vazio em `app.html`, com resumo legível no log do CI.
- **In scope / out of scope**: Inclui roteiro de testes, evidência, commits em pt-BR e revisão independente; exclui required checks, gate, migration, backend, credenciais, deploy, desativação de regras axe, medição do tema escuro e auditoria humana completa.
- **Success metrics**: 0 botões ícone sem nome (baseline 3); contraste ≥4,5:1 no tema claro (registro ≈5,84:1; baseline 3,73:1); `<title>` presente; varredura "sem achados" mantida em `/tasks` e `/channels`; 0 regras desativadas e 0 required checks alterados.
- **Carried-forward open questions**: [NEEDS CLARIFICATION: como medir contraste em modo escuro sem alterar as variáveis do tema?]; [NEEDS CLARIFICATION: capturas locais estão disponíveis para verificar regressão visual?]; [NEEDS CLARIFICATION: as chaves pt-BR `nav.openSidebar`/`nav.closeSidebar` e `nav.*AiPanel` já existem?]; [NEEDS CLARIFICATION: como registrar falso positivo de contraste como dívida sem relaxar regra?]; [NEEDS CLARIFICATION: quem fará a auditoria humana completa WCAG 2.2 AA?].
