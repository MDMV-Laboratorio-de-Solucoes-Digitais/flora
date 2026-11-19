# Research: instalação pinada do CLI `shadcn-svelte`

- **Coletado**: 2026-11-19T20:57Z · fontes: `apps/frontend/package.json`, `apps/frontend/bun.lock`, `apps/frontend/components.json`, `apps/frontend/prettier.config.js`, `.github/workflows/ci.yml`, registry npm/bun e docs oficiais do shadcn-svelte.

## Evidência local

- `apps/frontend/package.json:19-48` — `devDependencies` com entradas em caret (`^`); `:49-54` — `dependencies`: `bits-ui`, `clsx`, `lucide-svelte`, `tailwind-merge`. `grep -rn "shadcn"` em `package.json` + `bun.lock` → zero correspondências: CLI ausente.
- `apps/frontend/bun.lock:1-` — `lockfileVersion: 2`; o workspace `""` espelha as mesmas listas de dependências; nenhuma entrada `shadcn-svelte`.
- `apps/frontend/components.json:1-15` — já corrigido no ciclo 078: `tailwind.css = src/routes/layout.css`, `baseColor: zinc`, sem a chave `config` deprecada; aliases `$lib/components`, `$lib/utils`, `$lib/components/ui`, `$lib/hooks`. Não deve ser reescrito.
- `apps/frontend/src/lib/utils.ts` — util `cn` presente; `find apps/frontend/src/lib/components/ui -name '*.svelte' | wc -l` → 51. Ou seja, `init` seria redundante e destrutivo.
- `apps/frontend/prettier.config.js:5-8,11` — `useTabs: true`, `singleQuote: true`, `trailingComma: 'none'`, `printWidth: 100`, `tailwindStylesheet: './src/routes/layout.css'`: risco de formatação no `package.json` precisa ser checado após `bun add` (`bun run lint` roda `prettier --check .`).
- `.github/workflows/ci.yml:21-30` — `bun install` → `bun run check` → `bun run lint` → `bun run test:unit -- --run --project server` → `bun run build`; sem `--frozen-lockfile`, mas o lockfile é rastreado e deve ser commitado junto.

## Fontes primárias (dados; acessadas 2026-11-19)

- Registro npm/bun (`npm view shadcn-svelte@1.8.1 …` e `bun info shadcn-svelte`, 2026-11-19): `dist-tags.latest = 1.8.1`, `next = 1.0.0-next.19`; licença **MIT**; bin `shadcn-svelte` → `./dist/index.mjs`; 4 dependências (`cn ^0.3.0`, `postcss ^8.5.10`, `commander ^14.0.0`, `node-fetch-native ^1.6.4`); publicado em 2026-11-11T21:25Z; mantenedor `huntabyte`; repo `github.com/huntabyte/shadcn-svelte` (`packages/cli`). Fonte canônica: `https://registry.npmjs.org/shadcn-svelte`.
- `https://shadcn-svelte.com/docs/installation/sveltekit` (acessado 2026-11-19) — CLI invocado de forma efêmera (`bun x shadcn-svelte@latest init`, depois `… add button`); pergunta "Where is your global CSS file? (this file will be overwritten)" → `src/routes/layout.css`.
- `https://shadcn-svelte.com/docs/cli` (acessado 2026-11-19) — `init`: "initialize your project and install dependencies. The init command installs dependencies, adds the `cn` util, and creates CSS variables for the project."; opções `--reinstall` / `--no-reinstall`, `--css`, `--components-alias`, `--base-color`; `add` e `apply` também via `bun x …@latest`.
- `https://shadcn-svelte.com/docs/components-json` (acessado 2026-11-19) — "`components.json` file is optional and **only required if you're using the CLI**"; `tailwind.css` = "Path to the CSS file that imports Tailwind CSS into your project"; `tailwind.baseColor` não pode mudar após a inicialização.

## Riscos / lacunas

- `[NV]` execução de `bun add -D -E shadcn-svelte@1.8.1` — fora desta avaliação (escopo somente leitura); é o passo de implementação já autorizado. Cobertura por registry/docs e pelas checagens locais acima.
- `[NV]` `bun pm ls shadcn-svelte`, `bun run shadcn-svelte --help`, `bun install --frozen-lockfile --dry-run` e CI verde — dependem da instalação e da atualização do lockfile.
- `[NV]` `init`/`add`/`apply` deliberadamente não executados: sobrescreveriam o CSS global e `components.json` e poderiam reinstalar os 51 componentes (docs oficiais, 2026-11-19).
- Risco de formatação: `bun add` pode reescrever `package.json`; conferir o diff e `prettier --check` (tabs, sem trailing commas). Sem mutação de DB e sem segredos; único acesso de rede é o registry, explicitamente autorizado.
