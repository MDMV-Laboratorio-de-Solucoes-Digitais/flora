# Problem Definition: MVP self-hosted sem recorte claro

- **Slug**: 005-macro-alternative-mvp
- **Created**: 2026-11-25
- **Inputs used**: intake.md, research.md, decision.md

## Problem Statement

O pedido do ciclo 005 era alinhar a Flora ao objetivo de ser alternativa viável, performática e self-hosted ao Macro para pequenas e médias empresas em "qualquer VPS barata", com limites realistas. O Macro é descrito nos artefatos como workspace all-in-one AGPL-3.0 (email, chat, docs colaborativos, tarefas, calls, agentes/IA, CRM, databases, calendário, storage, integrações) e como monorepo de alta escala, com muitos crates Rust e ecossistema TS/JS pesado. Replicar essa superfície em VPS modesta é incompatível com a constituição da Flora (debloat, VPS test, modular monolith Rust/Postgres, Svelte 5, tiers Seed→Forest).

O estado real em 2026-11-15 era parcial e heterogêneo: auth/OIDC com lacunas, org/workspace/canais/mensagens/tarefas/arquivos/busca/notificações em diferentes graus, frontend canônico `apps/frontend` com paridade limitada, compose dev de Postgres/Valkey/Meilisearch recém-adicionado no ciclo 004 e smoke runtime ponta a ponta majoritariamente NV. Faltava um recorte explícito do que entra no MVP Seed, do que é substituído por alternativa mais barata e do que é adiado, além de evidência de demanda real do segmento PME e de requisitos mínimos de CPU/RAM/disco. Sem esse norte, o risco é gastar esforço replicando a complexidade operacional que a Flora propõe justamente evitar — e, no extremo oposto, super-simplificar por suposição. A incógnita é de priorização e viabilidade, não de disponibilidade de mais código.

## Affected Users & Stakeholders

- **Users**: pequenas e médias empresas que buscam uma alternativa self-hosted ao Macro em VPS barata; usuários finais dessas organizações, que dependem de poucas jornadas realmente utilizáveis (chat/tarefas/arquivos básicos) antes de qualquer paridade ampla; operadores que instalam e mantêm o Seed.
- **Stakeholders**: operador do repositório que definiu o norte estratégico; mantenedores e contribuidores, que precisam de priorização para não dispersar esforço; responsáveis pela governança do projeto (constituição/OBJETIVO/ROADMAP), que devem refletir o recorte sem promessas não medidas.

## Goals

- Caracterizar a ausência de recorte 80/20 e de debloat explícito como risco estratégico para a viabilidade em VPS barata.
- Distinguir o que é valor típico de PME do que é custo operacional elevado no workspace de referência, com base nas hipóteses registradas em research.md.
- Tornar visíveis as lacunas de evidência: demanda real do segmento, requisitos mínimos de hardware e conformidade de reimplementação limpa sob AGPL.
- Reduzir a ambiguidade entre "paridade com Macro" e "Seed utilizável em VPS modesta" como critério de sucesso do projeto.
- Preservar a direção de fatias verticais finas verificáveis como base para decisões posteriores, sem antecipar a solução.

## Non-Goals

- Não escolher neste artefato a lista final de features do MVP, desenhar API/modelo de dados ou escrever plano de implementação.
- Não prometer paridade com Macro, suporte a cargas específicas ou performance em VPS sem medição/evidência.
- Não propor reuso de código incompatível com AGPL ou engenharia reversa não autorizada de componentes proprietários.
- Não exigir deploy, infra de produção, segredos, serviços pagos, merges ou releases para resolver o problema descrito.

## Success Metrics

- Jornadas verticais do Seed verificáveis ponta a ponta em ambiente dev (baseline: 0/NV — research.md marca runtime integrado como não exercitado).
- Consumo de CPU/RAM/disco em VPS de referência de ~2 vCPU/4 GB (baseline: unknown — hipótese não medida, research.md).
- Capacidades do Macro classificadas com destino (core Seed/substituir/adiar) e justificativa de custo (baseline: hipótese em tabela, sem validação de demanda).
- Alinhamento entre OBJETIVO/ROADMAP e o estado observável do Seed (baseline: parcial — direção declarada, mas gaps e recorte não consolidados).

## Cost of Inaction

Sem norte recortado, cada contribuição tende a ampliar a superfície em direção ao Macro, elevando custo de build, operação e suporte exatamente onde a proposta promete leveza. A ausência de medição impede afirmar "qualquer VPS barata" com segurança, e a falta de validação de demanda pode levar a investir em capacidades que as PME não usam. O efeito combinado é atraso na entrega de um Seed utilizável e desgaste da credibilidade da alternativa.

## Open Questions

- [NEEDS CLARIFICATION: existe evidência de demanda do segmento PME pelas capacidades candidatas a core Seed (chat/tarefas/arquivos), ou a priorização atual é hipótese?]
- [NEEDS CLARIFICATION: quais são os limites reais de CPU/RAM/disco e a versão/alvo de VPS usada como referência para "barata"?]
- [NEEDS CLARIFICATION: como tratar a conformidade AGPL em reimplementação limpa face ao código e aos assets do Macro?]
