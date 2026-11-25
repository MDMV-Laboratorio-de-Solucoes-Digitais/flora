# Decision: 027 — Next vertical slice (fecho honesto do Seed)

- **Slug**: 027-next-vertical-slice
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (anterior) | ADR-0001 e cycles 023–026 (via intake/research)

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `flora-notifications` sem consumidores em Rust e `NotificationsConfig` nunca lida, enquanto docs anunciam PGMQ/Valkey como transporte (`research.md:3,5`; `problem.md:9-11`). |
| Evidence strength | adequate | R1 de 6 subagentes com contagem de 9 sítios de construção de `NotificationService`, feature `trace` habilitada sem anexação e diff de lockfile delimitado; exporter OTLP e oráculo de advisories NV (`research.md:3-8`). |
| Value vs. inaction | strong | Código e docs anunciam mecanismos que não executam; o caminho XADD é armadilha latente contra a dedup de 022 (`problem.md:39-41`). |
| Feasibility / appetite | adequate | Opção A (`medium`): remoção mecânica + uma linha de `TraceLayer` + reconciliação pontual, com risco concentrado em workspace/lockfile (`concept.md:9-13`). |
| Strategic fit | strong | Faz o Seed anunciar só o que roda, preservando Postgres-first e o perfil opcional `realtime` (`research.md:3,10-11`). |
| Risk posture | adequate | Controles explícitos: não tocar `NotificationService`, dedup Postgres nem perfil Valkey; verificar diff do `Cargo.lock` exatamente de uma stanza (decisão anterior:17-19). |

## Verdict & Rationale

Veredito histórico **`go (scoped)` preservado como `go`**. A Opção A é a única que fecha o conjunto de métricas (nenhuma referência a transporte morto, tracing anexado, docs sem afirmações superadas, lockfile com diff controlado); a Opção B manteria a armadilha e a Opção C introduziria serviços proibidos (`concept.md:27-29`). Os NVs listados para 028 não alteram o recorte.

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; perguntas abaixo ficam para o handoff).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: Base de código e documentação anunciam transporte de notificações e observabilidade que não executam, reabrindo decisões resolvidas (`problem.md:7-13`).
- **Chosen approach**: S1 / Opção A — "Fecho honesto do Seed": remover o crate não consumido, anexar `TraceLayer::new_for_http()` e reconciliar ADR/README/planning/specs (`concept.md:9-11,27-29`; decisão anterior:6).
- **In scope / out of scope**:
  - **In**: remover `flora-notifications` do workspace (member, workspace dep e diretório), o módulo `NotificationsConfig` e seu wiring em `config/mod.rs`, regenerando `Cargo.lock` (uma stanza); anexar `TraceLayer::new_for_http()` ao roteador da API (feature já habilitada, sem exporter/coletor); reconciliar ADR-0001 (FASE 0 CI + decisão de transporte resolvidos), README:38/70, ROADMAP/OBJETIVO (versão da constituição), specs/001 (afirmações Valkey), status de specs/026, lista de crates em AGENTS e nota de realtime em dev-smoke; verificação sem `pgmq|XADD|flora_notifications` em `.rs`/`.toml`, fmt, clippy estrito, cargo-deny, check, unit + seis suítes DB, CI verde, PR/merge/sync (decisão anterior:8-12).
  - **Out/NV (028)**: store/imports do cargo-vet + gate; browser e2e com JWT fresco; Dependabot/OSV; SBOM de release; exporter OTLP; Tauri; validação 2vCPU/4GB. Não atingir `NotificationService`, a dedup Postgres nem o perfil `realtime`; não reverter trabalho paralelo (decisão anterior:14-19).
- **Success metrics**: nenhuma referência a `pgmq`/`XADD`/`flora_notifications` em `.rs`/`.toml`, roteador com tracing por requisição, documentos sem afirmações superadas e `Cargo.lock` com diff de uma stanza (`problem.md:32-37`).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: perfil `realtime` com Valkey é parte do Seed ou explicitamente opcional?]; [NEEDS CLARIFICATION: quem valida a redação final do ADR-0001?]; [NEEDS CLARIFICATION: `TraceLayer` condicionado a config ou sempre anexado?]; [NEEDS CLARIFICATION: convenção para tarefas de specs que marcam work não conectado como concluído?] (`problem.md:43-49`).
