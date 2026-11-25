# Decision: Varredura automatizada WCAG 2.2 AA

- **Slug**: 111-wcag-axe
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (anterior)

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | O frontend canônico não tem verificação automatizada de acessibilidade: conformidade WCAG 2.2 AA depende só da revisão humana FR-07, lenta, não reproduzível e cega a regressões (intake.md:5-7; research.md:10). |
| Evidence strength | strong | `@playwright/test` 1.60 e smoke autenticado existem, mas `axe-core` não está declarado; assessment raiz F9/FR-06/FR-08 e fontes W3C/Deque trianguladas; o primeiro run encontrou 3 violações reais (research.md:7-10,15). |
| Value vs. inaction | strong | Sem varredura, regressões de `button-name`, `color-contrast` e `document-title` passam despercebidas até revisão manual; os achados do ciclo 111 viraram bug acionável (problem.md:42). |
| Feasibility / appetite | strong | Appetite small: duas rotas já usadas pelo smoke, sessão reaproveitada, dependência de dev autorizada pelo SE e lockfiles sincronizáveis (concept.md:10; decision.md anterior:9). |
| Strategic fit | strong | F9 **ADAPT** conforme assessment raiz (F9, FR-06, FR-08), sem required check novo e sem alterar o caráter advisory do `e2e-browser` (research.md:10; decision.md anterior:9,13). |
| Risk posture | adequate | Cobertura automatizada é parcial, com risco de falso positivo de contraste em tema escuro e divergência de lockfiles; nenhuma regra é relaxada e rollback é por reversão (concept.md:12; shape.md:16-17). |

## Verdict & Rationale

O veredito histórico **`go`** é preservado: a varredura leve materializa F9/FR-06/FR-08 sem ultrapassar a autorização concedida e com evidência primária verificável. Preservam-se os fatos da decisão anterior — F9 ADAPT conforme assessment raiz; autorização do SE para instalar dependências de dev já concedida; score 80/20 de **240/625** (critério e denominador registrados em `STATE.md`) — e todas as restrições: job `e2e-browser` permanece advisory, gate autoritativo `check_gate_integrity.py` inalterado, sem deploy, credenciais ou required check novo, R4 antes de R7 e `converge.md` append-only (decision.md anterior:9-15). Não se afirma conformidade AA plena nem comportamento em staging/produção ou jobs remotos após o merge **[NV]** (research.md:21-23).

## If needs-clarification

- **Blocking questions**: N/A — veredito `go` preservado; as perguntas sobre cobertura e promoção do job não bloqueiam o handoff.
- **Revisit stage**: N/A

## If go — Handoff to `/speckit.specify`

- **Problem**: Verificar de forma reproduzível as regras WCAG 2.2 AA nas rotas autenticadas já usadas pelo smoke, sem criar required check.
- **Chosen approach**: Opção A — varredura leve com `@axe-core/playwright` sobre `/tasks` e `/channels`, cinco tags (`wcag2a`, `wcag2aa`, `wcag21a`, `wcag21aa`, `wcag22aa`), achados legíveis em pt-BR no log do job informativo.
- **In scope / out of scope**: Inclui dependência de dev com `bun.lock`/`package-lock.json` sincronizados e sessão reaproveitada; exclui auditoria humana, correção de todos os achados, required checks, gate, deploy, credenciais, novas rotas e promoção do job a bloqueante.
- **Success metrics**: 2 rotas autenticadas com varredura; 5 tags WCAG por página; 0 achados no log informativo após correções (baseline: 3 violações reais); 4 required checks inalterados; dependência presente nos dois lockfiles.
- **Carried-forward open questions**: [NEEDS CLARIFICATION: como tratar falso positivo de contraste em tema escuro sem relaxar regra?]; [NEEDS CLARIFICATION: quais rotas além de `/tasks` e `/channels` entrarão na varredura?]; [NEEDS CLARIFICATION: qual lockfile é fonte de verdade se divergirem?]; [NEEDS CLARIFICATION: há critério para promover `e2e-browser` a required check?]; [NEEDS CLARIFICATION: como registrar revisão manual de critérios não automatizáveis?].
