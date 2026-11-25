# Decision: Briefing de Compreensão em mudanças críticas

- **Slug**: 109-comprehension-briefing
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (anterior)

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | Sob merge autônomo (ADR-0003), os gates provam que checks rodaram, não que alguém entendeu quais invariantes mudaram; F10 do assessment é ADOPT como antídoto à dívida de compreensão (problem.md:9; research.md:12-13). |
| Evidence strength | adequate | Decisão F10, nota de governança com cinco domínios críticos e template preexistente estão documentados, mas não há medição de compreensão efetiva nem de adesão futura **[NV]** (research.md:38,40,49). |
| Value vs. inaction | strong | Sem o registro, decisões implícitas em auth/RBAC/tenant/bootstrap/HITL se perdem e o SE depende da leitura integral do diff — alternativa rejeitada por custo e conflito com ADR-0003 (problem.md:43). |
| Feasibility / appetite | strong | Appetite small: artefato curto e padronizado, aplicação prospectiva a partir do ciclo 109, sem automação (concept.md:11,29). |
| Strategic fit | strong | Documentação pura, sem gate/required check/contrato novo, preservando a autoridade de ADR-0003 e do guarda do gate (decision.md anterior:5; define.md:7). |
| Risk posture | strong | Único risco é adesão processual e o briefing virar formulário preenchido por rotina; nenhuma credencial ou segredo no template (concept.md:12; research.md:16). |

## Verdict & Rationale

O veredito histórico **`go`** é preservado: o enquadramento é documental e compatível com a governança vigente, e nenhum artefato contradiz a autorização F10. Mantém-se o racional anterior — sem gate, required check ou contrato novo; risco limitado à adesão processual; benefício de rastreabilidade das invariantes para o SE (decision.md anterior:5). A eficácia real depende do uso humano nos próximos ciclos e não foi medida **[NV]**; a divergência sobre pagamentos e a exceção de nomenclatura do template seguem registradas, sem mudança silenciosa de escopo.

## If needs-clarification

- **Blocking questions**: N/A — veredito `go` preservado; perguntas sobre domínios e medição não bloqueiam o handoff.
- **Revisit stage**: N/A

## If go — Handoff to `/speckit.specify`

- **Problem**: Registrar intenção, invariantes alteradas e verificação de mudanças críticas de domínio para que o SE revise sem depender da leitura integral do diff.
- **Chosen approach**: Opção A — briefing curto e obrigatório nos cinco domínios críticos já registrados (autenticação/sessão, RBAC, tenant, bootstrap/recuperação, HITL), com intenção, antes/depois das invariantes, decisões implícitas, comando de verificação e pergunta-teste; aplicação prospectiva a partir do ciclo 109.
- **In scope / out of scope**: Inclui template e nota de governança; exclui gate, required check, automação de CI, alteração de contrato, reescrita dos ciclos grandfathered (096–101, 103, 106, 107, 108), renomeação retroativa do template e inclusão de pagamentos sem clarificar a divergência.
- **Success metrics**: 100% das mudanças críticas com briefing presente e referenciado a partir do ciclo 109; invariantes com registro antes/depois e como verificar; gate PASS e nenhum required check/código/migration novo; 0 credenciais no template.
- **Carried-forward open questions**: [NEEDS CLARIFICATION: quais domínios são "críticos"?]; [NEEDS CLARIFICATION: o briefing vira gate?]; [NEEDS CLARIFICATION: como medir adesão e compreensão sem criar required check?]; [NEEDS CLARIFICATION: o grandfathering deve ser revisado se algum domínio for alterado novamente?]; [NEEDS CLARIFICATION: normalizar a exceção de nomenclatura do template em ciclo futuro?].
