# Idea Research: Sincronização do lockfile npm

- **Slug**: 092-npm-lock-sync
- **Created**: 2026-11-25
- **Evidence confidence (overall)**: high — drift, comando e diff estão documentados em múltiplas fontes locais convergentes e foram confirmados por leitura direta dos lockfiles neste worktree; contagem viva de alerts pós-triagem é **[NV]**.

## Users & Demand

- A demanda é do maintainer/SE (`@lfgranja`), registrada como fix: `apps/frontend/package-lock.json` estava congelado e sem `shadcn-svelte`, embora `package.json` e o `bun.lock` canônico já o contivessem; o inventário de entrada era de 25 alertas do Dependabot. [source: `.specify/assessments/092-npm-lock-sync/intake.md:8-19`; `decision.md:12-17`] (confidence: high)
- A motivação registrada é eliminar divergência conhecida e ruído de alertas sem mudar processo ou infraestrutura — ou seja, o valor está na conformidade do artefato derivado, não em nova dependência. [source: `intake.md:12-14`; `.specify/assessments/092-npm-lock-sync/decision.md:50-55`] (confidence: high)
- Consumidores do `package-lock.json` são citados como hipótese ("falsas expectativas para quem consome o `package-lock.json`"), mas nenhum consumidor concreto é identificado nos artefatos; o intake registra essa incógnita. [source: `decision.md:16-17`; `intake.md:25`] (confidence: medium)

## Prior Art

- O ciclo 079 instalou exatamente `shadcn-svelte@1.8.1` como devDependency via `bun add -D -E` e gravou `+9` linhas em `bun.lock`, com verificação `bun pm ls shadcn-svelte → 1.8.1` e `bun install --frozen-lockfile --dry-run` exit 0. É a origem da entrada que depois faltaria no lock npm. [source: `.specify/memory/STATE.md:631-637`] (confidence: high)
- O próprio ciclo 091 já registrara os 25 advisories (7 high) em `apps/frontend/package-lock.json` e a recomendação de **não promover** advisories a required check — contexto herdado pelo ciclo 092. [source: `STATE.md:779`; `docs/governance/2026-11-20-091-relatorios-sem-scripts-sdd.md:40-44`] (confidence: high)
- A política de lockfile canônico para CI já estava estabelecida antes do ciclo: `apps/frontend/bun.lock` é a fonte de verdade; `package-lock.json` é artefato derivado. Nenhum ciclo anterior trocou o gerenciador do CI. [source: `.specify/assessments/092-npm-lock-sync/decision.md:12-13,52-55`; `specs/092-npm-lock-sync/spec.md:58-63`] (confidence: high)
- O risco de "corrigir" mudando de gerenciador, alterando o manifesto ou rodando lifecycle scripts foi explicitamente registrado como fronteira desde a decisão. [source: `decision.md:19-22`; `specs/092-npm-lock-sync/plan.md:25-26`] (confidence: high)

## Market & Context

- A CI do repositório é ancorada em `bun.lock`; o comando contratual `npm install --package-lock-only --ignore-scripts` regenera somente o lock derivado, sem tocar no manifesto nem no lock canônico. [source: `specs/092-npm-lock-sync/spec.md:45-56`; `decision.md:29-38`] (confidence: high)
- A contagem de alertas não é uma série temporal comparável: o baseline é do painel do Dependabot (25 alerts, 7 high) e a recontagem é de `npm audit --package-lock-only --json` (21 advisories: 5 high, 15 moderate, 1 low, 0 critical, `auditReportVersion 2`), com fontes distintas e sem comparação estrita. [source: `docs/governance/2026-11-20-092-package-lock-sync.md:19-24`; `STATE.md:788`] (confidence: high)
- O gate permanece inalterado: decisões 089/091/092 reafirmam que checagem de advisories não é required check; a autoridade é `scripts/ci/check_gate_integrity.py`. [source: `STATE.md:788-789`; `docs/governance/2026-11-20-092-package-lock-sync.md:24`] (confidence: high)
- Promoção `dev → main` e deploy estavam fora do gate e não foram executados pelo ciclo; o estado ao vivo de PR/CI/Coolify ficou **[NV]**. [source: `decision.md:57-62`; `STATE.md:789`] (confidence: high)

## Data & Constraints

