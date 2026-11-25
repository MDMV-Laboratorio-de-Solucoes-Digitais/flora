# Decision: Fechamento documental de bugs stale

- **Slug**: 113-bug-status
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (anterior)

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | Três registros em `.specify/bugs/` descrevem status proposto/planejado embora código e testes já estejam mesclados (D15, OIDC fallback, WCAG-111), inflando a dívida e desviando o próximo 80/20 (intake.md:5-7; problem.md:9). |
| Evidence strength | strong | Evidência primária local determinística: 2 passed em `cargo test -p flora-api hitl_receipt`, rota registrada em `flora-api/src/routes/auth.rs:83`, PR #370 mesclado e log axe "sem achados" (research.md:20-23). |
| Value vs. inaction | strong | O próprio critério 80/20 registra 375/625 para o fechamento documental contra 144/625 de automação de staging efêmero (problem.md:11; concept.md:29). |
| Feasibility / appetite | strong | Appetite small e sem mudança de código: tabela `bug → evidência → status → [NV]` e notas datadas append-only (concept.md:9-11). |
| Strategic fit | strong | R5 atua apenas como fechamento documental; nenhuma autorização nova é necessária e code/gate/required checks permanecem inalterados (decision.md anterior:9,13). |
| Risk posture | adequate | Risco de tratar evidência local como prova remota: produção, IdP externo e staging permanecem **[NV]**, e "axe sem achados" não é conformidade AA plena (~57% de cobertura, resultados `incomplete`) (problem.md:11). |

## Verdict & Rationale

O veredito histórico **`go`** é preservado: a evidência local é verificável e o fechamento é documental, sem mexer em código ou governança. Preservam-se os fatos anteriores — score **375/625** (critério em `STATE.md`), evidência primária local verificada, nenhuma autorização nova necessária, R5 aplicável apenas como fechamento documental porque os bugs já têm fix/test executados em ciclos anteriores (decision.md anterior:9) — e as restrições: preservar histórico, marcar **[NV]** para produção e não tocar required checks (decision.md anterior:13). A ressalva sobre cobertura parcial do axe é registrada, sem reescrita de notas append-only.

## If needs-clarification

- **Blocking questions**: N/A — veredito `go` preservado; a confirmação remota não é pré-condição do fechamento documental.
- **Revisit stage**: N/A

## If go — Handoff to `/speckit.specify`

- **Problem**: Registrar status verificado com evidência determinística para três bugs já corrigidos, preservando histórico e marcando **[NV]** o que depende de ambiente remoto.
- **Chosen approach**: Opção A — fechamento documental com notas datadas append-only ligando cada bug à evidência local, ao status resultante e ao limite **[NV]**; criar `fix.md`/`test.md` onde o arquivo não existe.
- **In scope / out of scope**: Inclui tabela `bug → evidência → status → [NV]` e SemVer PATCH apenas para documentação; exclui mudança de código, deploy, credenciais, required checks, verificação remota, automação de staging e promoção dos PRs #369/#371.
- **Success metrics**: 3/3 bugs rastreados a evidência local; D15 com 2 passed e OIDC com 1 passed; 1 log axe "sem achados" (run 45048150000) anexado; 0 regras/gates/required checks alterados; 3 itens com **[NV]** explícito para produção.
- **Carried-forward open questions**: [NEEDS CLARIFICATION: status em produção pode ser confirmado com ambiente e credenciais do SE?]; [NEEDS CLARIFICATION: revisão manual de critérios WCAG foi registrada além do resumo axe?]; [NEEDS CLARIFICATION: `544a075ca` é head do PR #370 ou âncora da branch do ciclo 113?]; [NEEDS CLARIFICATION: os PRs #369 e #371 foram promovidos e isso altera o inventário?]; [NEEDS CLARIFICATION: como conciliar "10 legs / 77 pisos" com o guarda atual sem reescrever notas append-only?].
