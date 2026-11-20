# Feature Specification: remoção da flag OIDC morta `local_fallback_enabled` — 080

**Feature Branch**: `feature/080-remove-dead-oidc-flag`
**Created**: 2026-11-19T21:35Z
**Status**: Approved (`decision.md` — go scoped, opção A, E=15,0)
**Input**: "remover flag OIDC morta `local_fallback_enabled`" — decidido em `.specify/assessments/080-remove-dead-oidc-flag/decision.md`; tratado como remoção de declaração write-only que promete um fallback de segurança inexistente, sem tocar em auth.

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Configuração sem garantia falsa de fallback (Priority: P1)

Como maintainer ou agente que lê `OidcConfig`, quero que a documentação e o campo `local_fallback_enabled` (`flora-core/src/config/oidc.rs:18-19`) deixem de existir, para que a config não sugira fallback local quando o Zitadel está indisponível — comportamento nunca implementado e registrado como risco P1 em G9.

**Why this priority**: é o cerne do ciclo. `validate()` (`:52-74`) ignora a flag e não há nenhum outro leitor em Rust; o único uso de `OidcConfig` fora do arquivo é `flora-core/src/config/mod.rs:24,59`. Manter o campo com default `true` preserva a "garantia de segurança falsa" que G9 (`docs/research/2026-11-16-seed-oidc-identity-gap.md:120`, recomendação `:147`: "remover/implementar") e NV-7 (`...016-auth-rbac-oidc-minimal-vps.md:56`) sinalizaram.

**Independent Test**: confrontar `flora-core/src/config/oidc.rs` com a base `e3944e962`: a doc `/// Local fallback enabled when Zitadel is unavailable (default: true in dev).` e a linha `pub local_fallback_enabled: bool,` não aparecem, e `code_challenge_method` (`:17`) é seguido diretamente do bloco de `allowed_redirect_uris` (`:20-23`, `#[serde(default)]` intacto).

**Acceptance Scenarios**:

1. **Given** `flora-core/src/config/oidc.rs:18-19` com a doc e o campo, **When** a remoção é aplicada, **Then** o struct mantém os 5 campos restantes e `allowed_redirect_uris` permanece com seu `#[serde(default)]` e seus dois comentários inalterados.
2. **Given** a doc enganosa removida, **When** um maintainer lê a config, **Then** nenhum comentário promete fallback local e nenhuma reescrita de `validate()` é necessária — o método segue com as mesmas 4 verificações e o `Ok(())` final em `:73`.

---

### User Story 2 — Default coerente, compatibilidade por ausência e sem substituto (Priority: P1)

Como operador ou maintainer, quero que a inicialização padrão deixe de setar `local_fallback_enabled: true` e que nenhum mecanismo substituto seja introduzido, para que um ambiente legado continue subindo sem erro e a remoção não troque uma dívida por outra.

**Why this priority**: FR-002 e FR-003 são a cerca de compatibilidade. `Config` (`mod.rs:17-31`) não tem `deny_unknown_fields` e `Config::load` (`:43-49`) faz merge de `Serialized::defaults(Self::default())` com `Env::prefixed("FLORA_").split("__")`: a chave legada `FLORA_OIDC__LOCAL_FALLBACK_ENABLED` passa a ser ignorada silenciosamente, sem mudança de contrato de leitura. Adicionar `deny_unknown_fields`, alias de env ou um novo `#[serde(default)]` tornaria a remoção uma mudança de comportamento de startup — explicitamente fora do escopo.

**Independent Test**: `grep -rn "local_fallback" --include='*.rs' flora-core/src apps` retorna zero; `cargo check -p flora-core`, `cargo clippy -p flora-core --all-targets -- -D warnings` e `cargo fmt --check` terminam limpos; uma leitura de `mod.rs:17-31,43-49` confirma que `deny_unknown_fields` não foi introduzido e que o merge de env permanece o mesmo.

**Acceptance Scenarios**:

1. **Given** `impl Default for OidcConfig` com 6 inicializações, **When** a linha `local_fallback_enabled: true,` (`:40`) é removida, **Then** restam exatamente as 5 inicializações (`:29-41`), com `code_challenge_method: "S256".to_owned(),` e `allowed_redirect_uris: Vec::new(),` inalteradas e adjacentes.
2. **Given** um ambiente com `FLORA_OIDC__LOCAL_FALLBACK_ENABLED` definida, **When** `Config::load` é executado, **Then** a chave é ignorada sem erro de startup, pois `Config` segue sem `deny_unknown_fields` — nenhum alias, shim ou campo substituto é adicionado.

---

### User Story 3 — Diff estreito, gate íntegro e reversível (Priority: P2)