- Leitura direta neste worktree confirma o estado final: `apps/frontend/package-lock.json:36` declara `"shadcn-svelte": "1.8.1"` na raiz e `:4065` contém a entrada `node_modules/shadcn-svelte`; `apps/frontend/bun.lock:33,531` registra a mesma versão. [source: `apps/frontend/package-lock.json:36,4065`; `apps/frontend/bun.lock:33,531`] (confidence: high)
- O diff registrado contra a baseline `bf00caf5c` é `+116/−0` em um único arquivo (`package-lock.json`), sem alteração em `package.json` ou `bun.lock`; a nota de governança detalha transitivas (`@emnapi/*`, `cn`, `commander`, `node-fetch-native`, `node_modules/shadcn-svelte`). [source: `decision.md:45-48`; `docs/governance/2026-11-20-092-package-lock-sync.md:17`; `STATE.md:787`] (confidence: high)
- O comando é de lockfile-only e com `--ignore-scripts`: nenhum lifecycle script é executado, `node_modules` não é alvo, e o comando é idempotente (reexecução com zero diffs adicionais registrada). [source: `decision.md:31-36`; `specs/092-npm-lock-sync/spec.md:33-39`; `specs/092-npm-lock-sync/converge.md:10-15`] (confidence: high)
- A SHA-256 do lock após a regeneração foi registrada como `68bd731c7e84b6e81e91a2d00b3a05c13500014fa7388a08d8b7e28a17790a28`; exit 1 do `npm audit` é comportamento normal quando há advisories. [source: `docs/governance/2026-11-20-092-package-lock-sync.md:22`] (confidence: high)
- Cobertura idiomática real nos artefatos lidos: **pt-BR_en** — prosa em pt-BR (inclusive "canónico"/"canônico" conforme o arquivo); comandos, caminhos e termos de registry em inglês. [source: artefatos do ciclo e lockfiles lidos diretamente] (confidence: high)
- `analyze` fechou com `critical=0`, `requirements_covered=5/5`, `sc=3/3`, `tasks=6/6`, `errors=[]`; a contagem viva de alerts após a triagem permanece **[NV]**. [source: `specs/092-npm-lock-sync/analyze.md:9-16,30-34`] (confidence: high)

## Evidence Against the Idea

- A queda registrada de 25 para 21 advisories não demonstra remediação: as fontes são distintas (Dependabot vs. registro npm lido do lock), o próprio documento declara "sem comparação estrita", e nenhum finding individual foi triado ou corrigido neste ciclo. [source: `docs/governance/2026-11-20-092-package-lock-sync.md:21-22`; `specs/092-npm-lock-sync/spec.md:38-39`] (confidence: high)
- A "sincronização" é de estrutura, não de conteúdo auditado: o comando é determinístico em relação ao manifesto, mas `--ignore-scripts` e `--package-lock-only` não exercitam a instalação real, então integridade de transitivas e comportamento de lifecycle scripts não foram observados. [source: `decision.md:31-36`; `specs/092-npm-lock-sync/tasks.md:19-28`] (confidence: medium)
- Idempotência é registrada como evidência consolidada do handoff; nesta pesquisa o comando não foi reexecutado e não há acesso ao registry (host não allowlisted), então a afirmação não foi verificada de forma independente. [source: `specs/092-npm-lock-sync/converge.md:12`; `decision.md:57-62`] (confidence: high)
- Não há política documentada que impeça novo drift futuro entre `package-lock.json` e as fontes; o ciclo corrige o estado atual, mas o intake registra essa lacuna como pergunta aberta. [source: `intake.md:23`] (confidence: high)
- Consumidores reais do `package-lock.json` não foram identificados; se ninguém consome o arquivo, o valor da sincronização é sobretudo documental, e a expectativa de "falsas expectativas" é inferência não measureada. [source: `intake.md:25` | ASSUMPTION] (confidence: low)

## Gaps & Open Questions

- [NEEDS CLARIFICATION: Qual política impede que o lockfile npm volte a divergir do manifesto e do `bun.lock` após esta sincronização?]
- [NEEDS CLARIFICATION: Qual é a contagem viva de alertas do Dependabot após a triagem, dado que só o inventário de entrada foi registrado ([NV])?]
- [NEEDS CLARIFICATION: Existem consumidores do `package-lock.json` que dependam da regeneração e quais são eles?]
- [NEEDS CLARIFICATION: Como confirmar a idempotência do comando contratual em um ambiente sem acesso ao registry?]
- [NEEDS CLARIFICATION: Qual é a expectativa de tratamento posterior para os 25 alertas registrados — contexto apenas ou triagem futura?]
- [NEEDS CLARIFICATION: Há requisito de evidência da sincronização fora dos artefatos documentais do ciclo?]

## Sources

- `.specify/assessments/092-npm-lock-sync/intake.md:1-28` (system; policy: fonte primária local; lido 2026-11-25)
- `.specify/assessments/092-npm-lock-sync/decision.md:12-17,26-38,40-62` (system; policy: fonte primária local; lido 2026-11-25)
- `specs/092-npm-lock-sync/spec.md:13-82`, `plan.md:8-41`, `tasks.md:12-35` (system; policy: fonte primária local; lido 2026-11-25)
- `specs/092-npm-lock-sync/analyze.md:9-34`, `converge.md:8-19` (system; policy: fonte primária local; lido 2026-11-25)
- `docs/governance/2026-11-20-092-package-lock-sync.md:7-32` (system; policy: fonte primária local; lido 2026-11-25)
- `.specify/memory/STATE.md:631-637,784-790` e `docs/PROGRESSO.md:253-259` (system; policy: registro append-only local; lido 2026-11-25)
- `apps/frontend/package-lock.json:36,4065`, `apps/frontend/bun.lock:33,531`, `apps/frontend/package.json:40` (system; policy: fonte primária local, leitura direta nesta pesquisa; lido 2026-11-25)
- `https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora` (host: github.com; policy: allowlisted; não buscado nesta passagem — painel Dependabot ao vivo **[NV]**)
- `https://registry.npmjs.org/shadcn-svelte/-/shadcn-svelte-1.8.1.tgz` (host: registry.npmjs.org; policy: auto-refused: host not allowlisted; [UNVERIFIED — fetch skipped: host not allowlisted])
