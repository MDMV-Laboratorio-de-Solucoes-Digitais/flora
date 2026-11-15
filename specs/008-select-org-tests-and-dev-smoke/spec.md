# Feature Specification: 008 — Select-Organization Tests & Reproducible Dev Smoke

**Feature Branch**: `feat/008-select-org-tests-dev-smoke`  
**Created**: 2026-11-15  
**Status**: Implemented (scoped), runtime-integrated smoke **NV** in this environment  
**Assessment**: `.specify/assessments/008-select-org-tests-and-dev-smoke/{intake,research,decision}.md`  
**Depends on**: `specs/007-org-scoped-auth-e2e/`

## Goal
1. Cobrir `POST /api/v1/auth/select-organization` com testes determinísticos CI-safe (sem DB real): rejeição sem autenticação e rejeição de `organization_id` nil (contrato atual: `422 Validation`) com bearer válido.
2. Viabilizar caminho de smoke reproduzível em dev sem OIDC real para `select org → workspace → task` via docker compose + migrations/startup + `dev-seed.sql` + bootstrap determinístico de sessão/JWT.
3. Documentar procedimento, evidência e limitações **NV**.

## User Scenarios & Testing
- **US1 (P1):** requisição não autenticada é rejeitada (`401`). ✅ contrato automatizado.
- **US2 (P1):** `organization_id` nil é rejeitado antes de acesso a sessão/membership (`422`). ✅ contrato automatizado.
- **US3 (P1, condicionado):** sucesso (membro + sessão ativa) e rejeições DB-dependentes (não-membro, sessão ausente/inválida/inativa) — **~ / NV em CI** (sem Postgres em `.github/workflows/rust.yml`); pré-condições documentadas.
- **US4 (P2):** mantenedor segue smoke dev reproduzível documentado — **~ / NV** (execução integrada real não realizada neste ambiente).

## Requirements
- **FR-001:** Testes automatizados para casos críticos do endpoint compatíveis com CI atual. ✅
- **FR-002:** Mecanismo/documentação para bootstrap de smoke dev sem OIDC real (uso local/dev). ✅ (`scripts/dev-smoke-session.sql`, `docs/dev-smoke.md`)
- **FR-003:** Evidência de execução e NV explícito. ✅

## Evidence
- `cargo fmt --all -- --check`: pass.
- `cargo check -p flora-api -p flora-core -p flora-tests`: pass.
- `cargo test -p flora-tests --lib contract::auth`: `9 passed` (inclui `test_select_organization_rejects_missing_auth`, `test_select_organization_rejects_nil_org`).
- Smoke integrado com serviços/DB/API reais: **NV** neste ambiente.

## Non-goals
- Cobertura e2e OIDC real/JWKS, testes DB-backed em CI, alinhamento frontend, deploy/produção, serviços pagos.
