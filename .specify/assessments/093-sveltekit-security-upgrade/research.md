# Idea Research: Upgrade de segurança SvelteKit 3

- **Slug**: 093-sveltekit-security-upgrade
- **Created**: 2026-11-25
- **Evidence confidence (overall)**: high — os quatro advisories foram confirmados no banco de advisories do GitHub (host allowlisted) e o inventário local é convergente em decisão, specs, STATE e nota de governança; o resultado remoto do `dependency-review` após o bump é **[NV]** nos artefatos do ciclo.

## Users & Demand

- A demanda é do maintainer/SE (`@lfgranja`), registrada como fix: o `dependency-review` da PR #318 (`dev` → `main`) bloqueava o merge por advisories em `@sveltejs/kit 2.70.3`. [source: `.specify/assessments/093-sveltekit-security-upgrade/intake.md:8-19`; `decision.md:12-16`] (confidence: high)
- O bloqueio foi lido ao vivo via `gh` às 2026-11-20T19:56Z: PR `OPEN`/`BLOCKED`, `dependency-review` `fail` no run `44806804503` com `fail-on-severity: moderate` em modo "PR diff only". [source: `docs/governance/2026-11-20-093-sveltekit-upgrade.md:7-9`; `STATE.md:795`] (confidence: high para o registro; estado atual **[NV]**)
- A motivação registrada é desbloquear o merge sem alterar gates, API ou infraestrutura — a correção é adotar versões corrigidas, não flexibilizar o `dependency-review`. [source: `intake.md:12-14`; `.specify/assessments/093-sveltekit-security-upgrade/decision.md:63-69`] (confidence: high)

## Prior Art

- O `dependency-review` já era um dos 4 required checks da ADR-0003; decisões 089/091/092 reafirmaram que advisories não devem ser promovidos a gate nem substituídos por script SDD da factory. [source: `STATE.md:798`; `docs/governance/2026-11-20-093-sveltekit-upgrade.md:32-34`] (confidence: high)
- O baseline do frontend era `kit ^2.63.0`, `adapter-node ^5.5.4`, `svelte ^5.56.1` (decisão), e o diff da PR #318 introduzia `kit@2.70.3`, `adapter-node@5.5.7`, `vite-plugin-svelte@7.3.0` e `svelte@5.56.10`. [source: `.specify/assessments/093-sveltekit-security-upgrade/decision.md:52-54`; `docs/governance/2026-11-20-093-sveltekit-upgrade.md:18`] (confidence: high)
- O ciclo 092 estabelecera que `bun.lock` é o lockfile canônico da CI e que `package-lock.json` é derivado — padrão preservado pelo upgrade. [source: `.specify/assessments/093-sveltekit-security-upgrade/decision.md:41-43`; `STATE.md:796`] (confidence: high)
- A migração de configuração é uma substituição direta registrada: SvelteKit 3 não lê `svelte.config.js` e não gera `$lib` automaticamente; a config passa para o plugin `sveltekit()` em `vite.config.ts`, com alias em `resolve.alias` e `paths` do tsconfig estendendo `$app/tsconfig`. [source: `decision.md:18-23`; `specs/093-sveltekit-security-upgrade/tasks.md:36-45`] (confidence: high)

## Market & Context

- Os quatro advisories são confirmados externamente: `GHSA-3r8g-jcqq-3p5j` (high, CVSS 7.5, DoS em parsing de form data de remote functions; afetados `>=2.42.0, <=3.1.0`; corrigido em `3.1.1`), `GHSA-mw6w-2fmv-pvw3` (moderate, `URIError` não tratado no adapter-node via `reroute`), `GHSA-rqp5-r3wg-vg35` (moderate, import protocol-relative na resolução de rota SSR) e `GHSA-6cf5-gc4q-fm3x` (moderate, colisão de `query.batch` privadas). [source: `https://github.com/advisories/GHSA-3r8g-jcqq-3p5j`; `https://github.com/advisories/GHSA-mw6w-2fmv-pvw3`; `https://github.com/advisories/GHSA-rqp5-r3wg-vg35`; `https://github.com/advisories/GHSA-6cf5-gc4q-fm3x` — host github.com, allowlisted, acessado 2026-11-25] (confidence: high)
- Dois dos advisories moderados (`GHSA-mw6w-2fmv-pvw3`, `GHSA-rqp5-r3wg-vg35`) registram também `2.70.4` como versão corrigida na linha 2.x; os outros dois (`GHSA-3r8g-jcqq-3p5j`, `GHSA-6cf5-gc4q-fm3x`) só registram `3.1.1`. Ou seja, a existência de um patch 2.x não cobre o conjunto completo dos quatro achados. [source: `https://github.com/advisories/GHSA-mw6w-2fmv-pvw3`; `https://github.com/advisories/GHSA-rqp5-r3wg-vg35`; `https://github.com/advisories/GHSA-3r8g-jcqq-3p5j`; `https://github.com/advisories/GHSA-6cf5-gc4q-fm3x`] (confidence: high)
- A compatibilidade de peers é pré-requisito explícito do upgrade: kit `3.1.1` exige `vite ^8.0.12`, `svelte ^5.57.1`, `typescript ^6` e `vite-plugin-svelte ^7`; `adapter-node 6.0.5` exige `kit ^3`. O manifesto já declara `vite ^8.0.16`, `typescript ^6.0.3` e `@sveltejs/vite-plugin-svelte ^7.1.2`. [source: `docs/governance/2026-11-20-093-sveltekit-upgrade.md:22-26`; `apps/frontend/package.json:25-27,41,45,47`] (confidence: high)
- Produção: app e DNS estavam prontos, mas o deploy dependia da promoção de `main`, fora do gate de merge (ADR-0003); a promoção foi registrada depois, em `2026-11-20T20:38Z`, com PR #318 mesclado e `dependency-review` limpo no head promovido. [source: `STATE.md:797,801-805`; `docs/PROGRESSO.md:270-272`] (confidence: high)

