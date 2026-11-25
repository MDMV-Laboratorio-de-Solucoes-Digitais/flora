# Decision: 028 — Next vertical slice (fecho não-bloqueante da FASE 0)

- **Slug**: 028-next-vertical-slice
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (anterior) | auditoria pós-027 (via intake/research)

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | Store do cargo-vet ignorado pelo Git, `multer 3.1.0` sem isenção, JWT estático expirado em 2026-11-16 e desvio residual de docs (`intake.md:8`; `research.md:3,7`). |
| Evidence strength | adequate | Auditoria pós-027 com gates VERIFICADOS exceto smoke, repo público com Dependabot ativo (83 alertas, 4 no `Cargo.lock` raiz) e planos R1 GO; execução local do smoke NV por portas 3000/5432 ocupadas (`research.md:3-8`). |
| Value vs. inaction | strong | Sem store versionado a FASE 0 some em checkout limpo; sem sinais independentes os 83 alertas não têm triagem comum e a última gate do ADR fica sem execução (`problem.md:39-42`). |
| Feasibility / appetite | adequate | Opção A (`medium`): uma isenção, job informativo, review de PR e varredura semanal em repo público; nada bloqueia merges (`research.md:3-4`). |
| Strategic fit | strong | Fecha o último item literal do marco FASE 0 sem novos serviços nem required checks (`research.md:9-11`). |
| Risk posture | adequate | `continue-on-error: true` em cargo-vet, advisory não-bloqueante e nenhuma credencial durável; emissão determinística em vez de token fixo (`research.md:3-4`; `concept.md:9-13`). |

## Verdict & Rationale

Veredito histórico **`go (scoped)` preservado como `go`**. Os planos R1 são GO para a Fatia A e o scorecard confirma que só a Opção A cobre store versionado, revisão de dependências, varredura OSV, smoke sem segredo vencido e docs coerentes; a Opção C criaria segredo durável e contrariaria evidência reproduzível (`concept.md:27-29`). O smoke local NV e o e2e diferido para 029 são limites registrados, não mudança de veredito.

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; incógnitas abaixo são carried-forward).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: FASE 0 e verificação do Smoke sem fecho executável: evidência de supply-chain não versionada, ausência de sinal advisory independente, JWT expirado e contradições ADR/README/snapshots (`problem.md:7-13`).
- **Chosen approach**: S1 / Opção A — "Fecho não-bloqueante da FASE 0" (`concept.md:9-13,27-29`; decisão anterior:6).
- **In scope / out of scope**:
  - **In**: versionar `supply-chain/` (force-add apesar do .gitignore), adicionar exatamente a isenção `multer 3.1.0` e job informativo `cargo vet --locked --no-minimize-exemptions` no CI (`continue-on-error: true`), sem auditoria em massa nem bloqueio; adicionar PR dependency-review e uma varredura semanal OSV-Scanner (ambas não-bloqueantes, repo público, $0); substituir o JWT estático expirado em `docs/dev-smoke.md` por comando de emissão determinístico e registrar por que a execução local do smoke é NV (portas 3000/5432 ocupadas); fechar desvio residual em ADR-0001 (observabilidade/consequências), README (linha FASE/infra), banners de status nos snapshots de research 027 e caminho obsoleto de crate no plan de specs/001; PR/CI verde, merge/sync e planejar 029 (decisão anterior:8-13).
  - **Out/NV (029)**: browser e2e com minter feature-gated, SBOM de release (sem processo de tag), ADR-0002 do Tauri, cargo-vet bloqueante + CODEOWNERS, OTLP, validação 2vCPU/4GB load/restore e clippy estrito `--all-targets`; também fora: novos serviços, reverter trabalho paralelo e checks que bloqueiem merges (decisão anterior:15-16).
- **Success metrics**: store versionado com checagem `--locked` passando, revisão de dependências em PR e varredura OSV semanal ambas não-bloqueantes, smoke sem JWT expirado com emissão determinística, ADR/README/snapshots sem contradição (`problem.md:32-37`).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: janela de observação antes de cargo-vet bloqueante?]; [NEEDS CLARIFICATION: criar required checks/branch protection neste marco?]; [NEEDS CLARIFICATION: critério de triagem dos 83 alertas (4 no `Cargo.lock` raiz)?]; [NEEDS CLARIFICATION: desativar license-check na revisão é aceitável como política?]; [NEEDS CLARIFICATION: qual porta isolada usar para nova tentativa de smoke?] (`problem.md:43-49`).
