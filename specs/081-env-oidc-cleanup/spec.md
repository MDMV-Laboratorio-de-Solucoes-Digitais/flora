# Feature Specification: limpeza das chaves OIDC em `.env.example` — 081

**Feature Branch**: `feature/081-env-oidc-cleanup`
**Created**: 2026-11-19T22:10Z
**Status**: Approved (`decision.md` — go (scoped), opção A, E=10,0)
**Input**: "limpeza das chaves OIDC em `.env.example`" — decidido em `.specify/assessments/081-env-oidc-cleanup/decision.md`; tratado como correção documental de drift de config, sem mudança de comportamento nem de contrato de leitura.

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Template que aponta para o mecanismo real de callback (Priority: P1)

Como operador, maintainer ou agente que copia `.env.example`, quero que o bloco OIDC deixe de documentar `FLORA_OIDC__REDIRECT_URI` como se configurasse o callback e passe a explicar `FLORA_OIDC__ALLOWED_REDIRECT_URIS`, para que eu saiba onde configurar URIs extras sem criar a ilusão de que a chave antiga tem efeito.

**Why this priority**: é o cerne do ciclo. `.env.example:62-63` apresenta `FLORA_OIDC__REDIRECT_URI` como "Redirect/callback URI (verify config key support in your flow)", mas `OidcConfig` (`flora-core/src/config/oidc.rs:7-22`) não tem esse campo e `Config` (`config/mod.rs:17-31,43-49`) o descarta em silêncio. O callback real deriva de `FLORA_APP__BASE_URL` (`flora-api/src/routes/auth.rs:101` → `{base_url}/api/v1/auth/callback`) e URIs extras exigem `FLORA_OIDC__ALLOWED_REDIRECT_URIS` (`oidc.rs:20-21`; allowlist exata em `auth.rs:105-117`) — nenhum dos dois aparece hoje no template. O drift está registrado em `docs/research/2026-11-16-016-macro-files-ops-parity.md:32`.

**Independent Test**: confrontar `.env.example` com a base `35b8d3142`: as linhas `:62-63` foram substituídas por exatamente 3 linhas de comentário — duas explicando o callback derivado de `FLORA_APP__BASE_URL` e a exigência de `FLORA_OIDC__ALLOWED_REDIRECT_URIS`, seguidas do exemplo **comentado** `# FLORA_OIDC__ALLOWED_REDIRECT_URIS=["http://127.0.0.1:4173/callback"]`. O `grep` de SC-001 mostra a nova chave e nenhuma occurrences de `FLORA_OIDC__REDIRECT_URI`.

**Acceptance Scenarios**:

1. **Given** `.env.example:62-63` com o comentário enganoso e `FLORA_OIDC__REDIRECT_URI=http://localhost:3000/auth/callback`, **When** a substituição é aplicada, **Then** o bloco passa a ter 3 linhas de comentário com a explicação do callback padrão e da allowlist, e o exemplo de `FLORA_OIDC__ALLOWED_REDIRECT_URIS` permanece comentado com aspas duplas internas.
2. **Given** um operador lendo apenas o template, **When** ele precisa aceitar um redirect extra, **Then** encontra a chave correta e a sintaxe Figment válida (`figment 0.10.19`, `Cargo.lock:941-942`), sem inferir que `FLORA_OIDC__REDIRECT_URI` funciona.

---

### User Story 2 — Alias histórico removido e leitores reais preservados (Priority: P1)

Como maintainer, quero remover a linha `:65` (`# OIDC_ISSUER_URL=http://localhost:8080`), que nunca é lida em Rust, e manter byte-idênticas as chaves com leitores reais (`FLORA_OIDC__*` e `ZITADEL_*`), para que a limpeza não altere a configuração de nenhum deploy.

**Why this priority**: FR-002 e FR-003 são a cerca de compatibilidade. `OIDC_ISSUER_URL` só aparece no contrato histórico `specs/001-flora-seed-v0-1/contracts/OIDC.md:10,17` e em docs antigos; nenhum `std::env::var` ou leitor Figment a consulta. Em contrapartida, `FLORA_OIDC__ISSUER_URL/CLIENT_ID/CLIENT_SECRET` são lidos via `config/mod.rs:43-46` → `oidc.rs:9,11,13`, e `ZITADEL_ISSUER_URL/CLIENT_ID/CLIENT_SECRET` via `std::env::var` em `oidc.rs:27,29,31`. Introduzir `deny_unknown_fields`, alias, variável nova ou tocar em Rust transformaria uma correção de texto em mudança de startup — explicitamente fora do escopo.

