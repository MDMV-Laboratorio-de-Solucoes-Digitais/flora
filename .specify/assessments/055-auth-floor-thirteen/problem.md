# Problem Definition: Piso do leg auth abaixo dos testes descobertos

- **Slug**: 055-auth-floor-thirteen
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md | define.md (equivalente local; `problem.md` ausente no baseline) | shape.md

## Problem Statement

O leg `auth` do job `db-tests` exige piso 12, mas o filtro `contract::auth::db_` descobre 13 testes `#[ignore]` — `cargo test -q -p flora-tests 'contract::auth::db_' -- --list` retornou 13 entradas (intake.md:9-11; research.md:5). `.github/workflows/rust.yml:107` registra `run_leg auth contract::auth::db_ 12` e `scripts/ci/check_gate_integrity.py:43` registra `"auth": 12`; `:120` imprime "49 total" (intake.md:12-14; research.md:6-8). Há, portanto, folga de 1 no check obrigatório de merge: um teste pode desaparecer ou ser ignorado sem que a CI fique vermelha, enquanto os outros sete legs usam piso exatamente igual à contagem (intake.md:11-12). No ciclo 053, os 8 legs rodaram com Postgres 18.6 real e auth teve 13 passed, sem teste condicional observado (intake.md:15; research.md:9). `docs/adr/0003:19` ainda cita "43" — dívida de governança fora de escopo (intake.md:17). Ranqueamento 80/20 = 10.0.

## Affected Users & Stakeholders

- **Users**: pessoas que dependem da proteção dos contratos DB de auth garantida pelo gate de merge (intake.md:11-12).
- **Stakeholders**: mantenedores e guardiões do gate determinístico, que precisam que cada piso corresponda aos testes realmente descobertos, sem alterar ADR ou proteção de branch (`define.md:7-10`; `decision.md:9-13`).

## Goals

- Exigir piso 13 para `run_leg auth` em `rust.yml` e `FLOORS["auth"]` no guardião (`define.md:7-8`; `shape.md:5-7`).
- Atualizar o texto de consolidação de "49 total" para "50 total" (`define.md:9`).
- Provar por mutação temporária que o piso 13 é exigido e que nenhum outro piso, check, job ou teste muda (`define.md:10`; `shape.md:17`).
- Manter os 13 testes passando com Postgres real, como observado no ciclo 053 (`research.md:9`).

## Non-Goals

- Não editar ADR-0003, constituição, outros pisos ou testes (`define.md:13`; `research.md:17-19`).
- Não alterar proteção de branch, C5/C6 ou deploy (`define.md:14`; `decision.md:12`).
- Não usar piso 14 (folga artificial) nem mudar o filtro, que já cobre os 13 (`research.md:14-16`).
- Não atualizar o ADR no mesmo PR: exige aprovação de maintainer (`research.md:18-19`).

## Success Metrics

- `--list` → 13 testes e guardião PASS com 13/13 (baseline: 13 descobertos, piso 12 — intake.md:9-12; define.md:13).
- Mutação (piso 13 sem 13 testes reais) → leg/auth falha (baseline: piso 12 aceitava 12 sem vermelho — define.md:14; shape.md:17).
- `cargo test --workspace` e gates frontend permanecem verdes (baseline: verdes — define.md:15).
- Diff de código ≤ 3 linhas em 2 arquivos (baseline: nenhuma mudança de código — define.md:16; decision.md:11).
- Execução do leg no runner do GitHub: **[NV]** — observar no PR antes do merge (decision.md:17).

## Cost of Inaction

A folga de 1 continua no check obrigatório: um teste de contrato de auth pode desaparecer ou ser ignorado sem tornar a CI vermelha. Como os outros sete legs são exatos, a exceção corrói a confiança no guardião determinístico e deixa o ADR-0003 defasado sem que o gate acuse (intake.md:11-17; research.md:17-19).

## Open Questions

- [NEEDS CLARIFICATION: a dívida documental do ADR-0003 será tratada em ciclo separado com aprovação de maintainer?] (`research.md:18-19`)
- [NEEDS CLARIFICATION: em qual PR o leg auth com DB real será observado?] (`decision.md:17`; **[NV]**)
- [NEEDS CLARIFICATION: os 13 testes descobertos devem permanecer todos com `#[ignore]` fora do ambiente do leg?] (`intake.md:9-11`; `research.md:9`)
