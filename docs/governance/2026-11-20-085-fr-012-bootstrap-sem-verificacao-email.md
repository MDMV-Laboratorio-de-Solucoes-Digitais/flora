# Nota de governança — 2026-11-20 (ciclo 085)

**Assunto:** exceção FR-012 — bootstrap sem verificação de e-mail.

- A rota pública `POST /api/v1/bootstrap` cria a primeira organização, administrador, papel, associação e workspace em uma transação, somente em **instalação vazia** (zero organizações não-sistema; a placeholder `system` da migration não conta).
- Por decisão documentada em `specs/085-local-first-seed/spec.md` (FR-012), **não há verificação de posse do e-mail** neste fluxo. A limitação é explícita no contrato `specs/085-local-first-seed/contracts/bootstrap.md`. Instalações com organização existente seguem no fluxo de login OIDC inalterado (`SC-005`); tentativas posteriores recebem `409 Conflict` sem gravar objeto.
- Evidência local do ciclo 085: gate de integridade PASS; clippy, contratos e vitest validados; E2E real `POST /api/v1/bootstrap` → `201`, segunda chamada → `409`, `GET /api/v1/tasks` com token → `200`.
- Contagens verificadas por `find` em 2026-11-20: `specs/` 75, `.specify/assessments/` 72, `.specify/validations/` 70, `.specify/releases/` 70, `.specify/bugs/` 4, `docs/research/` 137 + `index.md`.
- **[NV]** produção, CI futuro e Forgejo: esta nota registra apenas estado local; reconsultar antes de qualquer merge ou promoção.
