# Decision: 026 — Next vertical slice (gate de verificação executável)

- **Slug**: 026-next-vertical-slice
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (anterior) | ADR-0001 e cycles 023–025 (via intake/research)

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | A promessa "Seed v0.1 verificado" dependia de prosa: CI sem fmt/clippy/cargo-deny, ciclo 025 não finalizado, constituição em conflito (`research.md:3`; `problem.md:9-13`). |
| Evidence strength | adequate | Auditoria de verificação e execução local de `cargo deny` (exit 4 em licenses; correção de uma linha → exit 0; `bans`/`sources` já passam), com disponibilidade remota de `opentelemetry-otlp 0.22` NV (`research.md:4-6`). |
| Value vs. inaction | strong | Sem gate executável, o rótulo "verificado" pode ser reivindicado sem formatação, lint restrito ou checagem de licenças (`problem.md:40-42`). |
| Feasibility / appetite | adequate | Opção A (`medium`) com correção de regra, três passos de CI e ajustes documentais; risco de conflito com edições paralelas (`concept.md:9-13`). |
| Strategic fit | strong | Maior valor por esforço para tornar o ADR-0001 executável, fechar contradições de autoridade e remover armadilha de dependência (`research.md:10-11`). |
| Risk posture | adequate | Advisories com rede RustSec e cargo-vet bloqueante ficam fora; nenhum serviço novo e trabalho paralelo preservado (`research.md:4-5`; decisão anterior:20-22). |

## Verdict & Rationale

Veredito histórico **`go (scoped)` preservado como `go`**. Os artefatos confirmam que o corte S1 transforma o gate do ADR-0001 em controle de merge com custo baixo e evidência local já observada; a opção B deixaria o rótulo dependente de interpretação e a opção C não seria determinística em checkout limpo (`concept.md:27-29`). Nenhum achado novo contradiz a decisão anterior.

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; as perguntas abaixo não impedem o handoff).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: Gate de verificação do Seed não executável: CI sem fmt/clippy/cargo-deny, licença `AGPL-3.0-or-later` rejeitada, documentos normativos divergentes e dependência declarada inexistente (`problem.md:7-13`).
- **Chosen approach**: S1 / Opção A — "Gate executável e documentos honestos" (`concept.md:9-14,27-29`; decisão anterior:6-7).
- **In scope / out of scope**:
  - **In**: corrigir `deny.toml` para permitir `AGPL-3.0-or-later` e verificar `cargo deny check bans licenses sources` localmente; adicionar `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings` e cargo-deny a `.github/workflows/rust.yml`; finalizar evidência do ciclo 025 (tasks/validate/release com resultado real do PR #231); emendar constituição para redação de milestone/FASE mechanism-neutral com bump de versão, corrigir linha do cycle-025 no ROADMAP, notas obsoletas em AGENTS.md e cabeçalhos de contrato Seed onde praticável; remover declarações fantasma de OTel de `Cargo.toml` documentando NV/follow-up; registrar R1 e atualizações de planejamento, PR/CI verde, merge/sync (decisão anterior:8-14).
  - **Out/NV**: store/imports/exemptions do cargo-vet, job de advisories com rede RustSec e SBOM; aposentadoria do scaffolding PGMQ/Valkey, browser e2e (depende de JWT fresco), Tauri, exporter OTLP completo, validação de carga/restauração em 2vCPU/4GB e paridade Meilisearch; non-goals: novos serviços, mudanças no frontend e reverter trabalho paralelo (decisão anterior:19-22).
- **Success metrics**: fmt/clippy restrito/cargo-deny executando na CI; licenças sem erro; documentos de autoridade sem contradição com ADR-0001; nenhuma dependência apontando para versão inexistente; artefatos do ciclo 025 finalizados com PR real (`problem.md:32-38`).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: job de advisories informativo ou impeditivo?]; [NEEDS CLARIFICATION: qual versão da constituição é normativa após a emenda e como referenciar anteriores?]; [NEEDS CLARIFICATION: isenção `AGPL-3.0-or-later` global ou por crate?]; [NEEDS CLARIFICATION: destino de `TraceLayer`/OTLP após remoção das declarações fantasma?] (`problem.md:44-49`).
