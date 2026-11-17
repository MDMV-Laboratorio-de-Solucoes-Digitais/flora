# Define: 051 — Dead invite UI

## Problema

A página de configurações oferece um formulário de convite que sempre falha (404)
e esconde o erro do usuário. Isso é uma falsa affordance e ruído silencioso.

## Requisitos funcionais

- **FR-001**: A página de settings NÃO DEVE mais renderizar o formulário de convite.
- **FR-002**: Nenhum arquivo frontend DEVE conter a chamada `/api/v1/users/invite`.
- **FR-003**: O cenário obsoleto em `specs/002-.../spec.md:34` DEVE ser marcado como
  não implementado, preservando o texto original (append-only no local).
- **FR-004**: A remoção NÃO DEVE alterar OrganizationManager, WorkspaceCreate ou layout geral.

## Não-objetivos

- Não implementar convites, e-mail ou token.
- Não alterar backend, autorização, constituição, ADRs ou workflows.

## Critérios de sucesso

- `rg 'users/invite|UserManager' apps/frontend/src` → 0 ocorrências.
- Gates frontend (check/lint/test/build) e Rust verdes.
