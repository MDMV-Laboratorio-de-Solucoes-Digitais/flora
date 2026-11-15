# Plan: 005 — Macro Alternative MVP Reframe & 80/20 Direction

**Branch:** `[005-macro-alternative-mvp]`  
**Spec:** `specs/005-macro-alternative-mvp/spec.md`  
**Decision:** `go` (`.specify/assessments/005-macro-alternative-mvp/decision.md`)

## Summary

Ciclo de alinhamento estratégico e planejamento 80/20: consolidar evidências (Flora + Macro + comparáveis cheap-VPS), atualizar direção documental e definir o próximo incremento técnico como fatia vertical verificável do MVP Seed (PME, VPS barata, Rust+Postgres, Svelte 5).

## Approach

1. **Consolidar pesquisa:** sintetizar achados de subagentes (codebase, Macro/debloat, padrões cheap-VPS/Svelte+Tauri) em `docs/research/2026-11-15-macro-alternative-80-20-research.md` com fontes e NV.
2. **Atualizar direção:** revisar `OBJETIVO.md` e `ROADMAP.md` para refletir norte, MVP P0/P1/P2, princípios de debloat (Postgres-first, componentes opcionais) e gaps reais priorizados.
3. **Registrar SpecKit do ciclo:** assessment 005 + spec/plan/tasks/checklist com escopo, critérios e handoff para implementação.
4. **Definir próximo slice técnico (handoff):** escolher 1 fluxo núcleo verificável (recomendado: consistência auth/contexto nas rotas críticas + “workspace → canal → mensagem” ou “tarefa” ponta a ponta em dev) com passos de smoke e evidência.

## Proposed next technical slice (Cycle 006 candidate)

**Tema:** “Seed utilizável — fatia vertical mínima segura”  
**Escopo candidato:**
- Unificar autenticação/autorização/contexto de tenant nas rotas núcleo (eliminar confiança cega em headers onde aplicável; contrato claro para dev).
- Alinhar contrato mínimo frontend↔backend para o fluxo escolhido (ou script de smoke API se frontend atrasar).
- Garantir persistência e leitura coerente do fluxo (mensagem ou tarefa) com evidência em ambiente com Postgres/serviços necessários.
- Documentar dependências opcionais e comandos de verificação (NV quando não executado).

**Critérios de saída (a validar no ciclo técnico):**
- Passos reproduzíveis documentados.
- Evidência executada (logs/comandos/resultados) para o caminho feliz.
- Sem regressão nas verificações determinísticas aplicáveis (fmt/check/test/clippy conforme ambiente).

## Dependencies & prerequisites
- Docker/Compose e serviços (Postgres mínimo; Valkey/Meilisearch/OIDC conforme slice) para smoke runtime — NV neste ciclo documental.
- Decisões de produto/UX para escopo exato do slice (pode ser refinado em intake do ciclo 006).

## Risks & mitigations
- **Pesquisa incompleta:** manter NV e atualizar consolidado incrementalmente.
- **Scope creep para paridade Macro:** limitar P2 explicitamente; priorizar P0 verificável.
- **Ambiente indisponível para runtime:** aceitar ciclo documental com handoff verificável e critérios claros.
