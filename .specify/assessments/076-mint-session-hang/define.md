# Define: hang do setup E2E do minter de sessão

- **Slug**: 076-mint-session-hang · **Definido**: 2026-11-19T12:44Z

## Problema

O setup E2E compila o binário `flora-mint-session` dentro da janela de 180s do teste, via `execFileSync` sem timeout (`seed.setup.ts:36-53`). Com target frio, a compilação (30.18s com cache) consome a janela e a ausência de timeout permite pendurar o setup — o CI evita isso pré-compilando fora da janela (`.github/workflows/rust.yml:192-195`).

## Afetados / metas / não-metas

Afetados: maintainers e agentes que executam E2E localmente. Metas: mover a pré-compilação para fora da janela do teste, mantendo o setup determinístico. Não-metas: mudar o binário, o workflow, a constituição, ADRs, STATE, specs/076-*, código de produção ou DB compartilhado; fechar issues; leg RBAC.

## Requisitos funcionais

- **FR-001** — pré-compilar `flora-mint-session` com `--features dev-mint` no globalSetup do Playwright, antes dos testes.
- **FR-002** — o setup E2E deve consumir o binário já construído, sem disparar `cargo run` que recompile dentro do teste.
- **FR-003** — preservar `test.setTimeout(180_000)` (`seed.setup.ts:14`) sem aumento.
- **FR-004** — manter `execFileSync` fora da janela de 180s do teste.
- **FR-005** — não alterar código de produção, workflows nem DB compartilhado.

## Métricas

- **SC-001** — nenhuma compilação ocorre dentro da janela de 180s do teste.
- **SC-002** — setup E2E conclui sem pendurar com target frio.
- **SC-003** — diff restrito à config/setup E2E + artefatos do ciclo.
- **SC-004** — gates do alvo permanecem PASS; zero mutação de DB compartilhado.

## Baseline

Sem a pré-compilação no globalSetup, a compilação (30.18s com cache) roda dentro do teste e é mascarada no CI pela etapa de `.github/workflows/rust.yml:192-195`.
