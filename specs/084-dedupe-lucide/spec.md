# Feature Specification: deduplicar dependências lucide — 084

**Feature Branch**: `feature/084-dedupe-lucide`
**Created**: 2026-11-19T23:55Z
**Status**: Approved (`decision.md` — go (scoped), opção A, E=2,50)
**Input**: "deduplicar as dependências lucide do frontend: trocar os 3 imports legados de `lucide-svelte` para `@lucide/svelte`, remover o pacote deprecado e fixar `@lucide/svelte@1.33.0`" — decidido em `.specify/assessments/084-dedupe-lucide/decision.md`; tratado como migração mecânica, sem mudança de comportamento, API ou contrato de runtime.

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Imports legados migrados sem perder símbolos (Priority: P1)

Como maintainer ou agente que edita o frontend, quero que os três arquivos que ainda importam de `lucide-svelte` passem a importar de `@lucide/svelte`, preservando os mesmos 14 símbolos e a sintaxe de named import, para que a duplicação de pacotes desapareça sem nenhuma renomeação ou ajuste de JSX.

**Why this priority**: é o cerne do ciclo. `apps/frontend/src/lib/components/chat/ChatLayout.svelte:10` (`Send`, `Sparkles`), `apps/frontend/src/lib/components/search/SearchBox.svelte:2` (`Search`) e `apps/frontend/src/routes/(app)/+layout.svelte:8-19` (11 símbolos, com `Sparkles` repetido em outro arquivo) somam 14 ocorrências / 13 únicos vindos do pacote deprecado (`package-lock.json:3776`). O root de `@lucide/svelte@1.33.0` já exporta todos os nomes (`dist/icons/index.js` e `dist/aliases/aliases.js:751` para `CheckSquare`), então a troca é apenas a string da origem; os 13 `iconNode`s únicos são idênticos nos pacotes instalados.

**Independent Test**: `grep -rn "lucide-svelte" apps/frontend/src` retorna zero ocorrências; `grep -rn "from '@lucide/svelte'" apps/frontend/src` retorna exatamente 3 linhas (uma em cada arquivo de import) com os 14 símbolos intactos; os 4 componentes UI com subpath `/icons/*` permanecem byte-idênticos ao base `9da1aaea6`.

**Acceptance Scenarios**:

1. **Given** os 3 imports legados no base `9da1aaea6` (`ChatLayout.svelte:10`, `SearchBox.svelte:2`, `+layout.svelte:8-19`, fechando em `:19` com `} from 'lucide-svelte';`), **When** a string `'lucide-svelte'` é substituída por `'@lucide/svelte'` em cada um, **Then** os 14 símbolos e a sintaxe de named import permanecem idênticos, nenhum rename é necessário e `grep -rn "lucide-svelte" apps/frontend/src` retorna zero.
2. **Given** os 4 componentes UI já migrados para o subpath `/icons/*` (`dialog-content.svelte:3`, `dropdown-menu-checkbox-item.svelte:3-4`, `dropdown-menu-radio-item.svelte:3`, `dropdown-menu-sub-trigger.svelte:3`), **When** a troca dos 3 imports é aplicada, **Then** esses 4 arquivos seguem byte-idênticos — não são editados, renomeados nem alinhados ao padrão `XIcon`.

---

### User Story 2 — Dependência deprecada removida e versão fixada (Priority: P1)

Como maintainer que instala ou inspeciona o frontend, quero que `lucide-svelte` saia de `dependencies` e que `@lucide/svelte` fique exatamente em `1.33.0` em `devDependencies`, para que o projeto pare de depender de um pacote sem manutenção e não receba, sem avaliação, o latest `1.76.0` do registry.

**Why this priority**: `apps/frontend/package.json:53` declara `"lucide-svelte": "^1.0.1"` (deprecado em `package-lock.json:3776`) e `:22` declara `"@lucide/svelte": "^1.33.0"`. O caret permitiria resolver `1.76.0` (registry em 2026-11-19T23:47Z); Main confirmou manter o pin contratual `1.33.0`, que coincide com a versão já instalada (`bun.lock:95`, `package-lock.json:341-348`) e preserva os 13 `iconNode`s comparados. Sem essa etapa, a migração dos imports deixaria o pacote legado no manifesto.

