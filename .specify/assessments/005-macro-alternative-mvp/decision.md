# Decision: 005 — Macro Alternative MVP Reframe

**Date:** 2026-11-15  
**Verdict:** **go** (ciclo 005, escopo de reframe estratégico + planejamento 80/20)

## Problem validity
**Adequate.** A Flora precisa de direção explícita e priorização para se tornar alternativa self-hosted viável ao Macro para PMEs em VPS barata; sem recorte/debloat claro, o risco é replicar complexidade operacional incompatível com o público-alvo e com o estado atual do projeto.

## Evidence strength
**Adequate (com lacunas conhecidas).** Fundamentado na constituição, em `OBJETIVO.md`/`ROADMAP.md`/specs 001-004 e verificações locais prévias, e em evidências públicas iniciais do Macro (produto, AGPL, escala do monorepo). Pesquisa profunda (dependências/self-host/debloat detalhado, comparáveis cheap-VPS, mapeamento codebase) está em execução via subagentes e marcada como NV/parcial onde aplicável.

## Value vs inaction
**Strong.** Um norte e MVP bem recortados reduzem desperdício de esforço, orientam contribuições futuras e aumentam chance de entregar um Seed utilizável (valor percebido cedo) antes de investir em features pesadas (email completo, calls, CRM/databases genéricas, agentes avançados).

## Feasibility / appetite
**Adequate.** Este ciclo entrega artefatos de baixo risco (documentação, assessment, spec/plan/tasks de direcionamento e consolidação de pesquisa) sem exigir infra de produção, segredos, deploy ou mudanças irreversíveis.

## Strategic fit / risk posture
**Adequate.** Alinha com constituição (debloat, VPS test, Rust+Postgres, Svelte 5, AGPL) e com o pedido do usuário. Riscos: super-simplificação, decisões baseadas em suposições de demanda, ou drift se pesquisa pendente não for incorporada — mitigados por marcar NV, manter não-objetivos claros e priorizar fatias verticais verificáveis.

## Chosen approach (80/20)
1. Registrar assessment 005 (intake/research/decision, **go**).
2. Atualizar `OBJETIVO.md` e `ROADMAP.md` com norte “alternativa self-hosted ao Macro para PMEs em VPS barata”, princípios de debloat e MVP Seed proposto (chat/mensagens + tarefas + arquivos básicos + org/usuários/auth, com busca simples/Postgres-first quando viável).
3. Criar `specs/005-macro-alternative-mvp/` (spec/plan/tasks/checklist) para consolidar reframe e preparar o próximo incremento técnico.
4. Consolidar achados de pesquisa em `docs/research/2026-11-15-macro-alternative-80-20-research.md` (incorporar resultados de subagentes quando disponíveis).

## In scope (this cycle)
- Documentação estratégica e de planejamento: OBJETIVO/ROADMAP, assessment 005, spec/plan/tasks do ciclo, pesquisa consolidada.
- Definição de MVP/roadmap 80/20 e lista inicial de debloat/substituições/adiamentos.
- Nenhuma mudança que exija autorização explícita (deploy/produção, segredos, serviços pagos/proprietários, merges/releases).

## Out of scope / requires explicit authorization
- Implementação ampla de features, migrações de arquitetura de alto risco, ou alterações de CI/infra remota.
- Reuso de código incompatível com AGPL ou engenharia reversa não autorizada de componentes proprietários.
- Promessas de paridade com Macro ou de suporte a cargas específicas sem medição/evidência.

## Acceptance / traceability
- Evidências: `.specify/assessments/005-macro-alternative-mvp/{intake,research,decision}.md`, `specs/005-macro-alternative-mvp/*`, `docs/research/2026-11-15-macro-alternative-80-20-research.md`, `OBJETIVO.md`, `ROADMAP.md` atualizados.
- Próxima etapa recomendada (ciclo técnico): especificar/implementar fatia vertical mínima verificável do MVP Seed (ex.: auth→workspace→canal→mensagem ou tarefa) com critérios de smoke em ambiente dev reprodutível.
