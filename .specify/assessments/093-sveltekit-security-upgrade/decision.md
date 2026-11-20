# Decision: Upgrade de segurança SvelteKit 3 (ciclo 093)

- **Slug**: 093-sveltekit-security-upgrade
- **Decidido**: 2026-11-20
- **Verdict**: **go**
- **Branch**: `feature/093-sveltekit-security-upgrade` @ `b426e4f81`
- **Âncora**: `.specify/feature.json` → `specs/093-sveltekit-security-upgrade`
- **Artefatos revisados**: spec | plan | tasks

## Contexto / Problema

O `dependency-review` da PR #318 (`dev` → `main`) bloqueava o merge por
`GHSA-3r8g` (high) e 3 advisories moderate em `@sveltejs/kit 2.70.3`. A remediação
exige o salto para `@sveltejs/kit 3.1.1`, com `@sveltejs/adapter-node 6.0.5`
(peer `@sveltejs/kit ^3`) e `svelte 5.57.1`. `vite ^8.0.16`, `typescript ^6.0.3`
e `@sveltejs/vite-plugin-svelte ^7.1.2` já atendem aos peers do kit corrigido.

O SvelteKit 3 introduz duas quebras de configuração que precisam de migração
explícita: `svelte.config.js` deixa de ser lido (a configuração passa para o
plugin `sveltekit()` em `vite.config.ts`) e `$lib` deixa de ser gerado
automaticamente (alias declarado em `resolve.alias` e em `paths` do tsconfig,
que agora estende `$app/tsconfig`). Ignorar essas quebras produziria um upgrade
de dependências que não compila.

O risco a evitar é alargar o diff para API de aplicação, runtime, schema,
migration, workflows, required checks, PR/merge/deploy e para os artefatos de
governança (`validate/`, `release/`, `.specify/memory/STATE.md`).

## Decisão

Registrar o ciclo 093 como **upgrade de segurança restrito ao frontend**, com
cinco compromissos rastreáveis (FR-001..FR-005) e nove tarefas (T001..T009,
incluindo a migração de configuração do SvelteKit 3):

1. **Versões corrigidas (FR-001)**: `@sveltejs/kit 3.1.1`,
   `@sveltejs/adapter-node 6.0.5` e `svelte 5.57.1` no manifesto e em ambos os
   locks, sem nenhuma ocorrência residual de `2.70.3`, `5.5.7` ou `5.56.10`.
2. **Migração de configuração (FR-002)**: remover `apps/frontend/svelte.config.js`
   e declarar `vitePreprocess()` e `adapter()` dentro do plugin `sveltekit()` em
   `vite.config.ts`.
3. **Alias e locks (FR-003)**: preservar `$lib` via `resolve.alias` e `paths` do
   tsconfig (que estende `$app/tsconfig`); manter `bun.lock` como lockfile canónico
   da CI, com `package-lock.json` sincronizado e nenhuma troca de gerenciador.
4. **Verificação (FR-004)**: `npm run check` sem erros nem warnings, `npm run lint`
   OK, testes unitários do servidor 42/42 e `npm run build` OK.
5. **Fronteira (FR-005)**: diff limitado ao frontend e aos artefatos documentais
   do ciclo 093; nenhum toque em API, runtime, schema, migration, workflows,
   `validate/`, `release/` ou `STATE`.

## Evidência de entrada

- Inventário de entrada: `GHSA-3r8g` (high) + 3 moderate em `@sveltejs/kit 2.70.3`.
- Baseline `b426e4f81`: manifesto com `kit ^2.63.0`, `adapter-node ^5.5.4` e
  `svelte ^5.56.1`.
- Diff contra `b426e4f81`: `apps/frontend/package.json` (versões corrigidas),
  `bun.lock` e `package-lock.json` (mesmo conjunto de versões, zero ocorrências de
  `2.70.3`), exclusão de `apps/frontend/svelte.config.js`, `vite.config.ts`
  (preprocess/adapter no plugin e alias `$lib`) e `tsconfig.json` (`extends`:
  `$app/tsconfig`, `paths`, `include`/`exclude`).
- Gates locais verdes: `check` 0 erros/0 warnings, `lint` OK, `unit server` 42/42,
  `build` OK.

## Racional

**go.** O problema é medido pelo `dependency-review`, a correção é uma atualização
de versões com peers já compatíveis e a migração de configuração é a substituição
direta exigida pelo SvelteKit 3. As verificações locais estão verdes e o diff é
reversível por restauro via controlo de versões. O valor está em desbloquear o
merge sem alterar processo, infraestrutura ou fronteira de governança.

## [NV]

- **CI do PR/merge/produção**: nenhum PR, merge, tag ou deploy realizado/verificado
  neste snapshot; a execução ao vivo do GitHub Actions e o resultado remoto do
  `dependency-review` na PR #318 permanecem **[NV]**. Gate local verde não
  substitui a CI remota para promoção.
- **Contagem viva de advisories**: registrado o inventário de entrada; a contagem
  pós-upgrade no painel do GitHub não foi lida nesta passagem e permanece **[NV]**.
- **Reexecução**: as verificações relatadas são a evidência consolidada do handoff;
  este artefato não reexecutou gates, build, testes ou formatters.

## Fora de âmbito

Mudanças de API de aplicação, runtime, schema, migration, contrato de produção,
workflow, required checks, PR/merge/deploy, credenciais e produção, bem como
qualquer alteração em `validate/`, `release/` ou `.specify/memory/STATE.md`.

## Handoff

`/speckit.specify` — especificar o upgrade como requisitos verificáveis
(FR-001..FR-005 / SC-001..SC-003) e tarefas T001..T009, com T007 (remoção de
`svelte.config.js` e configuração no plugin Vite), T008 (alias `$lib` e tsconfig)
e T009 (verificação check/lint/unit server/build) concluídas.
