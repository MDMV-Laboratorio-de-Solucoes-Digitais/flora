# Concept: First Seed Journey local one-time bootstrap

- **Slug**: 085-local-first-seed
- **Created**: 2026-11-20T01:12:00Z
- **Recommended option**: Option A — one-time local bootstrap

## Options

### Option A — One-time local bootstrap
- **Sketch**: na primeira execução de uma instalação vazia, o formulário "First Seed Journey" cria a organização, o administrador, a associação, o papel e um workspace, e já devolve uma sessão utilizável. Qualquer tentativa posterior é recusada sem gravar dados. O usuário entra direto no workspace, sem IdP.
- **Appetite**: medium (contrato de API, guarda atômica, sessão, UI e testes).
- **Trade-offs**: entrega onboarding utilizável no Seed; sacrifica a prova de identidade via ID token e adiciona uma rota não autenticada que precisa de guarda rigorosa.
- **Rabbit holes**: definir "vazio" sob concorrência; emitir sessão com claims corretos; evitar duplicatas de slug; decidir como desabilitar a rota sem quebrar quem a descobre.

### Option B — Exigir Zitadel real
- **Sketch**: manter o botão como redirect OIDC e configurar um provedor externo (issuer, client, redirect allowlist) antes de usar a aplicação.
- **Appetite**: small em código, large em operação (dependência externa, credenciais, config do client).
- **Trade-offs**: preserva identidade verificada e não abre nova superfície; mas não funciona no ambiente atual e contradiz o perfil Seed Postgres-first.
- **Rabbit holes**: provisionar client/redirect no IdP, segredos, disponibilidade do provedor.

### Option C — Atalho apenas no frontend (gerar sessão no navegador)
- **Sketch**: o botão usa um token derivado de segredo local para entrar, sem endpoint novo.
- **Appetite**: small.
- **Trade-offs**: resolve a demo rapidamente; quebra a cadeia de confiança do servidor e equivale a expor credencial no bundle.
- **Rabbit holes**: nenhum — deve ser descartado por segurança.

## Recommendation

Option A. É a única que resolve o problema observado (instalação nova sem IdP) preservando a autoridade do servidor sobre sessão e dados; a guarda one-time limita a superfície não autenticada. Os critérios de sucesso do problema — entrada em `/tasks` e rejeição de reexecução — são diretamente observáveis nesta opção.

## Out of Scope (for the recommended option)

- Alterar login/callback OIDC já existente.
- Verificação de e-mail, convites, multi-tenant posterior ao bootstrap.
- Tauri, perfis Sprout/Grove, search/messaging.
- Expor o endpoint em instalação com dados existentes.

## Assumptions to Validate

- Banco vazio pode ser definido como "zero organizações" e atomizado no mesmo insert da primeira organização.
- Servidor já tem tudo para criar Role/Membership/Workspace e emitir JWT/sessão sem novo componente.
- O frontend consegue chamar o endpoint antes de qualquer autenticação e persistir a resposta com `AuthState.login()`.
- A rota deve continuar existindo como erro explícito (409), não desaparecer por config, para ser testável.
