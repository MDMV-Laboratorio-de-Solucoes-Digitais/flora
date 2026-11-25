# Problem Definition: Total de pisos escrito à mão no guardião

- **Slug**: 057-dynamic-floor-total
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md | define.md (equivalente local; `problem.md` ausente no baseline) | shape.md

## Problem Statement

`scripts/ci/check_gate_integrity.py:120` imprime uma mensagem literal com a quantidade de pisos e o total: `"all eight db contract floors intact (50 total)"`. A mesma informação já existe em `FLOORS` (`:42-51`): 8 chaves — auth 13, messaging 4, websocket 8, search 8, notifications 5, files 6, tasks 4, workspace 2 — soma 50 (intake.md:9-13; research.md:5-7). O literal foi editado manualmente em ciclos recentes (052: 43→49; 054: 49→50; 055: →50), sempre que `FLOORS` mudava (intake.md:11; research.md:10). É uma fonte duplicada de verdade dentro do próprio guardião determinístico: esquecer uma das duas edições produz saída deterministicamente incorreta. O literal está correto hoje — o defeito é latente (intake.md:14). Ranqueamento 80/20 = 7.5 (intake.md:20-24). A correção pós-implementação registrou que o numeral passou a ser `8`, derivado de `len(FLOORS)`, com conteúdo informacional idêntico (define.md:26-29).

## Affected Users & Stakeholders

- **Users**: mantenedores que leem a saída do guardião como estado verificado dos pisos de contrato DB (intake.md:11-14).
- **Stakeholders**: responsáveis pelo gate determinístico e pelas decisões de governança sobre ADRs, que devem continuar escalados e não alterados no mesmo ciclo (`decision.md:9-13`; `research.md:18`).

## Goals

- Derivar a mensagem de sucesso de `len(FLOORS)` e `sum(FLOORS.values())` (`define.md:7`; `research.md:14-16`).
- Manter a regex de validação e as mensagens de falha exatamente como estão (`define.md:8`; `shape.md:17-18`).
- Preservar a saída informacional atual: 8 pisos / 50 total (`define.md:9`, com a correção pós-implementação em `:26-29`).
- Alterar apenas 1 linha em 1 arquivo, sem tocar valores de `FLOORS`, required checks, workflows, ADRs ou testes (`define.md:10`; `decision.md:11`).

## Non-Goals

- Não editar ADR-0003/0006, constituição, testes ou CI (`define.md:13`; `decision.md:12`).
- Não alterar valores de `FLOORS` nem a regex (`define.md:14`).
- Não introduzir constante `TOTAL` duplicada, tabela de numerais ou remover a mensagem (`shape.md:11-13`; `research.md:17-19`).
- Não criar novo check: mudaria a proteção de branch de 4 contextos (`research.md:20`).

## Success Metrics

- Guardião PASS com a mesma saída informacional de antes: 8 pisos / 50 total (baseline: literal `"all eight ... (50 total)"` — define.md:16; intake.md:9-11).
- Mutação em cópia `/tmp` adicionando um 9º piso → mensagem passa a dizer 9 pisos / 54 total sem editar a mensagem (baseline: exigia edição manual do literal — define.md:17; research.md:10).
- Diff de código = 1 linha em 1 arquivo (baseline: nenhuma mudança de código — define.md:19; shape.md:5-7).
- Gates locais permanecem verdes (baseline: verdes — define.md:18).
- CI/PR ao vivo e aprovação para corrigir ADRs: **[NV]** (decision.md:17).

## Cost of Inaction

Toda mudança de piso continua exigindo duas edições sincronizadas no mesmo arquivo; se uma for esquecida, o guardião — que é a fonte autoritativa de integridade — imprime um número incorreto. O histórico de 052/054/055 mostra que o drift é recorrente, não hipotético (intake.md:11-14; research.md:10).

## Open Questions

- [NEEDS CLARIFICATION: a aprovação de maintainer para corrigir ADR-0003/0006 virá em qual ciclo?] (`decision.md:17`; `research.md:18`; **[NV]**)
- [NEEDS CLARIFICATION: o numeral `8` em vez de `eight` é aceitável como conteúdo informacional idêntico?] (`define.md:26-29`; `shape.md:12-13`)
- [NEEDS CLARIFICATION: a mutação em `/tmp` será registrada como evidência no PR?] (`define.md:17`; `shape.md:20`)