**Independent Test**: `grep -n '"@lucide/svelte"' apps/frontend/package.json` retorna exatamente `"@lucide/svelte": "1.33.0"` (sem `^`); `grep -n 'lucide-svelte' apps/frontend/package.json apps/frontend/bun.lock` retorna zero para o pacote legado; `bun pm ls` e `npm ls @lucide/svelte lucide-svelte` mostram somente `@lucide/svelte@1.33.0` resolvido e nenhuma entrada `lucide-svelte`.

**Acceptance Scenarios**:

1. **Given** `lucide-svelte@^1.0.1` em `dependencies` (`package.json:53`, `bun.lock:10` e `:517`), **When** `bun remove lucide-svelte` é executado em `apps/frontend`, **Then** a entrada desaparece do manifesto e do `bun.lock`, e `bits-ui:51`, `clsx:52`, `tailwind-merge:54` permanecem inalterados.
2. **Given** `@lucide/svelte@^1.33.0` em `devDependencies` (`package.json:22`, `bun.lock:16`), **When** `bun add -D -E @lucide/svelte@1.33.0` é executado, **Then** o manifesto registra `"@lucide/svelte": "1.33.0"` sem caret, o pacote continua em `devDependencies`, `bun.lock:95` segue em `1.33.0` e o `"dev": true` de `package-lock.json:345` é preservado.

---

### User Story 3 — Lockfiles sincronizados e diff estreito (Priority: P2)

Como revisor de mudanças, quero que o `package-lock.json` seja sincronizado sem tocar `node_modules` e que o diff rastreado fique restrito aos 3 imports, aos 3 manifestos/locks e aos artefatos do ciclo 084, com gates determinísticos e rollback de um único path, para que a deduplicação seja auditável e reversível sem efeitos colaterais.

**Why this priority**: FR-004/SC-001 e SC-002 dependem da remoção da entrada `node_modules/lucide-svelte` (`package-lock.json:3772-3781`) e da manutenção de `node_modules/@lucide/svelte` em `1.33.0` (`:341-348`). FR-005/SC-003 e SC-004 são a cerca de compatibilidade e a fronteira da mudança: churn não relacionado no lockfile, mutação de `node_modules/`, alteração de gate/CI/script/workflow, abertura de PR ou converter `[NV]` em fato transformariam o ciclo em mudança de contrato que exige reavaliação.

**Independent Test**: `git diff --name-only` lista somente os 3 imports + `apps/frontend/package.json` + `apps/frontend/bun.lock` + `apps/frontend/package-lock.json` + os 5 artefatos de assessment 084 + os documentos do próprio ciclo; o diff de `package-lock.json` não contém `node_modules/lucide-svelte`; `bun run check`, `bun run lint`, `bun run test:unit -- --run`, `bun run build` e `python3 scripts/ci/check_gate_integrity.py` terminam sem regressão; zero PRs abertos.

**Acceptance Scenarios**:

1. **Given** a entrada deprecada em `package-lock.json:3772-3781`, **When** `npm install --package-lock-only --ignore-scripts` é executado em `apps/frontend`, **Then** `node_modules/lucide-svelte` desaparece, `node_modules/@lucide/svelte` permanece em `1.33.0` e o restante do arquivo é preservado; churn não relacionado é descartado na revisão do diff.
2. **Given** a edição completa, **When** os gates determinísticos são executados sem DB, rede ou segredos, **Then** `git diff --name-only` não lista nenhum path além dos permitidos, o gate autoritativo segue PASS e nenhum `[NV]` é promovido a afirmação verificada.

---

### Edge Cases

