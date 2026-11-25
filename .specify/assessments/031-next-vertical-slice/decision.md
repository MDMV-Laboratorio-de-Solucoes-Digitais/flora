# Decision: 031 — Next vertical slice (prontidão de release sem tag)

- **Slug**: 031-next-vertical-slice
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (anterior) | auditoria pós-030 (via intake/research)

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | Versão `0.1.0` repetida nos oito manifests sem fonte única e nenhum mecanismo rastreável de release/SBOM, com zero tags (`intake.md:8`; `research.md:3-4`); ADR ainda registra 443 isenções e e2e NV quando o real é 455 e a lane fechada (`research.md:8`). |
| Evidence strength | adequate | Auditoria pós-030 com e2e-browser verde duas vezes e minter gating VERIFICados; chave `package.*` pontual confirmada localmente, sem-tag seguro; imutabilidade, tag push, dry-run do gerador e `--target` NV (`intake.md:8`; `research.md:3,8`). |
| Value vs. inaction | strong | Sem versão centralizada e inventário, futura publicação dependerá de edição manual em oito manifests e não produzirá rastreabilidade (`problem.md:37-39`). |
| Feasibility / appetite | adequate | Opção A (`medium`): edições de manifests + `release.yml` tag-triggered; capacidade de escrita permanece latente enquanto não houver tag (`concept.md:9-13`). |
| Strategic fit | strong | Converte "Seed verificado" em "Seed publicável" sem autorização de publicação (`research.md:10-11`). |
| Risk posture | adequate | Nenhuma criação/push de tag e nenhuma publicação; sem alterar branch protection/CODEOWNERS/rulesets e sem afirmar imutabilidade sem release (decisão anterior:10,17-19). |

## Verdict & Rationale

Veredito histórico **`go (scoped)` preservado como `go`**. A Opção A cobre as quatro métricas (versão nos oito manifests com `Cargo.lock` consistente, mecanismo tag-only com zero tags, ADR/AGENTS literalmente verdadeiros, CI verde sem publicação); a Opção B deixa a rastreabilidade sem mecanismo e a Opção C cria dependência externa sem autorização (`concept.md:27-29`). A latência da capacidade de release, longe de ser risco realizado, é justamente o controle registrado na decisão original.

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; perguntas abaixo ficam para o handoff).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: Distância entre "Seed verificado" e "Seed publicável com versão e documentação literalmente verdadeiras", sem autorização para publicar (`problem.md:7-9`).
- **Chosen approach**: S1 / Opção A — "Prontidão de release sem tag": centralizar versão e preparar `release.yml` tag-only com SBOM CycloneDX, sem criar tag (`concept.md:9-13,27-29`; decisão anterior:6).
- **In scope / out of scope**:
  - **In**: centralizar versão via `package.version` pontual na raiz e `version.workspace = true` em 7 members (oito manifests), regenerando `Cargo.lock`; adicionar `.github/workflows/release.yml` (somente por tag, draft-first, release imutável, geração CycloneDX 0.5.10 + validação v1_5, assert de versão em lockstep, upload de asset e verify) — **sem criar tag, sem publicar**; fechar desvio documental residual (ADR-0001: contagem de isenções 455 e e2e resolvido, lista de jobs em AGENTS, OBJETIVO/ROADMAP, redação de validação do 030 e banners de research); PR/CI verde, merge/sync e planejar 032 (decisão anterior:8-12).
  - **Out/NV (032)**: allowlist CORS, cargo-vet bloqueante + CODEOWNERS, load/restore, OTLP, criação/release/tag/bump efetivos e dry-run do gerador de SBOM; não alterar proteção de branch/CODEOWNERS/rulesets, não empurrar tags e não afirmar imutabilidade sem release publicada (decisão anterior:14-19; `problem.md:23-28`).
- **Success metrics**: versão centralizada nos oito manifests com `Cargo.lock` regenerado e consistente, mecanismo tag-only presente com zero tags criadas, ADR com 455 isenções e lane e2e resolvida, `AGENTS.md` listando `e2e-browser`, PR/CI verde sem publicação; CI remoto/Forgejo [NV] (`problem.md:30-35`).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: quem detém autorização explícita para criar a primeira tag/publicar?]; [NEEDS CLARIFICATION: política de imutabilidade e opção `--target` na publicação?]; [NEEDS CLARIFICATION: dry-run do gerador de SBOM é necessário antes da primeira tag?]; [NEEDS CLARIFICATION: quem valida o drift em ROADMAP/OBJETIVO/snapshots do 030 sem reintroduzir números obsoletos?] (`problem.md:41-47`; `concept.md:38-46`).
