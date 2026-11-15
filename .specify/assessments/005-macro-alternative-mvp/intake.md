# Intake: Flora 80/20 Cycle 005 — Macro Alternative MVP Reframe (2026-11-15)

## Captured request
Executar o próximo ciclo 80/20 para a Flora de forma autônoma, com pesquisa extensa/multilíngue e alta confiabilidade, alinhando o projeto ao objetivo de ser uma **alternativa viável, performática e self-hosted ao Macro** (https://github.com/macro-inc/macro) para pequenas e médias empresas, rodando em “qualquer VPS barata” (com limites realistas). Direção indicada: **debloat** das dependências proprietárias/pesadas do Macro original e **reimplementação gradual** das funcionalidades usando **Rust + PostgreSQL**, com **UI reescrita em Svelte 5** (empacotamento Tauri para desktop/mobile em fases posteriores).

## Restated idea
Reenquadrar estratégia, objetivo e roadmap da Flora em torno de um **MVP “Macro-like” enxuto para PMEs em VPS barata**: identificar o subconjunto 80/20 de capacidades do Macro que entrega valor real, mapear o que deve ser removido/substituído/adiado por custo operacional ou dependência proprietária/pesada, e definir o próximo incremento verificável (documental + técnico) que aproxima a Flora de um Seed utilizável e competitivo nesse nicho.

## Source and type
- **Source:** solicitação direta do usuário na sessão; constituição/artefatos existentes; evidências do repositório Flora; README/Cargo.toml/package.json/LICENSE públicos do Macro; pesquisas em andamento (subagentes em background).
- **Type:** assessment estratégico de produto/arquitetura + consolidação de direção (pré-spec/planejamento do ciclo), com foco em 80/20 e viabilidade self-hosted.

## Context (evidência conhecida)
- **Macro (referência):** workspace “all-in-one” (email, chat/canais/DMs, docs colaborativos, tarefas, canvas, agents/AI, calls, CRM, databases, calendar, storage, PRs, memória unificada/@links) — AGPL-3.0. Monorepo amplo com muitos crates Rust, múltiplos serviços/apps/pacotes/infra/tooling e frontend TS/JS com ecossistema pesado (Lexical/Loro/wasm/patches etc.), sinalizando alto custo operacional para replicar integralmente em VPS barata. **Detalhes de dependências/self-host:** em apuração (NV parcial).
- **Flora (estado atual):** monólito modular Rust (Axum/SQLx/Postgres) com auth/OIDC parcial, orgs/workspaces/canais/mensagens/tarefas/arquivos/busca/notificações em graus heterogêneos; `apps/frontend` canônico em Svelte 5; compose dev (Postgres/Valkey/Meilisearch) adicionado no ciclo 004; verificações determinísticas locais parciais (2026-11-15) e integrações runtime ponta a ponta majoritariamente NV/incompletas (ver `OBJETIVO.md`, `ROADMAP.md`, specs 001-004).
- **Constituição:** debloat/vendor-unlock vs Macro, “VPS test”, Rust+Postgres, Svelte 5 + Tauri futuro, tiers Seed→Forest, não-objetivos de escopo pesado prematuro.

## First-glance unknowns / constraints (NV ou em pesquisa)
- Subconjunto 80/20 de features Macro para PMEs; requisitos mínimos reais de CPU/RAM/disco.
- Substituições de baixo custo (Postgres FTS vs Meilisearch, realtime/filas/storage/auth) por perfil.
- Conformidade AGPL em reimplementação limpa vs fork de código.
- Resultados pendentes de subagentes: mapeamento Flora↔visão, análise Macro/debloat, padrões cheap-VPS/self-hosted.

## Desired outcome (ciclo 005)
- Assessment 005 (intake/research/decision, **go**) e spec/plan/tasks do ciclo.
- `OBJETIVO.md` e `ROADMAP.md` atualizados com norte “alternativa self-hosted ao Macro para PMEs em VPS barata”, princípios de debloat e MVP Seed proposto.
- Pesquisa consolidada em `docs/research/2026-11-15-macro-alternative-80-20-research.md` (com incorporação incremental de achados dos subagentes).
- Próximas ações técnicas verificáveis definidas, sem alegar implementação concluída sem evidência.
