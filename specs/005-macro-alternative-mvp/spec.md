# Feature Specification: 005 — Macro Alternative MVP Reframe & 80/20 Direction

**Feature Branch**: `[005-macro-alternative-mvp]`  
**Created**: 2026-11-15  
**Status**: Draft (assessment-backed, go — cycle 005)  
**Assessment**: `.specify/assessments/005-macro-alternative-mvp/{intake,research,decision}.md`  
**Research**: `docs/research/2026-11-15-macro-alternative-80-20-research.md`

## Context

A Flora é um fork/debloat em evolução do Macro, com núcleo Rust+Postgres parcialmente implementado e UI canônica Svelte 5 em estágio inicial. O Macro é um workspace “all-in-one” amplo (email, chat, docs, tarefas, calls, agents/IA, CRM, databases, calendar, storage, PRs) com arquitetura e dependências pesadas para operação em VPS barata. Para se tornar alternativa viável para PMEs em VPS modesta, a Flora precisa de recorte explícito (MVP 80/20), princípios de debloat e um próximo incremento técnico verificável.

## Goal

Consolidar o **reframe estratégico** e o **plano 80/20** deste ciclo:
- definir o norte “alternativa self-hosted ao Macro para PMEs em VPS barata”;
- explicitar MVP Seed, debloat/substituições e não-objetivos;
- atualizar artefatos de direção (`OBJETIVO.md`, `ROADMAP.md`, pesquisa, assessment/spec/plan/tasks);
- preparar o próximo ciclo técnico como **fatia vertical verificável** do núcleo (sem implementar paridade ampla neste ciclo).

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Direção estratégica clara (Priority: P1)
Como mantenedor/contribuidor, quero entender o posicionamento, o MVP e os limites da Flora para priorizar trabalho de alto impacto e baixo custo operacional.

**Independent Test:** `OBJETIVO.md`, `ROADMAP.md` e assessment 005 descrevem norte, MVP 80/20, debloat e não-objetivos com distinção entre verificado e NV.

**Acceptance Scenarios:**
1. **Given** os documentos atualizados, **When** um contribuidor lê o objetivo/roadmap, **Then** identifica o público-alvo (PMEs, VPS barata), o MVP Seed e o que está fora de escopo.
2. **Given** o assessment 005, **When** consulto intake/research/decision, **Then** encontro veredito `go`, evidências, lacunas NV e próximos passos rastreáveis.

### User Story 2 — Próximo incremento técnico definido (Priority: P1)
Como mantenedor, quero um próximo ciclo técnico bem recortado (fatia vertical verificável) para avançar de documentação para uso real.

**Independent Test:** `specs/005-macro-alternative-mvp/{plan,tasks}.md` define um slice, critérios de verificação e dependências/pré-condições explícitas.

**Acceptance Scenarios:**
1. **Given** o plano/tarefas 005, **When** inicio o próximo ciclo, **Then** sei qual fluxo mínimo validar (ex.: auth/contexto → workspace → canal/mensagem ou tarefa) e quais evidências coletar.
2. **Given** restrições de ambiente, **When** uma verificação não puder ser executada, **Then** ela é marcada como NV com comando/condição esperada.

### User Story 3 — Base de pesquisa consolidada (Priority: P2)
Como mantenedor, quero uma síntese confiável de Macro, comparáveis e estado da Flora para embasar decisões futuras.

**Independent Test:** `docs/research/2026-11-15-macro-alternative-80-20-research.md` existe, cita fontes primárias/oficiais e resume recomendações 80/20.

## Requirements *(mandatory)*

- **FR-001:** Criar assessment 005 (`intake`, `research`, `decision` go) alinhado ao pedido de “alternativa ao Macro para PMEs em VPS barata”.
- **FR-002:** Atualizar `OBJETIVO.md` e `ROADMAP.md` com norte estratégico, estado verificado/NV, MVP 80/20, backlog priorizado e não-objetivos/debloat.
- **FR-003:** Consolidar pesquisa em `docs/research/2026-11-15-macro-alternative-80-20-research.md` com fontes e limitações explícitas.
- **FR-004:** Produzir `specs/005-macro-alternative-mvp/{spec,plan,tasks,checklists/requirements}.md` definindo entregáveis deste ciclo e o próximo slice técnico verificável.
- **FR-005:** Não declarar funcionalidades/runtime como verdes sem evidência executada; marcar itens dependentes de serviços/ambiente como NV quando aplicável.

## Non-goals
- Implementar paridade com Macro (email completo, calls, CRM/databases/canvas, agentes/memória/rotinas avançadas, integrações profundas) neste ciclo.
- Deploy/infra de produção, merges/releases, mudanças de proteções remotas, ou provisionamento com segredos/serviços pagos sem autorização explícita.
- Introduzir dependências proprietárias obrigatórias ou componentes pesados como requisito do Seed.
- Reuso de código incompatível com AGPL ou engenharia reversa não autorizada.

## Risks & Notes
- Pesquisa baseada em fontes públicas e análise estática; detalhes operacionais do Macro/self-host podem variar (NV onde não confirmado).
- MVP é hipótese 80/20: deve ser validado com fatias verticais e, idealmente, feedback de usuários/PMEs.
- Gaps P0 da Flora (auth consistente, e2e realtime/arquivos, contrato frontend↔backend) permanecem trabalho técnico prioritário nos ciclos seguintes.
