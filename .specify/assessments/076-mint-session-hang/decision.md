# Decision: Hang do setup E2E do minter de sessão

- **Slug**: 076-mint-session-hang
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | define.md | concept.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `seed.setup.ts:36-53` roda `cargo run ... --features dev-mint` via `execFileSync` sem timeout dentro da janela de `test.setTimeout(180_000)` (`:14`); a compilação consome a janela (`intake.md:9`, `define.md:5-7`). |
| Evidence strength | strong | Tempos medidos (binário direto 0.00s, `cargo run` com target quente 0.39s, `cargo build` com cache 30.18s) e linhas coladas em `flora-mint-session.rs:47-103`, `Cargo.toml:59-68`, `rust.yml:192-195,150` (`research.md:7-14`). |
| Value vs. inaction | adequate | Sem pré-build, o pendura local fica sem sinal porque o CI pré-compila fora da janela e `e2e-browser` é `continue-on-error: true` (`research.md:13-16`, `concept.md:15-17`). |
| Feasibility / appetite | strong | Opção A small: mover a compilação para o globalSetup do Playwright, padrão já usado no CI (`concept.md:5-8`). |
| Strategic fit | adequate | Isolamento do setup E2E; sem toque em código de produção ou workflows (`define.md:9-11`). |
| Risk posture | strong | Causa atacada sem alterar o binário; risco de config mitigado por SC-003 e FR-003/FR-005 (`concept.md:8`, `define.md:15-19,21-26`). |

## Verdict & Rationale

Veredito histórico `go (scoped)` de 2026-11-19 preservado como `go`: nenhum critério é `weak`/`unknown` e os artefatos não trazem contradição. O candidato 076 foi eleito pelo 80/20 com `E=(4×5)/(2×2)=5.00`; o leg RBAC depende de maintainer e fica fora (`intake.md:4,11`, `decision anterior:5`). A opção B (aumentar o timeout) mascara o sintoma e mantém o `execFileSync` sem limite superior; a C mantém o problema escondido (`concept.md:11-17`). O binário é síncrono e rápido — a variável é a compilação, não o runtime (`research.md:7-10`). Tempo com target totalmente frio não medido; CI e produção permanecem `[NV]`.

## If needs-clarification

- **Blocking questions**: N/A — nenhuma; a lacuna do tempo com target frio é registrada como `[NV]`, não como bloqueio.
- **Revisit stage**: N/A

## If go — Handoff to `/speckit.specify`

- **Problem**: a compilação de `flora-mint-session` roda dentro da janela de 180s do setup E2E, sem timeout no `execFileSync`, podendo pendurar localmente.
- **Chosen approach**: Opção A — globalSetup do Playwright com pré-build (`cargo build -p flora-core --bin flora-mint-session --features dev-mint`), mantendo `test.setTimeout(180_000)` inalterado (`concept.md:5-8`, `define.md:15-19`).
- **In scope / out of scope**: dentro = config/setup E2E + artefatos do ciclo 076; fora = código de produção (inclusive `flora-core/src/bin/flora-mint-session.rs:47-103`), workflows, ADRs, constituição, `STATE.md`, `specs/076-*`, leg RBAC, issues e DB compartilhado (`concept.md:19-21`, `define.md:9-11`).
- **Success metrics**: FR-001..FR-005 de `define.md:13-19`; SC-001 nenhuma compilação dentro da janela de 180s; SC-002 setup conclui sem pendurar com target frio; SC-003 diff restrito à config/setup E2E + artefatos; SC-004 gates PASS e zero mutação de DB compartilhado (`define.md:21-26`).
- **Carried-forward open questions**: nenhuma bloqueante; tempo com target totalmente frio não medido e CI/produção `[NV]`; leg RBAC depende de maintainer.
