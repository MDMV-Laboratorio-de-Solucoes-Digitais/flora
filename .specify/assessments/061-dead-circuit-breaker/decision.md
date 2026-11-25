# Decision: 061 — Dead circuit breaker

- **Slug**: 061-dead-circuit-breaker
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | define.md | shape.md | decision.md (anterior) | specs/061-dead-circuit-breaker/converge.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | Placeholder de 68 linhas admite não observar falhas e nunca é declarado em `utils/mod.rs:3-4` (`jwt`, `pkce`); campos de config e variante de erro órfãos (intake.md:9-17). |
| Evidence strength | strong | `grep -rn circuit_breaker` aponta só o arquivo morto, `config/messaging.rs:19,21,33,34` e `error.rs:181,365`; verificador 6/6 PASS e 0 ocorrências após a limpeza (research.md; converge.md). |
| Value vs. inaction | adequate | Remove superfície de leitura enganosa e dívida morta, mas não altera comportamento de runtime nem entrega capacidade nova (I=2). |
| Feasibility / appetite | strong | 3 arquivos em `flora-core/src/` (1 exclusão, 2 edições), 79 remoções / 0 inserções (shape.md; converge.md). |
| Strategic fit | strong | Alinhado ao princípio I da constituição (debloat/leveza) sem abrir escopo de feature (concept.md Option A). |
| Risk posture | strong | Módulo nunca compilou; Figment sem `deny_unknown_fields` ignora env extra (`config/mod.rs:44-46`); reversível, sem credenciais; CI ao vivo **[NV]**. |

## Verdict & Rationale

Veredito histórico **`go (scoped)` preservado**: a evidência local é unívoca de que o código é morto, o diff é mínimo e nenhuma decisão de autorização, migração ou CI é alterada. O único ponto em aberto — CI/runner ao vivo **[NV]** (ADR-0003) — é observacional e fica para o PR. Âncora `github/dev@7ed232c46`, branch `feature/061-dead-circuit-breaker`; factory fonte `0cb34c4d…`, nada transplantado. Nenhuma afirmação de que existisse circuit-breaker em produção: o módulo nunca entrou na compilação.

## If needs-clarification

- **Blocking questions**: N/A — nenhuma pergunta bloqueia o avanço; veredito é `go`.
- **Revisit stage**: N/A

## If go — Handoff to `/speckit.specify`

- **Problem**: Código morto e configuração/variante de erro órfãs permanecem no repositório, sugerindo uma proteção que nunca existiu em runtime.
- **Chosen approach**: Option A de `concept.md` — excluir `flora-core/src/utils/circuit_breaker.rs`, remover os campos `circuit_breaker_*` e seus defaults em `config/messaging.rs` e a variante `Error::CircuitBreakerOpen` com seu braço em `error.rs`.
- **In scope / out of scope**: 3 arquivos em `flora-core/src/` + artefatos do ciclo; fora circuit-breaker real, refatoração ampla de `Error`, Figment/Config, ADRs, workflows, proteção de branch, migrações, C5/C6 e deploy.
- **Success metrics**: `grep -rn circuit_breaker` → 0 ocorrências no código; fmt/clippy/deny/build/test e frontend verdes; diff = 3 arquivos (1 exclusão, 2 edições) + artefatos; gates do alvo verdes (define.md SC-001–SC-004).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: algum env externo define `FLORA_*` com os campos removidos — **[NV]**]; [NEEDS CLARIFICATION: algum ADR ou documento cita o circuit-breaker além do código]; [NEEDS CLARIFICATION: em qual PR a CI ao vivo será observada — **[NV]**].