- **Import multilinha esquecido**: o bloco de `+layout.svelte` fecha em `:19` (`} from 'lucide-svelte';`); trocar apenas as linhas de símbolo e deixar o fechamento legado gera import quebrado. A busca `grep -rn "lucide-svelte" apps/frontend/src` cobre o fechamento.
- **Migração parcial (dual package)**: trocar só 2 dos 3 imports deixa ambos os pacotes no grafo do bundler. A sequência completa dos 3 imports + `bun remove lucide-svelte` elimina a duplicação; o grafo final é `[NV]`.
- **`bun add` sem `-E`**: reintroduz `^1.33.0` e permite `1.76.0` na próxima instalação. O comando contratual é `bun add -D -E @lucide/svelte@1.33.0`.
- **`npm install` sem `--package-lock-only`**: muta `node_modules/` (fora de escopo); usar sempre a flag e `--ignore-scripts`.
- **Churn amplo no `package-lock.json`**: a ferramenta pode reescrever flags `dev`, integrities ou ordem. Churn não relacionado a lucide é fora de escopo e deve ser descartado na revisão.
- **Renomear símbolos para `XIcon`**: os 4 componentes UI usam subpath `/icons/*`, mas os 14 nomes existem como named exports no root; renomear aumentaria o diff sem benefício (opção B rejeitada).
- **Versão do registry divergente**: latest `1.76.0` (2026-11-19T23:47Z) ≠ pin contratual `1.33.0`. Subir neste ciclo exigiria nova avaliação de escopo/visual; o upgrade fica registrado como `[NV]`.
- **`[NV]` nunca promovido**: execução dos comandos, regressão visual, grafo do bundler, churn real, CI/PR e upgrade futuro permanecem marcados até o handoff ao main agent.
- **Worktrees alheios, issues, leg RBAC e PRs**: fora de escopo; nenhum fechamento, movimentação ou abertura.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: trocar a origem dos 3 imports de `'lucide-svelte'` para `'@lucide/svelte'`, preservando os 14 símbolos e a sintaxe de named import: `apps/frontend/src/lib/components/chat/ChatLayout.svelte:10` (`Send`, `Sparkles`), `apps/frontend/src/lib/components/search/SearchBox.svelte:2` (`Search`) e `apps/frontend/src/routes/(app)/+layout.svelte:8-19` (11 símbolos, com `Sparkles` repetido). O root de `@lucide/svelte@1.33.0` exporta todos os nomes (`dist/icons/index.js`, `dist/aliases/aliases.js:751` para `CheckSquare`); nenhum rename é necessário.
- **FR-002**: remover `"lucide-svelte": "^1.0.1"` de `apps/frontend/package.json:53` (bloco `dependencies`) via `bun remove lucide-svelte`, com reflexo em `apps/frontend/bun.lock:10` e `:517`. Nenhuma outra dependência de `dependencies` é tocada (`bits-ui:51`, `clsx:52`, `tailwind-merge:54` permanecem).
- **FR-003**: fixar a entrada de devDependencies `apps/frontend/package.json:22` como `"@lucide/svelte": "1.33.0"` (sem `^`) via `bun add -D -E @lucide/svelte@1.33.0`, com reflexo em `apps/frontend/bun.lock:16` e `:95`. O pacote permanece em devDependencies; o `"dev": true` de `package-lock.json:345` é preservado.
- **FR-004**: sincronizar `apps/frontend/package-lock.json` via `npm install --package-lock-only --ignore-scripts`: remover a entrada `node_modules/lucide-svelte` (`:3772-3781`), manter `node_modules/@lucide/svelte` em `1.33.0` (`:341-348`) e preservar o restante do arquivo. Churn não relacionado é fora de escopo e deve ser descartado na revisão do diff.
- **FR-005**: limitar o diff rastreado a: 3 arquivos de import, `apps/frontend/package.json`, `apps/frontend/bun.lock`, `apps/frontend/package-lock.json` e os 5 artefatos `.specify/assessments/084-dedupe-lucide/`. Manter formatação local (tabs, aspas simples em Svelte/JS, nenhuma trailing comma — `prettier.config.js:5-7`). Nenhuma dependência nova, segredo, mutação de DB/rede, gate, CI, ADR, spec numerada ou PR.

### Key Entities

N/A — o ciclo é uma migração mecânica de dependência em arquivos de manifesto, lockfile e imports Svelte; nenhuma entidade de dados, schema, migração ou contrato de leitura é introduzido. As únicas "entidades" são as entradas de dependência (`dependencies`/`devDependencies`), as entradas dos lockfiles e os 3 imports migrados.

## Success Criteria *(mandatory)*

