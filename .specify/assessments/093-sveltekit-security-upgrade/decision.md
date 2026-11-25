# Decision: Upgrade de segurança SvelteKit 3 (ciclo 093)

- **Slug**: 093-sveltekit-security-upgrade
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (anterior, decidido em 2026-11-20) | specs/093-{spec,plan,tasks,analyze,converge}.md | docs/governance/2026-11-20-093-sveltekit-upgrade.md | advisories GHSA no GitHub | STATE.md/PROGRESSO.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `dependency-review` da PR #318 (`dev` → `main`) bloqueava o merge por `GHSA-3r8g` (high) e 3 moderates em `@sveltejs/kit 2.70.3`; 4 advisories confirmados no banco do GitHub (`decision.md:12-13`, `research.md:22-23`). |
| Evidence strength | strong | Inventário convergente em decisão/specs/STATE/governança, leitura direta do manifesto final (`package.json:25-27,40-47`), diff registrado contra `b426e4f81` e verificações locais consolidadas (`research.md:29-34`). |
| Value vs. inaction | strong | `2.70.4` cobre só 2 dos 4 achados; apenas `3.1.1` cobre o high, então a inação mantém o bloqueio do merge sem flexibilizar o gate (`concept.md:15-24`). |
| Feasibility / appetite | adequate | Opção A de appetite `medium`: upgrade de major com duas quebras de configuração, dois locks e verificação check/lint/testes/build; peers já declarados no manifesto (`concept.md:9-13`, `research.md:24`). |
| Strategic fit | strong | Mantém `bun.lock` canônico com `package-lock.json` sincronizado, preserva `$lib`, limita o diff ao frontend/artefatos 093 e não altera API, runtime, workflows ou required checks. |
| Risk posture | adequate | Green local não substitui CI remota; compatibilidade do `adapter-node` e contagem pós-upgrade são **[NV]**; diferença documental `T001..T009` vs. `tasks=6/6` registrada sem reescrita (`research.md:34,40-41`). |

## Verdict & Rationale

Veredito histórico **`go` preservado sem alteração**; decisão original em 2026-11-20, branch `feature/093-sveltekit-security-upgrade` @ `b426e4f81`, âncora `.specify/feature.json` → `specs/093-sveltekit-security-upgrade`. O problema é medido pelo `dependency-review` (`fail` no run `44806804503`, `fail-on-severity: moderate`, modo "PR diff only", lido via `gh` às 2026-11-20T19:56Z), a correção tem peers já compatíveis e a migração de configuração é a substituição direta exigida pelo SvelteKit 3; verificações locais verdes e diff reversível por VCS. Compromissos originais preservados: **FR-001** versões corrigidas `@sveltejs/kit 3.1.1`, `@sveltejs/adapter-node 6.0.5`, `svelte 5.57.1` no manifesto e em ambos os locks, sem resíduo de `2.70.3`, `5.5.7` ou `5.56.10`; **FR-002** remover `apps/frontend/svelte.config.js` e declarar `vitePreprocess()`/`adapter()` no plugin `sveltekit()` em `vite.config.ts`; **FR-003** preservar `$lib` via `resolve.alias` e `paths` do tsconfig (que estende `$app/tsconfig`), mantendo `bun.lock` canônico e `package-lock.json` sincronizado; **FR-004** `npm run check` 0 erros/0 warnings, `npm run lint` OK, unit server 42/42 e `npm run build` OK; **FR-005** fronteira no frontend e artefatos 093. Os quatro advisories confirmados em 2026-11-25: `GHSA-3r8g-jcqq-3p5j` (high, CVSS 7.5, corrigido em 3.1.1), `GHSA-mw6w-2fmv-pvw3`, `GHSA-rqp5-r3wg-vg35` (moderates, também `2.70.4` em 2.x) e `GHSA-6cf5-gc4q-fm3x` (moderate, só 3.1.1); o patch 2.x não cobre o conjunto completo. Diff registrado contra `b426e4f81`: manifesto, ambos os locks, exclusão de `svelte.config.js`, `vite.config.ts`, `tsconfig.json`, com `grep 2.70.3` → zero. Opções B (patch 2.x) e C (esperar upstream) rejeitadas em `concept.md:15-25`: deixariam o high e o merge bloqueado. A promoção posterior de `main` em `2026-11-20T20:38Z` com PR #318 mesclada e `dependency-review` limpo é registro append-only em `STATE.md`/`PROGRESSO.md`, não reexecução nesta avaliação.

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; todas as incógnitas abaixo são carried-forward).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: Advisories em `@sveltejs/kit 2.70.3` (incluindo `GHSA-3r8g`, high) bloqueavam o merge de `dev` para `main` via `dependency-review`, sem flexibilizar o required check.
- **Chosen approach**: Opção A — salto de major com versões corrigidas e migração explícita das duas quebras do SvelteKit 3; FR-001..FR-005 / SC-001..SC-003 e tarefas T001..T009 (T007 configuração no plugin Vite, T008 alias `$lib`/tsconfig, T009 verificação).
- **In scope / out of scope**:
  - **In**: `apps/frontend/package.json`, `bun.lock`, `package-lock.json`, remoção de `svelte.config.js`, `vite.config.ts`, `tsconfig.json`, mais artefatos documentais 093.
  - **Out**: API de aplicação, runtime, schema, migration, contrato de produção, workflows, required checks (inclusive qualquer flexibilização do `dependency-review`), PR/merge/tag/deploy, credenciais/produção, `validate/`, `release/` e `.specify/memory/STATE.md`; nenhuma troca de gerenciador de lockfile.
- **Success metrics**: zero ocorrências de `2.70.3`, `5.5.7`, `5.56.10` no manifesto e locks; `check` 0/0, `lint` OK, unit server 42/42, `build` OK (registro do handoff, não reexecutado); `$lib` e configuração preservados; `dependency-review` remoto limpo na PR promovida (resultado ao vivo **[NV]** no ciclo); diff limitado ao frontend/artefatos 093.
- **Carried-forward open questions**: `[NEEDS CLARIFICATION: qual é a contagem viva de advisories no painel do GitHub após o upgrade ([NV])?]`; `[NEEDS CLARIFICATION: o dependency-review da PR #318 voltou a passar na CI remota ([NV])?]`; `[NEEDS CLARIFICATION: existem consumidores de svelte.config.js ou do alias $lib fora de apps/frontend?]`; `[NEEDS CLARIFICATION: qual a expectativa de compatibilidade com o runtime de deploy (adapter-node) após o major?]`; `[NEEDS CLARIFICATION: há versões residuais das versões afetadas em outros locks do repositório?]`; `[NEEDS CLARIFICATION: qual a janela de reconsulta do estado remoto antes de qualquer promoção da PR #318?]`; `[NEEDS CLARIFICATION: como reconciliar T001..T009 em tasks.md/decision.md com tasks=6/6 de analyze.md/converge.md?]` (`intake.md:23-28`, `research.md:46-52`). `[NV]` reexecução de gates/build/testes/formatters e estado remoto ao vivo; gate local verde não substitui CI para promoção.
