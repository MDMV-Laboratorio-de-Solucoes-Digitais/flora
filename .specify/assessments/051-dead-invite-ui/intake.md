# Intake: 051 — Dead invite UI

**Data:** 2026-11-17T17:29Z · **Âncora:** `github/dev@b3bfda505` · branch `feature/051-dead-invite-ui`

## Sinal

C4 do ranking: `apps/frontend/src/lib/components/users/UserManager.svelte:18` faz
`POST /api/v1/users/invite`, mas `flora-api/src/routes/mod.rs:20-68` não monta `/users`.
O `catch` (`:24-25`) apenas envia ao logger local (in-process desde o ciclo 049);
a UI não exibe erro. Não há "sucesso falso": `successMessage` só é setado após 2xx.

## Evidência (verificada)

- `rg 'UserManager|users/invite'` → 3 arquivos: o componente, a página de settings
  e `specs/050-rate-limit-paths/spec.md:25` (uso apenas como exemplo de classificação).
- `rg 'invit' flora-api/src` → apenas `rate_limit.rs` (classificador do ciclo 050).
- Auditoria prévia: `docs/research/2026-11-15-012-frontend-api-contract-audit.md:52,145,151,314`
  — "delete the form" é uma das saídas documentadas.

## 80/20

`(I×C)/(E×R)` = (2×5)/(1×1) = **10.0** — à frente de osv-scan (6.0), C5 (2.08),
N3 (1.5) e C6 (0.80). Implementar convite exigiria serviço de e-mail/token e
decisão de autoridade (dono) — fora de escopo.

## Fronteiras

Escopo: remover componente, import/uso em settings e marcar drift em spec 002.
Fora: backend, autorização, C5/C6, constituição, workflows, merge.
Nenhum bug report em aberto em `.specify/bugs/`.
