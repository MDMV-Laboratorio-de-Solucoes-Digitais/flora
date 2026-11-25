# Concept: Guarda WCAG sem novo required check

- **Slug**: 114-wcag-ci-guard
- **Created**: 2026-11-25
- **Recommended option**: Opção B — helper puro + hardening do smoke

## Options

### Option A — Manter auditoria report-only
- **Sketch**: preservar anotações informativas e corrigir apenas violações descobertas manualmente.
- **Appetite**: small
- **Trade-offs**: zero risco de CI RED; nenhuma garantia de que regressões serão detectadas.
- **Rabbit holes**: discussão interminável sobre quais achados são “reais”; nenhum consumidor determinístico.

### Option B — Helper puro + hardening do smoke
- **Sketch**: extrair `a11yAudit.ts`, corrigir `/files` e fazer `auditAccessibility` exigir zero achados em `/tasks`, `/channels` e `/files`, mantendo o job `e2e-browser` advisory.
- **Appetite**: medium
- **Trade-offs**: detecta regressões reais no smoke sem criar required check; exige correção imediata de qualquer achado antes do merge.
- **Rabbit holes**: novas violações em tema escuro ou em estados ainda não exercitados pelo Chromium headless.

### Option C — Novo required check WCAG
- **Sketch**: promover a varredura a gates obrigatórios separados por rota.
- **Appetite**: large
- **Trade-offs**: máxima cobertura; muda ADR-0003 e aumenta tempo de CI.
- **Rabbit holes**: governança de required checks e flakiness do navegador.

## Recommendation

Opção B: liga a auditoria aos fatos já medidos pelo smoke, cobre `/files` e preserva a restrição de não criar required checks (`define.md:5-11`; `shape.md:5-11`).

## Out of Scope (for the recommended option)

- Novos required checks, alteração de gate floors, supressão de regras axe, backend/migrations e execução em tema escuro `[NV]`.

## Assumptions to Validate

- `e2e-browser` continua advisory e roda antes do merge em `dev` (`decision.md` 114).
- O helper puro é compatível com a forma de `AxeBuilder.analyze().violations` usada no smoke (`shape.md:5-8`).
