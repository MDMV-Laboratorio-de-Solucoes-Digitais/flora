# 040 — Seed UI debloat + board real: decisão 80/20 e evidência

**Data:** 2026-11-17 · **Tipo:** decisão 80/20 + auditoria · **Metodologia:** hub
`mdmv-dark-factory` aplicado ao Flora com fluxo enxuto (spec→tasks→implement→validate→
release), gates locais como autoridade (constituição V), nenhum gate do hub importado
"as-is" (pesquisa 036).

## 1. Decisão 80/20

Com o ciclo 039 tendo fechado a NV-1 de backup/restore (prova executada de VPS), o
próximo gargalo do objetivo "alternativa performática ao Macro em VPS barata" é o
**peso do checkout/frontend**: o repo carrega 5.844 arquivos / 43 MB do cliente Macro
original (`apps/web`, SolidJS + Lexical + CRDT + Tauri) que zero código do target usa.

| Hipótese | Impacto | Confiança | Esforço | Veredito |
|---|---|---|---|---|
| H1 — Remover `apps/web` + e2e mortos | Alto (debloat real) | Alta | Baixo | **FEITO** |
| H2 — Kanban real (props + PATCH status) | Médio (paridade Macro) | Alta | Baixo | **FEITO** |
| H3 — Ligar `RbacService` às rotas | Alto | Média | Médio | Não (muda authz, decisão de escopo) |
| H4 — Remover crates dead (argon2/bcrypt/dotenv/rust-i18n) | Médio | Alta | Médio | Próximo ciclo |
| H5 — Tauri / paridade Macro / Aroeira | Alto | Baixa | Alto | Não (ADR-0002, ROADMAP §7) |

**Autonomia:** ADR-0003 torna push/PR/merge decisão de máquina sob 4 required checks
verdes (verificados ao vivo: `frontend-build`, `rust-build`, `db-tests`,
`dependency-review`; `enforce_admins=true`, `required_reviews=0`). Não escalado nada.

## 2. Evidência primária (10 subagentes)

- **`apps/web`** — importado em `82ee437f2` (2026-08-18) do Macro original. Zero refs
  funcionais: nenhum workspace bun root, nenhum workflow, nenhum `vite.config.ts`/
  `tsconfig`/`package.json`/Dockerfile referencia `@coparse`. Risco = só drift
  documental + uma exceção órfã no `.gitignore`. [c040-01, c040-08]
- **E2e mortos** — `playwright.config.ts` usa `testMatch: '**/*.e2e.ts'`; `tasks.spec.ts`
  e `files.spec.ts` nunca executavam e usavam seletores obsoletos. [c040-05]
- **Kanban órfão** — `KanbanBoard.svelte` era 100% mock (tarefas t1–t4, tags, avatar U)
  com drag só em `$state` local; `TaskStore.svelte.ts` (23 LOC) nunca consumido;
  `ThreadReply.svelte` **não** é órfão (usado em `ChatLayout.svelte:8,181`). [c040-02]
- **API de tarefas** — CRUD completo; `PATCH /api/v1/tasks/{id}` aceita `status`
  (`Todo|InProgress|Done|Archived`, `models/task.rs:118-125`) e `assignee_id` tri-state.
  GET é assignee-scoped (`:149-157`), ordem por `created_at DESC`. **Mover card custa
  0 no backend.** [c040-06]
- **Macro UX** — `docs.macro.com/tasks` → 403 [NV]; `macro.com/tasks` (JSON-LD, 16/11):
  atalho `C`→`T`, propriedades Status/Priority/Assignee/Tags, visões list/board/timeline.
  Mínimo PME = criar com assignee+status, board, filtro, "minhas". [c040-03]
- **Svelte 5** — runes só em `.svelte.ts`; classe `$state` + singleton é o padrão
  correto; armadilha: `export let x = $state()` perde reatividade entre módulos.
  Projeto `server` do Vitest compila `.svelte.ts` com runes (prova: `TaskStore.test.ts`),
  e é o único que roda no gate `frontend-build`. [c040-04, c040-05]
- **Footprint** — 43 MB / 5.844 arquivos (27,7 MB aparentes); zero impacto no CI
  (ninguém instalava apps/web) — ganho é I/O de checkout em VPS. Crates dead confirmadas
  (argon2/bcrypt/dotenv/rust-i18n, ~40–60 crates no grafo); `flora-search` **não** é dead. [c040-08]

## 3. Correções metodológicas aplicadas do hub (síntese 00)

- LLM-judge banido como gate (7–17% flips, 25–71%/62–91% sob pushback); verificador
  author-blind em contexto fresco, contrato congelado antes do código.
- Nível 5 não verificado; degrau real L3.5 com gate de evidências por classe.
- SDD condicional: cerimônia só para mudança estrutural (esta é), gates curtos para
  limpeza mecânica (remoção de apps/web).
- Métrica = escaped defects / custo por mudança aceita, nunca LOC.

## 4. Verificação

Ver `.specify/validations/040-seed-ui-debloat/validate.md`. Gates frontend idênticos
ao baseline (check/lint/36 testes/build); gates Rust verdes; auditoria fresh-context
anexa. Nenhuma mudança de ADR, required checks, deploy ou release.
