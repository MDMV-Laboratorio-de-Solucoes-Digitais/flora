# Validation: 084 — deduplicação de dependências lucide

**Data**: 2026-11-20T00:21Z · **Branch**: `feature/084-dedupe-lucide` · **Baseline**: `9da1aaea6`
**Tipo**: migração mecânica de dependência (3 imports Svelte + `package.json` + 2 lockfiles); nenhuma mudança de API, comportamento ou contrato de runtime.
**Veredito R4**: **APROVADO**.

## Gates e verificações

| Gate | Resultado (saída colada) |
|---|---|
| `grep -rn "lucide-svelte" apps/frontend/src apps/frontend/package.json apps/frontend/bun.lock apps/frontend/package-lock.json` | **zero** ocorrências do pacote legado; o fechamento multilinha de `+layout.svelte:19` também foi trocado (FR-001, SC-001) |
| `grep -rn "from '@lucide/svelte'" apps/frontend/src/lib/components/chat/ChatLayout.svelte apps/frontend/src/lib/components/search/SearchBox.svelte 'apps/frontend/src/routes/(app)/+layout.svelte'` | exatamente **3 linhas**, uma por arquivo: `ChatLayout.svelte:10` (`Send`, `Sparkles`), `SearchBox.svelte:2` (`Search`) e `(app)/+layout.svelte:19` (11 símbolos: `Menu`, `MessageSquare`, `Hash`, `CheckSquare`, `Settings`, `User`, `LogOut`, `ChevronLeft`, `PanelRightClose`, `PanelRightOpen`, `Sparkles`). **14 símbolos / 13 únicos**, sintaxe de named import preservada, nenhum rename para `XIcon` (FR-001, SC-001) |
| 4 componentes UI com subpath `/icons/*` | `dialog-content.svelte`, `dropdown-menu-checkbox-item.svelte`, `dropdown-menu-radio-item.svelte`, `dropdown-menu-sub-trigger.svelte` — byte-idênticos ao base `9da1aaea6`, não editados (FR-001, SC-001) |
| `grep -n '"@lucide/svelte"' apps/frontend/package.json` | `22: "@lucide/svelte": "1.33.0",` — sem `^`, em `devDependencies`; entrada `lucide-svelte` saiu de `dependencies`, com `bits-ui:51`, `clsx:52`, `tailwind-merge:53` intactos (FR-002, FR-003, SC-002) |
| `bun pm ls` | `@lucide/svelte@1.33.0` resolvido; nenhuma entrada `lucide-svelte` na árvore (SC-002) |
| `git diff --numstat 9da1aaea6 -- apps/frontend/package-lock.json` | `+1/−12` — somente remoção da entrada `node_modules/lucide-svelte`, remoção do pacote legado de `dependencies` e pin de `@lucide/svelte@1.33.0`; nenhum marcador `shadcn-svelte`, `cn`, `commander`, `node-fetch-native` ou bloco wasm/@emnapi/tslib (FR-004) |
| `bun.lock` e `bun install --frozen-lockfile` (dry-run) | `bun.lock` sincronizado: `@lucide/svelte@1.33.0` mantido e pacote legado ausente; dry-run `--frozen-lockfile` `exit=0`; `node_modules/@lucide/svelte@1.33.0` preservado com `"dev": true`, sem mutação de `node_modules/` (FR-004, SC-002) |
| `bun run check` · `bun run lint` · `bun run test:unit -- --run` · `bun run build` | todos green; **42 testes** no run de unidade — sem regressão (SC-003) |
| `python3 scripts/ci/check_gate_integrity.py` (raiz) | gate do alvo **PASS** — sem regressão (SC-003) |
| `git diff --name-only 9da1aaea6` | exatamente os **6 paths de código** contratados (3 imports + `package.json` + `bun.lock` + `package-lock.json`) mais os artefatos do ciclo 084; zero PRs abertos (FR-005, SC-004) |
| `specs/084-dedupe-lucide/analyze.md` | `{"artifacts": 11, "errors": [], "fr": 5, "go": true, "sc": 4, "tasks": 5}` · `exit=0` na 1ª execução (R4) |
| Decisão 80/20 (`.specify/assessments/084-dedupe-lucide/decision.md`) | `E=(1×5)/(2×1)=2,50` — `go (scoped)`, opção A; nenhum critério `weak`/`unknown` |
| Verificador isolado `LucideVerifier` | **PASS-WITH-NITS** — migração correta nos 3 imports, no pin `1.33.0`, nos lockfiles, no grep zero e na fronteira de paths; nenhuma mudança de comportamento/API/contrato de runtime. Único nit (churn não relacionado no `package-lock.json`) fechado por `LockFix` e registrado em `converge.md` (entrada `2026-11-20T00:02Z`) |

### Saídas coladas

Imports migrados (estado atual dos 3 arquivos, base `9da1aaea6`):

```
apps/frontend/src/lib/components/chat/ChatLayout.svelte:10
	import { Send, Sparkles } from '@lucide/svelte';

apps/frontend/src/lib/components/search/SearchBox.svelte:2
	import { Search } from '@lucide/svelte';

apps/frontend/src/routes/(app)/+layout.svelte:8-19
	import {
		Menu,
		MessageSquare,
		Hash,
		CheckSquare,
		Settings,
		User,
		LogOut,
		ChevronLeft,
		PanelRightClose,
		PanelRightOpen,
		Sparkles
	} from '@lucide/svelte';
```

