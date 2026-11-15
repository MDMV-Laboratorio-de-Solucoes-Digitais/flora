# OBJETIVO — Flora (Seed v0.1)

**Data da consolidação:** 2026-11-15  
**Escopo:** repositório `flora` (`/home/luis/development/MDMV/projetos/Arquivados/flora`, branch `dev`, workspace Rust modular monolith + apps frontend).  
**Base desta versão:** inspeção local do repositório e artefatos SpecKit (`README.md`, `AGENTS.md`, `.specify/memory/constitution.md`, `specs/001-003`, `.specify/assessments/003-flora-80-20-cycle`), verificações locais executadas em 2026-11-15, e pesquisa externa atualizada (fontes primárias/oficiais e guias reputados; ver `docs/research/2026-11-15-flora-seed-80-20-research.md`).

## 1) Objetivo do produto (o que a Flora quer ser)

Construir uma **suíte corporativa/colaborativa self-hostable, modular e open-source (AGPL-3.0)** em **Rust**, com foco em controle de dados/infraestrutura por organizações e implantação viável em VPS barata, evoluindo de instalação mínima até topologia distribuída sem trocar o ecossistema tecnológico.

- **Visão (README/constituição):** “Each installation is a seed. Each module is a tree. Each deployment makes the ecosystem more fertile.”
- **Stack/arquitetura alvo:** monólito modular Rust (Axum, SQLx, PostgreSQL; Valkey/PGMQ/Redis conforme capacidades; Meilisearch opcional; OIDC/Zitadel; storage local/S3-compatível), frontend canônico **SvelteKit/Svelte 5** (`apps/frontend`, spec 002), Tauri em fases futuras.
- **Níveis de implantação previstos:** Seed (mínimo) → Sprout → Grove → Forest.

## 2) Função prática pretendida (Seed v0.1)

Entregar, de forma utilizável para uma organização pequena/PME piloto, o núcleo:
- organizações/workspaces e identidade (OIDC) com isolamento multi-tenant e RBAC;
- canais, mensagens e threads (comunicação interna básica);
- tarefas;
- arquivos (upload/compartilhamento/quota, conforme evolução);
- busca global (mensagens/tarefas/arquivos, quando indexação integrada);
- notificações in-app.

## 3) Quanto do caminho já percorremos (estado consolidado em 2026-11-15)

### 3.1 Artefatos, especificação e governança (evidência documental)
- **Constituição:** `.specify/memory/constitution.md` v1.2.0 (ratificada 2026-08-17, emendada 2026-08-22) — princípios, stack, roadmap e governança definidos.
- **Specs:**
  - `specs/001-flora-seed-v0-1`: spec/plan/tasks abrangentes (muitas tarefas marcadas `[X]`; `T116` PKCE ainda `[ ]` no tasks.md, mas tratada no ciclo 003 — drift documental).
  - `specs/002-svelte-frontend-rewrite`: spec/plan/tasks/checklists com fases/tarefas marcadas concluídas (convergência documentada; requer revalidação fresca).
  - `specs/003-flora-80-20-baseline` (2026-11-15, **go**): baseline, PKCE, alinhamento frontend/CI/docs e guardrails de escopo; parcialmente concluído com itens `[~]`.
- **Assessment 003:** `.specify/assessments/003-flora-80-20-cycle/{intake,research,decision,baseline}.md` presentes (go restrito). **Assessment 004** iniciado neste ciclo para o próximo passo 80/20 (ver `.specify/assessments/004-seed-runnable-baseline/`).
- **Bug flow SpecKit:** não utilizado no repositório (`.specify/bugs/` ausente; sem artefatos `bug.assess/fix/test`).

### 3.2 Implementação real observada no código (evidência de inspeção)
- **Entrypoint real:** `flora-api` carrega config, cria pool Postgres, executa migrações, monta `AppState` e serve router Axum; `src/main.rs` da raiz é placeholder.
- **Domínio/infra base:** `flora-core` concentra config, DB/migrations, modelos, repositórios, serviços, traits e storage; rotas em `flora-api/src/routes/*`.
- **Capacidades presentes (parciais/heterogêneas):** health endpoints (placeholders), fluxo auth/OIDC com PKCE/armazenamento de transação e emissão/refresh/revogação de JWT (com limitações), CRUD/rotas básicas para orgs/workspaces/canais/mensagens/tarefas, registros de arquivos (metadados/quota parcial) e consulta de busca/notificações; websocket com persistência/publicação parcial (sem fanout completo de assinaturas para clientes).
- **Integrações:** Postgres obrigatório no bootstrap/migrações; Valkey/Redis e Meilisearch configuráveis (Meilisearch inicializado no estado conforme config; Valkey opcional em partes do fluxo); OIDC depende de provedor configurado.

