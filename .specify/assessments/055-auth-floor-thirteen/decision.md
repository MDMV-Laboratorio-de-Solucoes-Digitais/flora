# Decision: Piso do leg auth sem folga (055)

- **Slug**: 055-auth-floor-thirteen
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (anterior) | spec.md (ciclo)

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | Leg `auth` exige piso 12, mas `contract::auth::db_` descobre 13 testes: é o único com folga entre os oito legs (intake.md:9-12; research.md:1-4). |
| Evidence strength | strong | `cargo test ... --list` → 13 entradas, `rust.yml:107` e `check_gate_integrity.py:43,120`; ciclo 053 com Postgres 18.6 real → auth 13 passed (intake.md:14; research.md:9). |
| Value vs. inaction | strong | Inação mantém a folga no check obrigatório: um teste pode desaparecer sem CI vermelha, corroendo a confiança no guardião (problem.md:Cost of Inaction). |
| Feasibility / appetite | strong | Appetite small: três edições literais em 2 arquivos, com mutação temporária como verificação; os 13 testes já passaram com banco real (concept.md:Option A). |
| Strategic fit | strong | Fortalecimento do gate existente, sem ADR, constituição ou proteção de branch; alinhado a ADR-0003 (define.md:Non-Goals). |
| Risk posture | strong | Piso 14 (folga artificial) e mudança de filtro descartados; diff de 3 linhas; execução do leg no runner **[NV]** (decision.md anterior:NV). |

## Verdict & Rationale

**go**, preservando o veredito histórico `go (scoped)` (decision.md anterior:3; spec.md:Status "Approved"). Todos os critérios do scorecard são favoráveis: a contagem de 13 testes é determinística e a correção é um fortalecimento de gate, não uma mudança de comportamento. O veredito permanece restrito ao escopo anterior — 2 arquivos existentes, 3 linhas (`.github/workflows/rust.yml`, `scripts/ci/check_gate_integrity.py`), artefatos do ciclo e notas datadas append-only; nenhuma decisão de autorização, ADR ou proteção de branch é alterada e nenhum trabalho alheio é tocado (decision.md anterior:Delimitadores). ADR-0003 mantém o texto antigo (43) como dívida de governança registrada, sem edição sem aprovação; a execução do leg no runner e a CI ao vivo são **[NV]**, a observar no PR antes do merge.

## If needs-clarification

- **Blocking questions**: n/a — veredito é `go`; as perguntas sobre ADR e PR de observação são acompanhamento.
- **Revisit stage**: n/a

## If go — Handoff to `/speckit.specify`

- **Problem**: o leg `auth` do `db-tests` exige 12 testes, mas o filtro descobre 13 — um teste pode sumir ou ser ignorado sem que a CI fique vermelha.
- **Chosen approach**: Option A — exigir os treze testes nos dois pontos (`run_leg auth` em `rust.yml` e `FLOORS["auth"]` no guardião) e atualizar o texto de consolidação de "49 total" para "50 total", com mutação temporária que comprove que o piso é exigido (concept.md:Recommendation).
- **In scope / out of scope**: in — `.github/workflows/rust.yml:107`, `scripts/ci/check_gate_integrity.py:43,120`, artefatos do ciclo e notas datadas append-only; out — ADR-0003, constituição, outros pisos, checks, nomes de job e testes Rust, proteção de branch, C5/C6, deploy, tag/release e trabalho alheio.
- **Success metrics**: `--list` → 13 testes e guardião PASS com 13/13; mutação (piso 13 sem 13 testes reais) → leg/auth falha; `cargo test --workspace` e gates frontend verdes; diff ≤ 3 linhas em 2 arquivos; execução do leg no runner **[NV]** antes do merge (ADR-0003).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: dívida documental de ADR-0003 tratada em ciclo separado com aprovação de maintainer?]; [NEEDS CLARIFICATION: em qual PR o leg auth com DB real será observado?] **[NV]**; [NEEDS CLARIFICATION: os 13 testes devem permanecer todos com `#[ignore]` fora do ambiente do leg?].