## Data & Constraints

- Leitura direta neste worktree confirma o estado final do manifesto: `@sveltejs/adapter-node ^6.0.5`, `@sveltejs/kit ^3.1.1`, `@sveltejs/vite-plugin-svelte ^7.1.2`, `shadcn-svelte 1.8.1`, `svelte ^5.57.1`, `typescript ^6.0.3`, `vite ^8.0.16`. [source: `apps/frontend/package.json:25-27,40-47`] (confidence: high)
- O diff registrado contra `b426e4f81` inclui manifesto, ambos os locks, remoção de `svelte.config.js`, `vite.config.ts`, `tsconfig.json`, com `grep 2.70.3` retornando zero ocorrências nos locks. [source: `decision.md:50-61`; `specs/093-sveltekit-security-upgrade/converge.md:9-15`] (confidence: high)
- Verificações locais registradas: `npm run check` com 0 erros/0 warnings, `npm run lint` OK, testes unitários do servidor 42/42 e `npm run build` OK. [source: `decision.md:44-45,60-61`; `specs/093-sveltekit-security-upgrade/spec.md:87-90`] (confidence: high para o registro; não reexecutado nesta pesquisa)
- A fronteira do ciclo limita o diff ao frontend e aos artefatos documentais 093; nenhum toque em API, runtime, schema, migration, workflows, `validate/`, `release/` ou `STATE`. [source: `decision.md:46-48`; `specs/093-sveltekit-security-upgrade/spec.md:91-94`] (confidence: high)
- Cobertura idiomática real nos artefatos lidos: **pt-BR_en** — prosa em pt-BR; nomes de pacotes, comandos, IDs GHSA/CVE e termos de gate em inglês. [source: artefatos do ciclo e páginas de advisory lidas diretamente] (confidence: high)
- O resultado remoto do `dependency-review` na PR #318 e a contagem viva de advisories após o upgrade permanecem **[NV]** nos artefatos do ciclo; a promoção posterior é registrada em append-only em `STATE.md`/`PROGRESSO.md`. [source: `decision.md:71-80`; `STATE.md:804`] (confidence: high)

## Evidence Against the Idea

