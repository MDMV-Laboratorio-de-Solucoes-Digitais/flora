# Idea Intake: Upgrade de segurança SvelteKit 3

- **Slug**: 093-sveltekit-security-upgrade
- **Created**: 2026-11-20
- **Source**: repositório local — `.specify/assessments/093-sveltekit-security-upgrade/decision.md` (+ `specs/093-sveltekit-security-upgrade/spec.md`)
- **Type**: fix

## Idea (as captured)

O `dependency-review` da PR #318 (`dev` → `main`) bloqueava o merge por `GHSA-3r8g` (high) e 3 advisories moderate em `@sveltejs/kit 2.70.3`; a remediação registrada exige o salto para `@sveltejs/kit 3.1.1`, com `@sveltejs/adapter-node 6.0.5` e `svelte 5.57.1`, enquanto `vite`, `typescript` e `@sveltejs/vite-plugin-svelte` já atendem aos peers do kit corrigido (`decision.md:12-16`). O artefato registra que o SvelteKit 3 introduz duas quebras de configuração — `svelte.config.js` deixa de ser lido e `$lib` deixa de ser gerado automaticamente — que exigem migração explícita para que o upgrade não resulte em um build que não compila (`decision.md:18-23`), e registra a decisão de um upgrade de segurança restrito ao frontend (`decision.md:29-33`). O `specs/093-sveltekit-security-upgrade/spec.md:13-22` descreve o mesmo objetivo de desbloquear o `dependency-review`. CI remota, resultado do `dependency-review` na PR #318 e contagem pós-upgrade de advisories permanecem **[NV]** (`decision.md:71-80`).

## Restated

Propõe-se atualizar as dependências afetadas do frontend e adaptar a configuração às quebras do SvelteKit 3, mantendo o diff no frontend e nos artefatos do ciclo. A motivação registrada é remover os advisories que bloqueiam o merge sem alterar gates, API ou infraestrutura.

## Origin & Context

- **Raised by**: Engenheiro de Software / maintainer da MDMV (`@lfgranja`), conforme registrado nos artefatos do ciclo.
- **Trigger**: `dependency-review` da PR #318 bloqueando o merge de `dev` para `main` por `GHSA-3r8g` (high) e 3 advisories moderate em `@sveltejs/kit 2.70.3` (`decision.md:12-13`; `specs/093-sveltekit-security-upgrade/spec.md:15-20`).

## First-Glance Unknowns

- [NEEDS CLARIFICATION: Qual é a contagem viva de advisories no painel do GitHub após o upgrade, dado que só o inventário de entrada foi registrado ([NV])?]
- [NEEDS CLARIFICATION: O `dependency-review` da PR #318 voltou a passar na CI remota, considerando que gate local verde não a substitui ([NV])?]
- [NEEDS CLARIFICATION: Existem consumidores de `svelte.config.js` ou do alias `$lib` fora de `apps/frontend` que precisem ser considerados?]
- [NEEDS CLARIFICATION: Qual é a expectativa de compatibilidade com o runtime de deploy (`adapter-node`) após a mudança de major do kit?]
- [NEEDS CLARIFICATION: Há versões residuais das versões afetadas em outros locks do repositório além dos dois do frontend?]
- [NEEDS CLARIFICATION: Qual é a janela de reconsulta do estado remoto antes de qualquer promoção da PR #318?]
