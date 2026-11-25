# Decision: Varredura OSV com ref que resolve (052)

- **Slug**: 052-osv-scan-ref-pin
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (anterior) | spec.md (ciclo)

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `.github/workflows/osv-scan.yml:14` referencia `@v2`; `git ls-remote` ao vivo não encontra `refs/heads/v2` nem `refs/tags/v2`, então o scan semanal não inicia (intake.md:9-13; research.md:5-9). |
| Evidence strength | strong | Três fontes independentes: `git ls-remote` (`v2.7.1` em `32d3cd4…`), documentação oficial do action e YAML do workflow no tag, com inputs `scan-args`/`fail-on-vuln` conferidos (research.md:5-13). |
| Value vs. inaction | strong | Inação mantém o sinal advisory de supply-chain silenciosamente ausente sem que o gate autoritativo mude de cor (problem.md:Affected Stakeholders; ADR-0003:23). |
| Feasibility / appetite | strong | Appetite small: troca de uma linha; SHA completo, `v2.6.0` e remover o job foram descartados com razão (concept.md:Options; research.md:Decisão). |
| Strategic fit | adequate | Restaura o job advisory sem promovê-lo a required check, preservando ADR-0003 e os 4 checks obrigatórios (define.md:Non-Goals). |
| Risk posture | strong | Diff de 1 linha, reversível por `git revert`; execução real do job no GitHub Actions após o pin **[NV]** (decision.md anterior:NV). |

## Verdict & Rationale

**go**, preservando o veredito histórico `go (scoped)` (decision.md anterior:3; spec.md:Status "Approved"). O scorecard é favorável: o ref inexistente é demonstrado por evidência primária remota e a correção é a menor verificada entre as opções. Mantém-se o escopo estrito — um arquivo de código existente, uma linha, artefatos do ciclo e notas datadas append-only; nenhuma decisão de autorização, CI obrigatória ou deploy é tomada, e nenhum PR/worktree/stash alheio é tocado (decision.md anterior:Delimitadores). A execução real do job segue como item a observar no próximo agendamento semanal ou via `workflow_dispatch` **[NV]**.

## If needs-clarification

- **Blocking questions**: n/a — veredito é `go`; a validação remota do job é acompanhamento, não bloqueio.
- **Revisit stage**: n/a

## If go — Handoff to `/speckit.specify`

- **Problem**: o workflow semanal de OSV scan aponta para `@v2`, um ref que não resolve no action, e por isso o sinal advisory de vulnerabilidades de dependências não é produzido.
- **Chosen approach**: Option A — fixar no tag documentado: trocar `@v2` por `@v2.7.1`, existente e recomendado pela documentação oficial, mantendo os parâmetros atuais do job (concept.md:Recommendation; research.md:Decisão).
- **In scope / out of scope**: in — `.github/workflows/osv-scan.yml` (1 linha), artefatos do ciclo e notas datadas append-only; out — gate autoritativo, ADR-0003, constituição, 4 required checks, `fail-on-vuln`, agendamento, permissões, Dependabot, pin por SHA, `v2.6.0`, remover/promover o job, outros workflows, RBAC/RLS, merge/deploy e PRs/worktrees/stashes alheios.
- **Success metrics**: `grep -n 'reusable.yml@' .github/workflows/osv-scan.yml` → `v2.7.1`; `check_gate_integrity.py` → PASS com 4 required checks e pisos intactos; diff de código = 1 linha em 1 arquivo; gates Rust e frontend verdes; execução real do job no GitHub Actions **[NV]**.
- **Carried-forward open questions**: [NEEDS CLARIFICATION: validação por `workflow_dispatch` ou próximo agendamento semanal?] **[NV]**; [NEEDS CLARIFICATION: `v2.7.1` continua sendo a convenção documentada na próxima atualização?]; [NEEDS CLARIFICATION: em qual PR o resultado da execução remota será registrado?] **[NV]**.
