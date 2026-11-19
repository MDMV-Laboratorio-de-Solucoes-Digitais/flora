# Define: caminhos de configuração do `components.json` (shadcn-svelte)

- **Slug**: 078-shadcn-config-paths · **Definido**: 2026-11-19T20:21Z

## Problema

`apps/frontend/components.json` descreve um setup que não existe: `config` aponta para `tailwind.config.ts` (inexistente e deprecado em Tailwind v4, conforme `schema.json`) e `css` aponta para `src/app.css` (inexistente). O CSS real é `src/routes/layout.css:1`, já reconhecido por `+layout.svelte:2` e `prettier.config.js:11`. Como o CLI `shadcn-svelte` não está instalado e nenhum consumidor local lê esses campos, o dano é de contrato enganoso: qualquer geração futura de componentes usaria caminhos inválidos.

## Afetados / metas / não-metas

Afetados: maintainers e agentes que leem/validam `components.json` ou executam o CLI no futuro. Metas: opção A — remover a chave deprecada `config` e corrigir `css` para o caminho real, preservando o restante. Não-metas: mudar `style`, `baseColor`, `cssVariables`, `$schema`, `aliases`, `prettier.config.js`, `package.json`, `vite.config.ts`, `layout.css`, `+layout.svelte`, gate/scripts/workflows, DB/rede, segredos, specs/078-* (exceto os 5 artefatos), ADRs, constituição, STATE, fechamento de issues, leg RBAC.

## Requisitos funcionais

- **FR-001** — remover `"config": "tailwind.config.ts"` de `tailwind` em `apps/frontend/components.json:5`; a chave é "DEPRECATED IN TAILWIND v4" no schema oficial (acessado 2026-11-19).
- **FR-002** — corrigir `"css": "src/app.css"` (linha 6) para `"css": "src/routes/layout.css"`, caminho que efetivamente importa Tailwind (`layout.css:1`, `+layout.svelte:2`) e já usado por `prettier.config.js:11`.
- **FR-003** — preservar byte-idênticos `$schema`, `style`, `baseColor: "zinc"`, `cssVariables: true` e todo o bloco `aliases` (`components.json:1-4,7-16`); manter a formatação do Prettier local: tabs e sem trailing commas (`prettier.config.js:6-8`); aspas duplas são obrigatórias em JSON (`singleQuote` vale para JS/TS).
- **FR-004** — não introduzir dependências, segredos, credenciais, `.tokens/`, mudanças de gate/scripts/workflows nem mutação de DB/rede; nenhum outro arquivo além de `components.json` e dos 5 artefatos 078.

## Métricas

- **SC-001** — `components.json` não contém mais a chave `tailwind.config`.
- **SC-002** — `tailwind.css` aponta para arquivo existente (`src/routes/layout.css`) que contém `@import 'tailwindcss'` e casa com `prettier.config.js:11`.
- **SC-003** — `python3 -c` com `json.load` valida o JSON; `bunx prettier --check components.json` limpo; `bun run check`, `bun run lint`, `bun run test:unit` e `scripts/ci/check_gate_integrity.py` sem regressão (sem DB/rede).
- **SC-004** — diff restrito a `apps/frontend/components.json` (1 linha removida + 1 valor trocado) e aos 5 artefatos 078; gate do alvo PASS; zero mutação de DB/rede/segredos.

## Baseline

`components.json:5-6` com `config`/`css` apontando para arquivos inexistentes; CSS real em `src/routes/layout.css:1`; Tailwind v4 via `@tailwindcss/vite` (`vite.config.ts:3,9`); zero consumidores locais; CLI `shadcn-svelte` ausente.
