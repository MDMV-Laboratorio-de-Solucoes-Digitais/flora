# Idea Intake: Sincronização do lockfile npm

- **Slug**: 092-npm-lock-sync
- **Created**: 2026-11-20
- **Source**: repositório local — `.specify/assessments/092-npm-lock-sync/decision.md` (+ `specs/092-npm-lock-sync/spec.md`)
- **Type**: fix

## Idea (as captured)

`apps/frontend/bun.lock` é o lockfile canónico para CI, mas `apps/frontend/package-lock.json` estava congelado: não continha `shadcn-svelte`, embora `package.json` já declarasse a dependência (`1.8.1`) e o `bun.lock` a registrasse; o lockfile npm acumulava ainda 25 alertas do Dependabot (`decision.md:12-17`). Há, portanto, drift entre o lockfile npm e a fonte de verdade, com falsas expectativas para quem consome o `package-lock.json`. O artefato registra o risco de "corrigir" mudando de gerenciador na CI, alterando o manifesto, executando scripts de dependências ou alargando o diff a source, workflows e artefatos de governança (`decision.md:19-22`), e registra a decisão de uma regeneração determinística e restrita ao lockfile npm (`decision.md:24-27`). O `specs/092-npm-lock-sync/spec.md:13-21` descreve o mesmo objetivo de conformidade do `package-lock.json` com o manifesto e o `bun.lock`. O painel do Dependabot não foi relido no ciclo, e CI/merge/deploy permanecem **[NV]** (`decision.md:57-62`).

## Restated

Propõe-se alinhar o lockfile npm ao manifesto e ao lockfile canónico da CI, sem alterar as fontes nem o pipeline. A motivação registrada é eliminar a divergência conhecida e o ruído de alertas sem mudar processo ou infraestrutura.

## Origin & Context

- **Raised by**: Engenheiro de Software / maintainer da MDMV (`@lfgranja`), conforme registrado nos artefatos do ciclo.
- **Trigger**: Drift observado entre `package-lock.json` congelado e as fontes (`package.json`/`bun.lock`), com a entrada `shadcn-svelte` ausente e inventário de 25 alertas do Dependabot (`decision.md:12-17`, `decision.md:40-44`).

## First-Glance Unknowns

- [NEEDS CLARIFICATION: Qual política impede que o lockfile npm volte a divergir do manifesto e do `bun.lock` após esta sincronização?]
- [NEEDS CLARIFICATION: Qual é a contagem viva de alertas do Dependabot após a triagem, dado que só o inventário de entrada foi registrado ([NV])?]
- [NEEDS CLARIFICATION: Existem consumidores do `package-lock.json` que dependam da regeneração e quais são eles?]
- [NEEDS CLARIFICATION: Como confirmar a idempotência do comando contratual em um ambiente sem acesso ao registry?]
- [NEEDS CLARIFICATION: Qual é a expectativa de tratamento posterior para os 25 alertas registrados — contexto apenas ou triagem futura?]
- [NEEDS CLARIFICATION: Há requisito de evidência da sincronização fora dos artefatos documentais do ciclo?]
