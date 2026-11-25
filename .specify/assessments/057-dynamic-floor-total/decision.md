# Decision: Total de pisos dinâmico no guardião (057)

- **Slug**: 057-dynamic-floor-total
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (anterior) | spec.md (ciclo)

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `check_gate_integrity.py:120` imprime literal "all eight … (50 total)" duplicando `FLOORS`; já foi editado em 052 (43→49), 054 (49→50) e 055 (→50) (intake.md:9-14; research.md:10). |
| Evidence strength | strong | Leitura de `check_gate_integrity.py:42-51,114-120` (8 chaves, soma 50) e histórico de edições manuais; guardião PASS hoje, defeito é latente (research.md:1-10). |
| Value vs. inaction | strong | Inação mantém duas edições sincronizadas por mudança de piso; esquecer uma produz saída deterministicamente incorreta na fonte autoritativa de integridade (problem.md:Cost of Inaction). |
| Feasibility / appetite | strong | Appetite small: substituir o literal por f-string derivada de `len(FLOORS)` e `sum(FLOORS.values())`, com regex e mensagens de falha intactas (concept.md:Option A). |
| Strategic fit | strong | Elimina a duplicação dentro do próprio guardião sem novo check, ADR, workflow ou mudança de proteção de branch (define.md:Non-Goals). |
| Risk posture | strong | Saída informacional preservada (8 pisos / 50 total); única mudança de forma é o numeral "8" no lugar de "eight"; mutação em `/tmp`; CI/PR e aprovação de ADRs **[NV]**. |

## Verdict & Rationale

**go**, preservando o veredito histórico `go (scoped)` (decision.md anterior:3; spec.md:Status "Approved"). O scorecard é favorável em todas as dimensões: o drift é recorrente e documentado, e a correção é uma linha que remove a causa raiz sem alterar o que o gate valida. Mantém-se o escopo do veredito anterior — 1 arquivo existente, 1 linha (`scripts/ci/check_gate_integrity.py`), artefatos do ciclo e notas datadas append-only; nenhuma decisão de autorização, ADR ou CI é alterada e nenhum trabalho alheio é tocado (decision.md anterior:Delimitadores). ADR-0003/0006 seguem com o texto antigo como dívida de governança escalada **[NV]**; a aprovação para corrigi-los e o estado de CI/PR ao vivo ficam como incertezas carregadas.

## If needs-clarification

- **Blocking questions**: n/a — veredito é `go`; as perguntas sobre ADRs e registro da mutação não bloqueiam a especificação.
- **Revisit stage**: n/a

## If go — Handoff to `/speckit.specify`

- **Problem**: o guardião imprime a contagem e o total de pisos escritos à mão, duplicando `FLOORS` e gerando drift recorrente a cada mudança de piso.
- **Chosen approach**: Option A — derivar a mensagem da fonte canônica: `f"all {len(FLOORS)} db contract floors intact ({sum(FLOORS.values())} total)"` na linha de sucesso, mantendo regex e mensagens de falha idênticas (concept.md:Recommendation; research.md:Decisão).
- **In scope / out of scope**: in — `scripts/ci/check_gate_integrity.py` (1 linha), artefatos do ciclo e notas datadas append-only; out — ADR-0003/0006, constituição, workflow, testes, valores de `FLOORS`, regex de validação, required checks, C5/C6, deploy e trabalho alheio.
- **Success metrics**: guardião PASS com a mesma saída informacional de antes (8 pisos / 50 total); mutação em cópia `/tmp` adicionando um 9º piso → mensagem passa a dizer 9 pisos / 54 total sem editar o texto; diff = 1 linha em 1 arquivo; gates locais verdes; CI/PR ao vivo **[NV]**.
- **Carried-forward open questions**: [NEEDS CLARIFICATION: aprovação de maintainer para corrigir ADR-0003/0006 virá em qual ciclo?] **[NV]**; [NEEDS CLARIFICATION: numeral "8" em vez de "eight" é aceitável como conteúdo informacional idêntico?]; [NEEDS CLARIFICATION: a mutação em `/tmp` será registrada como evidência no PR?].
