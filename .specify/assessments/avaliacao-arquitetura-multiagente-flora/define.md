# Define — avaliação da arquitetura multiagente aplicada ao Flora

**Data**: 2026-11-22 · **Slug**: `avaliacao-arquitetura-multiagente-flora`

## Problema

O plano `Arquitetura Multiagente Autônoma da MDMV.pdf` propõe uma stack extensa para cinco agentes, com premissas sometimes desatualizadas, e parte dela é apresentada como dependência do Flora Workspace. Sem triagem explícita, há risco de importar serviços obrigatórios para o Seed, dependências sem manutenção e requisitos que não cabem no modelo de tenant do Flora.

## Objetivo

Classificar cada proposta relevante ao Flora em **ADAPT / ADOPT / DEFER / AVOID / REJECT**, com evidência primary e fronteira explícita entre o que pertence ao Flora e o que pertence a outros sistemas (MDMV-Autônoma, dark factory, ERP, provedores externos).

## Fora de escopo

- Implementação de qualquer decisão nesta fase: nenhum código, gate, ADR, migration, required check ou credencial é alterado.
- Votos de governança que exijam autorização explícita do SE.
- Eixos não entregues por limitação de busca (auditoria/imutabilidade e alternativas de orquestração) — registrados **[NV]** em `research.md`.

## Critérios de decisão

1. **Aderência ao Seed (ADR-0001)**: nenhum serviço obrigatório novo; perfilamento Sprout/Grove quando aplicável.
2. **Evidência**: fonte primária verificável e ≥2 independentes para afirmação carregada.
3. **Manutenção/supply chain**: licença OSI, crates.io/publicação, atividade recente, ausência de dependência Git não auditada.
4. **Ajuste ao modelo atual**: tenant por `organization_id`, sessão OIDC + `jti`, RBAC real, traits existentes, gate determinístico do alvo.
5. **Custo operacional em 1–2 OCPU**: nenhum servidor separado sem necessidade comprovada.
6. **Direitos e privacidade**: LGPD/HITL/CDC como invariantes, não como feature opcional.

## Resultado esperado

Registro de decisões rastreável (152 itens), com relatórios por eixo e apêndice de governança; nenhuma transição de estado de código.