Como maintainer, quero que a mudança fique restrita a `flora-core/src/config/oidc.rs` (3 linhas removidas), além dos artefatos do ciclo 080, com o gate do alvo em PASS e zero mutação de auth, `.env.example`, docs, worktree, CI, DB ou segredos, para que o ciclo seja auditável e reversível pela restauração de um único path.

**Why this priority**: FR-004 e SC-004 são a fronteira da mudança. O custo é de subcão (1 arquivo, 3 linhas); qualquer efeito colateral — reformatar o arquivo, tocar `auth.rs`, atualizar `.env.example`, mexer no gate ou em workflows — transformaria uma remoção segura em uma mudança de contrato que exige reavaliação.

**Independent Test**: `git diff --stat e3944e962..HEAD` mostra somente `flora-core/src/config/oidc.rs` com 3 linhas a menos e os artefatos 080; `git diff --name-only e3944e962..HEAD` não lista nenhum outro path; `python3 scripts/ci/check_gate_integrity.py` permanece PASS.

**Acceptance Scenarios**:

1. **Given** as 3 linhas removidas, **When** o diff é auditado contra `e3944e962`, **Then** `oidc.rs` tem exatamente 3 linhas a menos, sem reindentação, e `auth.rs`, `services/auth_service.rs`, `.env.example`, docs de research, `config/mod.rs`, worktree `state-closure-072`, gate/`scripts/ci/check_gate_integrity.py`, `.github/workflows/`, ADRs, constituição, STATE, DB, rede, `.tokens/` e lockfiles permanecem byte-idênticos.
2. **Given** os gates determinísticos do alvo, **When** executados sem DB, rede ou segredos, **Then** permanecem PASS, e a restauração de `flora-core/src/config/oidc.rs` ao base devolve as 3 linhas sem force-push ou reescrita de histórico.

---

### Edge Cases

- **Ambiente legado (`FLORA_OIDC__LOCAL_FALLBACK_ENABLED`) em deploy fora do repo**: `Config` sem `deny_unknown_fields` (`mod.rs:17-31`) faz a chave ser ignorada por `Config::load` (`:43-49`); comportamento fora do repo é **[NV]**, sem inferência além da evidência local.
- **Consumidores externos de `OidcConfig`**: não há consumidor de `Serialize` sobre config no repo; a remoção não altera contrato de leitura interno. Serialização/consumo fora do repo é **[NV]** e está fora do escopo, sem shim de compatibilidade.
- **`allowed_redirect_uris` e seu `#[serde(default)]`**: permanecem intactos (`:20-23`); a proibição de FR-003 a novos `#[serde(default)]` não se aplica a este atributo pré-existente.
- **`deny_unknown_fields` como "correção"**: rejeitado — mudaria o comportamento de startup para ambientes com a chave legada; FR-003 exige explicitamente que ele não seja introduzido.
- **Lógica de fallback em auth**: `auth.rs` e `services/auth_service.rs` ficam intocados; nenhum comportamento novo é implementado (a decisão é remover, não implementar).
- **`.env.example`**: já não contém a chave (`:58-68`); permanece byte-idêntico, sem "documentar" a variável removida.
- **Worktree `state-closure-072` e docs de research históricos**: contêm a cópia/texto antigo; ficam fora do escopo — SC-001 restringe o grep a `flora-core/src` e `apps` (código Rust rastreado).
- **Reformatação automática**: a edição é somente remoção de linhas; `cargo fmt --check` deve ficar limpo sem reindentação. Se aparecer qualquer reindentação, o diff invalida FR-004.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: remover de `flora-core/src/config/oidc.rs` a doc `/// Local fallback enabled when Zitadel is unavailable (default: true in dev).` e a linha `pub local_fallback_enabled: bool,` (atual `:18-19`), deixando `code_challenge_method` seguido diretamente de `allowed_redirect_uris` (`:20-23`, com seu `#[serde(default)]` intacto).
- **FR-002**: remover a linha `local_fallback_enabled: true,` do `impl Default for OidcConfig` (atual `:40`), mantendo as demais 5 inicializações (`:29-41`) e o `Ok(())` final de `validate()` (`:73`) inalterados.
- **FR-003**: não introduzir `deny_unknown_fields`, `#[serde(default)]` novo, alias de env ou qualquer substituto para o campo: `Config` (`flora-core/src/config/mod.rs:17-31`) permanece sem `deny_unknown_fields` e `Config::load` (`:43-49`) continua ignorando a chave legada `FLORA_OIDC__LOCAL_FALLBACK_ENABLED` sem erro de startup.
- **FR-004**: limitar o diff rastreado a `flora-core/src/config/oidc.rs` (3 linhas removidas), além dos artefatos do ciclo 080; nenhum outro arquivo de código, config, doc, gate, CI ou lockfile é modificado.

