# Problem Definition: First Seed Journey local one-time bootstrap

- **Slug**: 085-local-first-seed
- **Created**: 2026-11-20T01:10:00Z
- **Inputs used**: intake.md | research.md | user input

## Problem Statement

Em uma instalação nova do Flora sem provedor OIDC configurado, a pessoa que abre a aplicação não consegue criar a primeira organização e entrar no workspace: o único caminho existente depende de um IdP externo ausente ou de injeção manual de sessão por ferramenta de teste. Isso impede a promessa de "First Seed Journey" de ser um começo utilizável e deixa o botão principal da tela inicial sem efeito útil.

## Affected Users & Stakeholders

- **Users**: administrador de uma instalação nova/Flora Seed sem Zitadel — encontra a tela "First Seed Journey", preenche nome e e-mail e não consegue entrar.
- **Stakeholders**: mantenedores do Flora — precisam preservar o modelo de identidade verificada e a superfície de segurança; operador da instalação — precisa de onboarding sem dependência externa.

## Goals

- Permitir que uma instalação vazia seja provisionada uma única vez a partir da própria UI, sem IdP externo.
- Criar os objetos mínimos necessários (organização, administrador, associação, papel e workspace) de forma atômica e auditável.
- Entregar uma sessão válida ao navegador ao final do bootstrap, sem exigir injeção manual de token.
- Impedir reexecução e uso em instalação que já tenha dados.

## Non-Goals

- Substituir OIDC em instalações que já possuem provedor configurado.
- Verificar posse do e-mail informado fora do fluxo OIDC (não há provedor neste cenário).
- Alterar a arquitetura de perfis Seed/Sprout/Grove nem reintroduzir dependências pesadas.
- Modificar o fluxo de login OIDC existente além da correção de feedback de erro já aplicada.

## Success Metrics

- Clique em "Sprout Workspace" em banco vazio resulta em `/tasks` com `organizationId` preenchido (baseline: erro 503 / sessão injetada manualmente).
- Segunda chamada ao endpoint de bootstrap retorna conflito (baseline: inexistente).
- Em banco com organização existente, endpoint retorna conflito sem gravar nada (baseline: inexistente).
- Testes automatizados cobrem sucesso, reexecução e banco não vazio (baseline: zero testes do fluxo).

## Cost of Inaction

O botão permanece decorativo; toda demo ou instalação sem Zitadel continua dependendo de `dev-seed.sql` + `flora-mint-session` para entrar, e o erro `redirect_uri is not allowed` continua sendo o estado final esperado da interface.

## Open Questions

- [NEEDS CLARIFICATION: "banco vazio" = zero organizações, zero usuários ou ambos?]
- [NEEDS CLARIFICATION: o bootstrap emite token próprio ou apenas cria os dados e delega ao login?]
- [NEEDS CLARIFICATION: papéis padrão e workspace inicial — nomes e slug]
- [NEEDS CLARIFICATION: como o frontend descobre que o bootstrap está disponível/habilitado?]
