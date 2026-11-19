# Feature Specification: remoção da feature `sqlite` em flora-core — 077

**Feature Branch**: `feature/077-remove-sqlite-feature`
**Created**: 2026-11-19T19:44Z
**Status**: Approved (`decision.md` — go scoped, opção A)
**Input**: "remover exclusivamente a linha `sqlite = ["sqlx/sqlite"]` de `[features]` em `flora-core/Cargo.toml:61`, preservando default, sqlx postgres-only e gate"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Manifest sem feature enganosa (Priority: P1)

Como maintainer ou agente que lê o manifest de `flora-core`, quero que o bloco `[features]` não contenha `sqlite`, para que o manifest não sugira suporte que a stack normativa revogou (§4, linha 52) e que o runtime já recusa (`.env.example:81`).

**Why this priority**: é o cerne do ciclo — a linha `flora-core/Cargo.toml:61` é inerte (`default = []` na linha 60, sqlx postgres-only em `Cargo.toml:143`, zero consumidores), mas permanece como superfície enganosa e contradiz o padrão normativo (§1 linha 13; §4 linhas 48, 51-52).

**Independent Test**: carregar `flora-core/Cargo.toml` com `tomllib` (stdlib) e confirmar que a chave `sqlite` não existe em `[features]`, enquanto `default`, `test` e `dev-mint` permanecem com os valores exatos originais.

**Acceptance Scenarios**:

1. **Given** o `[features]` com `sqlite = ["sqlx/sqlite"]` na linha 61, **When** a linha é removida, **Then** `tomllib` reporta `default = []`, `test = []`, `dev-mint = ["dep:clap", "dep:anyhow"]` e nenhuma chave `sqlite`.
2. **Given** um build que já compilava `flora-core`, **When** a feature é removida, **Then** nada muda: `sqlite` nunca esteve no `default`, então nenhum build existente é afetado.

---

### User Story 2 — Preservar default, sqlx e config (Priority: P1)

Como maintainer, quero que `default = []`, `test = []`, `dev-mint`, o sqlx postgres-only da workspace e a nota do `.env.example` fiquem byte-idênticos, para que a remoção seja local a um manifest e não altere dependências nem comportamento de runtime.

**Why this priority**: protege FR-002/FR-003 — o risco identificado em `concept.md` é justamente "mexer em `Cargo.toml` raiz, `.env.example`, `default` ou sqlx"; a remoção não pode tocar nenhum desses pontos.

**Independent Test**: `git diff` mostrando `Cargo.toml:143` e `.env.example:81` intocados, `cargo check -p flora-core --offline` em PASS e `cargo metadata --locked --offline` concluindo sem exigir alteração de `Cargo.lock`.

**Acceptance Scenarios**:

1. **Given** `default = []` em `flora-core/Cargo.toml:60`, **When** a linha 61 é removida, **Then** as linhas 60, 62 e 63 permanecem byte-idênticas e nenhuma feature default nova aparece.
2. **Given** `Cargo.toml:143` com sqlx `postgres` (sem `sqlite`) e `.env.example:81` ("SQLite runtime path is not supported"), **When** o diff é inspecionado, **Then** ambos os arquivos seguem inalterados.

---

### User Story 3 — Diff estreito e gate intacto (Priority: P2)

Como maintainer, quero que a mudança fique restrita à linha do manifest e aos artefatos do ciclo 077, com os gates do alvo em PASS e nenhuma mutação de DB, rede ou segredos, para que o ciclo seja auditável e reversível por uma única reversão.

**Why this priority**: SC-003/SC-004 são a cerca do ciclo; uma linha inerte não justifica tocar supply-chain (`deny.toml`), workflows, scripts de gate nem código de produção, e `check_gate_integrity.py` precisa permanecer PASS.

**Independent Test**: `git diff --name-only cc421e42d..HEAD` contém somente `flora-core/Cargo.toml` e os artefatos de `specs/077-remove-sqlite-feature/`; `python3 scripts/ci/check_gate_integrity.py` permanece PASS; `cargo tree -p flora-core --offline` não mostra `sqlx-sqlite` no grafo ativo.

**Acceptance Scenarios**:

1. **Given** as edições aplicadas, **When** o diff é auditado, **Then** não há mudanças em `Cargo.toml` raiz, `.env.example`, código Rust, workflows, gate/scripts, `deny.toml`, ADRs, constituição, STATE, worktree 073, DB/rede ou `.tokens/`.
2. **Given** os gates determinísticos do alvo, **When** executados, **Then** permanecem PASS, sem acesso a DB ou rede e sem credenciais novas.

---

