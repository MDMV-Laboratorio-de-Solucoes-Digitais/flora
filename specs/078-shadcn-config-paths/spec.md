# Feature Specification: caminhos de configuração do `components.json` (shadcn-svelte) — 078

**Feature Branch**: `feature/078-shadcn-config-paths`
**Created**: 2026-11-19T20:25Z
**Status**: Approved (`decision.md` — go scoped, opção A)
**Input**: "corrigir os caminhos de configuração em `apps/frontend/components.json`: remover a chave deprecada `tailwind.config` e apontar `tailwind.css` para o CSS real, preservando todo o restante do arquivo"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — `components.json` sem chave deprecada (Priority: P1)

Como maintainer ou agente que lê, valida ou pretende usar `apps/frontend/components.json` para gerar componentes, quero que o objeto `tailwind` não contenha `config`, para que o arquivo não descreva um `tailwind.config.ts` que não existe e que o schema oficial marca como "DEPRECATED IN TAILWIND v4".

**Why this priority**: é o cerne do ciclo. `components.json:5` aponta para `tailwind.config.ts` (inexistente — `ls apps/frontend/tailwind.config.*` retorna vazio), enquanto o toolchain local é Tailwind v4 CSS-first (`@tailwindcss/vite` em `vite.config.ts:3,9`, `@theme` em `layout.css:3`). A chave é inerte hoje (zero consumidores locais), mas segue como contrato enganoso para qualquer geração futura via CLI.

**Independent Test**: carregar `apps/frontend/components.json` com `json.load` (stdlib) e confirmar que `tailwind` não contém a chave `config`, enquanto `baseColor`, `cssVariables` e todo o bloco `aliases` permanecem com os valores exatos originais.

**Acceptance Scenarios**:

1. **Given** `"config": "tailwind.config.ts"` em `apps/frontend/components.json:5`, **When** a chave é removida, **Then** `json.load` reporta `tailwind == {"css": "src/routes/layout.css", "baseColor": "zinc", "cssVariables": true}` e nenhuma chave `config`.
2. **Given** um agente que valida o arquivo contra `https://shadcn-svelte.com/schema.json` (acessado 2026-11-19), **When** ele inspeciona `tailwind`, **Then** não encontra mais o membro deprecado `config` e o objeto segue dentro de `additionalProperties=false` com `required=["css","baseColor"]`.

---

### User Story 2 — `css` aponta para o CSS que importa Tailwind (Priority: P1)

Como maintainer ou agente que usa `components.json` como referência de estilos, quero que `tailwind.css` aponte para `src/routes/layout.css`, para que o caminho declarado coincida com o arquivo efetivamente carregado e com a configuração já existente do Prettier.

**Why this priority**: `components.json:6` diz `src/app.css` (inexistente), enquanto o CSS real é `src/routes/layout.css:1` (`@import 'tailwindcss'`), importado por `+layout.svelte:2` e já declarado em `prettier.config.js:11` (`tailwindStylesheet: './src/routes/layout.css'`). Corrigir o valor elimina a segunda metade do contrato enganoso sem tocar em runtime.

**Independent Test**: verificar que o valor de `tailwind.css` em `json.load` é `src/routes/layout.css`, que esse arquivo existe e contém `@import 'tailwindcss'`, e que a linha 11 de `apps/frontend/prettier.config.js` aponta para o mesmo caminho.

**Acceptance Scenarios**:

1. **Given** `"css": "src/app.css"` (arquivo inexistente), **When** o valor é trocado para `"src/routes/layout.css"`, **Then** o caminho existe, contém `@import 'tailwindcss'` e casa com `prettier.config.js:11`.
2. **Given** um build do frontend já em execução, **When** o valor de `css` é corrigido, **Then** nada muda em runtime: nenhum código lê esse campo (varredura determinística com zero consumidores) e `+layout.svelte`, `layout.css` e `vite.config.ts` seguem intocados.

---

### User Story 3 — Diff estreito, gate intacto e reversível (Priority: P2)

Como maintainer, quero que a mudança fique restrita a duas linhas de `components.json` e aos artefatos do ciclo 078, com os gates do alvo em PASS e nenhuma mutação de DB, rede ou segredos, para que o ciclo seja auditável e reversível por uma única reversão.

**Why this priority**: FR-003/FR-004 e SC-003/SC-004 são a cerca do ciclo. Um erro de contrato em dois campos não justifica tocar estilos, aliases, toolchain, supply-chain, gate/scripts/workflows nem código de produção; a formatação local do Prettier e a integridade do gate precisam permanecer intactas.

**Independent Test**: `git diff --name-only de141fc53..HEAD` contém somente `apps/frontend/components.json` e os artefatos do ciclo 078; `bunx prettier --check components.json` está limpo; `bun run check`, `bun run lint`, `bun run test:unit` (em `apps/frontend`) e `python3 scripts/ci/check_gate_integrity.py` (na raiz) permanecem PASS.

**Acceptance Scenarios**:

1. **Given** as edições aplicadas, **When** o diff é auditado, **Then** há exatamente 1 linha removida e 1 valor trocado em `components.json`, com `$schema`, `style`, `baseColor`, `cssVariables` e `aliases` byte-idênticos, e nenhuma mudança em `prettier.config.js`, `package.json`, `vite.config.ts`, `layout.css`, `+layout.svelte`, gate/scripts/workflows, ADRs, constituição, STATE, DB/rede ou `.tokens/`.
2. **Given** os gates determinísticos do alvo, **When** executados sem DB e sem rede, **Then** permanecem PASS, sem dependências, credenciais ou segredos novos, e a reversão do commit devolve o arquivo original.

---

### Edge Cases

