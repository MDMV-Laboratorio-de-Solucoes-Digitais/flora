# Decision: deduplicar dependências lucide (`lucide-svelte` → `@lucide/svelte`)

- **Slug**: 084-dedupe-lucide
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | define.md | concept.md | decision.md (anterior, decidido em 2026-11-19T23:47Z) | registry npm e `node_modules/` citados em research.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `lucide-svelte@1.0.1` deprecado (`package-lock.json:3776` + `npm view lucide-svelte deprecated`) convive com `@lucide/svelte` em 3 imports legados e 4 componentes UI já migrados (`intake.md:4,9`). |
| Evidence strength | strong | 3 imports colados, 2 lockfiles, campo de deprecation, exports dos dois pacotes instalados, 13/13 `iconNode`s idênticos e registry consultado com URL/data (`research.md:7-20`). |
| Value vs. inaction | strong | Inação mantém dependência sem manutenção e risco de dual package no bundle (`define.md:7`). |
| Feasibility / appetite | adequate | Opção A: 3 trocas de string + `package.json` + 2 lockfiles (appetite `pequeno`); nenhuma API, Svelte, gate, DB, rede ou segredo envolvido (`concept.md:5-9`). |
| Strategic fit | strong | Named exports disponíveis no root (`dist/icons/index.js`, `dist/aliases/aliases.js:751`), nenhum rename; pin exato congela a versão instalada e preserva os 4 componentes já migrados. |
| Risk posture | strong | Peer `svelte ^5` compatível com `svelte ^5.56.1`, 13/13 `iconNode`s idênticos entre 1.0.1 e 1.33.0 instalados, diff reversível (SC-003/SC-004). `[NV]` resíduo visual/grafo do bundler e upgrade futuro. |

## Verdict & Rationale

Veredito histórico **`go (scoped)` preservado como `go`**; decisão original em 2026-11-19T23:47Z, branch `feature/084-dedupe-lucide`, base `9da1aaea6`, gate PASS, zero PRs. 80/20 original integral: **E=(1×5)/(2×1)=2,50** — valor 1 (remove dependência deprecada e duplicada, sem ganho funcional novo), confiança 5 (3 imports colados, 2 lockfiles, deprecation field, exports dos dois pacotes instalados, 13/13 `iconNode`s idênticos, registry consultado), esforço 2 (3 imports + `package.json` + 2 lockfiles), risco 1 (peer `svelte ^5` compatível, ícones idênticos localmente; resíduo visual `[NV]`). Alternativas: flag OIDC 15,0 (080) · `.env.example` 10,0 (081) · `cssVariables` 5,0 (082) · docs 5,0 (083) · **lucide 2,50 (esta)** · OTLP 1,25 (`.specify/memory/STATE.md:644`). `specs/084-dedupe-lucide/spec.md` é template não preenchido e não é alvo. Registry (2026-11-19T23:47Z): `npm view lucide-svelte version deprecated` → `1.0.1` / "Package deprecated. Please use @lucide/svelte instead."; `npm view @lucide/svelte version peerDependencies` → `1.76.0`, `{svelte: '^5'}`; `npm view @lucide/svelte time --json` → publicação de `1.76.0` em `2026-11-19T06:16:31.915Z` (`https://registry.npmjs.org/`). Main confirmou manter o pin contratual `1.33.0` neste ciclo — coincide com a resolução já instalada (`bun.lock:95`, `package-lock.json:341-348`), evita churn e preserva os 13 `iconNode`s idênticos; upgrade para `1.76.0` fica registrado como `[NV]`. Opções B (`/icons/*`), C (manter ambos) e D (latest) rejeitadas em `concept.md:11-24` — diff maior sem ganho, dívida preservada e risco visual não avaliado, respectivamente.

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; upgrade futuro e renderização visual seguem como carried-forward).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: `apps/frontend` mantém `lucide-svelte@^1.0.1` deprecado em `dependencies` junto de `@lucide/svelte@^1.33.0`, com 3 imports legados (14 símbolos) e risco de dual package.
- **Chosen approach**: Opção A — trocar apenas as strings dos 3 imports para `'@lucide/svelte'`, `bun remove lucide-svelte`, `bun add -D -E @lucide/svelte@1.33.0`, `npm install --package-lock-only --ignore-scripts`. FR-001..FR-005 e SC-001..SC-004 de `define.md`.
- **In scope / out of scope**:
  - **In**: `ChatLayout.svelte:10`, `SearchBox.svelte:2`, `routes/(app)/+layout.svelte:19`, `apps/frontend/package.json`, `apps/frontend/bun.lock`, `apps/frontend/package-lock.json` + 5 artefatos 084.
  - **Out**: 4 componentes UI já migrados, `specs/084-dedupe-lucide/spec.md` (template), subida para `1.76.0`, `prettier.config.js`, gate/`scripts/ci/check_gate_integrity.py`, `.github/workflows/`, ADRs, constituição, STATE, specs/083-*, DB/rede, segredos, `node_modules/`, `.tokens/`, fechamento de issues, leg RBAC, PRs e worktree espelho.
- **Success metrics**: `grep -rn "lucide-svelte" src package.json bun.lock package-lock.json` → zero e 3 imports com `'@lucide/svelte'` (4 componentes UI byte-idênticos); `"@lucide/svelte": "1.33.0"` exato em devDependencies e `bun pm ls`/`npm ls` sem legado; `bun run check && bun run lint && bun run test:unit -- --run && bun run build` sem regressão; `python3 scripts/ci/check_gate_integrity.py` PASS; `git diff --name-only` só os 3 imports + 3 manifestos/locks + 5 artefatos, zero PRs; diff dos locks sem churn não relacionado (FR-004).
- **Carried-forward open questions**: `[NV]` execução dos comandos acima; `[NV]` regressão visual em browser (gates não detectam aparência); `[NV]` grafo final do bundler (dual package residual); `[NV]` churn real dos locks; `[NV]` CI/PR; `[NV]` upgrade futuro para `@lucide/svelte@1.76.0` (exige nova avaliação de escopo/visual). Gates/formatters **skipped** nesta avaliação; base `9da1aaea6` preservada.
