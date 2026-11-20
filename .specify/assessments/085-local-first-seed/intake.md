# Idea Intake: First Seed Journey local one-time bootstrap

- **Slug**: 085-local-first-seed
- **Created**: 2026-11-20T01:06:22Z
- **Source**: pasted text (decisão do usuário em resposta a opções apresentadas) + repo path `apps/frontend/src/routes/(auth)/+page.svelte`
- **Type**: new-capability

## Idea (as captured)

> "Implementar First Seed Journey local: endpoint one-time (só com banco vazio) cria org/admin/workspace e já autentica. É uma feature nova, com teste e ADR/registro de decisão. (Passar por todo o processo, passo a passo, de /speckit.assess.intake .. .. .. .. /speckit.implement .. /speckit.converge ++)"

Contexto imediato registrado na conversa: o botão "Sprout Workspace" exibia `Unable to start sign-in. OIDC provider error: redirect_uri is not allowed` após correção de feedback de erro na UI.

## Restated

Propor que a tela inicial de uma instalação nova consiga criar a primeira organização, o administrador e o workspace sem depender de um provedor OIDC externo, com uma única execução permitida apenas enquanto o banco estiver vazio. A feature deve passar pelo ciclo SDD completo, incluindo avaliação, especificação, implementação, converge e registro de decisão.

## Origin & Context

- **Raised by**: usuário (SE), decisão explícita após diagnóstico do erro de login
- **Trigger**: clique em "Sprout Workspace" não concluía o fluxo: primeiro silenciosamente (503 engolido), depois com erro visível; não há Zitadel local e o `mock_oidc` é placeholder

## First-Glance Unknowns

- [NEEDS CLARIFICATION: o que conta como "banco vazio" — zero organizações, zero usuários ou ambos?]
- [NEEDS CLARIFICATION: como a sessão retornada pelo bootstrap é persistida no frontend?]
- [NEEDS CLARIFICATION: qual é a relação entre este fluxo e o OIDC existente em instalações com provedor configurado?]
- [NEEDS CLARIFICATION: quais papéis e workspace padrão devem ser criados?]