### 3.3 O que funciona de forma verificada (não inferido) — verificação local 2026-11-15
Executado no workspace (sem serviços externos provisionados neste ambiente):
- `cargo check -p flora-api -p flora-core` — **sucesso**.
- `cargo fmt --all -- --check` — **sucesso**.
- `cargo test --workspace --no-run` — **sucesso** (compilação dos alvos de teste).
- `cargo test -p flora-core --lib` — **8 passed, 0 failed**.
- `cargo test -p flora-tests --lib` — **54 passed, 0 failed**.

Baseline 003 (registrado em `.specify/assessments/003-flora-80-20-cycle/baseline.md`, 2026-11-15): `cargo clippy --workspace --all-targets -- -D warnings` e `cargo test --workspace` reportados como passando no ambiente de baseline (90 passed, 0 failed, 1 ignored doctest); `apps/frontend` `install/check/build` passando, com lint pós-formatação/testes de unidade (browser/Playwright) parcialmente inconclusivos por limitações de ambiente (ENOSPC/instalação Playwright). **CI de frontend** (`.github/workflows/ci.yml`) e **CI Rust** (`.github/workflows/rust.yml`) existem e definem os comandos esperados, mas a execução remota/CI verde completa não foi revalidada nesta sessão.

### 3.4 O que deveria funcionar / está incompleto ou desalinhado (gaps prioritários)
- **Runtime end-to-end não verificado localmente:** fluxos que dependem de Postgres/Valkey/Meilisearch/OIDC, migrações em ambiente real e testes de integração/contrato com serviços.
- **Baseline de execução local “seed”:** ausência de `docker-compose`/stack dev reproduzível no checkout para subir dependências rapidamente (há `dev-seed.sql`, mas não composição de serviços).
- **Auth/OIDC:** PKCE implementado no código (ciclo 003), porém faltam coberturas de sucesso completo com provedor mockado/JWKS e alguns cenários (estado expirado explícito em teste), verificação de assinatura JWKS do ID token permanece limitada conforme baseline; armazenamento de transação em memória é fallback process-local.
- **Consistência de autenticação/autorização na API:** mistura de extração JWT e headers `x-user-id`/`x-organization-id` em rotas; RBAC/gerenciamento de papéis exposto parcialmente (endpoints de roles placeholders em partes).
- **Realtime/mensageria/notificações:** publish/persistência existem em pontos, mas falta fanout/assinatura efetiva para clientes WS e integração completa de eventos→notificação/indexação.
- **Busca:** endpoint de consulta existe; integração de indexação em create/update/delete e filtros completos (ex.: intervalo de data) não estão plenamente evidenciados/integrados.
- **Arquivos:** foco em metadados/validação/quota parcial; upload/download binário e contratos frontend↔backend apresentam desalinhamentos conhecidos.
- **Higiene documental:** README roadmap/checklists e marcações de tasks em specs apresentam drift vs estado verificado; `apps/web` permanece no repositório como não-canônico.

### 3.5 O que ainda não está implementado (além dos gaps acima)
- Itens do **roadmap Seed v0.1** ainda não comprovados como concluídos ponta a ponta (README com checkboxes em aberto).
- Fases futuras (Documentos, Email, Integrações, AI feature flags, Tauri, etc.) e capacidades avançadas de colaboração/escala (Forest, HA completa, etc.).
- Fluxo de bugs do SpecKit (`.specify/bugs`) e artefatos formais de análise/convergência em todos os pontos esperados.

### 3.6 O que o projeto NÃO faz (não-objetivos / limites)
- **Fora do Seed v0.1 (spec 001):** integrações de IA, CRM, calendário, chamadas áudio/vídeo, MCP/agents, cliente de e-mail, “clone do GitHub”, etc.
- **Guardrails (spec 003/assessment 003):** sem paridade total com Macro; sem stack pesada para Seed (Kafka/OpenSearch/FusionAuth/DO/agent-harness como requisito); sem CRDT-everywhere; sem adotar REA/Craft-Anything como dependências/core do produto; sem engenharia reversa não autorizada/proprietária e sem reuso incompatível com AGPL.
- **Processo:** não declarar “verde” sem evidência executada; não realizar deploy/infra de produção, merges/releases, manuseio de segredos ou adição de serviços pagos/proprietários sem autorização explícita.

## 4) Direção 80/20 (próximo ciclo)

**Próximo ciclo selecionado (004):** estabelecer uma **baseline local reproduzível do “Seed”** (composição de dependências de desenvolvimento, configuração e instruções verificáveis) para destravar validação prática ponta a ponta, reduzir atrito de onboarding e permitir ciclos seguintes focados em corrigir integrações críticas com evidência. Detalhes e decisão em `.specify/assessments/004-seed-runnable-baseline/` e `specs/004-seed-runnable-baseline/`.

## 5) Critérios de sucesso (horizonte imediato)

1. Um desenvolvedor consegue subir dependências locais e iniciar a API com passos documentados e reproduzíveis.
2. Baselines determinísticas (fmt/clippy/check/tests aplicáveis) permanecem rastreáveis e reproduzíveis.
3. Próximas entregas focam em **fatias verticais finas** do fluxo núcleo (auth→org/workspace→mensagem/tarefa) com evidência de execução, antes de ampliar escopo/paridade.
