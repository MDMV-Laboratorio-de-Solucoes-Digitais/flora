# Decision: Índice único de documentação na raiz (058)

- **Slug**: 058-root-doc-index
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (anterior) | spec.md (ciclo)

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `test -f index.md` → ausente, embora o Bloco 4 da orquestração o liste; os pontos de entrada existem (056) mas não há índice único que os conecte (intake.md:9-15; research.md:1-2). |
| Evidence strength | strong | `ls -1 *.md` e contagens `find` verificadas; `docs/PROGRESSO.md` e `docs/research/index.md` presentes desde o ciclo 056; gate PASS 8 pisos/50 (intake.md:11-19; research.md:3-5). |
| Value vs. inaction | strong | Inação mantém a descoberta dependente de nomes prévios e o estado verificado invisível para quem começa no README (problem.md:Cost of Inaction). |
| Feasibility / appetite | strong | Appetite small: 1 arquivo novo ≤ 40 linhas, links relativos e verificação por verificador isolado; nenhum código ou gate (concept.md:Option A). |
| Strategic fit | adequate | Ponto de entrada estável sem duplicar conteúdo; ADR-0003/0006 seguem escalados **[NV]** por aprovação de maintainer (decision.md anterior:Delimitadores). |
| Risk posture | strong | Arquivo novo e reversível, links validados; estado remoto/CI e aprovação para corrigir ADRs **[NV]** (decision.md anterior:NV). |

## Verdict & Rationale

**go**, preservando o veredito histórico `go (scoped)` (decision.md anterior:3; spec.md:Status "Approved"). O scorecard justifica a decisão: a ausência do índice é verificada e a opção recomendada apenas aponta para documentos já existentes, sem duplicar conteúdo. Mantém-se o escopo do veredito anterior — 1 arquivo novo, artefatos do ciclo e notas datadas; nenhuma decisão de autorização, CI ou deploy é alterada e nenhum trabalho alheio é tocado (decision.md anterior:Delimitadores). ADR-0003/0006 seguem escalados **[NV]** por exigir aprovação de maintainer; estado remoto/CI permanece **[NV]**. Nenhuma métrica, usuário ou aprovação é inventado além do que os artefatos registram.

## If needs-clarification

- **Blocking questions**: n/a — veredito é `go`; as perguntas sobre alvos extras, âncora e estado remoto não bloqueiam.
- **Revisit stage**: n/a

## If go — Handoff to `/speckit.specify`

- **Problem**: não há um único ponto de entrada na raiz que conecte visão, roadmap, estado verificado, índice de research e ADRs sem exigir conhecimento prévio dos nomes de arquivo.
- **Chosen approach**: Option A — índice curto na raiz com links relativos e uma linha de descrição para cada documento de direção, estado verificado e referência, declarando data/âncora da verificação e a marcação **[NV]** para o estado remoto (concept.md:Recommendation; research.md:Decisão).
- **In scope / out of scope**: in — 1 arquivo novo `index.md`, artefatos do ciclo e notas datadas; out — `README.md`, ADRs, constituição, código, workflows, C5/C6, deploy, criar índices em `docs/` (já existem), listar specs/assessments individualmente, duplicar contagens/resumos e trabalho alheio.
- **Success metrics**: todos os links relativos resolvem para arquivos/diretórios existentes; índice com ≤ 40 linhas; verificador isolado confirma a cobertura dos pontos de entrada; estado verificado e **[NV]** distintos no texto; gates do alvo permanecem verdes (gate PASS 8 pisos/50 com total derivado de `FLOORS`); estado remoto/CI **[NV]**.
- **Carried-forward open questions**: [NEEDS CLARIFICATION: além dos sete alvos listados, algum outro ponto de entrada deve constar no índice?]; [NEEDS CLARIFICATION: qual âncora/data exata imprimir no cabeçalho?]; [NEEDS CLARIFICATION: estado remoto/CI e aprovação para corrigir ADR-0003/0006 — quando serão observados?] **[NV]**.
