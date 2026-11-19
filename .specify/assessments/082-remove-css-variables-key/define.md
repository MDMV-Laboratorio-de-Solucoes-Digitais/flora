# Define: remover a chave `cssVariables` de `components.json`

- **Slug**: 082-remove-css-variables-key · **Definido**: 2026-11-19T22:31Z

## Problema

`apps/frontend/components.json:7` declara `"cssVariables": true` dentro de `tailwind`, mas o schema oficial (`https://shadcn-svelte.com/schema.json`, acessado 2026-11-19) declara apenas `css`, `baseColor` e `config` nesse objeto, com `additionalProperties = false` e `required = ["css","baseColor"]`: `cssVariables` não existe em nenhum nível e invalida validação estrita. O CLI instalado `shadcn-svelte@1.8.1` usa Zod `$strip` (`apps/frontend/node_modules/shadcn-svelte/dist/schema-Bq2uExa4.d.mts:407-414`) e descarta a chave em silêncio; nenhum consumidor local a lê. O resultado é metadado inválido e enganoso em um arquivo que 078 já corrigiu parcialmente (naquela ocasião, `cssVariables` foi preservado por escopo mínimo).

## Afetados / metas / não-metas

Afetados: maintainers e agentes que validam `components.json` contra o schema oficial ou executam o CLI no futuro; nenhum runtime (nenhum script/CI lê o arquivo). Metas: remover exatamente a chave `cssVariables` e ajustar a vírgula resultante, preservando todo o restante byte-idêntico. Não-metas: `style` (declarado no schema, apenas deprecado em v4 — manter), `baseColor`, `$schema`, `aliases`, `tailwind.css`, `prettier.config.js`, `package.json`, `vite.config.ts`, `layout.css`, `+layout.svelte`, `node_modules/`, gate/scripts/workflows, DB/rede, segredos, ADRs, constituição, STATE, specs/078-* e specs/082-* (exceto os 5 artefatos).

## Requisitos funcionais

- **FR-001** — remover a linha `apps/frontend/components.json:7` (`"cssVariables": true`) do objeto `tailwind`. A chave não existe em nenhum nível do schema oficial (`https://shadcn-svelte.com/schema.json`, acessado 2026-11-19: `tailwind.properties = {css, baseColor, config}`, `tailwind.additionalProperties = false`) e é descartada por `$strip` no parser Zod do CLI instalado (`schema-Bq2uExa4.d.mts:410-414`).
- **FR-002** — ajustar a linha `apps/frontend/components.json:6` de `"baseColor": "zinc",` para `"baseColor": "zinc"` (remover a vírgula que separava `baseColor` da chave agora excluída), preservando o valor `zinc`. Resultado: `tailwind` com exatamente `css` e `baseColor`, JSON válido e sem trailing comma (`prettier.config.js:7`).
- **FR-003** — preservar byte-idênticos `$schema` (`:2`), `style` (`:3`), `tailwind.css` (`:5`), o valor de `tailwind.baseColor` (`:6`) e todo o bloco `aliases` (`:9-14`). Manter a formatação do Prettier local: tabs (`prettier.config.js:5`), aspas duplas obrigatórias em JSON (`singleQuote` em `:6` vale para JS/TS) e nenhuma trailing comma (`:7`). **Não remover `style`**: ele está declarado no schema (`schema-Bq2uExa4.d.mts:409`; `properties.style` online) e apenas deprecado em v4.
- **FR-004** — limitar o diff rastreado a `apps/frontend/components.json` (1 linha removida + 1 vírgula ajustada) e aos 5 artefatos `.specify/assessments/082-remove-css-variables-key/`; nenhum outro arquivo de código, config, doc, gate, CI, lockfile ou `node_modules/` é modificado; base `8f24519fe` preservada em todo o resto. Nenhuma dependência nova, segredo ou mutação de DB/rede.

## Métricas

- **SC-001** — `python3 -c` com `json.load` em `apps/frontend/components.json` retorna `set(obj["tailwind"]) == {"css", "baseColor"}` → `True`, e `cssVariables` está ausente em todos os níveis do objeto carregado. `[NV]` depende da edição.
- **SC-002** — varredura determinística por `cssVariables` no checkout (excluindo `node_modules`/`.git`) não retorna `apps/frontend/components.json` nem qualquer arquivo de código/script/CI; apenas os registros históricos já existentes em `.specify/memory/STATE.md:626,644` e `specs/078-*`/`.specify/assessments/078-*` permanecem. `$schema`, `style`, `baseColor`, `css` e `aliases` seguem byte-idênticos.
- **SC-003** — em `apps/frontend`, `bunx prettier --check components.json` termina com exit 0; `bun run check`, `bun run lint`, `bun run test:unit` e `bun run build` seguem sem regressão; `python3 scripts/ci/check_gate_integrity.py` mantém gate PASS. Nenhum desses comandos lê `cssVariables` em runtime. `[NV]` não executados nesta avaliação.
- **SC-004** — `git diff --name-only` lista somente `apps/frontend/components.json` e os 5 artefatos 082; zero mutação de gate, CI, scripts, workflows, lockfiles, `node_modules/`, ADRs, specs numeradas, DB, rede ou segredos; zero PRs abertos.

## Baseline

Branch `feature/082-remove-css-variables-key`, HEAD `8f24519fe`, gate PASS, zero PRs. `apps/frontend/components.json:4-8` com `tailwind = {css, baseColor, cssVariables}` e vírgula em `:6`; schema oficial (acessado 2026-11-19) sem `cssVariables` e com `style` válido; CLI `shadcn-svelte@1.8.1` instalado com parser `$strip` (`package.json:39`, `schema-Bq2uExa4.d.mts:407-414`); zero consumidores locais; 078 registrou a chave como pendência de escopo mínimo.
