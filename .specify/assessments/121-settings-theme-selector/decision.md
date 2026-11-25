# Decision: Seletor de tema em `/settings` e contraste WCAG AA

- **Slug**: 121-settings-theme-selector
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | define.md | shape.md | decision.md (original 2026-11-24) | specs/121-settings-theme-selector/{spec,plan}.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `intake.md:7-13` registra `BF-04` sem controle interativo de tema, contraste de `--destructive` abaixo de AA no tema claro e polimento LOW pendente do ACR `#409`. |
| Evidence strength | strong | `research.md:7-16` traz file:line e números verificáveis: `app.html:16-22`, `layout.css:56` (`3.59:1` → `6.24:1`), `RoleManager.svelte:40-45,133-146,247`. |
| Value vs. inaction | adequate | Fecha `BF-04` e corrige barreira de acessibilidade real em `/settings`, mas impacto menor que o da ponte HTTP HITL (`1500/3125` vs `2000/3125` na decisão original). |
| Feasibility / appetite | strong | Escopo pequeno e de baixo risco (`esforço=1`, `risco=1` na decisão original): funções puras, store Runes e componente, sem backend nem SQL. |
| Strategic fit | adequate | Alinhado à fila de fechamento e ao ACR `#409`, porém é polimento de UI e não fecha dependência de contrato ou tenant. |
| Risk posture | adequate | Compatibilidade com o bootstrap inline de `app.html`, `.dark` preservado e resiliência a SSR/`localStorage` bloqueado reduzem risco já baixo (`define.md:19`, `shape.md:1-4`). |

## Verdict & Rationale

Veredito histórico **`go` preservado**: os artefatos não contradizem a decisão de 2026-11-24. O scorecard sustenta o `go` pela evidência numérica de contraste e pelo acoplamento entre seletor, correção AA e polimento pendente; o score original **1500 / 3125** (`impacto=4 × confiança=5 × fechamento=3 × (6−esforço=1)=5 × (6−risco=1)=5`, tipo `IMPLEMENTAÇÃO` + `CORREÇÃO`) permanece registrado. Restrições transportadas: compatibilidade estrita com `localStorage['flora.theme']` e a classe `.dark` em `apps/frontend/src/app.html`; nenhuma alteração em migrations ou pisos DB (`85` preservados); SemVer `MINOR` (`0.12.0 → 0.13.0`); zero binários/PNGs no repositório (screenshots em Gist); R6 com 4 issues (`#415`–`#418`) sem `dark-factory:eligible`.

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; nenhuma incógnita bloqueante nos artefatos).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: Não há controle para alternar claro/escuro em `/settings`, `--destructive` no tema claro tem contraste `3.59:1` (abaixo de `4.5:1`) e `RoleManager.svelte` retém mensagens de erro/sucesso em troca de contexto (`intake.md:7-13`, `research.md:10-14`).
- **Chosen approach**: Utilitários puros de tema + `ThemeStore.svelte.ts` (Runes) + `ThemeSelector.svelte` com `aria-pressed`/`aria-live`, ajuste de `:root --destructive` para `0 84.2% 40%` mantendo `.dark` intacto, e limpeza determinística de mensagens no `RoleManager.svelte` (`shape.md:1-4`).
- **In scope / out of scope**: **In** — chave `flora.theme`, persistência e `.dark`, UI em `pt-BR`, correção WCAG AA, polimento LOW do ACR `#409`, TDD unitário, lint/build e E2E com Axe e `page.reload()`. **Out** — modo automático `system` com `matchMedia` dinâmico, mudanças no backend Rust e migrations SQL (`define.md:21`).
- **Success metrics**: SC-001..SC-004 em `specs/121-settings-theme-selector/spec.md` — `theme.test.ts` `RED→GREEN`, `bun run check`/`lint`/`build` com código `0`, clique `Claro` → `reload()` → `Escuro` → `Claro` com `0` violações WCAG 2.2 AA em ambos os temas, e `85` pisos DB preservados.
- **Carried-forward open questions**: `[NV]` execução em navegador real, capturas publicadas em Gist e promoção `dev → main` escalada ao SWE (`spec.md:FR-004`); `[NEEDS CLARIFICATION: se o modo `system` for posteriormente adotado, o bootstrap de `app.html:16-22` deve ser alterado ou apenas a store deve resolver a preferência?]` (`define.md:21`); `[NEEDS CLARIFICATION: a limpeza de `errorMessage`/`successMessage` deve preservar confirmações imediatas por quanto tempo?]` (`shape.md:4`).
