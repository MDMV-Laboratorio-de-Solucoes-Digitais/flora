# Problem Definition: Auditoria WCAG 2.2 AA como guarda determinística

- **Slug**: 114-wcag-ci-guard
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | define.md (equivalente local) | shape.md | decision.md

## Problem Statement

A varredura WCAG do smoke e2e era report-only e cobria apenas `/tasks` e `/channels`, deixando `/files` sem verificação e permitindo que violações reais passassem despercebidas no Seed. O problema importa agora porque o frontend canônico Svelte 5 já tem os fluxos núcleo e a constituição exige acessibilidade e fallback de erro (`define.md:5-9`; `shape.md:5-11`).

## Affected Users & Stakeholders

- **Users**: pessoas que usam `/files` com leitor de tela ou contraste elevado, que precisam de rótulos e estados de erro legíveis (`define.md:7`).
- **Stakeholders**: maintainer do Smoke e do CI, que precisa de sinal determinístico sem criar required check novo (`decision.md` 114, restrições).

## Goals

- Tornar a auditoria WCAG 2.2 AA uma checagem determinística nos fluxos já cobertos, estendendo-a a `/files` (`define.md:9`).
- Corrigir violações reais de contraste e rotulagem nos componentes de arquivo sem desativar regras axe (`define.md:7-9`).
- Manter a lógica de formatação testável fora do navegador (`shape.md:5-8`).

## Non-Goals

- Criar novo required check ou alterar gate floors, backend, migrations ou supressões de regra (`define.md:11`).
- Exercitar tema escuro no smoke; permanece `[NV]` (`shape.md:15`).

## Success Metrics

- `auditAccessibility` falha com `expect(findings, summary).toEqual([])` em `/tasks`, `/channels` e `/files` (baseline: report-only e só duas rotas — `define.md:9`).
- Zero achados axe em `/files` no Chromium headless (baseline: não medido; `[NV]`).
- `bun run check`/`lint` e Vitest `server` sem erros (baseline: não medido neste artefato).

## Cost of Inaction

Violações de `button-name`, `color-contrast` ou `document-title` continuam invisíveis no merge gate, e o fluxo de arquivos do Seed segue inacessível para parte dos usuários; a correção posterior exigiria reabrir o mesmo frontend e o mesmo smoke (`define.md:5-9`; `shape.md:13-15`).

## Open Questions

- [NEEDS CLARIFICATION: o Chromium headless do CI reproduz todas as combinações de tema/contraste do `/files`?] (`shape.md:15`)
- [NEEDS CLARIFICATION: quais consumidores dependem do formato anterior da anotação `wcag-axe`?] (`define.md:9`)
