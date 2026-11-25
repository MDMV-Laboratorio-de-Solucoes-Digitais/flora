# Decision: 068 — refresh verificado de documentação

- **Slug**: 068-verified-docs-refresh
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | define.md (problem equivalente) | shape.md (concept equivalente) | decision.md (anterior) | specs/068-verified-docs-refresh/converge.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `PROGRESSO.md`, `research/index.md`, `OBJETIVO.md` e `ROADMAP.md` divergem do filesystem em até 21 ciclos e seis contagens; `rbac.rs:4-5` chama "placeholders" handlers já reais desde 066 (intake.md:9-21; research.md:1-5). |
| Evidence strength | strong | `find` determinístico (58/280 specs, 55 assessments, 54 validations, 54 releases, 4 bugs, 137 research, 6 adr), `gh` observado ao vivo, gate `--remote` PASS; `analyze` encontrou 1 conflito e foi corrigido; verificador PASS (intake.md:23-33; converge.md). |
| Value vs. inaction | strong | Restaura confiança no "estado verificado" e evita decisões baseadas em cifras obsoletas; E=5,00 (intake.md:37). |
| Feasibility / appetite | adequate | Cinco arquivos com diff mínimo e auditável, mas exige disciplina append-only e cuidado semântico na tabela do índice; indexação retroativa 047–067 (E=3,75) fica de fora. |
| Strategic fit | strong | Sweep de verdade documental dentro das fronteiras de autorização: nenhum workflow, ADR, constituição, required check, migração, deploy ou produção (define.md FR-001–FR-005). |
| Risk posture | strong | Só documentação e um comentário, zero comportamento; merge deliberado após CI verde observada ao vivo, sem auto-merge/force-push; futuro remoto/Forgejo/produção **[NV]**. |

## Verdict & Rationale

Veredito histórico **`go (scoped)` preservado**: não era necessária autorização adicional — trata-se de correção de estado verificado com evidência determinística local e de um comentário contradito pelo próprio código (converge.md). A emenda da constituição, que pontuaria 7,50, foi **escalada [NV]** por exigir aprovação de maintainer (Constituição §Governance) e não foi executada. Âncora `dev@cbad031b5` (= `github/dev@cbad031b5`, verificado ao vivo), branch `feature/068-verified-docs-refresh`; factory fonte `c81f0f81…`. Nenhuma linha histórica de `PROGRESSO.md`, `OBJETIVO.md`, `ROADMAP.md` ou `STATE.md` foi reescrita; em `rbac.rs` só o bloco `//!` foi tocado.

## If needs-clarification

- **Blocking questions**: N/A — nenhuma pergunta bloqueia o avanço; veredito é `go`.
- **Revisit stage**: N/A

## If go — Handoff to `/speckit.specify`

- **Problem**: Retratos documentais do estado do repositório divergem do filesystem em até 21 ciclos e seis contagens, e um comentário descreve handlers que já são reais.
- **Chosen approach**: Sweep em cinco arquivos, um commit por arquivo — atualizar cabeçalho/tabela de `docs/research/index.md` preservando a dívida 047+ em nota append-only; nova seção datada em `docs/PROGRESSO.md`; notas em `OBJETIVO.md`/`ROADMAP.md`; corrigir só o bloco `//!` de `rbac.rs`; apêndice em `STATE.md` (`shape.md`).
- **In scope / out of scope**: 5 arquivos de documentação/comentário + artefatos do ciclo; fora indexação retroativa 047–067, emenda da constituição, contrato RBAC vácuo, stub `AuthService`, workflows, ADRs, migrations, required checks, deploy e qualquer merge sem CI verde observada ao vivo.
- **Success metrics**: contagens publicadas coincidem com `find` (58/280, 55, 54, 54, 4, 137, 6); `check_gate_integrity.py` PASS local e `--remote` e gates Rust/frontend exit 0; `git diff` limitado aos paths declarados, sem mudança em workflows/migrations/ADRs/constituição/produção (define.md SC-001–SC-003).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: emenda da constituição exige aprovação de maintainer — **[NV]**]; [NEEDS CLARIFICATION: CI futura, branch protection em mutação, Forgejo, produção e aprovações — **[NV]**, reconsultar ao vivo antes do merge]; [NEEDS CLARIFICATION: indexação retroativa de 047–067 (E=3,75) em qual ciclo].
