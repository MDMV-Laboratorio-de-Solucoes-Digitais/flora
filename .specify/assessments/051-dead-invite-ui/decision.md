# Decision: 051 — Dead invite UI

**Verdict:** go (scoped)

## Decisão

Remover a affordance de convite sem backend e marcar o cenário obsoleto na spec 002.
Nenhuma decisão de autorização é tomada aqui: nenhum endpoint é criado.

## Delimitadores

- Apenas 3 arquivos existentes (`UserManager.svelte`, `settings/+page.svelte`, `specs/002/spec.md`) + artefatos do ciclo.
- Sem backend, e-mail, token, RBAC, constituição, ADRs ou workflows.
- PR independente; PR #265 não tocado.

## NV

Intenção futura do dono sobre convites: **[NV]**.
