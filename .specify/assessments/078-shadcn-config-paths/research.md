# Research: caminhos de configuração do `components.json` (shadcn-svelte)

- **Coletado**: 2026-11-19T20:21Z · fontes: `apps/frontend/components.json`, toolchain Tailwind v4 local, docs e schema oficiais do shadcn-svelte, varredura determinística de consumidores.

## Evidência local

- `apps/frontend/components.json:5` — `"config": "tailwind.config.ts"`; `ls apps/frontend/tailwind.config.*` → arquivo inexistente.
- `apps/frontend/components.json:6` — `"css": "src/app.css"`; `ls apps/frontend/src/app.css` → arquivo inexistente.
- `apps/frontend/src/routes/layout.css:1` — `@import 'tailwindcss'`: é o CSS real que importa Tailwind.
- `apps/frontend/src/routes/+layout.svelte:2` — `import './layout.css';`: confirma o arquivo efetivamente carregado.
- `apps/frontend/prettier.config.js:11` — `tailwindStylesheet: './src/routes/layout.css'`: segunda fonte local a apontar o caminho correto.
- `apps/frontend/package.json` — `tailwindcss ^4.3.0`, `@tailwindcss/vite ^4.3.0`; `apps/frontend/vite.config.ts:3,9` — `@tailwindcss/vite` registrado como plugin. Tailwind v4 não usa `tailwind.config.*`.
- Varredura determinística (`grep -rn "components.json"` em *.ts/*.js/*.json/*.yml/*.yaml/*.sh/*.py, excluindo `node_modules` e worktrees): zero correspondências — nenhum script, CI, gate ou CLI local lê o arquivo. `grep` por `tailwindStylesheet` retorna apenas `prettier.config.js:11`. `package.json` não contém `shadcn` — CLI não instalado.

## Fontes primárias (dados; acessadas 2026-11-19)

- `https://shadcn-svelte.com/docs/components-json` — `tailwind.css` é "Path to the CSS file that imports Tailwind CSS into your project"; `tailwind.baseColor` gera a paleta inicial; `components.json` é opcional e "only required if you're using the CLI". A página documenta `css`/`baseColor` e não documenta `tailwind.config`.
- `https://shadcn-svelte.com/schema.json` — `tailwind.config`: "DEPRECATED IN TAILWIND v4! The path to your `tailwind.config.[js|ts]` file."; `tailwind.required = ["css","baseColor"]`; `tailwind.additionalProperties = false`; `style` também marcado "DEPRECATED IN TAILWIND v4!". O guia de migração v4 recomenda remover `tailwind.config`.

## Riscos / lacunas

- `[NV]` validação pelo CLI externo `shadcn-svelte` (não instalado; `bunx shadcn-svelte@latest` exigiria rede) — não executada; cobertura por schema/docs oficiais e checagens locais determinísticas.
- Observação de escopo: o schema declara apenas `css`, `baseColor` e `config` dentro de `tailwind` (`additionalProperties=false`), e `style` está deprecado em v4; `components.json:3,8` mantém `style`/`cssVariables`. Nenhum consumidor local; permanecem intactos para manter o diff mínimo (decisão go scoped).
- Sem mutação de DB/rede/segredos. Nenhuma extensão não coberta pela varredura altera a conclusão, pois o arquivo não é lido em runtime.