### Key Entities

N/A — a mudança é a remoção de um campo de configuração write-only (Rust + `serde`); nenhuma entidade de dados, schema de banco, migração ou contrato de leitura é introduzido. `OidcConfig` permanece como struct de configuração já existente, apenas sem o campo morto.

## Success Criteria *(mandatory)*

- **SC-001**: `grep -rn "local_fallback" --include='*.rs' flora-core/src apps` retorna **zero** correspondências (as únicas menções restantes no repo são os 3 docs de research históricos e a cópia em worktree, fora do escopo). Depende das duas remoções de FR-001 e FR-002; sua execução é `[NV]` nesta avaliação.
- **SC-002**: `cargo check -p flora-core` e `cargo clippy -p flora-core --all-targets -- -D warnings` terminam sem erro nem novo warning; `cargo fmt --check` limpo (a edição é só remoção de linhas, sem reindentação). **[NV]** não executado nesta avaliação.
- **SC-003**: `python3 scripts/ci/check_gate_integrity.py` passa e o gate do alvo segue PASS; `Config::load` continua aceitando um ambiente com a chave legada definida (compat por ausência de `deny_unknown_fields`). **[NV]** depende da execução pós-edição.
- **SC-004**: `git diff --stat` mostra somente `flora-core/src/config/oidc.rs` com 3 linhas a menos e os artefatos 080; zero mutação de `.env.example`, auth, docs, worktree, gate, CI, DB ou segredos; base `e3944e962` preservada em todo o resto.

## Scope / Out of scope

**Dentro**: `flora-core/src/config/oidc.rs` (3 linhas removidas: doc+campo em `:18-19` e inicialização em `:40`), os artefatos do ciclo 080 (`spec.md`, `clarify.md`, `plan.md`, `checklists/requirements.md`, `checklist.md`, `tasks.md`) e os 5 artefatos de assessment já registrados em `.specify/assessments/080-remove-dead-oidc-flag/` (`intake.md`, `research.md`, `define.md`, `concept.md`, `decision.md`).

**Fora**: código de auth (`auth.rs`, `services/auth_service.rs`), `.env.example`, docs de research, worktree `state-closure-072`, `config/mod.rs` e demais módulos, gate/`scripts/ci/check_gate_integrity.py`, `.github/workflows/`, ADRs, constituição, `.specify/memory/STATE.md`, `specs/080-*` (exceto os artefatos aqui listados), DB/rede, segredos, `.tokens/`, fechamento de issues, leg RBAC e lockfiles.

## Assumptions

- A flag nunca é lida: `validate()` (`:52-74`) a ignora, o único uso de `OidcConfig` fora do arquivo é `mod.rs:24,59` e `grep` em Rust retorna zero leitores.
- `Config` (`mod.rs:17-31`) não tem `deny_unknown_fields` e `Config::load` (`:43-49`) usa `Serialized::defaults(Self::default())` + `Env::prefixed("FLORA_").split("__")`; chaves desconhecidas são ignoradas.
- G9 (`docs/research/2026-11-16-seed-oidc-identity-gap.md:120`, recomendação `:147`) e NV-7 (`...016-auth-rbac-oidc-minimal-vps.md:56`) registraram a lacuna "declared but never consulted".
- A base de comparação do diff é `e3944e962` (HEAD da branch `feature/080-remove-dead-oidc-flag` no início do ciclo; merge `chore(frontend): merge instalacao do shadcn-svelte (#303)` que entregou 079).
- `.specify/feature.json` é estado local ignorado (`.specify/.gitignore:6`), reescrito a cada troca de feature; hoje aponta `specs/080-remove-dead-oidc-flag`.

## [NV]

- Execução dos gates determinísticos (`cargo check -p flora-core`, `cargo fmt --check`, `cargo clippy -p flora-core --all-targets -- -D warnings`, `grep -rn "local_fallback" --include='*.rs' flora-core/src apps`, `python3 scripts/ci/check_gate_integrity.py`, auditoria de `git diff`) — fica com o main agent após a edição; não executada nesta avaliação (marcadores em SC-002 e SC-003).
- Comportamento da chave legada `FLORA_OIDC__LOCAL_FALLBACK_ENABLED` em deploy fora do repo e de consumidores externos que eventualmente serializem/leiam `OidcConfig` — não verificado; sem consumidor de `Serialize` sobre config no repo e sem shim de compatibilidade.
- Cópia em worktree `state-closure-072` e os 3 docs de research históricos — fora do escopo por SC-001/SC-004.
- Status de CI ao vivo, merge, PRs, Forgejo, produção e fechamento de issues — não verificados nesta spec; nenhuma inferência além da evidência local e de `decision.md`.
