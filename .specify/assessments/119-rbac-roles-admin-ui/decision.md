# Decision: UI de administração de papéis RBAC em `/settings`

- **Slug**: 119-rbac-roles-admin-ui
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | define.md | shape.md | decision.md (original 2026-11-23) | specs/119-rbac-roles-admin-ui/{spec,plan}.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `intake.md:9-13` mostra o CRUD backend já funcional (`flora-api/src/routes/rbac.rs`, contrato `3/3`) e `/settings` sem qualquer interface de papéis. |
| Evidence strength | adequate | `research.md:5-16` detalha autorizações por rota, `403` em papéis nativos, 27 permissões canônicas e mapeamento de `409`; execução E2E/Axe real fica no handoff (`spec.md:SC-003`). |
| Value vs. inaction | strong | Fecha a lacuna de administração de `BF-02` dando consumidor real aos endpoints `/api/v1/roles`; sem isso, o CRUD permanece inacessível ao administrador. |
| Feasibility / appetite | adequate | `shape.md:5-11` recompõe padrão já conhecido (camada pura, `ApiClient`, `RoleStore` com `syncContext`/`loadSequence`/`pendingLoad`) e corrige loop de `$effect`; sem schema SQL. |
| Strategic fit | adequate | Alinhado ao fechamento de `BF-02`, mas entrega parcial: convites e atribuição por membro ficam explicitamente para o próximo PR (`define.md:15-16`). |
| Risk posture | adequate | Servidor impõe `403` em papéis nativos e escopo por org; cliente deduplica cargas, descarta respostas obsoletas e mapeia `403`/`409` em `pt-BR` (`shape.md:5-11`). |

## Verdict & Rationale

Veredito histórico **`go` preservado**: os artefatos não contradizem a decisão de 2026-11-23. O scorecard sustenta o `go` pela lacuna de consumidor observável e pelo backend já verificado; o score original **1280 / 3125** (`impacto=4 × confiança=5 × fechamento=4 × (6−esforço=2)=4 × (6−risco=2)=4`, tipo `IMPLEMENTAÇÃO`) permanece registrado. Restrições transportadas: Briefing de Compreensão (`auth`/`rbac`) comprovando permissões canônicas, imutabilidade de papéis nativos no servidor (`403`) e escopo da organização ativa; abertura de 4 issues R6 (`T-001`..`T-004`) sem `dark-factory:eligible`; E2E Playwright com capturas `18-settings-roles-dark.png` e `19-settings-roles-light.png` e WCAG 2.2 AA, anexadas via Gist (zero binários em git); nenhuma alteração de schema SQL ou piso DB (`82` intactos); SemVer `MINOR` (`0.10.1 → 0.11.0`).

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; nenhuma incógnita bloqueante nos artefatos).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: `/settings` não expõe visualização, criação, edição ou exclusão de papéis, embora o backend já ofereça `GET/POST/PATCH/DELETE /api/v1/roles` escopados por organização (`intake.md:9-13`).
- **Chosen approach**: Implementar o consumidor frontend completo — utilitários puros (`utils/roles.ts`), cliente HTTP (`api/roles.ts`), `RoleStore.svelte.ts` com isolamento por contexto e `RoleManager.svelte` em `/settings` (`shape.md:5-9`), preservando a imutabilidade dos papéis nativos imposta pelo servidor.
- **In scope / out of scope**: **In** — CRUD de papéis personalizados, exibição honesta de papéis nativos, mensagens `pt-BR` para `403`/`409`, prevenção de loop de `$effect`, testes Vitest, `svelte-check`, lint/build e E2E WCAG 2.2 AA em temas claro e escuro. **Out** — convite de usuários por e-mail (`/users/invite` não existe) e matriz de atribuição/revogação por membro (`/roles/{id}/assign`/`revoke` sem listagem conectada), mantidos para o próximo PR de `BF-02` (`define.md:15-16`).
- **Success metrics**: SC-001..SC-004 em `specs/119-rbac-roles-admin-ui/spec.md` — dedupe/isolamento do `RoleStore`, `201/200/204` no CRUD, `403` e `409` em `pt-BR`, zero violações WCAG 2.2 AA (incluindo WCAG 2.5.3) em ambos os temas, sobrevivência a `page.reload()`, gate DB `82` pisos intacto.
- **Carried-forward open questions**: `[NV]` deploy em staging/produção (Coolify) e execução real do roteiro E2E com Axe (`spec.md:[NV]`); `[NEEDS CLARIFICATION: qual mensagem pt-BR específica deve substituir o texto genérico de `conflict` no mapeamento de `409`, considerando que `i18n/index.ts` aponta para `error.conflict`?]` (`research.md:16`); `[NEEDS CLARIFICATION: a recarga manual deve ser o único mecanismo de atualização da lista ou haverá sincronização pós-mutação?]` (`shape.md:9`).