### Edge Cases

- **`Cargo.lock` retém `sqlx-sqlite`**: as entradas opcionais em `Cargo.lock:3176`, `:3247` e `:3320` permanecem — a remoção da feature não exige atualização de lock (o pacote é dependência opcional do sqlx, não ativado pelo `default = []`). `cargo metadata --locked --offline` deve concluir sem diferença de lock; forçar a poda do lock seria toque fora de escopo.
- **Alguém compila com `--features sqlite` hoje?**: não — a varredura determinística em `*.toml/*.rs/*.yml/*.yaml/*.sh` retornou zero consumidores e a feature não é default; removê-la não quebra build existente.
- **Documentar em vez de remover (opção B) / banir via `deny.toml` (opção C)**: ambas rejeitadas em `concept.md` — documentação não remove a superfície enganosa e `deny.toml` exige toque em supply-chain/gate, fora de escopo.
- **Necessidade real de operação offline**: se existir, decide-se por ADR de projeto (§4, linha 52), nunca por padrão; não é tratada neste ciclo — **[NV]**.
- **Extensões não varridas**: a varredura cobre apenas `*.toml/*.rs/*.yml/*.yaml/*.sh`; consumidores em outras extensões são **[NV]**.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Remover a linha `sqlite = ["sqlx/sqlite"]` de `[features]` em `flora-core/Cargo.toml:61`.
- **FR-002**: Preservar integralmente `default = []`, `test = []` e `dev-mint = ["dep:clap", "dep:anyhow"]` (`flora-core/Cargo.toml:60,62-63`).
- **FR-003**: Manter `Cargo.toml:143` (sqlx 0.9 com `postgres`, sem `sqlite`) e `.env.example:81` inalterados.
- **FR-004**: Não introduzir segredos, credenciais, `.tokens/`, dependências novas nem mudanças em gate/scripts/workflows.

### Key Entities

N/A — a mudança é de manifest Cargo; nenhuma entidade de dados, schema ou migração é introduzida.

## Success Criteria *(mandatory)*

- **SC-001**: `flora-core/Cargo.toml` não contém mais a chave `sqlite` em `[features]` (verificado por `tomllib`).
- **SC-002**: `default = []` e as demais features permanecem byte-idênticas; nenhuma feature default nova; `cargo check -p flora-core --offline` permanece PASS.
- **SC-003**: nenhum consumidor novo ou existente referencia `sqlite` — a varredura determinística por `grep` segue com zero correspondências e `cargo tree -p flora-core --offline` não ativa `sqlx-sqlite`.
- **SC-004**: diff restrito a `flora-core/Cargo.toml` (1 linha), `Cargo.lock` (1 aresta `cc` removida de `libsqlite3-sys`) e artefatos 077; gate PASS; `cargo metadata --locked --offline` exit 0; zero mutação de DB/rede/segredos.

## Scope / Out of scope

**Dentro**: `flora-core/Cargo.toml` (exatamente a linha 61), `Cargo.lock` (atualização determinística da resolução) e os artefatos do ciclo 077 (`spec.md`, `clarify.md`, `plan.md`, `checklists/requirements.md`, `checklist.md`, `tasks.md`), além dos 5 artefatos de assessment já registrados em `.specify/assessments/077-remove-sqlite-feature/`.

**Fora**: `Cargo.toml` raiz, `.env.example`, código Rust, workflows (`.github/workflows/*`), gate/scripts, `deny.toml`, ADRs, constituição, `.specify/memory/STATE.md`, `specs/077-*` além dos artefatos aqui listados, worktree/branch 073, leg RBAC, fechamento de issues, DB/rede, `.tokens/` e segredos.

## Assumptions

- `default = []` (`flora-core/Cargo.toml:60`); `sqlx` da workspace é postgres-only (`Cargo.toml:143`).
- A varredura determinística de consumidores retornou zero correspondências (`research.md`).
- `.env.example:81` já declara o runtime SQLite não suportado.
- `Cargo.lock` mantém `sqlx-sqlite` como dependência opcional do sqlx; a remoção da feature não altera o lock.
- O ambiente roda offline, sem DB, sem rede e sem segredos; gates determinísticos não dependem de serviços externos.

## [NV]

- Extensões de arquivo não cobertas pela varredura determinística (`*.toml/*.rs/*.yml/*.yaml/*.sh`); nenhum consumidor identificado nas extensões varridas.
- Necessidade offline real — decidiria por ADR de projeto (§4, linha 52), não avaliada neste ciclo.
- Status de CI ao vivo, merge, PRs, Forgejo e produção não são verificados nesta spec.
