# Feature Specification: 093 — Upgrade de segurança SvelteKit 3

**Feature Branch**: `feature/093-sveltekit-security-upgrade`

**Created**: 2026-11-20

**Status**: Implemented

**Input**: Decisão `go` de `.specify/assessments/093-sveltekit-security-upgrade/decision.md`

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Cadeia de dependências sem advisories bloqueantes (Priority: P1)

Como mantenedor do frontend, preciso que `@sveltejs/kit`, `@sveltejs/adapter-node`
e `svelte` estejam nas versões corrigidas no manifesto e em ambos os locks, para
que o `dependency-review` da PR #318 (`dev` → `main`) deixe de bloquear o merge.

**Why this priority**: é o objetivo do ciclo; sem as versões corrigidas e os locks
sincronizados, `GHSA-3r8g` (high) e os 3 moderate permanecem como achados.

**Independent Test**: ler `apps/frontend/package.json` e os dois locks e procurar
`2.70.3`, `5.5.7` e `5.56.10`, verificando zero ocorrências.

**Acceptance Scenarios**:

1. **Given** o baseline `b426e4f81` com `@sveltejs/kit 2.70.3`, **When** o upgrade
   é aplicado, **Then** manifesto, `bun.lock` e `package-lock.json` registram
   `3.1.1` / `6.0.5` / `5.57.1` sem versões residuais.
2. **Given** `bun.lock` como lockfile canónico da CI, **When** os locks são
   regenerados, **Then** `package-lock.json` reflete o mesmo manifesto, sem troca
   de gerenciador.

---

### User Story 2 — Configuração compatível com SvelteKit 3 (Priority: P1)

Como desenvolvedor do frontend, preciso que a configuração do SvelteKit 3 viva no
plugin Vite e que `$lib` continue resolvendo, para que o upgrade não quebre a
compilação nem os caminhos de importação.

**Why this priority**: o SvelteKit 3 não lê `svelte.config.js` e não gera `$lib`;
sem a migração, `check` e `build` falhariam apesar das versões corretas.

**Independent Test**: confirmar ausência de `svelte.config.js`, presença de
`vitePreprocess()` e `adapter()` no plugin `sveltekit()` e resolução de `$lib` em
`vite.config.ts` e `tsconfig.json`.

**Acceptance Scenarios**:

1. **Given** `apps/frontend/svelte.config.js` no baseline, **When** a migração é
   aplicada, **Then** o ficheiro é removido e a configuração passa a estar em
   `vite.config.ts`, dentro do plugin `sveltekit()`.
2. **Given** `$lib` sem geração automática no SvelteKit 3, **When** o tsconfig e o
   Vite são configurados, **Then** `$lib` e `$lib/*` resolvem via `resolve.alias`
   e `paths`, com `extends: $app/tsconfig`.

---

### Edge Cases

- Se `svelte.config.js` permanecer, o SvelteKit 3 o ignora; a configuração deve
  estar exclusivamente no plugin Vite para evitar dupla fonte de verdade.
- Sem `resolve.alias` e `paths` explícitos, imports `$lib` quebram porque o
  SvelteKit 3 não gera mais o alias automaticamente.
- `bun.lock` continua canónico da CI; `package-lock.json` é sincronizado, nunca
  substituto do gerenciador.
- CI remota, `dependency-review` ao vivo e contagem pós-upgrade de advisories são
  **[NV]** neste snapshot: gates locais verdes não substituem a CI remota.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: O sistema DEVE fixar `@sveltejs/kit 3.1.1`,
  `@sveltejs/adapter-node 6.0.5` e `svelte 5.57.1` no manifesto e em ambos os
  locks, mantendo `vite ^8.0.16`, `typescript ^6.0.3` e
  `@sveltejs/vite-plugin-svelte ^7.1.2` dentro dos peers exigidos, sem nenhuma
  ocorrência residual de `2.70.3`, `5.5.7` ou `5.56.10`.
- **FR-002**: A migração DEVE remover `apps/frontend/svelte.config.js` e DEVE
  declarar `vitePreprocess()` e `adapter()` dentro de `sveltekit({ ... })` em
  `apps/frontend/vite.config.ts`, conforme exigido pelo SvelteKit 3.
- **FR-003**: O alias `$lib` DEVE ser preservado explicitamente via `resolve.alias`
  em `vite.config.ts` e `paths` em `tsconfig.json`, com `tsconfig.json` estendendo
  `$app/tsconfig`; `bun.lock` DEVE permanecer o lockfile canónico da CI, com
  `package-lock.json` sincronizado e nenhuma troca de gerenciador.
- **FR-004**: As verificações locais DEVEM fechar sem falhas: `npm run check` com
  0 erros e 0 warnings, `npm run lint` OK, testes unitários do servidor 42/42 e
  `npm run build` OK; o resultado remoto da CI/`dependency-review` permanece
  **[NV]**.
- **FR-005**: O diff DEVE ficar restrito a `apps/frontend/package.json`, aos dois
  locks, à remoção de `svelte.config.js`, a `vite.config.ts`, a `tsconfig.json` e
  aos artefatos documentais do ciclo 093, sem tocar em API, runtime, schema,
  migration, workflows, `validate/`, `release/` ou `STATE`.

### Key Entities

- **package.json**: manifesto do frontend; declara as versões corrigidas e os
  peers compatíveis.
- **bun.lock / package-lock.json**: lockfiles sincronizados; `bun.lock` é a fonte
  de verdade da CI.
- **vite.config.ts**: ponto único de configuração do SvelteKit 3 (plugin,
  preprocess, adapter e alias `$lib`).
- **tsconfig.json**: estende `$app/tsconfig` e declara explicitamente `paths`,
  `include` e `exclude`.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Manifesto e ambos os locks registram `kit 3.1.1`,
  `adapter-node 6.0.5` e `svelte 5.57.1`; a busca por `2.70.3`, `5.5.7` e
  `5.56.10` retorna zero ocorrências e não existe `svelte.config.js`.
- **SC-002**: `npm run check` fecha com 0 erros/0 warnings, `npm run lint` passa,
  os testes unitários do servidor fecham 42/42 e `npm run build` passa; `$lib`
  resolve em build/test e os locks permanecem sincronizados.
- **SC-003**: O diff respeita a fronteira do frontend e dos artefatos 093 —
  nenhum workflow, required check, API, runtime, schema, migration, `validate/`,
  `release/` ou `STATE` alterado; CI remota e `dependency-review` permanecem
  **[NV]** neste snapshot.

## Assumptions

- O inventário de advisories registrado é o de entrada (`GHSA-3r8g` high + 3
  moderate em `@sveltejs/kit 2.70.3`); a contagem viva pós-upgrade não foi lida.
- `bun.lock` continua sendo o lockfile canónico da CI; este ciclo não troca o
  gerenciador.
- As verificações locais são a evidência consolidada do handoff; nenhuma delas
  foi reexecutada ao escrever este artefato.
