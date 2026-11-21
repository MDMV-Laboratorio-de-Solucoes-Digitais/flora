# Validation: 099 — Interface 100% pt-BR via dicionário fixo

**Data**: 2026-11-21 · **Branch**: `feature/099-ptbr-ui` @ `7183a33fc` · **Baseline**: `7183a33fc`
**Tipo**: tradução integral da interface — dicionário fixo pt-BR (`src/lib/i18n/pt-BR.ts` + `index.ts`), preservação de `lang="pt-BR"` em `app.html`, cobertura de rotas `(auth)`/`(app)` e componentes de domínio, mapeamento de erros do backend e atualização de seletores de teste. **Nenhuma alteração de gate, required check, constituição, ADR, contratos de API, valores de status, schema, PR/merge/tag, `STATE.md` ou `PROGRESSO*` nesta validação.**
**Veredito R4**: **APROVADO**, com gates e verificações **medidos às 2026-11-21T03:36:56Z–03:41Z** (saídas reais coladas abaixo) e registro explícito de **[NV]** para CI remota, deploy/Coolify e renderização em produção **até o deploy final**. Esta R4 **não reexecutou** gates, build, testes, formatters nem comandos git de mutação; as saídas foram medidas pelo main agent e consolidadas aqui.

## Gates e verificações

| Gate | Resultado (saída colada) |
|---|---|
| `python3 scripts/ci/check_gate_integrity.py` | **gate-integrity: PASS**, `exit=0`, medido às `2026-11-21T03:36:56Z–03:41Z` — único gate autoritativo sob ADR-0003, nenhum required check acrescentado ou relaxado neste ciclo (FR-001..FR-008, SC-003; T010) |
| `git diff --check` | **exit=0**, medido às `2026-11-21T03:36:56Z–03:41Z` — diff limpo contra `7183a33fc` (SC-003; T010) |
| `npm --prefix apps/frontend run check` (svelte-check) | **0 errors**, medido às `2026-11-21T03:36:56Z–03:41Z` — nenhum erro de tipo nos componentes migrados para `t.*` (FR-001, FR-003..FR-006, SC-001, SC-003; T002, T004..T007) |
| `npm run lint` (prettier + eslint) | **sem issues**, medido às `2026-11-21T03:36:56Z–03:41Z` — formatação e lint do frontend limpos (FR-008, SC-003; T009, T010) |
| testes vitest / Playwright (`*.spec.ts`) | seletores de texto ajustados às strings pt-BR no diff revisado; execução de vitest/Playwright **não medida nesta janela** — não listada entre as verificações reais do handoff (FR-008, SC-003; T009) |
| grep de strings EN residuais (rotas/componentes ativos) | **sem matches**, medido às `2026-11-21T03:36:56Z–03:41Z` — nenhuma literal em inglês fora da allowlist `OIDC`, `OAuth`, `API`, `JWT`, `REST`, `AI`, `SSO`, `slug`; valores de status `TODO`/`IN_PROGRESS`/`DONE` preservados (FR-001, FR-005, FR-006, SC-001, SC-002; T002, T006, T007, T010) |
| `apps/frontend/src/app.html` | `<html lang="pt-BR">` **confirmado em `:2`**, medido às `2026-11-21T03:36:56Z–03:41Z` — semântica do documento coerente com o texto visível (FR-002, SC-001; T003, T010) |
| mapeamento de erros backend | códigos `oidc_error`, `invalid_credentials`, `rate_limited`, `conflict`, `validation` convertidos para mensagens pt-BR com fallback genérico; `flora-api/src/routes/auth.rs` e `flora-core/src/error.rs` intocados, endpoints/OIDC/sessões/`config.js`/CORS preservados (FR-007, SC-001, SC-002; T008) |
| `git diff` / `git status` contra `7183a33fc` | mudanças restritas a `apps/frontend/src/**` (`i18n/`, rotas, componentes, `*.spec.ts`) e aos artefatos do ciclo 099 (`docs/governance/2026-11-21-099-ptbr-ui.md`, `intake`/`research`/`define`/`shape`/`decision`, `spec.md`, `plan.md`, `tasks.md`, `analyze.md`, `converge.md`, este R4 e o R7), além dos apêndices de memória/governança; nenhum toque em gate, required check, constituição, ADR, contratos de API, valores de status, schema, `STATE.md` histórico, `PROGRESSO*`, `validate/` pré-existente, `release/`, PR/merge/tag (FR-008, SC-003; T001, T010) |
| PRs no ciclo | nenhum PR, merge, tag ou deploy realizado por esta R4; publicação e promoção permanecem manuais e **[NV]** (SC-003) |

Nota de proveniência: todas as verificações da tabela foram **medidas pelo main agent às 2026-11-21T03:36:56Z–03:41Z**, com as saídas reais coladas no bloco abaixo; esta passagem R4 **não reexecutou** gates, build, testes, formatters, git de mutação, CI ao vivo, navegador real nem ambiente de produção. Nenhuma inferência além dessa evidência.

## Saídas coladas

Evidência **medida pelo main agent às 2026-11-21T03:36:56Z–03:41Z** e consolidada aqui, mais o diagnóstico lido diretamente no baseline (esta R4 não reexecutou nada):

