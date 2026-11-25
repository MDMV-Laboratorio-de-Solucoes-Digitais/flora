# Problem Definition: Prontidão de release e integridade documental do Seed

- **Slug**: 031-next-vertical-slice
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md

## Problem Statement

Após 030, o Seed está verificado, mas não está pronto para publicação: a versão `0.1.0` aparece repetida nos oito manifests do workspace sem uma fonte única (research.md:5) e não há mecanismo rastreável de release com SBOM — existem zero tags hoje (intake.md:8; research.md:3). A documentação de governança também contradiz o estado corrente: ADR-0001 ainda registra 443 isenções e a lane e2e como NV, quando o número atual é 455 e a lane foi fechada; `AGENTS.md` omite o job `e2e-browser` (research.md:8). O problema é a distância entre "Seed verificado" e "Seed publicável com configuração e documentação literalmente verdadeiras", sem que isso autorize publicar: criar tag ou release exige autorização humana explícita (intake.md:13).

## Affected Users & Stakeholders

- **Users**: maintainer/SE que prepara o Seed para distribuição e precisa de versão inequívoca e de um inventário de componentes.
- **Stakeholders**: operador da instalação — depende de documentação confiável para saber o que foi verificado; mantenedores e revisores de governança — precisam de afirmações documentais que correspondam ao estado real do repositório.

## Goals

- Estabelecer uma única fonte de versão para o workspace de modo que manifests e `Cargo.lock` não possam divergir.
- Tornar o Seed publicável com rastreabilidade de componentes sem, por isso, publicar nada.
- Fazer os documentos de governança descreverem fielmente isenções, jobs de CI e o estado da lane e2e.
- Manter verdes os checks required e preservar o trabalho paralelo.

## Non-Goals

- Não criar tag, não publicar release e não empurrar tags; a capacidade de release permanece latente (intake.md:13; decision.md, "Risk controls").
- Não entregar allowlist CORS, promoção de cargo-vet a required, load/restore, OTLP, dry-run do gerador de SBOM ou decisão de `--target` neste ciclo.
- Não alterar proteção de branch, CODEOWNERS ou rulesets sem autorização.
- Não afirmar imutabilidade de release sem que exista release publicada.

## Success Metrics

- Versão centralizada nos oito manifests com `Cargo.lock` regenerado e consistente (baseline: oito manifestos independentes em `0.1.0`).
- Mecanismo de release tag-only presente no repositório com zero tags criadas (baseline: ausente; research.md:3).
- ADR-0001 registra 455 isenções e e2e como lane resolvida, e `AGENTS.md` lista o job `e2e-browser` (baseline: 443 e e2e NV; job omitido).
- PR/CI permanece verde nos checks required, sem publicação (baseline: verde); resultado de CI remoto e Forgejo [NV].

## Cost of Inaction

Sem fonte única de versão e sem rastreabilidade de componentes, uma futura publicação dependerá de edição manual em oito manifests e não produzirá inventário confiável; a documentação continuará a induzir auditores a erro sobre isenções e checks. O custo é risco de release inconsistente e perda de confiança na governança, não um ganho financeiro medido [NV].

## Open Questions

- [NEEDS CLARIFICATION: quem detém a autorização explícita para criar a primeira tag e publicar um release?]
- [NEEDS CLARIFICATION: qual política de imutabilidade de release e qual opção `--target` devem ser consideradas quando a publicação for autorizada?]
- [NEEDS CLARIFICATION: um dry-run do gerador de SBOM é necessário antes da primeira tag, ou basta a validação do artifact em CI?]
- [NEEDS CLARIFICATION: quem valida que a correção de drift documental cobre `ROADMAP`, `OBJETIVO` e os snapshots de 030 sem reintroduzir números obsoletos?]
