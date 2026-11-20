# Implementation Plan: First Seed Journey local one-time bootstrap

**Branch**: `feature/085-local-first-seed` | **Date**: 2026-11-20T01:30:00Z | **Spec**: [spec.md](./spec.md)

**Input**: `/specs/085-local-first-seed/spec.md`

## Summary

Habilitar o botão "Sprout Workspace" em instalações vazias via um endpoint único, não autenticado e one-time, que cria em transação a organização, o administrador, o papel Admin, a associação, o workspace inicial e a sessão já escopada à organização. O fluxo OIDC existente permanece intacto para instalações com dados.

## Technical Context

**Language/Version**: Rust 1.98.1, edition 2024 (workspace); TypeScript/Svelte 5 no frontend

**Primary Dependencies**: axum + sqlx (Postgres), `validator`, `jsonwebtoken` via `flora_core::utils::jwt`, `uuid`; Vitest browser + Playwright no frontend

**Storage**: PostgreSQL local/auto-hospedado (Seed); sem fila, cache, busca ou objeto externo

**Testing**: `cargo test -p flora-tests` (contratos com Postgres), `cargo clippy`, `npm run check/lint/test:unit`, verificação em navegador real

**Target Platform**: Linux server + navegador (Vite dev/preview)

**Project Type**: web service (API Rust) + SPA SvelteKit

**Performance Goals**: uma transação por bootstrap; resposta < 2 s em condições locais normais

**Constraints**: nenhuma dependência externa nova; sem `unsafe`; sem `.unwrap()` em produção; `max_width = 100`; TDD obrigatório

**Scale/Scope**: uma inicialização por instalação; 6 entidades criadas; 1 rota nova

## Constitution Check

*Gate pré-Phase 0*: **PASS com tensão documentada**.

- Seed = Flora + PostgreSQL: OK — nenhum Valkey/RustFS/Meilisearch/Zitadel introduzido.
- `Result<T, E>` + `thiserror`, sem `unwrap` em produto: OK — novo erro usa `Error::DuplicateEntry`/`Validation`.
- Testes para toda API pública + TDD: OK — contrato DB cobre sucesso, conflito e concorrência.
- Identidade verificada por ID token (Seed milestone): **tensão** — o bootstrap não verifica e-mail; limitado a banco vazio e registrado em FR-012/SC-005. Justificado na tabela abaixo.
- Segurança: superfície não autenticada mitigada por travamento transacional, validação, rate limit e auditoria.

*Re-check pós-Phase 1*: **PASS** — desenho não altera o fluxo OIDC, não adiciona serviços e mantém autoridade do servidor sobre a sessão.

## Project Structure

### Documentation (this feature)

```text
specs/085-local-first-seed/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── bootstrap.md
├── checklists/requirements.md
└── tasks.md
```

### Source Code (repository root)

```text
flora-api/src/routes/
├── mod.rs            # registra a rota /bootstrap no router público
└── bootstrap.rs      # novo handler + payload + transação atômica

flora-core/src/
├── error.rs          # reusa DuplicateEntry/Validation; sem novo variant se possível
├── models/           # reutiliza Organization/User/Role/Membership/Workspace/Session
└── utils/jwt.rs      # reutiliza FloraClaims + encode_token

flora-tests/src/contract/
└── bootstrap.rs      # testes DB: sucesso, 409, concorrência, sessão válida

apps/frontend/src/routes/(auth)/
├── +page.svelte                  # "Sprout Workspace" chama bootstrap
└── login-error.svelte.spec.ts    # feedback de erro (já existe, estendido)

docs/architecture/                # diagrama atualizado se a rota pública mudar
docs/governance/                  # ADR/decisão de exceção Seed
docs/PROGRESSO.md                 # nota datada de estado verificado
```

**Structure Decision**: manter a separação existente (`flora-api` para HTTP, `flora-core` para modelos/regras, `flora-tests` para contratos e `apps/frontend` para UI). Nenhum crate novo.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| Bootstrap sem verificação de posse de e-mail (tensão com Seed OIDC) | Instalação nova sem IdP não tem como provar identidade; sem isso o botão continua inútil | Exigir Zitadel contradiz perfil Seed e não funciona no ambiente; gerar token no cliente expõe segredo no bundle |

## Phase 0 — Research Decisions

Registradas em [research.md](./research.md): guarda atômica, emissão de sessão, papéis e validação.

## Phase 1 — Design

- [data-model.md](./data-model.md): entidades criadas e regras de integridade.
- [contracts/bootstrap.md](./contracts/bootstrap.md): contrato HTTP `POST /api/v1/bootstrap`.
- [quickstart.md](./quickstart.md): como validar o fluxo de ponta a ponta.

## Next Phase

`/speckit.tasks` — decompor em tarefas verificáveis, com testes primeiro.
