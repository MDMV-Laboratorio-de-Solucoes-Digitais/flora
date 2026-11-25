# Problem Definition: Mudanças críticas sem compreensão registrada

- **Slug**: 109-comprehension-briefing
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md | define.md | shape.md

## Problem Statement

Sob o merge autônomo de ADR-0003, os gates provam que os checks rodaram, mas não que alguém entendeu quais invariantes mudaram ou por quê: mudanças autônomas em domínios críticos podem passar nos gates e ainda deixar o SE sem compreensão (define.md:3; research.md:13,24). O assessment multiagente registrou a decisão F10 (**ADOPT**) como antídoto à dívida de compreensão e rejeitou tanto a revisão humana linha a linha (custo e contrariedade ao ADR-0003) quanto "confiar só na spec", porque specs não capturam decisões implícitas (intake.md:5-16; research.md:12).

Os domínios observados no repositório e registrados no diagnóstico são autenticação/sessão, RBAC, tenant, bootstrap/recovery e HITL; pagamentos aparecem como críticos fora do core no research original, mas não entraram na norma publicada — divergência a esclarecer (research.md:35-38,47). O pedido capturado é um artefato curto e padronizado que registre intenção, invariantes alteradas e como verificar a mudança (intake.md:8-16). A eficácia real depende do uso humano nos próximos ciclos e não foi medida: **[NV]** (decision.md:7; research.md:8,40).

## Affected Users & Stakeholders

- **Users**: o SE na revisão de PR, que precisa validar a compreensão da mudança sem reler todo o diff; os autores de mudanças em auth/sessão, RBAC, tenant, bootstrap/recovery e HITL, que precisam tornar explícitas decisões implícitas (research.md:15,37-39; spec citada em research.md:15).
- **Stakeholders**: o guardião de ADR-0003 e dos required checks, que preserva a autoridade dos gates sem adicionar checagem automática; os revisores de processo que avaliam a adesão documental; os ciclos grandfathered (096–101, 103, 106, 107 e 108), cujo histórico não é reescrito (research.md:23,24,31).

## Goals

- Registrar, por mudança crítica, intenção, invariantes antes/depois, decisões implícitas, comando de verificação e pergunta-teste ao SE, de modo que a revisão não dependa da leitura integral do diff (research.md:15,21).
- Tornar rastreável ao SE o porquê das mudanças aprovadas por gates, sem transformar compreensão em checagem determinística (research.md:13,24,31).
- Manter a obrigatoriedade documental nos cinco domínios críticos listados na nota de governança, com aplicação prospectiva a partir do ciclo 109 (research.md:38,23).
- Preservar ADR-0003, os required checks e o contrato de código (define.md:7; research.md:30).
- Permitir avaliar futura adesão e compreensão sem criar required check (research.md:55).

## Non-Goals

- Não criar gate, required check, automação de CI ou mudança de contrato de código (define.md:7; research.md:30).
- Não reescrever ciclos grandfathered nem renomear os cinco templates preexistentes: a exceção de nomenclatura do template é registrada, não corrigida retroativamente (research.md:23).
- Não incluir pagamentos na obrigatoriedade documental sem clarificar a divergência com o diagnóstico (research.md:38,47).
- Não afirmar compreensão efetiva ou adesão futura sem uso observado: **[NV]** (research.md:8,40,49).

## Success Metrics

- Mudanças em domínios críticos com briefing presente e referenciado: alvo 100% a partir do ciclo 109; baseline: 0 templates e 0 gates que determinassem a presença do briefing (research.md:20,23).
- Invariantes com registro antes/depois, decisões implícitas e como verificar: alvo presentes no briefing de cada mudança crítica; baseline: unknown — não há medição de compreensão no repositório (research.md:21,49).
- Revisão de PR sem reler todo o diff: alvo viável pela pergunta-teste do template; baseline: unknown, efeito real **[NV]** (research.md:15,40).
- Gate PASS e nenhum required check/código/migration novo: baseline: 10 legs / 77 pisos no handoff do ciclo; o guarda atual tem 11 entradas / 87 (research.md:32,48).
- Credenciais ou segredos no template: alvo 0; baseline: 0 no artefato entregue (research.md:16).

## Cost of Inaction

Sem o registro da compreensão, decisões implícitas em auth, RBAC, tenant, bootstrap ou HITL se perdem mesmo quando o gate passa, e o SE passa a depender da leitura integral do diff ou de revisão linha a linha — alternativa rejeitada pelo assessment por custo e conflito com ADR-0003 (research.md:12,24). A dívida de compreensão cresce junto com o merge autônomo, e a rastreabilidade das invariantes exigida pelo guardião fica restrita ao que os checks cobrem, não ao que foi alterado (research.md:13,24).

## Open Questions

- [NEEDS CLARIFICATION: quais domínios são "críticos"?] — a norma publicada fixou cinco, mas o research listava pagamentos fora do core (intake.md:22; research.md:53).
- [NEEDS CLARIFICATION: o briefing vira gate?] — a resposta registrada foi "não", mas segue como decisão de governança revisável (intake.md:23; research.md:54).
- [NEEDS CLARIFICATION: como medir adesão e compreensão efetiva nos próximos ciclos sem criar required check?] (research.md:55).
- [NEEDS CLARIFICATION: o grandfathering dos ciclos 096–101, 103, 106, 107 e 108 deve ser revisado se algum desses domínios for alterado novamente?] (research.md:56).
- [NEEDS CLARIFICATION: a exceção de nomenclatura (`briefing-compreensao.md` sem sufixo `-template.md`) deve ser normalizada em ciclo futuro?] (research.md:57).
