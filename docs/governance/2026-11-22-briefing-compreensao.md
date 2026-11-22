# Briefing de Compreensão — obrigatoriedade documental

**Data**: 2026-11-22 · **Decisão**: F10 do assessment multiagente (**ADOPT**)

## Regra

Todo ciclo que altera **auth/sessão, RBAC, tenant, bootstrap/recovery ou HITL** deve incluir, no PR, um briefing preenchido a partir de `.specify/templates/briefing-compreensao.md`, com intenção, invariantes antes/depois, decisões implícitas, comando de verificação e uma pergunta-teste ao SE.
Nenhum required check novo (ADR-0003 inalterado). Adesão é processo de governança de PR, não checagem automática. O briefing é append-only.

## Justificativa

Gates determinísticos provam que o código passa nos testes; não provam que um humano compreende as invariantes alteradas. O briefing é o antídoto documental à dívida de compreensão sem reintroduzir revisão linha a linha (rejeitada no assessment).

## Fronteira

Nenhum required check novo (ADR-0003 inalterado). Adesão é processo de gov petição de PR, não checagem automática. O briefing é append-only.

**[NV]**: eficácia real depende do uso pelo SE nos próximos ciclos.
