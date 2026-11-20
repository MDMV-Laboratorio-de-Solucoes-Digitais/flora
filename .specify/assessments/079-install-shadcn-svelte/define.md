# Define: instalação pinada do CLI `shadcn-svelte`

- **Slug**: 079-install-shadcn-svelte · **Definido**: 2026-11-19T20:57Z

## Problema

O frontend usa componentes e contratos shadcn-svelte, mas o CLI que os gera/valida não é dependência declarada: ausente de `apps/frontend/package.json` e `bun.lock`. Quem precisar executar `shadcn-svelte` hoje depende de `bun x …@latest` (rede e versão flutuante). Instalar o pacote localmente com versão exata fecha a lacuna sem tocar os artefatos já corretos do ciclo 078.

## Afetados / metas / não-metas

Afetados: maintainers e agentes que executam o CLI no frontend; CI (lockfile rastreado). Metas: adicionar `shadcn-svelte` exatamente `1.8.1` em `devDependencies` via `bun add -D -E` em `apps/frontend`, commitar `bun.lock` e **não** executar `init`/`add`/`apply`. Não-metas: mudar `components.json`, `layout.css`, os 51 componentes, `utils.ts`, `prettier.config.js`, `vite.config.ts`, scripts/workflows/CI, gate, DB/rede fora do registry, segredos, `.tokens/`, specs/079-* (exceto os 5 artefatos), ADRs, constituição, STATE, fechamento de issues, leg RBAC e a tag `next`.

## Requisitos funcionais

- **FR-001** — declarar `"shadcn-svelte": "1.8.1"` (exato, sem `^` e sem `~`) em `devDependencies` de `apps/frontend/package.json`, gerado por `bun add -D -E shadcn-svelte@1.8.1` executado em `apps/frontend` (`latest = 1.8.1` no registry em 2026-11-19).
- **FR-002** — registrar a resolução em `apps/frontend/bun.lock` (entrada do pacote + 4 deps transitivas `cn`, `postcss`, `commander`, `node-fetch-native`), mantendo lockfile rastreado e consistente com `package.json`.
- **FR-003** — preservar byte-idênticos `$schema`, `style`, `tailwind` e `aliases` de `apps/frontend/components.json:1-15`, `src/routes/layout.css`, os 51 `.svelte` de `src/lib/components/ui/` e `src/lib/utils.ts`; não executar `init`, `add` nem `apply` (docs oficiais: `init` sobrescreve o CSS global, recria `components.json` e reinstala componentes).
- **FR-004** — manter a formatação de `package.json` conforme `prettier.config.js:5-8` (tabs, sem trailing commas; aspas duplas obrigatórias em JSON) e não introduzir segredos, `.tokens/`, mudanças de gate/scripts/workflows nem mutação de DB/rede fora do registry.
- **FR-005** — limitar o diff rastreado a `apps/frontend/package.json` + `apps/frontend/bun.lock` (além dos 5 artefatos 079); `node_modules/` permanece ignorado e local.

## Métricas

- **SC-001** — `bun pm ls shadcn-svelte` reporta `shadcn-svelte@1.8.1` no workspace `apps/frontend`; `grep` em `package.json`/`bun.lock` confirma a versão exata, sem caret. `[NV]` depende da instalação.
- **SC-002** — `bun run shadcn-svelte --help` (em `apps/frontend`) lista `init`, `add`, `apply` e `registry build`: bin resolvido localmente, sem `bun x`. `[NV]` depende da instalação.
- **SC-003** — `bun install --frozen-lockfile --dry-run` em `apps/frontend` termina sem diff de lockfile; `bun run lint` (inclui `prettier --check .`) e `bun run check` sem regressão de formatação/tipo. `[NV]` depende da instalação.
- **SC-004** — `git diff --stat` mostra só `apps/frontend/package.json` (1 linha adicionada) e `apps/frontend/bun.lock`, mais os 5 artefatos 079; gate do alvo PASS; zero mutação de DB/segredos; `components.json` e os 51 componentes byte-idênticos ao base `ea5cd2069`.

## Baseline

`apps/frontend/package.json:19-54` sem `shadcn`; `bun.lock` sem a entrada; `components.json:1-15` já corrigido (078); 51 componentes em `src/lib/components/ui/`; `src/lib/utils.ts` com `cn`; CI em `.github/workflows/ci.yml:21-30` roda `bun install` sem `--frozen-lockfile`.
