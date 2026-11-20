# Release: 084 — deduplicação de dependências lucide

**Ciclo**: 084 · **Data**: 2026-11-20T00:21Z · **Branch**: `feature/084-dedupe-lucide` · **Baseline**: `9da1aaea6`
**Tipo**: migração mecânica de dependência (`lucide-svelte` → `@lucide/svelte`), sem mudança de API, comportamento ou contrato de runtime. **Nenhuma tag, release, deploy, merge ou produção.**

## Entregue

- `apps/frontend/src/lib/components/chat/ChatLayout.svelte:10` — origem do import trocada para `'@lucide/svelte'`, preservando `Send` e `Sparkles`.
- `apps/frontend/src/lib/components/search/SearchBox.svelte:2` — origem trocada, preservando `Search`.
- `apps/frontend/src/routes/(app)/+layout.svelte:8-19` — bloco multilinha trocado (inclusive o fechamento em `:19`), preservando os 11 símbolos (`Menu`, `MessageSquare`, `Hash`, `CheckSquare`, `Settings`, `User`, `LogOut`, `ChevronLeft`, `PanelRightClose`, `PanelRightOpen`, `Sparkles`). Total: **14 símbolos / 13 únicos**, named imports intactos, nenhum rename.
- `apps/frontend/package.json` — `"lucide-svelte": "^1.0.1"` removido de `dependencies`; `"@lucide/svelte": "1.33.0"` fixado sem caret em `devDependencies` (`:22`). `bits-ui`, `clsx` e `tailwind-merge` intactos.
- `apps/frontend/bun.lock` — pacote legado removido; `@lucide/svelte@1.33.0` mantido.
- `apps/frontend/package-lock.json` — `+1/−12`: entrada `node_modules/lucide-svelte` removida, pacote legado fora de `dependencies`, `node_modules/@lucide/svelte@1.33.0` preservado com `"dev": true`. Churn não relacionado injetado por `npm install --package-lock-only` (shadcn-svelte/cn/commander/node-fetch-native/wasm) removido por `LockFix`; nenhum marcador permanece.
- Verificação: `grep lucide-svelte` → zero; `bun pm ls` → `@lucide/svelte@1.33.0`; `--frozen-lockfile` dry-run `exit=0`; `check`/`lint`/`test:unit`/`build` green (42 testes); gate **PASS**; `git diff --name-only` restrito aos 6 paths + artefatos 084; zero PRs.
- Os 4 componentes UI com subpath `/icons/*` permanecem byte-idênticos ao base. Artefatos completos: 5 de assessment, 6 documentos do ciclo, `analyze.md`, `converge.md` (entrada `2026-11-20T00:02Z`), `validate.md` R4 aprovado e este release R7. Verificador isolado `LucideVerifier`: **PASS-WITH-NITS**, com o único nit (churn do lockfile) fechado por `LockFix`.

## SemVer

**Proposta:** patch `0.3.4` → `0.3.5` (deduplicação mecânica de dependência deprecada, sem mudança de API, comportamento, contrato de runtime, CI ou config; a branch 083 entregou `0.3.3 → 0.3.4`). **Ressalva:** o maintainer pode optar por minor `0.4.0`, dado o precedente 079 de mudança de superfície de dependência. Registro opcional a critério do maintainer. **Nenhuma tag criada.**

## Changelog (FR / SC / T)

