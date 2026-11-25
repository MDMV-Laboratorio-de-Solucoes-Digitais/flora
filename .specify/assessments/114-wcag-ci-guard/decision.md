# Decision: Blindagem WCAG 2.2 AA no e2e-browser e rota /files

- **Slug**: 114-wcag-ci-guard
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | define.md | decision.md (anterior) — `problem.md` e `concept.md` ausentes no baseline

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | Após o ciclo 112, `/tasks` e `/channels` estão sem achados, mas `auditAccessibility()` só registra anotação/log; regressões passariam silenciosamente, e `/files` não é visitado nem auditado e tem defeitos determinísticos (intake.md:5-11; research.md:7-15). |
| Evidence strength | strong | Código e linhas verificados em `b32f7cafe`, cálculo de contraste WCAG 1.4.3 (3,62:1 e 3,54:1 → 6,18:1/6,41:1) e log do job `e2e-browser` no PR #371 com "sem achados" (research.md:9-21). |
| Value vs. inaction | strong | Maior candidato autônomo em `STATE.md` (400/625): sem asserção, o sinal de a11y do ciclo 111 não protege contra regressões e `/files` segue com barreiras reais (decision.md anterior:9). |
| Feasibility / appetite | adequate | Extração de utilitário puro, correções localizadas e extensão do mesmo teste E2E reaproveitam a sessão rotacionada; exige coordenação frontend e testes unitários (research.md:8; define.md:7-9). |
| Strategic fit | strong | Alinhado à Política 0.2/Bloco 5 e à Constituição v1.4.0 §IV (rigor frontend, a11y obrigatória, zero `any`, zero `console.*`) sem alterar required checks nem burlar ADR-0003 (decision.md anterior:9). |
| Risk posture | adequate | Nenhuma regra axe desabilitada e nenhum piso alterado; tema escuro, staging e produção permanecem **[NV]**, e rollback é por commit de reversão sem force-push (research.md:30; decision.md anterior:11-17). |

## Verdict & Rationale

O veredito histórico **`go`** é preservado: o problema é verificável no código, a pontuação 80/20 é a maior candidatura autônoma e a solução fortalece a confiança da CI sem mudar governança. Preservam-se integralmente o cálculo e os fatos anteriores — score **400/625** (`impacto=4 × confiança=5 × (6−esforço=2)=4 × (6−risco=1)=5`), maior candidato autônomo em `STATE.md`, alinhamento à Política 0.2/Bloco 5 e à Constituição v1.4.0 §IV (decision.md anterior:9) — e as restrições: nenhuma regra axe desabilitada; nenhum required check (`frontend-build`, `rust-build`, `db-tests`, `dependency-review`) ou piso de `check_gate_integrity.py` alterado; passo `11b taskstoissues` obrigatório após `analyze` e antes de `implement`; roteiro de testes, execução no CI, `evidence.md` e Code Review agêntico independente obrigatórios antes do merge em `dev`; rollback por novo commit de reversão, sem force-push (decision.md anterior:11-17). Nenhuma contradição nos artefatos justifica mudança de veredito.

## If needs-clarification

- **Blocking questions**: N/A — veredito `go` preservado; nenhuma pergunta bloqueia o handoff.
- **Revisit stage**: N/A

## If go — Handoff to `/speckit.specify`

- **Problem**: Transformar a auditoria WCAG 2.2 AA do smoke E2E em asserção determinística e auditar `/files`, corrigindo seus defeitos de acessibilidade sem alterar required checks.
- **Chosen approach**: Extrair `apps/frontend/src/lib/utils/a11yAudit.ts` com `WCAG_22_AA_TAGS`, `formatWcagViolations` e `summarizeWcagAudit` (TDD em Vitest), falhar com `expect(findings, summary).toEqual([])`, corrigir `FileUpload.svelte`/`FilePreview.svelte` e adicionar o passo `/files` em `seed-flow.e2e.ts` reaproveitando a sessão rotacionada.
- **In scope / out of scope**: Inclui utilitário puro, testes unitários, Pt-BR, documentação append-only, roteiro/evidência e ACR; exclui required checks, `check_gate_integrity.py`, backend/Rust/migrations, endpoint `GET /api/v1/files` (candidato 115), tema escuro no E2E **[NV]** e deploy.
- **Success metrics**: 0 violações nas três rotas auditadas com falha determinística em qualquer achado; 0 regras axe desabilitadas; 0 alterações em required checks/pisos; ESLint e `svelte-check` com 0 erros; execução no CI, `evidence.md` e ACR antes do merge.
- **Carried-forward open questions**: [NEEDS CLARIFICATION: tema escuro e staging/produção permanecem **[NV]** — quando serão observados?]; [NEEDS CLARIFICATION: a extração do utilitário em `a11yAudit.ts` altera a superfície pública além do previsto em FR-001?].
