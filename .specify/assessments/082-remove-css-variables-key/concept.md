# Shape: remover a chave `cssVariables` de `components.json`

- **Slug**: 082-remove-css-variables-key · **Recomendado**: Opção A (remover `:7` e a vírgula de `:6`) · **Data**: 2026-11-19T22:31Z

## Opção A — remover a chave inválida, preservar o resto (recomendada)

- **Sketch**: em `apps/frontend/components.json`, deletar a linha `:7` (`"cssVariables": true`) e trocar `"baseColor": "zinc",` (`:6`) por `"baseColor": "zinc"`. Appetite: trivial — 1 linha removida, 1 vírgula ajustada, 1 arquivo.
- **Trade-offs**: alinha o arquivo ao schema oficial (`tailwind.properties = {css, baseColor, config}`, `additionalProperties = false`, `https://shadcn-svelte.com/schema.json` acessado 2026-11-19), eliminando metadado que falha validação estrita, sem tocar comportamento: o CLI `shadcn-svelte@1.8.1` já descarta a chave via Zod `$strip` (`schema-Bq2uExa4.d.mts:410-414`, `index-Ch0UJ9Cm.d.mts:13-16`) e nenhum consumidor local a lê. `style` fica intacto porque é declarado no schema (apenas deprecado em v4, `:409`). `[NV]` validação online do arquivo editado e comandos pós-edição não executados nesta avaliação.
- **Rabbit holes**: remover `style` por também ser deprecado em v4 (é válido e declarado — fora de escopo); esquecer a vírgula de `:6` (JSON inválido + `trailingComma: 'none'` em `prettier.config.js:7`); renomear `cssVariables` para outra grafia (mantém chave inexistente no schema); editar `baseColor`, `css`, `aliases`, `prettier.config.js`, `package.json` ou `layout.css`; instalar/atualizar o CLI (já presente em `package.json:39`); tocar os registros históricos em `STATE.md`/`specs/078-*`.

## Opção B — manter `cssVariables` e confiar no `$strip` do CLI

- **Sketch**: não editar; aceitar que o parser descarta a chave.
- **Trade-offs**: zero diff e nenhuma regressão de runtime, mas preserva exatamente o metadado que invalida validação estrita do schema (`additionalProperties = false`). É o estado que 078 deixou como pendência e que a auditoria ranqueou em 5,0 (`STATE.md:644`). Rejeitada: o custo da correção é uma vírgula.

## Opção C — migrar `cssVariables` para um campo equivalente no schema

- **Sketch**: inventar/renomear para alguma chave supostamente equivalente (ex.: `iconLibrary`, `menuColor`) ou mover o booleano para outro bloco.
- **Trade-offs**: não existe campo equivalente no schema oficial; criaria semântica nova, expandiria o diff e exigiria decisão de produto/ADR. Rejeitada.

## Fora de escopo

`style`, `baseColor` (valor), `$schema`, `aliases`, `tailwind.css`, `apps/frontend/prettier.config.js`, `apps/frontend/package.json`, `vite.config.ts`, `layout.css`, `+layout.svelte`, `apps/frontend/node_modules/`, gate e `scripts/ci/check_gate_integrity.py`, `.github/workflows/`, supply-chain, ADRs, constituição, STATE, specs/078-*, specs/082-* (exceto os 5 artefatos), DB/rede, segredos, `.tokens/`, fechamento de issues, leg RBAC e lockfiles.

## Premissas

`cssVariables` não existe em nenhum nível do schema oficial (`https://shadcn-svelte.com/schema.json`, acessado 2026-11-19) e é descartada pelo parser Zod `$strip` do CLI instalado (`schema-Bq2uExa4.d.mts:407-414`; `package.json:39` fixa `shadcn-svelte 1.8.1`); `style` está declarado e só é deprecado em v4 (`:409`), logo permanece; nenhum script/CI/runtime lê `components.json` (varredura determinística); aspas duplas são obrigatórias em JSON e `singleQuote: true` (`prettier.config.js:6`) não se aplica; FR-001..FR-004 e SC-001..SC-004 de `define.md` regem a implementação. `[NV]` validação online contra o schema e execução dos comandos pós-edição — não verificados nesta avaliação, sem inferência além da evidência local e das fontes oficiais.
