# Decision: deduplicar dependências lucide (`lucide-svelte` → `@lucide/svelte`)

- **Slug**: 084-dedupe-lucide · **Decidido**: 2026-11-19T23:47Z
- **Verdict**: **go (scoped)** · Artefatos: intake | research | define | concept
- **80/20**: E=(1×5)/(2×1)=2,50 — valor 1 (remove dependência deprecada e duplicada, sem ganho funcional novo), confiança 5 (3 imports colados, 2 lockfiles, deprecation field, exports dos dois pacotes instalados, 13/13 `iconNode`s idênticos, registry consultado), esforço 2 (3 imports + `package.json` + 2 lockfiles), risco 1 (peer `svelte ^5` compatível, ícones idênticos localmente; resíduo visual `[NV]`). Alternativas: flag OIDC 15,0 (080) · `.env.example` 10,0 (081) · `cssVariables` 5,0 (082) · docs 5,0 (083) · **lucide 2,50 (esta)** · OTLP 1,25 (`.specify/memory/STATE.md:644`). Registry (2026-11-19T23:47Z) informa latest `1.76.0`; Main confirmou manter o pin contratual `1.33.0` neste ciclo — upgrade futuro fica registrado como `[NV]`.

## Pontuação

| Critério | Nota | Justificativa |
|---|---|---|
| Validade | strong | `lucide-svelte@1.0.1` deprecado (`package-lock.json:3776` + `npm view lucide-svelte deprecated`); 3 imports legados convivem com 4 componentes UI no pacote novo |
| Evidência | strong | `package.json:22,53`; `bun.lock:10,16,95,517`; `package-lock.json:19,341-348,3772-3781`; exports e `iconNode`s lidos de `node_modules/`; registry com URL e data |
| Valor vs. inação | strong | Inação mantém dependência sem manutenção e risco de dual package no bundle |
| Viabilidade | strong | Opção A: 3 trocas de string + 3 comandos determinísticos; nenhuma API, Svelte, gate, DB, rede ou segredo envolvido |
| Fit | strong | Named exports já disponíveis no root (`dist/icons/index.js`, `dist/aliases/aliases.js:751`); nenhum rename; pin exato congela a versão instalada |
| Risco | strong | 13/13 `iconNode`s idênticos entre 1.0.1 e 1.33.0 instalados; peer `^5` compatível com `svelte ^5.56.1`; diff reversível (SC-003/SC-004). Resíduo visual `[NV]` — gates não detectam aparência |

## Handoff

Opção A. Dentro = `ChatLayout.svelte:10`, `SearchBox.svelte:2`, `routes/(app)/+layout.svelte:19`, `apps/frontend/package.json`, `apps/frontend/bun.lock`, `apps/frontend/package-lock.json` + 5 artefatos 084. Fora = 4 componentes UI já migrados, `specs/084-dedupe-lucide/spec.md` (template), subida para `1.76.0`, `prettier.config.js`, gate/`scripts/ci/check_gate_integrity.py`, `.github/workflows/`, ADRs, constituição, STATE, specs/083-*, DB/rede, segredos, `node_modules/`, `.tokens/`, fechamento de issues, leg RBAC, PRs e worktree espelho. FR-001..FR-005 e SC-001..SC-004 de `define.md`.

Comandos determinísticos pós-edição (main agent — rodar em `apps/frontend` salvo o gate):

1. `grep -rn "lucide-svelte" src package.json bun.lock package-lock.json` — esperado: zero correspondências; 3 imports com `'@lucide/svelte'`; 4 componentes UI byte-idênticos (SC-001).
2. `grep -n '"@lucide/svelte"' package.json` — esperado: `"@lucide/svelte": "1.33.0"` em devDependencies, sem `^`; `bun pm ls` e `npm ls @lucide/svelte lucide-svelte` — esperado: só `1.33.0`, sem legado (SC-002). `[NV]` depende da edição.
3. `bun run check && bun run lint && bun run test:unit -- --run && bun run build` — esperado: sem regressão (`package.json:8,11,13,15-16`); `prettier --check` garante tabs/aspas simples/sem trailing comma (`prettier.config.js:5-7`).
4. `python3 scripts/ci/check_gate_integrity.py` (na raiz) — gate segue PASS; `git diff --name-only` — esperado: 3 imports + 3 manifestos/locks + 5 artefatos 084, nada mais; zero PRs (SC-004).
5. Revisar `git diff` de `package-lock.json` e `bun.lock`: ausência da entrada `node_modules/lucide-svelte`; `@lucide/svelte` em `1.33.0`; zero churn não relacionado (FR-004).

`[NV]` não verificados nesta avaliação: execução dos comandos acima; regressão visual em browser (os 13 `iconNode`s comparados são idênticos, mas gates não detectam aparência); grafo final do bundler (dual package residual); churn real dos lockfiles após os comandos; CI/PR; upgrade futuro para `@lucide/svelte@1.76.0` — latest informado por `npm view @lucide/svelte version` em `https://registry.npmjs.org/` (acesso 2026-11-19T23:47Z; publicação `2026-11-19T06:16:31.915Z` segundo `npm view @lucide/svelte time --json`), divergente do `1.33.0` registrado no contexto; Main aprovou manter `1.33.0` neste ciclo, e uma subida exigiria nova avaliação de escopo/visual. Gates/formatters **skipped** nesta avaliação; base `9da1aaea6` preservada.