**Independent Test**: `git diff --exit-code 35b8d3142 -- flora-core/src/config/mod.rs flora-core/src/config/oidc.rs flora-api/src/routes/auth.rs` não imprime nada; `grep -n "FLORA_OIDC__\|OIDC_ISSUER_URL\|ZITADEL_" .env.example` mostra o alias isolado removido e as sequências `ZITADEL_ISSUER_URL` preservadas.

**Acceptance Scenarios**:

1. **Given** `# OIDC_ISSUER_URL=http://localhost:8080` em `:65`, **When** a linha é removida, **Then** nenhuma ocorrência isolada de `OIDC_ISSUER_URL` permanece no template e a sequência `ZITADEL_ISSUER_URL` em `:66` continua intacta.
2. **Given** um deploy que ainda exporta a chave legada `FLORA_OIDC__REDIRECT_URI`, **When** a config é carregada, **Then** a chave segue sendo ignorada sem erro de startup, pois `Config` (`config/mod.rs:17-31`) permanece sem `deny_unknown_fields` — nenhum shim ou substituto é introduzido.

---

### User Story 3 — Diff estreito, gates determinísticos e reversível (Priority: P2)

Como maintainer, quero que a mudança fique restrita a `.env.example` (2 linhas substituídas, 1 removida, no máximo 1 redação ajustada em `:64`), além dos artefatos do ciclo 081, com gate em PASS e zero mutação de auth, docs, worktree, CI, DB ou segredos, para que o ciclo seja auditável e reversível pela restauração de um único path.

**Why this priority**: FR-004 e SC-004 são a fronteira da mudança. O custo é de edição de template (1 arquivo); qualquer efeito colateral — reformatar o bloco, tocar `oidc.rs`, atualizar o contrato OIDC, mexer no gate ou em workflows — transformaria uma correção documental em mudança de contrato que exige reavaliação.

**Independent Test**: `git diff --name-only 35b8d3142..HEAD` e `git diff --stat 35b8d3142..HEAD` mostram somente `.env.example` e os artefatos 081; `python3 scripts/ci/check_gate_integrity.py` permanece PASS e `cargo check -p flora-core` termina sem erro. A restauração de `.env.example` ao base devolve as linhas originais sem force-push.

**Acceptance Scenarios**:

1. **Given** a edição aplicada, **When** o diff é auditado contra `35b8d3142`, **Then** `:59-61` (`FLORA_OIDC__*`) e `:66-68` (`ZITADEL_*`) permanecem byte-idênticos, `:58` (cabeçalho) e `:69-70` (blank + próxima seção) não mudam, e nenhum outro path é listado.
2. **Given** os gates determinísticos do alvo, **When** executados sem DB, rede ou segredos, **Then** permanecem PASS e nenhum deles lê `.env` em runtime (`.env.example:33-34`: "this checkout has no Rust code that auto-loads `.env`").

---

### Edge Cases

