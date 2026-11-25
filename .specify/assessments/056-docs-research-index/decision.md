# Decision: Entrada verificada para o corpus de research (056)

- **Slug**: 056-docs-research-index
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | define.md (equivalente local; problem.md/concept.md ausentes no baseline) | shape.md | decision.md (anterior) | spec.md (ciclo)

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `docs/research/` tem 135 arquivos sem índice e não há `docs/PROGRESSO.md`; recuperação depende de `find`/`grep` ad hoc (intake.md:9-13; research.md:1-4). |
| Evidence strength | strong | `find`/`ls` determinísticos (135 arquivos, convenção `YYYY-MM-DD[-NNN]-slug.md`), ausência confirmada por `test -f` e gate local PASS (intake.md:15-19; research.md:1-4). |
| Value vs. inaction | strong | Inação mantém a descoberta ad hoc e nenhum registro datado de estado verificado do repositório (define.md:Problema). |
| Feasibility / appetite | strong | Dois arquivos novos gerados por script determinístico; risco de drift futuro declarado no próprio índice (shape.md:Risco). |
| Strategic fit | adequate | Fecha dívida documental sem tocar código, CI ou ADRs; ADR-0003 segue escalado **[NV]** por exigir aprovação de maintainer (decision.md anterior:Delimitadores). |
| Risk posture | strong | Nenhum código/gate afetado, reversível por `git revert`; qualquer afirmação sobre estado remoto/CI permanece **[NV]** — apenas evidência local usada (decision.md anterior:NV). |

## Verdict & Rationale

**go**, preservando o veredito histórico `go (scoped)` (decision.md anterior:3; spec.md:Status "Approved"). O scorecard justifica a decisão: a ausência dos pontos de entrada é verificada por contagens determinísticas e a geração automatizada evita omissão e drift imediato. Mantém-se o escopo do veredito anterior — 2 arquivos novos em `docs/`, artefatos do ciclo e notas datadas; nenhuma decisão de autorização, CI ou deploy é alterada e nenhum trabalho alheio é tocado (decision.md anterior:Delimitadores). ADR-0003 continua escalado **[NV]** por exigir aprovação de maintainer; qualquer afirmação remota segue **[NV]**, pois nesta avaliação apenas evidência local foi usada. Não havia `problem.md`/`concept.md` no baseline; `define.md` e `shape.md` servem como equivalente local.

## If needs-clarification

- **Blocking questions**: n/a — veredito é `go`; a aprovação de maintainer para ADR-0003 é escalonamento, não bloqueio deste ciclo.
- **Revisit stage**: n/a

## If go — Handoff to `/speckit.specify`

- **Problem**: o corpus de 135 documentos de research não tem índice e o repositório não tem um `PROGRESSO.md` com estado verificado.
- **Chosen approach**: gerar `docs/research/index.md` deterministicamente dos nomes de arquivo e do primeiro cabeçalho `# ` de cada documento, agrupados por data, e `docs/PROGRESSO.md` como nota datada de estado verificado no padrão append-only de `OBJETIVO.md`/`ROADMAP.md` (research.md:Decisão; shape.md:Forma escolhida).
- **In scope / out of scope**: in — 2 arquivos novos em `docs/`, artefatos do ciclo e notas datadas; out — ADR-0003, constituição, código, workflows, testes, C5/C6, deploy, `index.md` na raiz, inclusão de ADRs/agents no índice e trabalho alheio.
- **Success metrics**: entradas do índice = `find docs/research -type f | wc -l` = 135; todas as entradas são links relativos que resolvem; verificação independente (subagent) confirma cobertura integral; gates determinísticos do alvo permanecem verdes; estado remoto/CI **[NV]**.
- **Carried-forward open questions**: [NEEDS CLARIFICATION: ADR-0003 escalado por aprovação de maintainer — quando será tratado?] **[NV]**; qualquer afirmação sobre estado remoto/CI permanece **[NV]** — apenas evidência local foi usada nesta sessão.
