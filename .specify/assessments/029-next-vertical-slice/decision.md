# Decision: 029 — Next vertical slice (integridade do Seed)

- **Slug**: 029-next-vertical-slice
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (anterior) | auditoria pós-028 (via intake/research)

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | Pós-028: clippy `--all-targets` NV, diferimento do Tauri sem ADR, novo desvio documental e smoke nunca executado (`intake.md:8`; `problem.md:9-13`). |
| Evidence strength | adequate | Auditoria pós-028 com CI/seis suítes DB/artefatos VERIFICados, sete `expect_used` contados em testes e runbook do smoke com variáveis/override identificadas; execução do smoke bloqueada por portas 3000/5432 (`research.md:3-8`). |
| Value vs. inaction | strong | Sem exceções visíveis e ADR formal, o diferimento volta a cada ciclo e a última gate do ADR permanece sem evidência (`problem.md:40-42`). |
| Feasibility / appetite | adequate | Opção A (`medium`): anotações em nível de módulo com precedente em `extractors/auth.rs`, adoção de rascunho coerente e correções documentais; tentativa de smoke depende de portas isoladas (`research.md:3-5`). |
| Strategic fit | strong | Maior integridade por esforço: fecha quatro NVs sem alterar comportamento da aplicação (`research.md:10-11`). |
| Risk posture | adequate | Sem edição de corpos de teste, sem acesso a produção, sem reverter trabalho paralelo e smoke não vira gate de merge (decisão anterior:18-19; `concept.md:31-35`). |

## Verdict & Rationale

Veredito histórico **`go (scoped)` preservado como `go`**. A Opção A atende às cinco métricas do problema (lint `--all-targets`, sete exceções visíveis, ADR-0002 adotado, docs coerentes, smoke com rc=0 ou NV explícito); a Opção B deixa dois NVs e a Opção C não produz evidência executável (`concept.md:27-29`). A incerteza sobre portas e a eventual saída NV do smoke já estavam previstas na decisão anterior, logo não contradizem o `go`.

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; perguntas abaixo transportadas).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: Integridade do Seed parcialmente não executada: lint estrito exclui testes, Tauri não formalizado, documentos do 028 divergentes e smoke sem evidência (`problem.md:7-13`).
- **Chosen approach**: S1 / Opção A — "Fechamento da integridade" (`concept.md:9-13,27-29`; decisão anterior:6).
- **In scope / out of scope**:
  - **In**: `#[expect(clippy::expect_used, reason = "...")]` em nível de módulo nos testes de `health.rs` (4) e `oidc.rs` (3), promover CI clippy de `--lib --bins` para `--all-targets` e verificar localmente/na CI; adotar ADR-0002 (Tauri deferido, Accepted) e atualizar Consequências do ADR-0001; fechar desvio do 028 (README:34/74, AGENTS:7-8, ADR-0001:45, banners de supersession nos docs de research 028); tentar smoke integrado em portas isoladas conforme runbook e registrar rc=0 ou NV explícito com bloqueio; PR/CI verde, merge/sync e planejar 030 (decisão anterior:8-13).
  - **Out/NV (030)**: browser e2e com `retries: 0`, release/tag + SBOM, cargo-vet bloqueante/CODEOWNERS, OTLP, load/restore 2vCPU/4GB. Não editar corpos de teste, reverter trabalho paralelo, acessar produção ou alterar comportamento; usar `-p flora-smoke-alt` para qualquer stack local (decisão anterior:15-19).
- **Success metrics**: CI com `--all-targets`; sete exceções registradas em nível de módulo com corpos inalterados; ADR-0002 adotado e ADR-0001 atualizado; README/AGENTS/ADR/snapshots sem contradição; smoke com evidência de sucesso ou NV com bloqueio identificado (`problem.md:32-38`).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: quais portas isoladas e nome de projeto usar no smoke?]; [NEEDS CLARIFICATION: bloqueio de portas deve ser NV mesmo com runbook disponível?]; [NEEDS CLARIFICATION: adoção do ADR-0002 exige aprovação adicional?]; [NEEDS CLARIFICATION: qual versão do README prevalece?]; [NEEDS CLARIFICATION: `retries: 0` é definitivo ou temporário diante da rotação de `jti`?] (`problem.md:44-49`).
