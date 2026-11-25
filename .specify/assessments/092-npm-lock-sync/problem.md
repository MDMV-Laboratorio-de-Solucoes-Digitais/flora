# Problem Definition: Divergência do lockfile npm derivado

- **Slug**: 092-npm-lock-sync
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md

## Problem Statement

`apps/frontend/bun.lock` é o lockfile canônico para CI, mas `apps/frontend/package-lock.json` estava congelado: não continha `shadcn-svelte`, embora `package.json` já declarasse a dependência (`1.8.1`) e o `bun.lock` a registrasse (`decision.md:12-17`; `research.md:52`). O lockfile npm acumulava ainda 25 alertas do Dependabot no inventário de entrada (`decision.md:12-17`). Há, portanto, drift entre o artefato derivado e a fonte de verdade, com falsas expectativas para quem consome o `package-lock.json` — consumidores concretos não foram identificados nos artefatos (`research.md:33`). O risco registrado é "corrigir" mudando de gerenciador na CI, alterando o manifesto, executando lifecycle scripts ou alargando o diff a source, workflows e artefatos de governança (`decision.md:19-22`). No checkout corrente, `package-lock.json:36,4065` já contém a entrada e a versão, com diff de `+116/−0` em um único arquivo (`research.md:52-53`); a contagem viva de alertas após a triagem é **[NV]** e as fontes (Dependabot vs. `npm audit`) não são estritamente comparáveis (`research.md:56`).

## Affected Users & Stakeholders

- **Users**: maintainer/SE, que registrou o defeito como `fix` e precisa eliminar a divergência conhecida sem mudar processo ou infraestrutura (`research.md:31`).
- **Stakeholders**: dono da política de lockfile canônico (`bun.lock` para CI); equipe de CI; responsável pela triagem de advisories; [ASSUMPTION] consumidores do `package-lock.json`, citados como hipótese mas não identificados (`research.md:33`).

## Goals

- Restabelecer a conformidade do lockfile npm derivado com o manifesto e com o lockfile canônico da CI.
- Eliminar a divergência conhecida e reduzir ruído de alertas sem mudar processo, gerenciador ou infraestrutura (`intake.md:12-14`).
- Preservar as fontes e o pipeline: manifesto e lock canônico não são alvo da correção (`decision.md:33-35`).
- Manter o diff restrito ao lockfile npm, com evidência de idempotência e rastreabilidade documental (`decision.md:36-38`).

## Non-Goals

- Trocar o gerenciador da CI ou alterar `package.json` e `bun.lock`.
- Executar lifecycle scripts, instalar `node_modules` ou modificar source, CI, `validate/`, `release/` ou `STATE` (`decision.md:33-35`).
- Remediar individualmente os 25 alertas do Dependabot — o inventário registrado é contexto, não escopo (`decision.md` 092, "Fora de âmbito").
- Afirmar contagem pós-triagem ou execução ao vivo de GitHub Actions sem **[NV]** (`decision.md:57-62`).

## Success Metrics

- `package-lock.json` reflete `package.json` e `bun.lock`, incluindo `shadcn-svelte 1.8.1` na raiz e na entrada transitiva (baseline: dependência ausente antes da regeneração; `package-lock.json:36,4065` no checkout — `research.md:52`).
- Alteração de código confinada ao lockfile npm (baseline: `+116/−0` em um arquivo, sem toque em `package.json` ou `bun.lock` — `research.md:53`).
- Regeneração idempotente (baseline: registrada no handoff, não reexecutada nesta avaliação — `research.md:58`).
- CI permanece ancorada em `bun.lock`, sem troca de gerenciador (baseline: política registrada — `decision.md:12-13,52-55`).

## Cost of Inaction

O lockfile npm permanece congelado e divergente das fontes, de modo que quem o consome pode assumir um grafo de dependências que não corresponde ao manifesto. O inventário de 25 alertas segue sem triagem, e a política de lockfile canônico perde credibilidade sempre que o artefato derivado é usado como referência.

## Open Questions

- [NEEDS CLARIFICATION: qual política impede que o lockfile npm volte a divergir do manifesto e do `bun.lock` após esta sincronização?] (`intake.md:23`; `research.md:59`)
- [NEEDS CLARIFICATION: qual é a contagem viva de alertas do Dependabot após a triagem, dado que só o inventário de entrada foi registrado (**[NV]**)?] (`intake.md:24`; `decision.md:57-62`)
- [NEEDS CLARIFICATION: existem consumidores do `package-lock.json` que dependam da regeneração e quais são eles?] (`intake.md:25`)
- [NEEDS CLARIFICATION: como confirmar a idempotência do comando contratual em um ambiente sem acesso ao registry?] (`intake.md:26`)
- [NEEDS CLARIFICATION: qual é a expectativa de tratamento posterior para os 25 alertas registrados — contexto apenas ou triagem futura?] (`intake.md:27`)
- [NEEDS CLARIFICATION: há requisito de evidência da sincronização fora dos artefatos documentais do ciclo?] (`intake.md:28`)