Manifesto e lockfiles (evidência consolidada do handoff):

```
$ grep -n '"@lucide/svelte"' apps/frontend/package.json
22:		"@lucide/svelte": "1.33.0",
# "lucide-svelte" ausente de dependencies; bits-ui:51, clsx:52, tailwind-merge:53 intactos

$ grep -rn "lucide-svelte" apps/frontend/src apps/frontend/package.json \
    apps/frontend/bun.lock apps/frontend/package-lock.json
(zero correspondências)

$ bun pm ls
@lucide/svelte@1.33.0

$ git diff --numstat 9da1aaea6 -- apps/frontend/package-lock.json
1   12  apps/frontend/package-lock.json
# +1/−12; sem shadcn-svelte / cn / commander / node-fetch-native / wasm

$ bun install --frozen-lockfile --dry-run
exit=0
```

Quality gates e analyzer:

```
bun run check      → green
bun run lint       → green
bun run test:unit -- --run → green (42 testes)
bun run build      → green
python3 scripts/ci/check_gate_integrity.py → PASS

specs/084-dedupe-lucide/analyze.md:
{"artifacts": 11, "errors": [], "fr": 5, "go": true, "sc": 4, "tasks": 5}
exit=0
```

### Nits fechados

- **Churn não relacionado no `package-lock.json`**: `npm install --package-lock-only` havia injetado ~105 linhas fora do escopo (entrada `shadcn-svelte@1.8.1`, `cn`/`commander`/`node-fetch-native`, 6 blocos wasm/@emnapi/tslib). `LockFix` removeu **somente** esses blocos (CUTs) preservando as 3 mudanças contratadas — remoção de `lucide-svelte` de `dependencies`, remoção de `node_modules/lucide-svelte` e pin de `@lucide/svelte@1.33.0` em `devDependencies`. Checagem somente leitura às `2026-11-20T00:02Z` já não encontra os marcadores de churn (registrado em `converge.md`, entrada 1). Resultado final: `+1/−12`. **Fechado**.
- Nenhum outro finding do `LucideVerifier`; nenhuma correção de fonte necessária no analyzer.

Nota de proveniência: esta passagem R4 não reexecutou gates, build, testes, formatters, git nem inspeção visual — as saídas acima são a evidência consolidada do ciclo (implementação + verificador isolado `LucideVerifier` **PASS-WITH-NITS** com nit fechado + auditoria determinística do main agent no handoff). Nenhuma inferência além dela. A contagem exata de `bun.lock` não foi recontada nesta passagem; sua sincronização está registrada em `converge.md`.

## Fronteira

Somente 6 paths de código mudaram contra a base `9da1aaea6`: `apps/frontend/src/lib/components/chat/ChatLayout.svelte` (`:10`), `apps/frontend/src/lib/components/search/SearchBox.svelte` (`:2`), `apps/frontend/src/routes/(app)/+layout.svelte` (`:19`), `apps/frontend/package.json`, `apps/frontend/bun.lock` e `apps/frontend/package-lock.json` (`+1/−12`). A mudança é a troca da string de origem de 3 imports (14 símbolos preservados), a remoção de `"lucide-svelte": "^1.0.1"` de `dependencies`, o pin `"@lucide/svelte": "1.33.0"` (sem `^`) em `devDependencies` e a sincronização dos dois lockfiles. Os 4 componentes UI com subpath `/icons/*`, `node_modules/`, `prettier.config.js`, gate/`scripts/ci/check_gate_integrity.py`, `.github/workflows/`, ADRs, constituição, STATE, `specs/083-*`, DB/rede, segredos, issues, leg RBAC, PRs e worktrees alheios permanecem intactos. Zero PRs abertos no snapshot. Nenhum `[NV]` foi promovido a fato verificado.

## [NV]

- **Regressão visual em browser**: os 13 `iconNode`s únicos comparados são idênticos entre `1.0.1` e `1.33.0`, mas não houve captura de tela nem inspeção visual; gates não detectam aparência de ícones.
- **Grafo final do bundler**: dual package residual não inspecionado após a troca dos 3 imports.
- **Upgrade `@lucide/svelte@1.76.0`**: latest informado pelo registry em `2026-11-19T23:47Z`, divergente do pin `1.33.0` mantido por decisão de Main; subida exigiria nova avaliação de risco visual e de escopo.
- **CI / PR / Forgejo / produção**: zero PRs no snapshot; status de CI ao vivo, merge, proteção de branch, aprovações e deploy não verificados neste ciclo.
- **Reexecução local de gates/formatters/git**: não executada nesta passagem de documentação; cobertura determinística atribuída ao main agent no handoff.
- **Contagem exata de `bun.lock`**: não recontada aqui; sincronização registrada em `converge.md`.

## Portarias

R1 `go (scoped)` antes da implementação (`decision.md`, `E=2,50`); R3 zero esclarecimentos pendentes; R4 este documento aprovado **antes** de `release.md`; R5 nenhum bug reportado em `.specify/bugs/` acionado neste ciclo; R7 release com SemVer `0.3.4 → 0.3.5`, changelog FR/SC/T e rollback dos 6 paths. Nenhuma invocação de `/speckit.validate` ou `/speckit.release` — ambos são artefatos escritos diretamente. Nenhuma tag, deploy ou merge executados; PR a critério do maintainer.
