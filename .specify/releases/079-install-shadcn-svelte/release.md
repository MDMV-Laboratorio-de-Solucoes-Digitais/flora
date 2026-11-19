# Release: 079 — instalação pinada do CLI `shadcn-svelte`

**Ciclo**: 079 · **Data**: 2026-11-19T21:18Z · **Branch**: `feature/079-install-shadcn-svelte` · **Baseline**: `ea5cd2069`
**Tipo**: nova dependência de ferramenta (tooling de geração) declarada como `devDependency` local e pinada. **Nenhuma tag, release, deploy, merge ou produção.**

## Entregue

- `apps/frontend/package.json` — 1 linha adicionada em `devDependencies`: `"shadcn-svelte": "1.8.1"` (exata, sem `^` e sem `~`), gerada por `bun add -D -E shadcn-svelte@1.8.1` executado em `apps/frontend` (`latest = 1.8.1` no registry em 2026-11-19, licença MIT, publicado em 2026-11-11T21:25Z).
- `apps/frontend/bun.lock` — +9 linhas: entrada `"shadcn-svelte": "1.8.1"` na lista do workspace e resolução do pacote `shadcn-svelte@1.8.1` com as transitivas `cn@0.3.3`, `commander@14.0.3`, `node-fetch-native@1.6.7` (`postcss@^8.5.10` já resolvido no lockfile); `bin: shadcn-svelte → ./dist/index.mjs`.
- Bin resolvido localmente: `bun run shadcn-svelte --help` lista `init`, `add`, `apply`, `registry build` **sem** executá-los; `bun pm ls shadcn-svelte` reporta `1.8.1`; `bun install --frozen-lockfile --dry-run` termina sem diff.
- `components.json`, `layout.css`, os 51 componentes e `utils.ts` byte-idênticos ao base `ea5cd2069` (verificador isolado `ShadcnVerifier` **PASS**).
- Artefatos completos: assess (5), spec, clarify, plan, checklist/tasks, analyze, converge append-only, validate R4, release R7.

## SemVer

**Proposta:** minor `0.2.9` → `0.3.0`. Motivo: adição de nova dependência de tooling ao workspace do frontend (superfície de ferramental ampliada, sem quebra de API/runtime e sem mudança de comportamento de runtime). A patch de 078 fora `0.2.8` → `0.2.9`; esta é a primeira minor desde aquela linha. Nenhuma tag criada.

## Changelog (FR / SC / T)

- `FR-001` `"shadcn-svelte": "1.8.1"` exato em `devDependencies` de `apps/frontend/package.json`; `FR-002` resolução registrada em `apps/frontend/bun.lock` (pacote + transitivas `cn`, `commander`, `node-fetch-native`, `postcss`); `FR-003` `components.json`, `layout.css`, os 51 `.svelte` e `utils.ts` preservados byte-a-byte, sem `init`/`add`/`apply`; `FR-004` formatação Prettier preservada (tabs, sem trailing commas, aspas duplas em JSON) e zero segredos, `.tokens/`, mudanças de gate/scripts/workflows ou mutação de DB/rede fora do registry; `FR-005` diff rastreado limitado aos 2 paths de código + artefatos 079, com `node_modules/` ignorado.
- `SC-001` `bun pm ls shadcn-svelte` → `1.8.1` e `grep` confirma versão exata sem caret; `SC-002` `bun run shadcn-svelte --help` lista os 4 subcomandos via bin local; `SC-003` `bun install --frozen-lockfile --dry-run` sem diff, `bun run lint` e `bun run check` sem regressão; `SC-004` `git diff --stat` mostra só `package.json` (1 linha) + `bun.lock` + artefatos 079, gate do alvo PASS, zero mutação de DB/segredos e 51 componentes byte-idênticos.
- `T001`..`T005` executados; verificador isolado `ShadcnVerifier` **PASS**; `analyze` `{"artifacts": 11, "errors": [], "fr": 5, "go": true, "sc": 4, "tasks": 5}`; formatters/check/lint/test:unit (42/42)/build/`check_gate_integrity.py` (PASS) todos `exit 0`; nenhum bug de `.specify/bugs/` acionado; leg RBAC permanece bloqueada por maintainer (fora do ciclo).

## Evidência

`.specify/validations/079-install-shadcn-svelte/validate.md` · `specs/079-install-shadcn-svelte/converge.md` · `specs/079-install-shadcn-svelte/analyze.md`.

## Rollback

Reverter, em `dev/`, exclusivamente os 2 paths — `apps/frontend/package.json` (removendo a linha `"shadcn-svelte": "1.8.1"` de `devDependencies`) e `apps/frontend/bun.lock` (restaurando o lockfile original do base `ea5cd2069`, sem a entrada do pacote nem as transitivas adicionadas) — com novo commit de reversão, sem force-push ou reescrita de histórico. Nenhum outro path é modificado: `components.json`, `layout.css`, os 51 componentes, `utils.ts`, `prettier.config.js`, `vite.config.ts`, gate/scripts/workflows, ADRs, constituição, STATE, specs alheias, DB, rede e segredos permanecem intactos. Os artefatos do ciclo 079 são documentação e podem ser revertidos em bloco.

## [NV]

CLI não executado além de `--help` (`init`/`add`/`apply` deliberadamente não executados por serem destrutivos); formatação e gates validados por saídas locais, sem CI ao vivo; Forgejo, aprovações, merge, produção e leg RBAC/maintainer: **[NV]**. Upgrade futuro de versão e dist-tag `next` (`1.0.0-next.19`) fora do ciclo. Nenhuma inferência além da evidência local, do registry e das fontes oficiais.