- **Chave legada `FLORA_OIDC__REDIRECT_URI` exportada em deploy fora do repo**: `Config` sem `deny_unknown_fields` (`config/mod.rs:17-31`) faz a chave ser ignorada por `Config::load` (`:43-49`); comportamento fora do repo é **[NV]**, sem inferência além da evidência local (compatibilidade preservada pela ausência do atributo, não por shim).
- **Consumidores externos do template**: repos que copiam `.env.example` podem esperar `__REDIRECT_URI`, hoje já inerte; a remoção não quebra leitura de config. Consumo externo é **[NV]**.
- **`OIDC_ISSUER_URL` no contrato histórico**: `specs/001-flora-seed-v0-1/contracts/OIDC.md:10,17` e docs de research continuam com a menção — fora do escopo (SC-004); o grep de SC-001 é restrito a `.env.example`.
- **Exemplo descomentado com sintaxe inválida**: a linha deve usar aspas duplas internas (como `.env.example:35`); descomentá-la com aspas simples ou sem aspas não produz um array Figment válido. O comportamento de um deploy que o fizer é **[NV]** e não é induzido pelo template.
- **Duplicação do callback padrão na allowlist**: `auth.rs:105-117` já aceita o callback derivado de `FLORA_APP__BASE_URL` sem listagem; o exemplo comentado usa `http://127.0.0.1:4173/callback` (origem distinta de `localhost:3000`), preservando a allowlist exata sem duplicar o padrão.
- **Redação opcional de `:64`**: trocar `# Legacy/provider aliases used in some flows/docs:` por `# Legacy fallbacks read by \`OidcConfig::default()\` via \`std::env::var:\`` é permitido, mas não obrigatório; qualquer outra linha do bloco permanece intocada.
- **Worktree `state-closure-072` e docs de research**: contêm texto antigo; ficam fora do escopo por SC-004.
- **Reformatação automática**: a edição é local ao bloco OIDC; qualquer reindentação ou alteração de `:31-39` (exemplo de CORS) invalida FR-003/FR-004.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: substituir, em `.env.example`, as linhas atuais `:62` (`# Redirect/callback URI (verify config key support in your flow):`) e `:63` (`FLORA_OIDC__REDIRECT_URI=http://localhost:3000/auth/callback`) por comentário explicando que o callback padrão é derivado de `FLORA_APP__BASE_URL` (`flora-api/src/routes/auth.rs:101`, `{base_url}/api/v1/auth/callback`) e que URIs extras exigem `FLORA_OIDC__ALLOWED_REDIRECT_URIS` (`flora-core/src/config/oidc.rs:20-21`, `flora-api/src/routes/auth.rs:105-117`), seguido do exemplo **comentado** `# FLORA_OIDC__ALLOWED_REDIRECT_URIS=["http://127.0.0.1:4173/callback"]` (aspas duplas internas, sintaxe Figment 0.10.19 / `Cargo.lock:941-942`). Resultado: 3 linhas de comentário no lugar das 2 atuais.
- **FR-002**: remover a linha atual `:65` (`# OIDC_ISSUER_URL=http://localhost:8080`), que nunca é lida em Rust (apenas `specs/001-flora-seed-v0-1/contracts/OIDC.md:10,17` e docs históricos).
- **FR-003**: manter byte-idênticas as linhas `:58-61` (cabeçalho em `:58` e `FLORA_OIDC__ISSUER_URL/CLIENT_ID/CLIENT_SECRET` em `:59-61`, com leitores Figment via `config/mod.rs:43-46` → `oidc.rs:9,11,13`) e `:66-68` (`ZITADEL_*`, lidas por `std::env::var` em `oidc.rs:27,29,31`), além de `:64` salvo pela alteração opcional de redação. Não introduzir `deny_unknown_fields`, alias, variável nova nem qualquer mudança em arquivo Rust: uma chave legada `FLORA_OIDC__REDIRECT_URI` exportada por algum deploy continua sendo ignorada sem erro de startup (`config/mod.rs:17-31,43-49`).
- **FR-004**: limitar o diff rastreado a `.env.example` (2 linhas substituídas, 1 removida, no máximo 1 redação ajustada em `:64`), além dos 5 artefatos de assessment em `.specify/assessments/081-env-oidc-cleanup/` e dos documentos do próprio ciclo em `specs/081-env-oidc-cleanup/`; nenhum outro arquivo de código, config, doc, gate, CI ou lockfile é modificado; base `35b8d3142` preservada em todo o resto.

### Key Entities

N/A — a mudança é uma correção de documentação em `.env.example` (template de variáveis de ambiente); nenhuma entidade de dados, schema, migração ou contrato de leitura é introduzido. `OidcConfig` e `Config` permanecem byte-idênticos e nenhum novo campo, alias ou chave é criado.

## Success Criteria *(mandatory)*

- **SC-001**: `grep -n "FLORA_OIDC__\|OIDC_ISSUER_URL\|ZITADEL_" .env.example` mostra `FLORA_OIDC__ISSUER_URL`, `__CLIENT_ID`, `__CLIENT_SECRET` e `FLORA_OIDC__ALLOWED_REDIRECT_URIS` (comentado), e **zero** ocorrências de `FLORA_OIDC__REDIRECT_URI` e de `OIDC_ISSUER_URL` fora da sequência `ZITADEL_ISSUER_URL`. `[NV]` depende da edição.
- **SC-002**: a linha de exemplo permanece **comentada** e usa aspas duplas internas idênticas a `.env.example:35`; descomentá-la produz um array Figment válido (`figment 0.10.19`) que `Vec<String>` aceita. O callback derivado de `FLORA_APP__BASE_URL` continua válido e a allowlist exata de `auth.rs:105-117` não é duplicada com o callback padrão.
- **SC-003**: `git diff --name-only` lista somente `.env.example` e os artefatos 081 — os 5 de assessment em `.specify/assessments/081-env-oidc-cleanup/` (`intake.md`, `research.md`, `define.md`, `concept.md`, `decision.md`) e os 6 documentos do ciclo em `specs/081-env-oidc-cleanup/` (`spec.md`, `clarify.md`, `plan.md`, `checklists/requirements.md`, `checklist.md`, `tasks.md`); `python3 scripts/ci/check_gate_integrity.py` passa e o gate segue PASS; `cargo check -p flora-core` termina sem erro — nenhum desses comandos lê `.env` em runtime (`.env.example:33-34`). `[NV]` não executado nesta avaliação.
- **SC-004**: nenhuma mudança de comportamento de auth: `flora-core/src/config/oidc.rs`, `flora-core/src/config/mod.rs` e `flora-api/src/routes/auth.rs` permanecem byte-idênticos; zero mutação de docs de research, contrato OIDC, worktree `state-closure-072`, gate, CI, DB ou segredos; consumidores externos do template e CI `[NV]` — não verificáveis localmente, mas sem quebra de leitura de config.