```
$ # verificações reais medidas pelo main agent — 2026-11-21T03:36:56Z–03:41Z
$ git diff --check
exit=0

$ npm --prefix apps/frontend run check          # svelte-check
0 errors

$ npm run lint                                  # prettier + eslint
sem issues

$ python3 scripts/ci/check_gate_integrity.py
gate-integrity: PASS   exit=0   (único gate autoritativo — ADR-0003)

$ # grep de strings EN residuais — rotas/componentes ativos
sem matches
allowlist preservada: OIDC OAuth API JWT REST AI SSO slug
status preservados: TODO | IN_PROGRESS | DONE

$ # app.html
$ grep -n 'html lang' apps/frontend/src/app.html
2:<html lang="pt-BR">

$ # vitest / Playwright: não medido nesta janela

$ # diagnóstico — leitura direta no baseline 7183a33fc
$ grep -n 'html lang' apps/frontend/src/app.html
2:<html lang="pt-BR">        # já correto no baseline; FR-002 confirma/preserva

$ # antes do ciclo: sem i18n, ~140 strings em ~15 arquivos
$ grep -rniE '(Sign In|Loading|Select|Settings|Tasks|Channels|Failed to|Send|Reply)' \
    apps/frontend/src/routes apps/frontend/src/lib/components
~140 literais em ~15 arquivos ativos (inventário I18nScout)

$ # depois do ciclo: único ponto de manutenção
$ grep -n 'export const t' apps/frontend/src/lib/i18n/pt-BR.ts
export const t = { ... } as const

$ # erros do backend traduzidos na UI, origem intocada
códigos mapeados: oidc_error invalid_credentials rate_limited conflict validation
flora-api/src/routes/auth.rs, flora-core/src/error.rs: intocados

PRs no ciclo: 0   merge: 0   tag: 0   deploy executado por esta R4: 0
CI remota / deploy / renderização em produção: [NV] até o deploy final
```

**Estado final confirmado**: a leitura direta no baseline confirma o diagnóstico — `app.html:2` já com `lang="pt-BR"`, nenhuma camada de i18n e ≈140 literais em ≈15 arquivos ativos, com texto de erro do backend interpolado na tela de login. A decisão e os contratos seguem `intake`/`research`/`define`/`shape`/`decision`, `spec.md`, `plan.md` e `tasks.md`: um dicionário fixo pt-BR (`t` achatado), cobertura integral das rotas e componentes, allowlist restrita de termos técnicos, mapeamento de erros no frontend e seletores de teste atualizados — sem tocar em contratos de API, valores de status, OIDC, `config.js` ou CORS.

## Fronteira

Contra a base `7183a33fc`, as mudanças de implementação ficam restritas a `apps/frontend/src/**` (módulo `i18n`, rotas `(auth)`/`(app)`, componentes de domínio e `*.spec.ts`) e aos artefatos do próprio ciclo. Nada mais foi alterado pela implementação: sem gate, required check, constituição, ADR, contratos de API, valores de status `TODO`/`IN_PROGRESS`/`DONE`, schema, `STATE.md` histórico, `PROGRESSO*`, OIDC/allowlist, `config.js`, CORS, PR, merge, tag ou deploy. `validate.md` (R4) e `release.md` (R7) são registros novos criados nas fases seguintes do mesmo ciclo e não afetam o sistema; por isso T010 os exclui do escopo de mudança de implementação.

## [NV]

- **Renderização em produção**: a conferência visual de 100% pt-BR em cada rota, o fluxo de erro OIDC traduzido e a varredura de literais no bundle publicado não foram observados ao vivo; permanecem **[NV]** até o deploy final (SC-001, SC-002).
- **CI/PR/produção**: verificações locais **medidas PASS às 2026-11-21T03:36:56Z–03:41Z** (saídas reais na tabela e no bloco acima), mas nenhuma CI remota, PR, merge, tag, aprovação de maintainer, proteção de branch ou ambiente de produção foi verificado neste ciclo; gate local não substitui CI remota para promoção (SC-003).
- **Coolify/Forgejo/DNS**: reapontamento do app web, publicação de imagem, resultado do deploy e E2E autenticado em navegador real não foram observados; permanecem **[NV]** até o deploy final (SC-003).
- **Textos na origem backend**: o mapeamento de códigos de erro é feito no frontend; a eventual tradução das mensagens em `flora-api`/`flora-core` está fora deste ciclo e permanece **[NV]** como decisão de escopo (FR-007).
- **Reexecução nesta passagem**: gates, build, testes, formatters e comandos git não foram reexecutados ao escrever este R4; as saídas foram medidas pelo main agent às 2026-11-21T03:36:56Z–03:41Z.

## Portarias

R1 `go` com diagnóstico determinístico, decisão materializada em `.specify/assessments/099-ptbr-ui/{intake,research,define,shape,decision}.md` e em `docs/governance/2026-11-21-099-ptbr-ui.md` (2026-11-21, append-only); R2 analyze READ-ONLY com `critical=0`, `requirements_covered=8/8`, `fr=8/8`, `sc=3/3` e `tasks=10/10` (`specs/099-ptbr-ui/analyze.md`); R3 zero esclarecimentos pendentes; R4 este documento aprovado **antes** de `release.md`, com gates e verificações **medidos PASS às 2026-11-21T03:36:56Z–03:41Z** (`git diff --check` exit=0, `npm --prefix apps/frontend run check` 0 errors, `npm run lint` sem issues, `check_gate_integrity.py` PASS exit=0, grep de strings EN residuais sem matches e `lang="pt-BR"` confirmado) e **[NV]** explícito para CI remota, deploy/Coolify e renderização em produção até o deploy final; R7 `release.md` registro posterior, com SemVer MINOR `0.7.1` → `0.8.0` e procedimento de rollback.