- Uma atualização de segurança que cruza um major do kit (`2.70.3 → 3.1.1`) carrega duas quebras de configuração registradas (`svelte.config.js` ignorado e `$lib` sem geração automática); se a migração falhar, o resultado é build que não compila, não apenas advisories resolvidos. [source: `decision.md:18-23`; `specs/093-sveltekit-security-upgrade/spec.md:60-69`] (confidence: high)
- Existe um patch na linha 2.x (`2.70.4`) para dois dos quatro advisories, o que mostra que o major não era a única via de remediação para esses dois; a escolha do major é registrada nos artefatos, mas não há neles uma comparação de custo/risco entre as opções. [source: `https://github.com/advisories/GHSA-mw6w-2fmv-pvw3`; `https://github.com/advisories/GHSA-rqp5-r3wg-vg35`; ausência de comparação em `decision.md:50-69`] (confidence: medium)
- Há divergência documental interna: `decision.md` e `tasks.md` descrevem nove tarefas (T001..T009, incluindo migração e verificação), enquanto `analyze.md` e `converge.md` registram `tasks=6/6` com T001..T006. O registro de análise aparece congelado num snapshot anterior ao da decisão final. [source: `.specify/assessments/093-sveltekit-security-upgrade/decision.md:31-33`; `specs/093-sveltekit-security-upgrade/tasks.md:12-53`; `analyze.md:9-14`; `converge.md:19`] (confidence: high)
- As verificações locais são evidência consolidada do handoff e não substituem a CI remota; o próprio artefato marca o `dependency-review` ao vivo como **[NV]** no momento do ciclo, de modo que "advisories resolvidos" não foi observado nesta pesquisa. [source: `decision.md:71-80`; `specs/093-sveltekit-security-upgrade/analyze.md:30-35`] (confidence: high)
- Três dos quatro advisories só afetam apps com flags experimentais (`remote functions`, `router.resolution: 'server'`); se o alvo não usar essas flags, a exposição prática é menor do que a contagem de severidade sugere — hipótese de escopo, não confirmada nos artefatos do ciclo. [source: `https://github.com/advisories/GHSA-3r8g-jcqq-3p5j`; `https://github.com/advisories/GHSA-rqp5-r3wg-vg35`; `https://github.com/advisories/GHSA-6cf5-gc4q-fm3x` | ASSUMPTION] (confidence: low)

## Gaps & Open Questions

- [NEEDS CLARIFICATION: Qual é a contagem viva de advisories no painel do GitHub após o upgrade, dado que só o inventário de entrada foi registrado ([NV])?]
- [NEEDS CLARIFICATION: O `dependency-review` da PR #318 voltou a passar na CI remota, considerando que gate local verde não a substitui ([NV])?]
- [NEEDS CLARIFICATION: Existem consumidores de `svelte.config.js` ou do alias `$lib` fora de `apps/frontend` que precisem ser considerados?]
- [NEEDS CLARIFICATION: Qual é a expectativa de compatibilidade com o runtime de deploy (`adapter-node`) após a mudança de major do kit?]
- [NEEDS CLARIFICATION: Há versões residuais das versões afetadas em outros locks do repositório além dos dois do frontend?]
- [NEEDS CLARIFICATION: Qual é a janela de reconsulta do estado remoto antes de qualquer promoção da PR #318?]
- [NEEDS CLARIFICATION: Como reconciliar o registro de T001..T009 em `tasks.md`/`decision.md` com o `tasks=6/6` de `analyze.md`/`converge.md`?]

## Sources

- `.specify/assessments/093-sveltekit-security-upgrade/intake.md:1-28` (system; policy: fonte primária local; lido 2026-11-25)
- `.specify/assessments/093-sveltekit-security-upgrade/decision.md:12-33,50-80` (system; policy: fonte primária local; lido 2026-11-25)
- `specs/093-sveltekit-security-upgrade/spec.md:13-129`, `plan.md:8-64`, `tasks.md:12-59` (system; policy: fonte primária local; lido 2026-11-25)
- `specs/093-sveltekit-security-upgrade/analyze.md:1-38`, `converge.md:6-24` (system; policy: fonte primária local; lido 2026-11-25)
- `docs/governance/2026-11-20-093-sveltekit-upgrade.md:7-38` (system; policy: fonte primária local; lido 2026-11-25)
- `.specify/memory/STATE.md:792-805` e `docs/PROGRESSO.md:261-272` (system; policy: registro append-only local; lido 2026-11-25)
- `apps/frontend/package.json:25-27,40-47` (system; policy: fonte primária local, leitura direta; lido 2026-11-25)
- `https://github.com/advisories/GHSA-3r8g-jcqq-3p5j` (host: github.com; policy: allowlisted; acessado 2026-11-25 — high, corrigido em 3.1.1)
- `https://github.com/advisories/GHSA-mw6w-2fmv-pvw3` (host: github.com; policy: allowlisted; acessado 2026-11-25 — moderate, corrigido em 3.1.1 e 2.70.4)
- `https://github.com/advisories/GHSA-rqp5-r3wg-vg35` (host: github.com; policy: allowlisted; acessado 2026-11-25 — moderate, corrigido em 3.1.1 e 2.70.4)
- `https://github.com/advisories/GHSA-6cf5-gc4q-fm3x` (host: github.com; policy: allowlisted; acessado 2026-11-25 — moderate, corrigido em 3.1.1)
- `https://nvd.nist.gov/vuln/detail/CVE-2026-128120` (host: nvd.nist.gov; policy: auto-refused: host not allowlisted; [UNVERIFIED — fetch skipped: host not allowlisted])
- `https://svelte.dev/docs/kit` (host: svelte.dev; policy: auto-refused: host not allowlisted; [UNVERIFIED — fetch skipped: host not allowlisted])