- **SC-001**: `grep -rn "lucide-svelte" apps/frontend/src apps/frontend/package.json apps/frontend/bun.lock apps/frontend/package-lock.json` retorna zero ocorrências do pacote legado; os 3 imports apontam para `'@lucide/svelte'` com os mesmos 14 símbolos; os 4 componentes UI com subpath `/icons/*` seguem byte-idênticos.
- **SC-002**: `apps/frontend/package.json:22` é exatamente `"@lucide/svelte": "1.33.0"`; `bun pm ls` / `npm ls @lucide/svelte lucide-svelte` mostram apenas `@lucide/svelte@1.33.0` resolvido e nenhuma entrada `lucide-svelte`. `[NV]` depende da edição.
- **SC-003**: em `apps/frontend`: `bun run check`, `bun run lint`, `bun run test:unit -- --run` e `bun run build` terminam sem regressão (`package.json:8,11,13,15-16`); `python3 scripts/ci/check_gate_integrity.py` mantém gate PASS. `[NV]` regressão visual permanece fora da cobertura desses comandos.
- **SC-004**: `git diff --name-only` lista somente os 3 imports + `package.json` + `bun.lock` + `package-lock.json` + 5 artefatos 084; zero mutação de gate/CI/scripts/workflows/ADRs/STATE/specs/DB/rede/segredos/`node_modules/`; zero PRs abertos.

## Scope / Out of scope

**Dentro**: os 3 imports (`apps/frontend/src/lib/components/chat/ChatLayout.svelte:10`, `apps/frontend/src/lib/components/search/SearchBox.svelte:2`, `apps/frontend/src/routes/(app)/+layout.svelte:8-19`); os 3 manifestos/locks (`apps/frontend/package.json`, `apps/frontend/bun.lock`, `apps/frontend/package-lock.json`); os 5 artefatos de assessment em `.specify/assessments/084-dedupe-lucide/` (`intake.md`, `research.md`, `define.md`, `concept.md`, `decision.md`); e os documentos do próprio ciclo em `specs/084-dedupe-lucide/` (`spec.md`, `clarify.md`, `plan.md`, `checklists/requirements.md`, `checklist.md`, `tasks.md`).

**Fora**: os 4 componentes UI já migrados, o template `specs/084-dedupe-lucide/spec.md` antes desta substituição, a subida para `1.76.0`, mudança de Svelte, `apps/frontend/prettier.config.js`, gate e `scripts/ci/check_gate_integrity.py`, `.github/workflows/`, ADRs, constituição, STATE, `specs/083-*`, supply-chain além dos 3 arquivos de manifesto, DB/rede, segredos, `node_modules/`, `.tokens/`, fechamento de issues, leg RBAC, PRs e worktree espelho. Os documentos do ciclo acompanham o diff como artefatos de especificação, não como código.

## Assumptions

- Base de comparação do diff: `9da1aaea6` (HEAD da branch `feature/084-dedupe-lucide` no início do ciclo), gate PASS, zero PRs.
- Os 14 nomes usados existem como named exports no root de `@lucide/svelte@1.33.0` e os 13 `iconNode`s únicos comparados são idênticos entre as versões instaladas; a migração não altera paths localmente.
- `svelte ^5.56.1` (`package.json:40`) satisfaz o peer `^5` do pacote novo; nenhuma mudança de Svelte é necessária.
- O pin `1.33.0` foi confirmado por Main apesar do latest `1.76.0` no registry (2026-11-19T23:47Z); upgrade futuro exige nova avaliação de escopo e risco visual.
- Os comandos `bun run ...` rodam em `apps/frontend` (não há `package.json` na raiz); o gate roda na raiz.
- `.specify/feature.json` é estado local ignorado, reescrito a cada troca de feature; hoje registra `specs/084-dedupe-lucide` (a divergência 083→084 está resolvida em `clarify.md`).

## [NV]

- Execução dos comandos determinísticos pós-edição (greps, `bun pm ls`, `npm ls`, `bun run check/lint/test:unit/build`, reexecução de `python3 scripts/ci/check_gate_integrity.py` e auditoria de `git diff`) — fica com o main agent; não executada nesta avaliação (marcadores em SC-002, SC-003 e SC-004).
- Regressão visual em browser: os 13 `iconNode`s comparados são idênticos, mas não houve captura de tela nem inspeção visual; gates não detectam aparência de ícones.
- Grafo final do bundler quanto a dual package residual; churn real dos lockfiles após os 3 comandos.
- CI, Forgejo/PRs (zero PRs no snapshot), merge, proteção de branch, produção e aprovações — nenhuma inferência além da evidência local e de `decision.md`.
- Upgrade futuro para `@lucide/svelte@1.76.0` (latest informado pelo registry em 2026-11-19T23:47Z) — não avaliado neste ciclo; exigiria nova avaliação de risco visual e de escopo.
