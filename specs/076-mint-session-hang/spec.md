# Feature Specification: hang do setup E2E do minter de sessão — 076

**Feature Branch**: `feature/076-mint-session-hang`
**Created**: 2026-11-19T12:45Z
**Status**: Approved (`decision.md` — go scoped, opção A)
**Input**: "mover a pré-compilação de `flora-mint-session` para fora da janela de 180s do setup E2E, sem tocar código, workflows ou DB compartilhado"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Pré-build antes dos testes (Priority: P1)

Como maintainer ou agente que executa os E2E localmente, quero que o binário `flora-mint-session` seja compilado no `globalSetup` do Playwright, antes de qualquer teste, para que a janela de 180s do setup não seja consumida pela compilação.

**Why this priority**: é a causa raiz medida — `cargo build -p flora-core --bin flora-mint-session --features dev-mint` leva 30.18s com cache, enquanto o binário pronto roda em 0.00s (direto) e 0.39s (target quente). Sem a pré-compilação, o `execFileSync` sem timeout em `seed.setup.ts:36-53` pode pendurar o setup local sem sinal, e o job `e2e-browser` do CI é `continue-on-error: true` (`.github/workflows/rust.yml:150`).

**Independent Test**: com `playwright test --list`, confirmar que o arquivo de globalSetup é carregado pela chave `globalSetup` da config e não é coletado como teste; em seguida executar isoladamente o comando de build do minter e conferir que termina fora da janela do teste.

**Acceptance Scenarios**:

1. **Given** a config do Playwright sem `globalSetup`, **When** o ciclo E2E inicia, **Then** o build do minter roda antes da execução dos testes e fora da janela de `test.setTimeout(180_000)`.
2. **Given** uma falha no comando de pré-build, **When** o globalSetup é executado, **Then** o erro é explícito antes da janela do teste (stdio `inherit`), sem pendurar o setup.

---

### User Story 2 — Setup determinístico sem recompilação (Priority: P1)

Como agente que roda o smoke de browser, quero que o `cargo run` existente em `seed.setup.ts` apenas consuma o binário já construído, sem recompilar, e que o timeout de 180s permaneça intacto.

**Why this priority**: satisfaz FR-002/FR-003 sem alterar o setup — com target quente o `cargo run` cai para ~0.39s e o `test.setTimeout(180_000)` continua como guarda superior, sem aumento.

**Independent Test**: `git diff` mostrando `apps/frontend/tests/e2e/seed.setup.ts` intocado (linha 14 e bloco 36-53 preservados) e medição do `cargo run` com target quente.

**Acceptance Scenarios**:

1. **Given** o binário pré-compilado pelo globalSetup, **When** o setup E2E roda, **Then** nenhuma compilação ocorre dentro da janela de 180s.
2. **Given** a janela atual de 180s, **When** o setup termina, **Then** `test.setTimeout(180_000)` permanece sem aumento.

---

### User Story 3 — Diff estreito e rastreável (Priority: P2)

Como maintainer, quero que a mudança fique restrita à config/setup E2E e aos artefatos do ciclo, com os gates do alvo em PASS e nenhuma mutação de DB compartilhado.

**Why this priority**: SC-003/SC-004 protegem o alvo; o pré-build já é padrão no CI (`.github/workflows/rust.yml:192-195`), então duplicar lógica em código de produção ou workflow seria ruído desnecessário.

**Independent Test**: `git diff --name-only dev..HEAD` contém somente os paths permitidos e `python3 scripts/ci/check_gate_integrity.py` permanece PASS.

**Acceptance Scenarios**:

1. **Given** as edições aplicadas, **When** o diff é inspecionado, **Then** não há mudanças em código de produção, workflows, ADRs, constituição, STATE ou DB compartilhado.
2. **Given** os gates locais do alvo, **When** executados, **Then** permanecem PASS.

---

### Edge Cases

