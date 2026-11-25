# Problem Definition: Advisories de SvelteKit bloqueando o merge

- **Slug**: 093-sveltekit-security-upgrade
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md

## Problem Statement

O `dependency-review` da PR #318 (`dev` → `main`) bloqueava o merge por `GHSA-3r8g` (high) e três advisories moderate em `@sveltejs/kit 2.70.3` (`decision.md:12-13`; `research.md:49`). Os quatro achados foram confirmados no banco de advisories do GitHub: `GHSA-3r8g-jcqq-3p5j` (high, corrigido em `3.1.1`), `GHSA-mw6w-2fmv-pvw3`, `GHSA-rqp5-r3wg-vg35` e `GHSA-6cf5-gc4q-fm3x` (moderate) — dois deles também com patch `2.70.4` na linha 2.x, que não cobre o conjunto completo (`research.md:60-62`). A remediação registrada exige o salto para `@sveltejs/kit 3.1.1`, com `@sveltejs/adapter-node 6.0.5` e `svelte 5.57.1`; peers já atendidos pelo manifesto (`decision.md:14-16`). O SvelteKit 3 introduz duas quebras — `svelte.config.js` deixa de ser lido e `$lib` deixa de ser gerado automaticamente — que exigem migração explícita, sob pena de um upgrade que não compila (`decision.md:18-23`). O risco a evitar é alargar o diff para API, runtime, schema, migration, workflows, required checks, PR/merge/deploy e artefatos de governança (`decision.md:27-28`). Estado remoto da PR e contagem viva de advisories são **[NV]** nos artefatos do ciclo (`research.md:64`).

## Affected Users & Stakeholders

- **Users**: maintainer/SE, que precisa desbloquear o merge de `dev` para `main` sem flexibilizar o gate (`research.md:49`). [ASSUMPTION] usuários da aplicação frontend exposta aos advisories — não entrevistados diretamente nos artefatos.
- **Stakeholders**: dono do required check `dependency-review` sob ADR-0003; maintainers; responsável pelo runtime de deploy (`adapter-node`); responsável pela triagem de advisories.

## Goals

- Remover os advisories que bloqueiam o merge sem relaxar o `dependency-review` (`intake.md:12-14`).
- Alcançar versões corrigidas compatíveis com os peers já declarados, cobrindo o conjunto completo dos quatro achados (`decision.md:14-16`; `research.md:62`).
- Manter o build compilável após a mudança de major, tratando as quebras de configuração registradas (`decision.md:18-23`).
- Preservar a configuração e os aliases necessários à aplicação e manter `bun.lock` como lockfile canônico, com `package-lock.json` sincronizado (`decision.md:41-43`).
- Manter o diff no frontend e nos artefatos documentais do ciclo.

## Non-Goals

- Alterar API de aplicação, runtime, schema, migration, contrato de produção, workflows ou required checks.
- Realizar PR, merge, tag ou deploy neste ciclo; promover advisories a gate ou reescrever registro histórico (`decision.md` 093, "Fora de âmbito").
- Afirmar que a CI remota passou ou que a contagem de advisories é zero sem **[NV]** (`research.md:64`).
- Tocar `validate/`, `release/` ou `.specify/memory/STATE.md`.

## Success Metrics

- Nenhuma ocorrência residual das versões afetadas (`2.70.3`, `5.5.7`, `5.56.10`) no manifesto e nos locks (baseline: quatro advisories em `@sveltejs/kit 2.70.3`; `grep` com zero ocorrências registrado — `research.md:64`).
- `check`, `lint`, testes unitários do servidor e `build` sem erros (baseline: registrado `0/0`, OK, `42/42`, OK — não reexecutado nesta avaliação — `research.md:63`).
- `dependency-review` remoto limpo na PR promovida (baseline: registro de `fail` no run `44806804503`; resultado atual **[NV]** — `research.md:50,64`).
- Alias `$lib` e configuração preservados após a mudança de major (baseline: duas quebras registradas em `decision.md:18-23`).

## Cost of Inaction

O merge de `dev` para `main` continua bloqueado pelo `dependency-review`, e os advisories — incluindo um de severidade high — seguem presentes na dependência do frontend. Uma migração que ignore as quebras do SvelteKit 3 troca um bloqueio por um build que não compila; versões residuais em outros locks podem manter o achado vivo após a correção parcial.

## Open Questions

- [NEEDS CLARIFICATION: qual é a contagem viva de advisories no painel do GitHub após a correção, dado que só o inventário de entrada foi registrado (**[NV]**)?] (`intake.md:31`; `research.md:64`)
- [NEEDS CLARIFICATION: o `dependency-review` da PR #318 voltou a passar na CI remota, considerando que gate local verde não a substitui (**[NV]**)?] (`intake.md:32`)
- [NEEDS CLARIFICATION: existem consumidores de `svelte.config.js` ou do alias `$lib` fora de `apps/frontend` que precisem ser considerados?] (`intake.md:33`)
- [NEEDS CLARIFICATION: qual é a expectativa de compatibilidade com o runtime de deploy (`adapter-node`) após a mudança de major?] (`intake.md:34`)
- [NEEDS CLARIFICATION: há versões residuais das versões afetadas em outros locks do repositório além dos dois do frontend?] (`intake.md:35`)
- [NEEDS CLARIFICATION: qual é a janela de reconsulta do estado remoto antes de qualquer promoção da PR #318?] (`intake.md:36`)
- [NEEDS CLARIFICATION: como reconciliar o registro de T001..T009 em `tasks.md`/`decision.md` com o `tasks=6/6` de `analyze.md`/`converge.md`?] (`research.md:67`)
