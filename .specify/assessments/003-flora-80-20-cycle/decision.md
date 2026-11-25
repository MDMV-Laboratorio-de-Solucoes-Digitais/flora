# Decision: 003 — Flora 80/20 Cycle (Baseline, PKCE, Direction)

- **Slug**: 003-flora-80-20-cycle
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (original 2026-11-15) | baseline.md | specs/003-flora-80-20-baseline/

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | Specs 001/002 marcavam convergência ampla, mas runtime não evidenciado, T116 (PKCE) aberta e `apps/frontend`/`apps/web` com canônico ambíguo (`intake.md:8`, `research.md:26`, `problem.md:9`). |
| Evidence strength | adequate | Pesquisa multilíngue com fontes primárias (Macro AGPL-3.0, REA, Craft-Anything) e baseline local: clippy workspace PASS, `cargo test --workspace` 90 passed/0 failed/1 ignored; frontend `check`/`build` ok, lint e browser NV (`research.md:7-29`, `baseline.md:9-24`). |
| Value vs. inaction | strong | Provar toolchain e fechar T116 entrega confiança/segurança antes de novas features; RE como eixo só acrescentaria risco legal/clean-room sem informação nova (`problem.md:9,11`, `research.md:18-19`). |
| Feasibility / appetite | adequate | Opção B (`medium`): estabilizar, fechar PKCE e reconciliar; artefatos/planos primeiro, implementação gateada por evidência (`concept.md:16-21`; decisão original:15-17,25). |
| Strategic fit | strong | VPS-first debloat, monólito modular trait-driven, spec/test-first e Svelte 5/Tauri; Macro AGPL é a via autoritativa, REA/Craft-Anything só referência tática (`research.md:9,19,23`; decisão original:13). |
| Risk posture | adequate | Sem RE não autorizada, sem reuso AGPL-incompatível e sem "green" não executado; lacunas PKCE (sem callback mock completo, sem teste de state expirado, fallback in-memory) registradas como NV (`baseline.md:30-31`; `intake.md:12`). |

## Verdict & Rationale

Veredito histórico **`go (narrow, autonomous cycle)` preservado como `go`**: nada nos artefatos contradiz a decisão de 2026-11-15. O scorecard sustenta o recorte 80/20 — estabilizar o baseline, fechar T116 (PKCE) e reconciliar frontend/CI/docs, com `specs/003-flora-80-20-baseline/` como entrega do ciclo (decisão original:7,25). Registra-se explicitamente **não** adotar REA (`morluto/rea`) nem Craft-Anything (`storytold/craft-anything`) como dependências ou estratégia central: o Macro é AGPL-3.0 de código aberto, logo fonte/docs são a via limpa; ambas ficam, no máximo, como referência tática sob uso legal/autorizado (`research.md:17-23`; decisão original:12). Onde o ambiente não executou runtime (browser tests com ENOSPC, lint pós-format, CI remoto), o resultado permanece NV, não "verde" presumido (`baseline.md:19-21`).

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; incógnitas abaixo transportadas, não bloqueantes).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: Ciclo sem critério verificável de priorização: specs/checklists "prontos" sem runtime evidenciado, T116 (OIDC PKCE) aberta e canônico frontend/CI ambíguo (`problem.md:9`).
- **Chosen approach**: Opção B — estabilizar, fechar PKCE e reconciliar direção (`concept.md:16-20,32`); Opção C (engenharia reversa como via estratégica) rejeitada por risco legal/ToS, contaminação de clean room e baixa validade (`concept.md:23-28`).
- **In scope / out of scope**:
  - **In (decisão original:15-17,25)**: assessment (intake/research/decision) e pacote `specs/003-flora-80-20-baseline/` — verificação de baseline, completude PKCE, reconciliação frontend/CI/docs e guardrails de escopo informados pelo Macro; nesta etapa, mudanças documentais/planejamento, implementação gateada por verificação.
  - **Out**: deploys/infra de produção, segredos, merges/releases, serviços pagos/proprietários; RE não autorizada e reuso AGPL-incompatível; reescritas amplas e "green" sem execução observável (decisão original:19-22).
- **Success metrics**: comandos determinísticos com evidência (baseline: parcial — `cargo fmt --all -- --check` falhou em `flora-api/src/routes/auth.rs:75` e foi corrigido; clippy workspace PASS; test workspace 90/0/1 em 19 suítes; `apps/frontend` `check`/`build` ok, lint pós-format inconclusivo e browser tests FAIL por ENOSPC — `baseline.md:9-22`); T116 aberta→implementada com state+verifier+S256, transação single-use TTL 600s e state consumido antes do token exchange, com gaps de teste NV (`baseline.md:26-31`); canônico `apps/frontend` definido, `apps/web` não canônico, CI em `apps/frontend` (`baseline.md:16,23-24`).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: ambiente/autorização para CI remoto e integrações externas?]; [NEEDS CLARIFICATION: o aceite da T116 inclui casos de falha/state/nonce com evidência?]; [NEEDS CLARIFICATION: REA/Craft-Anything apenas referência tática para artefatos opacos autorizados ou avaliação separada?] (`problem.md:46-48`).