- **Target totalmente frio**: duração exata não medida; o pré-build sai da janela de qualquer forma, mas o tempo com target frio é **[NV]** (sem inferência a partir dos 30.18s com cache).
- **Falha no `cargo build`**: o globalSetup deve falhar rápido e com erro visível (stdio `inherit`), em vez de deixar a janela do teste se esgotar silenciosamente.
- **Target quente / CI que já pré-compila**: o build do globalSetup é idempotente (no-op) e não altera o comportamento do job `e2e-browser`.
- **Descoberta de testes**: `global-setup.ts` não pode casar com `testMatch: '**/*.e2e.ts'` (config:9) nem com `testMatch: /.*\.setup\.ts/` do projeto `setup` (config:19); é carregado somente pela chave `globalSetup`.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: O Playwright DEVE pré-compilar `flora-mint-session` com `--features dev-mint` no `globalSetup`, antes da execução de qualquer teste.
- **FR-002**: O setup E2E DEVE consumir o binário já construído; o `cargo run` remanescente em `seed.setup.ts` NÃO PODE disparar recompilação dentro da janela do teste.
- **FR-003**: O `test.setTimeout(180_000)` de `seed.setup.ts:14` DEVE ser preservado, sem aumento.
- **FR-004**: O `execFileSync` que carrega a compilação DEVE permanecer fora da janela de 180s do teste, no `globalSetup`.
- **FR-005**: NÃO DEVE haver alteração em código de produção, workflows, ADRs, constituição, STATE ou DB compartilhado.

### Key Entities

N/A — a mudança é de configuração E2E; nenhuma entidade de dados, schema ou migração é introduzida.

## Success Criteria *(mandatory)*

- **SC-001**: Nenhuma compilação ocorre dentro da janela de 180s do teste; `playwright test --list` confirma que o pré-build está no `globalSetup` e o comando de build é executado como etapa separada.
- **SC-002**: O setup E2E conclui sem pendurar com target frio; o mecanismo é verificado pela pré-compilação fora da janela, enquanto a duração exata com target totalmente frio permanece **[NV]**.
- **SC-003**: O diff de `dev..HEAD` fica restrito a `apps/frontend/playwright.config.ts`, `apps/frontend/playwright/global-setup.ts` e artefatos do ciclo 076.
- **SC-004**: Os gates do alvo permanecem PASS (`bun run check`/`lint`/`test:unit`, `check_gate_integrity.py`) e há zero mutação de DB compartilhado.

## Scope / Out of scope

**Dentro**: `apps/frontend/playwright.config.ts` (uma linha: `globalSetup: './playwright/global-setup.ts',`), o novo `apps/frontend/playwright/global-setup.ts` e os artefatos do ciclo 076 (`spec.md`, `clarify.md`, `plan.md`).

**Fora**: `apps/frontend/tests/e2e/seed.setup.ts`, workflows (`.github/workflows/*`), código de produção Rust/Svelte, ADRs, constituição, `.specify/memory/STATE.md`, `specs/076-*` além dos três artefatos aqui listados, worktree/branch 073, leg RBAC, fechamento de issues, DB compartilhado e produção.

## Assumptions

- `cargo` está disponível no PATH do ambiente que executa o Playwright e a feature `dev-mint` existe em `flora-core` (`flora-core/Cargo.toml:59-68`).
- A raiz do repositório é resolvida a partir da localização de `global-setup.ts`, como já faz `seed.setup.ts:6`.
- O binário do minter é síncrono e rápido (sem DB, rede, stdin ou `await`); a variável é a compilação, não o runtime.
- O CI já pré-compila o minter em `.github/workflows/rust.yml:192-195`; o `globalSetup` é idempotente nesse cenário.

## [NV]

- Tempo de compilação com target totalmente frio não foi medido (apenas 30.18s com cache).
- Status de CI ao vivo, merge, proteção de branch, Forgejo e produção não são verificados nesta spec.
- Estado do DB compartilhado não é inspecionado; a garantia é de não-mutação por construção (nenhum comando de DB é adicionado).
