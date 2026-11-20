# Release: 076 — hang do setup E2E do minter

**Ciclo**: 076 · **Data**: 2026-11-19T19:14:42Z · **Baseline**: `dev@f22a2a32c`
**Tipo**: configuração de teste E2E. **Nenhuma tag, release, deploy, merge ou produção.**

## Entregue

- `apps/frontend/playwright.config.ts:10` — `globalSetup: './playwright/global-setup.ts',`.
- `apps/frontend/playwright/global-setup.ts` — pré-build de `flora-mint-session --features dev-mint` com `cwd` na raiz do repo e `stdio: 'inherit'`, fora do cronômetro de 180s do teste.
- `seed.setup.ts`, workflows, Cargo.toml, código de produção e DB compartilhado intactos.
- Artefatos completos: assess (5), spec, clarify, plan, checklist/tasks, analyze, converge (com apêndice de lint), validate R4, release R7.

## SemVer

**Proposta:** patch `0.2.6` → `0.2.7` (confiabilidade do setup E2E, sem mudança de comportamento do produto). Nenhuma tag criada.

## Changelog

- `FR-001` pré-build no globalSetup; `FR-002` setup consome binário quente; `FR-003` timeout 180s preservado; `FR-004` build fora da janela; `FR-005` fronteira de paths.
- `SC-001`..`SC-004` verificados por comandos determinísticos e diff.
- `T001`..`T005` executados; nenhum bug de `.specify/bugs/` acionado.

## Evidência

`.specify/validations/076-mint-session-hang/validate.md` · `specs/076-mint-session-hang/converge.md` · `analyze.md`.

## Rollback

Reverter os commits dos dois paths de teste em `dev/` com novo commit de reversão, sem force-push ou reescrita. Nada de código de produção, schema, required check ou produção é alterado.

## [NV]

Target frio real, CI ao vivo, Forgejo, aprovações e merge: **[NV]**.
