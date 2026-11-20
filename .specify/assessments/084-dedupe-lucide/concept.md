# Shape: deduplicar dependências lucide (`lucide-svelte` → `@lucide/svelte`)

- **Slug**: 084-dedupe-lucide · **Recomendado**: Opção A (named imports do root + pin `-E` 1.33.0) · **Data**: 2026-11-19T23:47Z

## Opção A — named imports do root, remoção do legado, pin exato (recomendada)

- **Sketch**: em `ChatLayout.svelte:10`, `SearchBox.svelte:2` e `routes/(app)/+layout.svelte:19`, trocar apenas a string `'lucide-svelte'` por `'@lucide/svelte'`; `bun remove lucide-svelte`; `bun add -D -E @lucide/svelte@1.33.0`; `npm install --package-lock-only --ignore-scripts`. Appetite: pequeno — 3 linhas de import, 3 arquivos de manifesto/lock, 5 artefatos.
- **Trade-offs**: elimina a dependência deprecada (`package-lock.json:3776`) sem renomear símbolos: o root do pacote novo exporta os 14 nomes (`dist/icons/index.js` + `dist/aliases/aliases.js:751` para `CheckSquare`) e os 13 `iconNode`s únicos são idênticos nos pacotes instalados. O pin `-E` congela `1.33.0`, evitando que `^1.33.0` traga o latest `1.76.0` (registry 2026-11-19) sem avaliação. Peer `svelte ^5` é compatível com `svelte ^5.56.1` (`package.json:40`). `[NV]` renderização visual e comandos pós-edição não executados nesta avaliação.
- **Rabbit holes**: esquecer o import multilinha de `+layout.svelte:8-19` (fecha em `:19`); trocar só 2 dos 3 imports (dual package residual); usar `bun add` sem `-E` e reintroduzir `^`; rodar `npm install` sem `--package-lock-only` e mutar `node_modules/`; aceitar churn amplo no `package-lock.json` sem revisar; renomear símbolos para o padrão `XIcon` dos componentes UI (diff maior, sem benefício); editar os 4 componentes UI já migrados; subir para `1.76.0` neste ciclo.

## Opção B — alinhar os 3 imports ao subpath `/icons/*`

- **Sketch**: converter cada símbolo em `import Send from '@lucide/svelte/icons/send';` etc., seguindo o padrão dos 4 componentes UI (`dialog-content.svelte:3`).
- **Trade-offs**: consistência de estilo com o código já migrado, mas 14 linhas de import em vez de 3 e necessidade de mapear aliases (`CheckSquare` → `check-square`, `Hash` → `hash`, `PanelRightClose` → `panel-right-close`). Diff maior e risco de typo sem ganho funcional. Rejeitada: Opção A resolve a duplicação com menor superfície; a convenção mista (root + subpath) já existe no checkout e não é regressão.

## Opção C — manter ambos os pacotes

- **Sketch**: não migrar; aceitar a coexistência de `lucide-svelte` e `@lucide/svelte`.
- **Trade-offs**: zero diff e zero risco imediato, mas preserva dependência deprecada e duplicação de ícones, exatamente o candidato ranqueado em 2,50 (`STATE.md:644`). Rejeitada: o custo da correção é baixo e a evidência de compatibilidade é integral.

## Opção D — atualizar direto para o latest do registry (`1.76.0`)

- **Sketch**: `bun add -D -E @lucide/svelte@1.76.0` em vez de `1.33.0`.
- **Trade-offs**: alinharia ao latest informado pelo registry em 2026-11-19T23:47Z (`npm view @lucide/svelte version`), mas foge ao contrato aprovado por Main, amplia o diff dos lockfiles e exigiria recomparar `iconNode`s de uma versão não instalada — risco visual `[NV]` maior. Rejeitada neste ciclo; upgrade futuro registrado como `[NV]` em `research.md`/`decision.md`.

## Fora de escopo

4 componentes UI já migrados, `specs/084-dedupe-lucide/spec.md` (template não preenchido), `apps/frontend/prettier.config.js`, gate e `scripts/ci/check_gate_integrity.py`, `.github/workflows/`, supply-chain além dos 3 arquivos de manifesto, ADRs, constituição, STATE, specs/083-*, DB/rede, segredos, `node_modules/`, `.tokens/`, fechamento de issues, leg RBAC, PRs e worktree espelho.

## Premissas

Os 14 símbolos existem como named exports no root de `@lucide/svelte@1.33.0` (`dist/icons/index.js`, `dist/aliases/aliases.js:751`); os 13 `iconNode`s únicos comparados são idênticos entre as versões instaladas, logo a migração não altera paths localmente; `svelte ^5.56.1` satisfaz o peer `^5` do pacote novo; o pin `1.33.0` foi confirmado por Main apesar do latest `1.76.0` no registry; FR-001..FR-005 e SC-001..SC-004 de `define.md` regem a implementação. `[NV]` renderização em browser, execução dos comandos pós-edição e upgrade futuro — não verificados nesta avaliação.
