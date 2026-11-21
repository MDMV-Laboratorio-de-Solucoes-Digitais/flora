# Intake: 099 — interface 100% pt-BR

**Data:** 2026-11-21T00:00Z · **Âncora:** `feature/099-ptbr-ui@7183a33fc` · `.specify/feature.json` → `specs/099-ptbr-ui`

## Sinal

Requisito do usuário: **100% pt-BR na interface**. Outras línguas só são
permitidas em termos técnicos consagrados (OIDC, OAuth, API, JWT, REST, AI, SSO,
slug). O levantamento do I18nScout apontou ~140 strings literais em inglês
distribuídas em ~15 arquivos de `apps/frontend`, sem nenhum mecanismo de
internacionalização (nenhum módulo `i18n`, dicionário ou helper de tradução).
`apps/frontend/src/app.html:2` já declara `lang="pt-BR"` — o atributo está correto
e precisa ser preservado, não alterado. Gate do alvo registrado como **PASS** no
handoff consolidado.

## Evidência

```
apps/frontend/src/app.html:2                     lang="pt-BR" (já correto)
I18nScout (handoff)                              ~140 literais ingleses em ~15 arquivos
grep -ri18n em apps/frontend/src                 0 módulos de tradução no baseline
apps/frontend/src/routes/(app)/+layout.svelte:129,137,145   Channels | DMs | Tasks
apps/frontend/src/routes/(app)/+layout.svelte:155,158       Channels / "Open Channels to browse..."
apps/frontend/src/routes/(app)/+layout.svelte:189,195,206   My Account | Settings | Log out
apps/frontend/src/routes/(app)/+layout.svelte:255,270       AI Insight Panel / "How can I help you today?"
apps/frontend/src/routes/(app)/tasks/+page.svelte:136,170,191,196,198
                                                 tasks / No workspaces available / Create task / Loading...
apps/frontend/src/routes/(app)/channels/+page.svelte:114,117,129,151,170,172,196
                                                 Channels / New channel / No channels yet. / No channel selected
apps/frontend/src/routes/(auth)/callback/+page.svelte:69     "Authenticating..." (tela de callback)
apps/frontend/src/lib/components/chat/ChatLayout.svelte:104,195,208,225
                                                 "Select a channel..." / "No messages yet." / "Write a message…" / Send
apps/frontend/src/lib/components/search/SearchBox.svelte:55,56,78
                                                 aria-label/placeholder em inglês / "No results for..."
apps/frontend/src/lib/components/files/FileUpload.svelte:20,24,50
                                                 "File too large..." / "Workspace is required..." / "Upload failed."
apps/frontend/src/lib/components/workspace/WorkspaceCreate.svelte:31,36
                                                 "Create Workspace" / "Create"
apps/frontend/src/lib/components/organization/OrganizationManager.svelte:33,42,46
                                                 "Create Organization" / "Organization Name" / "Create"
apps/frontend/src/lib/components/tasks/kanban.svelte.ts:13-16  To Do | In Progress | Done
apps/frontend/src/lib/utils/search.ts:1-6         RESULT_LABELS = Message | Task | File
apps/frontend/src/routes/(auth)/login-error.svelte.spec.ts:48+
                                                 seletores e asserções já em pt-BR (ciclo 096)
```

A tela de login `(auth)/+page.svelte` e o spec de login já estavam em pt-BR desde
o ciclo 096; a lacuna está no shell do app, nas páginas autenticadas e nos
componentes compartilhados, além dos textos de erro devolvidos pelo backend.

## Autorização

Modo autônomo. Decisão `go` registrada em
`.specify/assessments/099-ptbr-ui/decision.md` antes da implementação.
Escopo: um dicionário pt-BR centralizado, preservação de `lang="pt-BR"`,
tradução de auth/callback, layout/navegação, páginas do app e componentes
compartilhados, mapeamento pt-BR dos erros do backend e atualização dos testes e
selectors. Oito requisitos (FR-001..FR-008), três critérios (SC-001..SC-003) e
dez tarefas (T001..T010).

## Fronteira

Somente `apps/frontend` e os artefatos do ciclo
(`.specify/assessments/099-ptbr-ui/` e `specs/099-ptbr-ui/`). Sem toques em
`.specify/validations/`, `.specify/releases/`, `.specify/memory/STATE.md`,
`docs/PROGRESSO*`, `specs/*/analyze.md` ou `specs/*/converge.md`. Nenhuma mudança
de gate, workflow, ADR ou constituição. IdP externo, deploy remoto e
comportamento em produção permanecem **[NV]** — a verificação é local e por
inspeção.
