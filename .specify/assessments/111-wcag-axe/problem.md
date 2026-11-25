# Problem Definition: Acessibilidade sem verificação automatizada

- **Slug**: 111-wcag-axe
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md | define.md | shape.md

## Problem Statement

O frontend canônico (Svelte 5, `apps/frontend`) não tem verificação automatizada de acessibilidade: a conformidade WCAG 2.2 AA depende apenas da revisão humana prevista em FR-07, que é lenta, não reproduzível e não detecta regressões (intake.md:5-7; research.md:10). Há `@playwright/test` 1.60 e Vitest 4 no projeto, mas nenhum `axe-core` declarado, enquanto o smoke autenticado já roda em `apps/frontend/tests/e2e/*.e2e.ts` com `playwright/.auth/alice.json` (research.md:7-8). O job `e2e-browser` é advisory e os required checks são `frontend-build`, `rust-build`, `db-tests` e `dependency-review` (research.md:9); a decisão F9 do assessment é **ADAPT** com FR-06/FR-08 (research.md:10).

O enquadramento registrado pede uma varredura automatizada sobre fluxos já cobertos pelo e2e, em job informativo, sem required check novo (intake.md:9-11). As páginas representativas usadas pelo smoke são `/tasks` e `/channels`, com tags `wcag2a`, `wcag2aa`, `wcag21a`, `wcag21aa` e `wcag22aa` citadas como critério (define.md:8). A varredura precisou ser determinística — sem dados temporais, reaproveitando a sessão existente — e os lockfiles `bun.lock` e `package-lock.json` precisaram permanecer sincronizados (define.md:7,14; shape.md:17). Comportamento em produção/staging e jobs remotos após o merge não foi observado: **[NV]** (research.md:21-23).

## Affected Users & Stakeholders

- **Users**: usuários do frontend canônico que dependem de interfaces acessíveis, incluindo quem usa tecnologias assistivas; revisores humanos de acessibilidade, que hoje arcam sozinhos com FR-07 e não recebem sinal de regressão (intake.md:5-7; research.md:10).
- **Stakeholders**: o guardião dos required checks e do gate `check_gate_integrity.py`, que deve continuar inalterado com `e2e-browser` advisory; os mantenedores do e2e Playwright e dos lockfiles; o SE que autorizou dependências de dev sem autorizar mudança de governança (research.md:9; decision.md:9-15).

## Goals

- Tornar reproduzível a verificação das regras WCAG 2.2 AA nas páginas autenticadas já usadas pelo smoke (`/tasks`, `/channels`), reduzindo a dependência exclusiva de revisão humana (define.md:8; research.md:7-10).
- Detectar regressões de acessibilidade no e2e existente, com achados legíveis em pt-BR na mensagem de asserção (shape.md:7).
- Manter o job `e2e-browser` informativo e o gate autoritativo inalterado (define.md:9; decision.md:11-13).
- Preservar determinismo: sessão já injetada pelo setup, sem dependência de dados temporais e sem credenciais no repositório ou em logs (define.md:14; shape.md:7).
- Registrar falso positivo de contraste como dívida **[NV]** em vez de relaxar regra, e manter rollback por commit de reversão sem force-push (shape.md:16; define.md:15).

## Non-Goals

- Não realizar auditoria humana, corrigir todos os achados, mudar required checks, fazer deploy ou versionar credenciais (intake.md:13; decision.md:13-15).
- Não substituir FR-07: a varredura automatizada tem cobertura parcial e não equivale a conformidade AA completa (research.md:10; assessment citado em 113-bug-status/research.md:47).
- Não afirmar comportamento em staging/produção ou resultados de jobs remotos após o merge sem observação: **[NV]** (research.md:21-22).

## Success Metrics

- Rotas autenticadas com varredura automatizada: alvo 2 (`/tasks`, `/channels`); baseline: 0 — nenhum `axe-core` no projeto (define.md:8; research.md:7-8).
- Tags WCAG executadas por página: alvo 5 (`wcag2a`, `wcag2aa`, `wcag21a`, `wcag21aa`, `wcag22aa`); baseline: unknown (define.md:8).
- Achados reportados no log do job informativo: alvo 0 após correções; baseline: 3 violações reais encontradas pela varredura do ciclo 111 e registradas em `.specify/bugs/wcag-axe-2026-11-22/` (112-wcag-fixes/intake.md:5-8; 113-bug-status/research.md:22).
- Required checks: alvo 4 inalterados, `e2e-browser` advisory; baseline: 4 (research.md:9-10).
- Dependências de dev e lockfiles sincronizados: alvo `@axe-core/playwright` presente em `bun.lock` e `package-lock.json`; baseline: ausente (define.md:7; research.md:7).

## Cost of Inaction

Sem varredura automatizada, regressões de `button-name`, `color-contrast` e `document-title` passam despercebidas até uma revisão humana manual, que não é reproduzível nem detecta mudanças introduzidas depois dela (intake.md:5-7; 112-wcag-fixes/research.md:7-11). O custo de depender só de FR-07 cresce com cada nova rota, e a ausência de sinal no e2e permite que violações críticas cheguem ao log de CI sem ser vistas — como ocorreu com as três violações reportadas no run do ciclo 111 (112-wcag-fixes/intake.md:5-8). Comportamento remoto permanece **[NV]**.

## Open Questions

- [NEEDS CLARIFICATION: como tratar falso positivo de contraste em tema escuro sem relaxar regra nem transformar dívida em gate?] (shape.md:16).
- [NEEDS CLARIFICATION: quais rotas além de `/tasks` e `/channels` devem entrar na varredura, dado que a cobertura é parcial?] (define.md:8; 113-bug-status/research.md:47).
- [NEEDS CLARIFICATION: divergirem `bun.lock` e `package-lock.json`, qual é a fonte de verdade para a dependência de dev?] (shape.md:17).
- [NEEDS CLARIFICATION: o job `e2e-browser` deve permanecer advisory indefinidamente ou há critério para promovê-lo a required check?] (intake.md:11; decision.md:13).
- [NEEDS CLARIFICATION: como registrar a revisão manual de critérios não automatizáveis junto ao resultado do axe?] (113-bug-status/research.md:56).
