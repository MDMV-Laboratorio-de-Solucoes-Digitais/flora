# Research: 005 — Macro Alternative MVP Reframe

**Date:** 2026-11-15  
**Confidence:** média (direção estratégica e restrições macro bem suportadas por evidências locais/públicas; recomendações de recorte/priorização dependem de síntese e de pesquisas em andamento — itens NV marcados).

## 1) Evidência interna (Flora)

- **Objetivo declarado (constituição):** fork/debloat do Macro, AGPL, rodar em VPS barata (“VPS test”), modular monolith Rust, Postgres núcleo, Svelte 5 + Tauri futuro, tiers Seed→Forest, rigor de lint/test e spec-driven.
- **Estado real (2026-11-15):** núcleo parcial (auth/OIDC com lacunas, org/workspace, canais/mensagens/tarefas, registros de arquivos, busca/notificações e realtime com integrações incompletas); frontend canônico `apps/frontend` (Svelte 5) mas paridade/UI limitada; compose dev para Postgres/Valkey/Meilisearch existe (ciclo 004) porém smoke runtime ponta a ponta NV neste ambiente; verificação determinística local parcial registrada em `OBJETIVO.md`/baseline anterior.
- **Implicação:** maior alavancagem atual é convergir para poucas jornadas verticais utilizáveis e operáveis em VPS modesta, não ampliar superfície.

## 2) Evidência pública inicial sobre Macro (referência)

- **Produto:** “all-in-one workspace” (email, mensagens/canais, docs, tarefas, calls, agents, CRM, databases, calendar, storage, PRs, memória unificada/@links). Posicionamento para times/startups; alta integração e features assistidas por IA.
- **Licença:** AGPL-3.0 (`LICENSE.txt`), permitindo self-host sob termos AGPL e reimplementações compatíveis (reuso de código exige conformidade com copyleft/rede; reimplementação limpa deve evitar cópia indevida).
- **Complexidade aparente do monorepo:** grande quantidade de crates Rust, múltiplos serviços/apps/pacotes/infra/tooling (ex.: `Cargo.toml` workspace extenso; `package.json` com workspaces TS/JS, Lexical/Loro/wasm/patches etc.). Sinal forte de alto custo operacional e de build para replicar “tudo” em VPS barata.
- **Gaps de pesquisa (NV, em andamento):** inventário detalhado de dependências externas/proprietárias, requisitos mínimos de self-host, componentes obrigatórios por feature, e análise comparativa profunda — delegados a subagentes de pesquisa.

## 3) Hipóteses de 80/20 para “Macro-like” em VPS barata (PME)

| Domínio Macro | Valor PME típico | Custo/complexidade self-host | Recomendação inicial (80/20) |
|---|---|---|---|
| Chat/canais/DMs + notificações básicas | Alto | Médio | **Core Seed** (MVP) — persistência Postgres + realtime simples/eficiente |
| Tarefas/projetos leves ligados a conversa | Alto | Baixo/médio | **Core Seed** (MVP) |
| Arquivos básicos + permissões | Alto | Médio (storage/cotas) | **Core Seed** (MVP, local/S3-compat opcional) |
| Docs colaborativos leves (markdown/notas) | Médio/alto | Médio/alto (CRDT/colab) | **Fase seguinte**; iniciar sem CRDT pesado |
| Busca | Médio/alto | Baixo/médio (Postgres FTS) a médio (Meilisearch opcional) | **Core Seed**: Postgres FTS primeiro; Meilisearch opcional |
| Email multi-conta unificado estilo cliente completo | Médio | Alto (sync, parsing, OAuth, compliance) | **Adiar/substituir** (integrações mínimas depois) |
| Calls/transcrição/gravação | Médio | Alto (mídia, TURN, storage, custo) | **Adiar** |
| CRM/databases genéricas avançadas | Variável | Médio/alto (modelo flexível, UI) | **Adiar**; extrair padrões simples de contatos/tarefas se necessário |
| Agentes/IA com memória/código/rotinas | Variável | Alto (custos API, sandbox, observabilidade) | **Opcional por feature flag**, nunca requisito Seed; preferir BYO API |
| Calendar/booking/integrações profundas | Médio | Médio/alto | **Adiar** |

> Nota: tabela é hipótese de priorização baseada em custo operacional típico e estado do Flora; deve ser refinada com pesquisa comparativa e feedback/uso real (NV para demanda específica de segmento).

## 4) Princípios de debloat/viabilidade (derivados)

- **Postgres como gravidade:** dados relacionais + JSONB quando necessário; avaliar Postgres FTS/pgmq/listen-notify antes de adicionar motores dedicados.
- **Componentes opcionais:** Valkey/Meilisearch/RustFS/Zitadel etc. apenas por perfil/necessidade comprovada; Seed deve degradar graciosamente.
- **Sem dependências proprietárias obrigatórias:** IA/e-mail/SMS/chamadas como integrações plugáveis ou fora do núcleo.
- **Fatias verticais finas:** “org + usuários + canal + mensagem + tarefa + arquivo básico” exercitáveis com evidência antes de paridade ampla.
- **UI enxuta:** Svelte 5 com performance/responsividade; evitar replicar window manager/superfícies complexas do Macro no MVP.

## 5) Opções de próximo 80/20

| Opção | Descrição | Prós | Contras | Veredito |
|---|---|---|---|---|
| A | Perseguir paridade ampla de features Macro | Visão completa | Inviável em VPS barata; alto risco/tempo | **Não** |
| B | **Reframe estratégico + MVP slice definido + docs/roadmap atualizados + spec do próximo incremento técnico verificável** | Alinha execução, reduz escopo, prepara implementação | Não entrega feature usuário final imediato | **Recomendado (go)** |
| C | Apenas pesquisa sem registrar decisão/artefatos | Baixo esforço imediato | Conhecimento se perde; sem direção executável | Não |

## 6) Recomendações imediatas (ciclo 005)
1. Atualizar `OBJETIVO.md`/`ROADMAP.md` com norte “Macro-alternative para PME/VPS barata”, pilares de debloat e MVP proposto.
2. Registrar decisão **go** (escopo documental/planejamento) e criar `specs/005-macro-alternative-mvp/` com entregáveis verificáveis deste ciclo.
3. Consolidar pesquisa externa em `docs/research/2026-11-15-macro-alternative-80-20-research.md` conforme subagentes concluírem (atualização incremental permitida; marcar NV).
4. Próximo ciclo técnico: fatia vertical mínima verificável (auth/contexto + workspace + canal/mensagem ou tarefa) sobre stack Seed barata, com smoke e evidência.

**Pesquisa em andamento:** subagentes background (codebase map, análise Macro, padrões cheap-VPS/Svelte+Tauri). Resultados devem ser incorporados ao consolidado quando disponíveis.