- **`style` também está deprecado e `cssVariables` não está declarado no schema**: `components.json:3,8` permanecem intactos por escolha de escopo mínimo; corrigi-los exigiria decisão de design alheia a este ciclo — **[NV]**.
- **Validação pelo CLI `shadcn-svelte`**: o CLI não está instalado (`package.json` não contém `shadcn`) e `bunx shadcn-svelte@latest` exigiria rede, fora da fronteira; a validação é feita por schema/docs oficiais e checagens locais determinísticas — **[NV]**.
- **Opção B (manter `config` vazio) / opção C (criar `tailwind.config.ts`)**: rejeitadas em `concept.md` — a primeira mantém metadado enganoso, a segunda contradiz Tailwind v4 e expande o diff para fora de `components.json`.
- **`prettier --check` sobre o JSON**: `singleQuote: true` vale para JS/TS, não para JSON; aspas duplas são obrigatórias e tabs/sem trailing commas devem ser preservados (`prettier.config.js:5-7`).
- **Extensões não varridas na busca de consumidores**: a varredura cobre `*.ts/*.js/*.json/*.yml/*.yaml/*.sh/*.py` (excluindo `node_modules` e worktrees); consumidores em outras extensões são **[NV]**.
- **Status de CI ao vivo, merge e produção**: não verificados nesta spec.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: remover `"config": "tailwind.config.ts"` de `tailwind` em `apps/frontend/components.json:5`; a chave é "DEPRECATED IN TAILWIND v4" no schema oficial (acessado 2026-11-19).
- **FR-002**: corrigir `"css": "src/app.css"` (linha 6) para `"css": "src/routes/layout.css"`, caminho que efetivamente importa Tailwind (`layout.css:1`, `+layout.svelte:2`) e já usado por `prettier.config.js:11`.
- **FR-003**: preservar byte-idênticos `$schema`, `style`, `baseColor: "zinc"`, `cssVariables: true` e todo o bloco `aliases` (`components.json:1-4,7-16`); manter a formatação do Prettier local: tabs e sem trailing commas (`prettier.config.js:6-8`); aspas duplas são obrigatórias em JSON (`singleQuote` vale para JS/TS).
- **FR-004**: não introduzir dependências, segredos, credenciais, `.tokens/`, mudanças de gate/scripts/workflows nem mutação de DB/rede; nenhum outro arquivo além de `components.json` e dos 5 artefatos 078.

### Key Entities

N/A — a mudança é de um arquivo de configuração de CLI (JSON); nenhuma entidade de dados, schema de banco ou migração é introduzida.

## Success Criteria *(mandatory)*

- **SC-001**: `components.json` não contém mais a chave `tailwind.config`.
- **SC-002**: `tailwind.css` aponta para arquivo existente (`src/routes/layout.css`) que contém `@import 'tailwindcss'` e casa com `prettier.config.js:11`.
- **SC-003**: `python3 -c` com `json.load` valida o JSON; `bunx prettier --check components.json` limpo; `bun run check`, `bun run lint`, `bun run test:unit` e `scripts/ci/check_gate_integrity.py` sem regressão (sem DB/rede).
- **SC-004**: diff restrito a `apps/frontend/components.json` (1 linha removida + 1 valor trocado) e aos 5 artefatos 078; gate do alvo PASS; zero mutação de DB/rede/segredos.

## Scope / Out of scope

**Dentro**: `apps/frontend/components.json` (exatamente as linhas 5-6) e os artefatos do ciclo 078 (`spec.md`, `clarify.md`, `plan.md`, `checklists/requirements.md`, `checklist.md`, `tasks.md`), além dos 5 artefatos de assessment já registrados em `.specify/assessments/078-shadcn-config-paths/` (`intake.md`, `research.md`, `define.md`, `concept.md`, `decision.md`).

**Fora**: `style`, `baseColor`, `cssVariables`, `$schema` e `aliases` de `components.json`; `prettier.config.js`, `package.json`, `vite.config.ts`, `layout.css`, `+layout.svelte`; gate/scripts/workflows (inclui `scripts/ci/check_gate_integrity.py`), supply-chain; ADRs, constituição, `.specify/memory/STATE.md`; `specs/078-*` além dos artefatos aqui listados; DB/rede, `.tokens/`, segredos; fechamento de issues, leg RBAC e instalação do CLI `shadcn-svelte`.

## Assumptions

- O schema oficial `https://shadcn-svelte.com/schema.json` (acessado 2026-11-19) marca `tailwind.config` como "DEPRECATED IN TAILWIND v4", declara `tailwind.required = ["css","baseColor"]` e `additionalProperties=false`.
- O CSS real é `apps/frontend/src/routes/layout.css:1` (`@import 'tailwindcss'`), carregado por `+layout.svelte:2` e referenciado em `prettier.config.js:11`; `src/app.css` e `tailwind.config.ts` não existem.
- A varredura determinística de consumidores retornou zero correspondências; `shadcn-svelte` não está instalado.
- O ambiente roda sem DB, sem rede e sem segredos; gates determinísticos não dependem de serviços externos.
- A base de comparação do diff é `de141fc53` (branch `feature/078-shadcn-config-paths`).

## [NV]

- Validação pelo CLI externo `shadcn-svelte` (não instalado; exigiria rede) — não executada; cobertura por schema/docs oficiais e checagens locais determinísticas.
- `style` deprecado em v4 e `cssVariables` não declarado no schema: preservados por escopo mínimo, sem inferência sobre intenção futura.
- Extensões de arquivo não cobertas pela varredura determinística (`*.ts/*.js/*.json/*.yml/*.yaml/*.sh/*.py`); nenhum consumidor identificado nas extensões varridas.
- Status de CI ao vivo, merge, PRs, Forgejo e produção não são verificados nesta spec.
