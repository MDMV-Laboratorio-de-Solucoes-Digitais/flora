# Implementation Plan: 093 — Upgrade de segurança SvelteKit 3

**Branch**: `feature/093-sveltekit-security-upgrade` | **Date**: 2026-11-20 |
**Spec**: [spec.md](./spec.md)

**Input**: `.specify/assessments/093-sveltekit-security-upgrade/decision.md` (go)

## Summary

Subir `@sveltejs/kit` para 3.1.1, `@sveltejs/adapter-node` para 6.0.5 e `svelte`
para 5.57.1, regenerar os locks e migrar a configuração exigida pelo SvelteKit 3
(`svelte.config.js` → plugin Vite; `$lib` explícito no alias e no tsconfig), com
as verificações locais verdes.

## Technical Context

**Primary Dependencies**: `@sveltejs/kit 3.1.1`, `@sveltejs/adapter-node 6.0.5`,
`svelte 5.57.1`; peers já compatíveis: `vite ^8.0.16`, `typescript ^6.0.3`,
`@sveltejs/vite-plugin-svelte ^7.1.2`.

**Target Path**: `apps/frontend/package.json`, `bun.lock`, `package-lock.json`,
`vite.config.ts`, `tsconfig.json` e remoção de `apps/frontend/svelte.config.js`.

**Testing**: `npm run check` (0 erros/0 warnings), `npm run lint`, testes
unitários do servidor 42/42 e `npm run build`.

**Constraints**: `bun.lock` canónico da CI; sem tocar em API, runtime, schema,
migration, workflows, `validate/`, `release/` ou `STATE`; CI remota **[NV]**.

## Steps

1. **Evidência**: baseline `b426e4f81`, advisories e versões alvo (T001–T002).
2. **Dependências e locks**: manifesto e ambos os locks, zero `2.70.3` (T003–T005).
3. **Fronteira e rastro**: estender o registro documental do ciclo a T001..T009
   (T006).
4. **SvelteKit 3**: remover `svelte.config.js`, configuração no plugin Vite e
   `$lib` via alias/`paths`/`$app/tsconfig` (T007–T008).
5. **Verificação**: check/lint/unit server/build e fecho do ciclo (T009).

## Constitution Check

Sem violações: nenhuma mudança de required check, workflow, schema ou contrato de
produção; diffs restritos ao frontend e aos artefatos documentais 093.

## Project Structure

```text
.specify/assessments/093-sveltekit-security-upgrade/decision.md
specs/093-sveltekit-security-upgrade/
├── spec.md
├── plan.md
└── tasks.md
apps/frontend/
├── package.json          # versões corrigidas
├── bun.lock              # canónico CI, sincronizado
├── package-lock.json     # sincronizado
├── svelte.config.js      # removido (SvelteKit 3)
├── vite.config.ts        # plugin + preprocess + adapter + alias $lib
└── tsconfig.json         # extends $app/tsconfig + paths $lib
```

## Complexity Tracking

Nenhuma. A migração de configuração é substituição direta exigida pelo SvelteKit 3.
