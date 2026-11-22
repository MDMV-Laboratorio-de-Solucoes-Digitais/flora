# Briefing de Compreensão — obrigatoriedade documental

**Data**: 2026-11-22 · **Decisão**: F10 do assessment multiagente (**ADOPT**)

## Regra

Todo ciclo que altera **auth/sessão, RBAC, tenant, bootstrap/recovery ou HITL** deve incluir, no PR, um briefing preenchido a partir de `.specify/templates/briefing-compreensao.md`, com intenção, invariantes antes/depois, decisões implícitas, comando de verificação e uma pergunta-teste ao SE.
Nenhum required check novo (ADR-0003 inalterado). Adesão é processo de governança de PR, não checagem automática. O briefing é append-only.

## Justificativa

Gates determinísticos provam que o código passa nos testes; não provam que um humano compreende as invariantes alteradas. O briefing é o antídoto documental à dívida de compreensão sem reintroduzir revisão linha a linha (rejeitada no assessment).

## Fronteira

Nenhum required check novo (ADR-0003 inalterado). Adesão é processo de governança de PR, não checagem automática. O briefing é append-only.

**[NV]**: eficácia real depende do uso pelo SE nos próximos ciclos.

### Correção append-only — 2026-11-22 (após ciclo 109)

- A obrigatoriedade é **prospectiva, a partir do ciclo 109**. Ciclos anteriores que tocaram os domínios críticos — 096–101 (auth), 103 (RBAC), 106 (HITL), 107 (bootstrap/recovery) e 108 (proptests de permissão) — são **grandfathered**: não são retroativamente não-conformes e não terão briefings forjados.
- Convenção de nomes: os cinco templates preexistentes usam o sufixo `-template.md`. O arquivo `.specify/templates/briefing-compreensao.md` é uma **exceção registrada** para manter o nome legível nas referências de PR; nenhum template existente foi renomeado.