## Scope / Out of scope

**Dentro**: `.env.example` — substituição de `:62-63` por 3 linhas de comentário (FR-001), remoção de `:65` (FR-002) e, no máximo, o realinhamento opcional da redação de `:64` para "Legacy fallbacks read by `OidcConfig::default()` via `std::env::var:`"; os 6 documentos do ciclo em `specs/081-env-oidc-cleanup/` (`spec.md`, `clarify.md`, `plan.md`, `checklists/requirements.md`, `checklist.md`, `tasks.md`); e os 5 artefatos de assessment já registrados em `.specify/assessments/081-env-oidc-cleanup/` (`intake.md`, `research.md`, `define.md`, `concept.md`, `decision.md`).

**Fora**: `flora-core/src/config/mod.rs`, `flora-core/src/config/oidc.rs`, `flora-api/src/routes/auth.rs`, as demais linhas do bloco OIDC (`:58-61`, `:66-68`) e o exemplo de CORS (`:31-39`), docs/specs de research, contrato `specs/001-flora-seed-v0-1/contracts/OIDC.md`, worktree espelho `state-closure-072`, gate/`scripts/ci/check_gate_integrity.py`, `.github/workflows/`, ADRs, constituição, `.specify/memory/STATE.md`, `specs/081-*` (exceto os artefatos do ciclo), DB/rede, segredos, `.tokens/`, fechamento de issues, leg RBAC e lockfiles.

## Assumptions

- O template não é lido em runtime: `.env.example:33-34` registra que não há código Rust que auto-carregue `.env`; a mudança é puramente documental (`.env.example:35` já mostra a sintaxe de array Figment com aspas duplas).
- `OidcConfig` (`oidc.rs:7-22`) não tem campo `redirect_uri`; `Config` (`config/mod.rs:17-31,43-49`) faz merge de `Serialized::defaults(Self::default())` com `Env::prefixed("FLORA_").split("__")` e não tem `deny_unknown_fields`, então chaves desconhecidas são ignoradas.
- `FLORA_OIDC__ALLOWED_REDIRECT_URIS` é o único mecanismo para URIs extras, com allowlist exata em `auth.rs:105-117`; o callback padrão `{base_url}/api/v1/auth/callback` deriva de `auth.rs:101`.
- `ZITADEL_*` são lidas por `std::env::var` em `oidc.rs:27,29,31`; `FLORA_OIDC__*` são lidas por Figment via `config/mod.rs:43-46` → `oidc.rs:9,11,13`.
- A base de comparação do diff é `35b8d3142` (HEAD da branch `feature/081-env-oidc-cleanup` no início do ciclo; merge `refactor(config): merge remocao da flag oidc inerte (#304)` que entregou 080).
- `.specify/feature.json` é estado local ignorado (`.specify/.gitignore:6`), reescrito a cada troca de feature; hoje registra `specs/081-env-oidc-cleanup` (a divergência 080→081 está resolvida em `clarify.md`).

## [NV]

- Execução dos gates determinísticos (`grep -n "FLORA_OIDC__\|OIDC_ISSUER_URL\|ZITADEL_" .env.example`, `git diff --name-only`, `python3 scripts/ci/check_gate_integrity.py`, `cargo check -p flora-core`, auditoria de `git diff --exit-code` nos paths sensíveis) — fica com o main agent após a edição; não executada nesta avaliação (marcadores em SC-001 e SC-003).
- Consumidores externos do template (repos que copiam `.env.example` e podem esperar `__REDIRECT_URI`, hoje inerte) e CI ao vivo — não verificáveis localmente; sem quebra de leitura de config por ausência de `deny_unknown_fields` em `config/mod.rs:17-31`.
- Comportamento de um deploy que descomente o exemplo com sintaxe inválida (deve usar aspas duplas, como `.env.example:35`) — não verificado e não induzido pelo template.
- Cópia em worktree `state-closure-072`, contrato OIDC histórico e docs de research — fora do escopo por SC-004.
- Status de CI ao vivo, merge, PRs, Forgejo, produção e fechamento de issues — não verificados nesta spec; nenhuma inferência além da evidência local e de `decision.md`.
