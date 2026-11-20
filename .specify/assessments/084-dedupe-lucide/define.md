# Define: deduplicar dependências lucide (`lucide-svelte` → `@lucide/svelte`)

- **Slug**: 084-dedupe-lucide · **Definido**: 2026-11-19T23:47Z

## Problema

`apps/frontend` mantém duas dependências lucide: `lucide-svelte@^1.0.1` em `dependencies` (`package.json:53`, deprecado em `package-lock.json:3776`) e `@lucide/svelte@^1.33.0` em `devDependencies` (`:22`, já usado por 4 componentes UI). Três arquivos ainda importam 14 símbolos do pacote deprecado (`ChatLayout.svelte:10`, `SearchBox.svelte:2`, `routes/(app)/+layout.svelte:8-19`). O resultado é duplicação de dependência, exposição a um pacote sem manutenção e risco de dois conjuntos de ícones no bundle. A migração é mecânica e o contrato exige pin exato `1.33.0` para não introduzir upgrade não avaliado (registry hoje informa `1.76.0` como latest — ver `research.md`).

## Afetados / metas / não-metas

Afetados: maintainers e agentes que instalam/inspecionam o frontend; nenhuma API ou comportamento de runtime muda (os `iconNode`s dos 13 símbolos únicos são idênticos nos pacotes instalados). Metas: migrar os 3 imports sem alterar símbolos, remover a dependência deprecada, fixar `@lucide/svelte@1.33.0` em devDependencies e sincronizar os 2 lockfiles. Não-metas: os 4 componentes UI já migrados, `specs/084-dedupe-lucide/spec.md` (template), subida para `1.76.0`, mudança de Svelte, gate/CI/scripts/workflows, ADRs, constituição, STATE, DB/rede, segredos, `node_modules/`, PRs e worktree espelho.

## Requisitos funcionais

- **FR-001** — trocar a origem dos 3 imports de `'lucide-svelte'` para `'@lucide/svelte'`, preservando os 14 símbolos e a sintaxe de named import: `ChatLayout.svelte:10` (`Send`, `Sparkles`), `SearchBox.svelte:2` (`Search`) e `routes/(app)/+layout.svelte:8-19` (11 símbolos, com `Sparkles` repetido). O root de `@lucide/svelte@1.33.0` exporta todos os nomes (`dist/icons/index.js`, `dist/aliases/aliases.js:751` para `CheckSquare`), logo nenhum rename é necessário.
- **FR-002** — remover `"lucide-svelte": "^1.0.1"` de `apps/frontend/package.json:53` (bloco `dependencies`) via `bun remove lucide-svelte`, com reflexo em `apps/frontend/bun.lock:10` e `:517`. Nenhuma outra dependência de `dependencies` é tocada (`bits-ui:51`, `clsx:52`, `tailwind-merge:54` permanecem).
- **FR-003** — fixar a entrada de devDependencies `apps/frontend/package.json:22` como `"@lucide/svelte": "1.33.0"` (sem `^`) via `bun add -D -E @lucide/svelte@1.33.0`, com reflexo em `apps/frontend/bun.lock:16` e `:95`. O pacote permanece em devDependencies; o `"dev": true` de `package-lock.json:345` é preservado.
- **FR-004** — sincronizar `apps/frontend/package-lock.json` via `npm install --package-lock-only --ignore-scripts`: remover a entrada `node_modules/lucide-svelte` (`:3772-3781`), manter `node_modules/@lucide/svelte` em `1.33.0` (`:341-348`) e preservar o restante do arquivo. Churn não relacionado é fora de escopo e deve ser descartado na revisão do diff.
- **FR-005** — limitar o diff rastreado a: 3 arquivos de import, `apps/frontend/package.json`, `apps/frontend/bun.lock`, `apps/frontend/package-lock.json` e os 5 artefatos `.specify/assessments/084-dedupe-lucide/`. Manter formatação local (tabs, aspas simples em Svelte/JS, nenhuma trailing comma — `prettier.config.js:5-7`). Nenhuma dependência nova, segredo, mutação de DB/rede, gate, CI, ADR, spec numerada ou PR.

## Métricas

- **SC-001** — `grep -rn "lucide-svelte" apps/frontend/src apps/frontend/package.json apps/frontend/bun.lock apps/frontend/package-lock.json` retorna zero ocorrências do pacote legado; os 3 imports apontam para `'@lucide/svelte'` com os mesmos 14 símbolos; os 4 componentes UI com subpath `/icons/*` seguem byte-idênticos.
- **SC-002** — `apps/frontend/package.json:22` é exatamente `"@lucide/svelte": "1.33.0"`; `bun pm ls` / `npm ls @lucide/svelte lucide-svelte` mostram apenas `@lucide/svelte@1.33.0` resolvido e nenhuma entrada `lucide-svelte`. `[NV]` depende da edição.
- **SC-003** — em `apps/frontend`: `bun run check`, `bun run lint`, `bun run test:unit -- --run` e `bun run build` terminam sem regressão (`package.json:8,11,13,15-16`); `python3 scripts/ci/check_gate_integrity.py` mantém gate PASS. `[NV]` regressão visual permanece fora da cobertura desses comandos.
- **SC-004** — `git diff --name-only` lista somente os 3 imports + `package.json` + `bun.lock` + `package-lock.json` + 5 artefatos 084; zero mutação de gate/CI/scripts/workflows/ADRs/STATE/specs/DB/rede/segredos/`node_modules/`; zero PRs abertos.

## Baseline

Branch `feature/084-dedupe-lucide`, HEAD `9da1aaea6`, gate PASS, zero PRs. `package.json:22` com `"@lucide/svelte": "^1.33.0"` em devDependencies e `:53` com `"lucide-svelte": "^1.0.1"` em dependencies; `bun.lock:10,16,95,517` e `package-lock.json:19,341-348,3772-3781` coerentes com esse estado; 3 imports legados e 4 componentes UI no pacote novo; 13/13 `iconNode`s idênticos entre as versões instaladas; registry (2026-11-19T23:47Z) com latest `1.76.0`, pin contratual mantido em `1.33.0` por decisão de Main.