- `FR-001` 3 imports migrados para `'@lucide/svelte'` com os 14 símbolos e a sintaxe de named import preservados (`ChatLayout.svelte:10`, `SearchBox.svelte:2`, `(app)/+layout.svelte:8-19`, incluindo o fechamento em `:19`); nenhum rename; os 4 componentes UI `/icons/*` não tocados; `FR-002` `"lucide-svelte": "^1.0.1"` removido de `dependencies` com reflexo em `bun.lock`; `FR-003` `"@lucide/svelte": "1.33.0"` fixado sem caret em `devDependencies` (`package.json:22`) com reflexo em `bun.lock` e `"dev": true` preservado no lockfile; `FR-004` `package-lock.json` sincronizado (`+1/−12`) com remoção de `node_modules/lucide-svelte` e manutenção de `@lucide/svelte@1.33.0`, sem churn; `FR-005` diff rastreado limitado aos 6 paths de código + artefatos 084, com formatação local preservada (tabs, aspas simples, sem trailing comma) e nenhuma dependência nova, segredo, gate, CI, ADR ou PR.
- `SC-001` `grep` do pacote legado retorna zero e os 3 imports apontam para o root novo com 14 símbolos / 13 únicos; `SC-002` pin exato `"1.33.0"` sem `^` e `bun pm ls` mostra somente `@lucide/svelte@1.33.0`; `SC-003` `check`, `lint`, `test:unit -- --run` (42 testes) e `build` green e gate autoritativo PASS — regressão visual permanece `[NV]`; `SC-004` `git diff --name-only` lista somente os 6 paths + artefatos 084, zero mutação de gate/CI/scripts/workflows/ADRs/STATE/specs/DB/rede/segredos/`node_modules/` e zero PRs.
- `T001`..`T005` executados, IDs únicos e sequenciais; verificador isolado `LucideVerifier` **PASS-WITH-NITS** com nit de churn fechado por `LockFix` e registrado em `converge.md`; `analyze` `{"artifacts": 11, "errors": [], "fr": 5, "go": true, "sc": 4, "tasks": 5}` (`exit=0`, 1ª execução); decisão `E=(1×5)/(2×1)=2,50` (`go (scoped)`, opção A). Nenhum bug de `.specify/bugs/` acionado; leg RBAC/maintainer permanece fora do ciclo.

## Evidência

`.specify/validations/084-dedupe-lucide/validate.md` (R4 **APROVADO**) · `specs/084-dedupe-lucide/converge.md` · `specs/084-dedupe-lucide/analyze.md` · `.specify/assessments/084-dedupe-lucide/decision.md`.

## Rollback

Reverter, em `dev/`, exclusivamente os 6 paths rastreados ao estado do base `9da1aaea6`:

```
git checkout 9da1aaea6 -- \
  apps/frontend/src/lib/components/chat/ChatLayout.svelte \
  apps/frontend/src/lib/components/search/SearchBox.svelte \
  'apps/frontend/src/routes/(app)/+layout.svelte' \
  apps/frontend/package.json apps/frontend/bun.lock apps/frontend/package-lock.json
```

Isso devolve os 3 imports ao pacote legado (`'lucide-svelte'`), `"@lucide/svelte": "^1.33.0"` em `devDependencies`, `"lucide-svelte": "^1.0.1"` em `dependencies` e os dois lockfiles byte-idênticos ao base. Os 4 componentes UI `/icons/*` não são tocados; `node_modules/` não é rastreado (reconstrução da árvore é assunto de ambiente, `[NV]`). Os artefatos do ciclo 084 (5 de assessment e 6 documentos, mais analyze/converge/validate/release) são documentação e podem ser revertidos em bloco. Novo commit de reversão, sem force-push ou reescrita de histórico. Nenhum outro path é tocado — gate, CI, workflows, ADRs, constituição, STATE, specs alheias, PRs, produção, DB/rede, segredos e worktrees alheios permanecem intactos. Após a reversão, repetir os gates 1–5 de `plan.md` para confirmar o retorno ao estado base.

## [NV]

Regressão visual em browser (13 `iconNode`s idênticos, mas sem captura/inspeção); grafo final do bundler quanto a dual package residual; contagem exata de `bun.lock`; reexecução local de gates/formatters/git nesta passagem de documentação; CI ao vivo, Forgejo/PRs, merge, proteção de branch, aprovações e deploy; upgrade futuro para `@lucide/svelte@1.76.0` (latest em 2026-11-19T23:47Z), que exigiria nova avaliação de risco visual e de escopo. Nenhuma inferência além da evidência local consolidada no ciclo.
