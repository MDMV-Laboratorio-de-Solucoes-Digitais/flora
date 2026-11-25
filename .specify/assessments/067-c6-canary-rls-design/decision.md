# Decision: 067 — C6 canário RLS

- **Slug**: 067-c6-canary-rls-design
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | define.md (problem equivalente) | shape.md (concept equivalente) | decision.md (anterior) | specs/067-c6-canary-rls-design/converge.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | 11 tabelas com `ENABLE ROW LEVEL SECURITY` (`init_schema.sql:196-215`), mas 0 policies ativas, 0 `set_config`, 0 papéis de DB e 0 wrapper de transação — isolamento depende só do filtro da app (intake.md:9-13; research.md:1-8). |
| Evidence strength | strong | Auditoria local com `file:line`, ADR-0005/0006 e gate 8 pisos/51; PR #285 merged com 6 checks verdes observado ao vivo; verificador; execução em app/produção **[NV]** (intake.md:19-21; converge.md). |
| Value vs. inaction | strong | Último item autorizado da tabela de escalonamento (E=8,33); sem desenho de canário com rollback, a promoção de RLS em produção é insegura (intake.md:29). |
| Feasibility / appetite | adequate | Neste ciclo é só um documento, mas o desenho é denso (5 fases, SQL de referência, matriz (a)–(d), rollback em 6 passos) e qualquer execução futura exige autorização SE (ADR-0006 §3). |
| Strategic fit | strong | Fecha C6 dentro da autorização prévia do SE e empurra runtime para aprovação explícita, sem substituir os filtros da aplicação (define.md FR-001–FR-007). |
| Risk posture | strong | Risco operacional nulo neste ciclo: zero arquivos de runtime/workflow/migration/ADR, nenhuma credencial e nenhum `FORCE`/`CREATE POLICY` aplicado; risco do desenho futuro documentado; CI remota **[NV]**. |

## Verdict & Rationale

Veredito histórico **`go (scoped — autorização SE prévia em STATE.md 13:04Z / 14:38Z / 14:50Z)` preservado**: a autorização cobria **documentar** o canário, não executá-lo. O entregável foi `docs/research/2026-11-18-067-c6-canary-rls-design.md` com estado verificado, pré-condição ADR-0006(a) descrita e não implementada, fases em DB descartável, SQL de referência, ressalvas de `users`/`organizations`, matriz (a)–(d) **[NV]** e rollback sem dado destrutivo; refresh append-only de `docs/PROGRESSO.md` e `docs/research/index.md`. Âncora `github/dev@9d5f6656f`, branch `feature/067-c6-canary-rls-design`; factory lida, nunca modificada. Qualquer Fase 2–4, deploy, credencial ou mudança de required checks continua exigindo autorização explícita do SE (ADR-0006 §3).

## If needs-clarification

- **Blocking questions**: N/A — nenhuma pergunta bloqueia o avanço; veredito é `go` e a autorização SE para documentar está registrada.
- **Revisit stage**: N/A

## If go — Handoff to `/speckit.specify`

- **Problem**: RLS está ligado em 11 tabelas, porém inerte; a promoção depende de um desenho de canário com rollback verificável antes de qualquer mudança em produção.
- **Chosen approach**: Documento de desenho (`docs/research/2026-11-18-067-c6-canary-rls-design.md`) com estado verificado, ameaça/fronteira, pré-condição de wrapper de transação (descrita, não implementada), fases em DB descartável, SQL `RESTRICTIVE`+`FORCE` para 9 tabelas, ressalvas, matriz (a)–(d) **[NV]** e rollback em 6 passos (`shape.md`).
- **In scope / out of scope**: 1 documento de pesquisa + artefatos do ciclo + refresh de 2 índices de docs + apêndice em `STATE.md`; zero código, workflows, migrations, ADRs, constituição, required checks, credenciais, deploy ou produção.
- **Success metrics**: `check_gate_integrity.py` PASS (8 pisos/51) e CI verde antes do merge; nenhuma alteração em código/workflows/migrações/ADRs/constituição; documento contém fases, SQL, rollback, **[NV]** e cita ADR-0005/0006; contagens conferem com `find` e histórico preservado (define.md SC-001–SC-004).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: execução real do canário e A/B em app — **[NV]**]; [NEEDS CLARIFICATION: produção e CI remota pós-merge — **[NV]**, observar ao vivo antes do merge]; [NEEDS CLARIFICATION: autorização SE explícita para as Fases 2–4 (ADR-0006 §3)].
