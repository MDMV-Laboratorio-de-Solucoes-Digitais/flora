# Implementation Plan: 099 — interface 100% pt-BR

**Branch**: `feature/099-ptbr-ui` @ `7183a33fc` | **Date**: 2026-11-21 |
**Spec**: [spec.md](./spec.md)

**Input**: `.specify/assessments/099-ptbr-ui/decision.md` (go) e handoff I18nScout

## Summary

Centralizar todo o texto visível da interface em um dicionário pt-BR tipado
(`lib/i18n/pt-BR.ts`), substituir os ~140 literais em inglês das páginas, do
shell, dos componentes compartilhados e dos módulos derivados, e mapear os erros
do backend para mensagens pt-BR. `app.html` já declara `lang="pt-BR"` e permanece
inalterado. Nenhum contrato de API, rota ou fluxo de autenticação é modificado;
IdP externo e produção permanecem **[NV]**.

## Technical Context

**Root cause**: ausência de camada de tradução — literais inline em
`routes/(app)/+layout.svelte:129-270`, páginas autenticadas, componentes
compartilhados e módulos `.ts` (`kanban.svelte.ts:13-16`, `utils/search.ts:1-6`).
`app.html:2` já está `lang="pt-BR"`.

**Target Path**: `apps/frontend/src/lib/i18n/{pt-BR,index}.ts`,
`apps/frontend/src/routes/**`, `apps/frontend/src/lib/components/**`,
`apps/frontend/src/lib/components/tasks/kanban.svelte.ts`,
`apps/frontend/src/lib/utils/search.ts` e
`apps/frontend/src/routes/(auth)/login-error.svelte.spec.ts`.

**Testing**: varredura de literais e exceções técnicas; `npm run check`
(svelte-check), `npm run lint` (prettier + eslint) e `npm run test`
(vitest + playwright). Gate do alvo registrado como PASS no handoff; ambiente
remoto/IdP **[NV]**.

**Constraints**: zero literais em inglês na UI ativa; apenas OIDC, OAuth, API,
JWT, REST, AI, SSO e slug como exceções; selectors pt-BR do ciclo 096 preservados;
nenhuma mudança de contrato de API, gates, workflows, ADRs ou constituição; sem
toques em `analyze/`, `converge/`, `validate/`, `release/`, `STATE.md` ou
`PROGRESSO*`.

## Steps

1. **Dicionário** — criar `lib/i18n/pt-BR.ts` (`t` com `as const`,
   `TranslationKey`) e `lib/i18n/index.ts` (`format`) (T001).
2. **Documento** — confirmar `lang="pt-BR"` em `app.html:2`, sem regressão
   (T002).
3. **Autenticação** — migrar `(auth)/+page.svelte` e
   `(auth)/callback/+page.svelte` ao dicionário, preservando textos/selectors
   (T003).
4. **Shell** — traduzir `(app)/+layout.svelte` e `routes/+layout.svelte`,
   incluindo `title`/`aria-label` e painel de IA (T004).
5. **Páginas** — traduzir `tasks`, `channels`, `dms`, `select-org`, `settings` e
   `demo/**` (T005).
6. **Componentes** — traduzir chat/thread, arquivos, organização, busca e
   workspace (T006).
7. **Domínio derivado** — traduzir colunas do Kanban e labels de resultado de
   busca (T007).
8. **Erros do backend** — implementar `backendErrorMessage` com códigos,
   `field === 'admin_password'`, statuses 401/409/422/429 e fallback (T008).
9. **Testes** — manter/ajustar os selectors pt-BR em
   `(auth)/login-error.svelte.spec.ts` (T009).
10. **Fecho** — varredura de zero literais, exceções técnicas, gates/lint/testes
    verdes e rastreabilidade T001..T010 (T010).

## Constitution Check

Sem violações: nenhum required check, gate, contrato de API, rota, workflow, ADR
ou constituição é alterado; nenhum idioma além de pt-BR é introduzido na
interface; o diff fica restrito ao frontend e aos artefatos do ciclo 099.

## Project Structure

```text
.specify/assessments/099-ptbr-ui/{intake,research,define,shape,decision}.md
specs/099-ptbr-ui/
├── spec.md      # FR-001..FR-008 / SC-001..SC-003
├── plan.md      # T001..T010
└── tasks.md     # todas [X]
apps/frontend/src/app.html                         # lang="pt-BR" (inalterado)
apps/frontend/src/lib/i18n/pt-BR.ts                # t + TranslationKey
apps/frontend/src/lib/i18n/index.ts                # t, format, backendErrorMessage
apps/frontend/src/routes/(auth)/+page.svelte       # textos via t
apps/frontend/src/routes/(auth)/callback/+page.svelte
apps/frontend/src/routes/(auth)/login-error.svelte.spec.ts  # selectors pt-BR
apps/frontend/src/routes/(app)/+layout.svelte      # menu/conta/painel de IA
apps/frontend/src/routes/(app)/{tasks,channels,dms,select-org,settings}/+page.svelte
apps/frontend/src/routes/demo/**                   # páginas de demonstração
apps/frontend/src/lib/components/{chat,files,organization,search,tasks,workspace}/**
apps/frontend/src/lib/components/tasks/kanban.svelte.ts      # colunas
apps/frontend/src/lib/utils/search.ts                        # labels de tipo
```

## Complexity Tracking

Nenhuma: um dicionário `as const`, um helper de interpolação, um mapeador de
erros e substituições de texto em consumidores já existentes. Nenhuma nova
dependência, biblioteca de i18n, detecção de idioma ou mudança de contrato.
