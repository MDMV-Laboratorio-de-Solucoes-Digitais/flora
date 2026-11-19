# Shape: caminhos de configuração do `components.json` (shadcn-svelte)

- **Slug**: 078-shadcn-config-paths · **Recomendado**: Opção A (remover `config` + corrigir `css`) · **Data**: 2026-11-19T20:21Z

## Opção A — remover a chave deprecada e apontar o CSS real (recomendada)

- **Sketch**: em `apps/frontend/components.json`, deletar a linha 5 (`"config": "tailwind.config.ts"`) e trocar o valor da linha 6 para `"src/routes/layout.css"`. Appetite: trivial (2 linhas, 1 arquivo).
- **Trade-offs**: alinha o arquivo ao schema oficial (`config` deprecado em v4; `css` é o caminho do CSS que importa Tailwind) e às duas fontes locais (`+layout.svelte:2`, `prettier.config.js:11`), sem tocar runtime, gate, dependências ou segredos. Zero consumidores locais ⇒ risco de regressão nulo no curto prazo. Risco: nenhum identificado; `[NV]` validação pelo CLI externo não instalada.
- **Rabbit holes**: "criar" um `tailwind.config.ts` vazio para satisfazer a chave; trocar `baseColor`/`style`; editar `prettier.config.js`, `vite.config.ts` ou `layout.css` para acomodar o JSON; instalar `shadcn-svelte` só para validar.

## Opção B — manter `config` com valor vazio (`"config": ""`)

- **Sketch**: preservar a chave, esvaziando o valor.
- **Trade-offs**: não resolve nada — a chave continua deprecada no schema, aponta para lugar nenhum e adiciona um valor que nem é um caminho; `additionalProperties=false` no objeto `tailwind` não a remove. Pior que A: mantém metadado enganoso com aparência de "corrigido".

## Opção C — recriar `tailwind.config.ts` / migrar para config JS

- **Sketch**: criar arquivo de configuração e manter a chave.
- **Trade-offs**: contradiz Tailwind v4 (`@tailwindcss/vite` em `vite.config.ts:3,9`, com config CSS-first via `@theme` em `layout.css:3`), adiciona arquivo morto e expande o diff para fora de `components.json`. Custo > benefício; fora de escopo.

## Fora de escopo

`style`, `baseColor`, `cssVariables`, `$schema`, `aliases`, `prettier.config.js`, `package.json`, `vite.config.ts`, `layout.css`, `+layout.svelte`, gate/scripts/workflows (`scripts/ci/check_gate_integrity.py`), supply-chain, ADRs, constituição, STATE, specs/078-* (exceto os 5 artefatos), DB/rede, segredos, fechamento de issues, leg RBAC, instalação do CLI `shadcn-svelte`.

## Premissas

O arquivo é opcional e "only required if you're using the CLI" (`docs/components-json`, acessado 2026-11-19); Tailwind v4 não usa `tailwind.config.*`; nenhum consumidor local dos campos (varredura determinística); `prettier.config.js:11` já aponta `src/routes/layout.css`. `style` (deprecado em v4) e `cssVariables` (não declarado no schema) permanecem intactos por escolha de escopo mínimo — `[NV]` validação pelo CLI externo não instalada.
