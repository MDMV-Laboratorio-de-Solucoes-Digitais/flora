# Idea Research: 041 — Rust dependency debloat

- **Slug**: 041-rust-dead-crates
- **Created**: 2026-11-25
- **Evidence confidence (overall)**: medium

**Cobertura idiomática:** pt-BR_en (prosa em pt-BR; nomes de crates, comandos e rotas mantidos em inglês).

## Users & Demand

- O demandante é o maintainer/SE da MDMV; o tipo registrado é `cost-saving` e o gatilho é o objetivo "VPS barata" como contrapartida backend do debloat frontend do ciclo 040 (5.844 arquivos / 43 MB removidos). [source: .specify/assessments/041-rust-dead-crates/intake.md:6,10,19; specs/041-rust-dead-crates/spec.md:7-9 | system] (confidence: high)
- O benefício visado é redução do grafo de dependências e do checkout, não funcionalidade de usuário final: a proposta é remoção pura, com efeito comprovado apenas sobre tamanho de grafo/configuração. [source: .specify/assessments/041-rust-dead-crates/intake.md:14; .specify/assessments/041-rust-dead-crates/decision.md:19-22 | system] (confidence: high)
- Não há medição de memória/CPU, tempo de build em VPS ou custo de hospedagem anexada ao ciclo; qualquer ganho operacional além do tamanho do grafo é inferência. [source: inferência sem medição de runtime | ASSUMPTION] (confidence: medium)

## Prior Art

- Precedente interno: o ciclo 026 já removia dependências workspace fantasmas (`opentelemetry-otlp 0.22` inexistente no registro) e registrava a razão no próprio `Cargo.toml`, com tooling de tracing + `/metrics` como alternativa. [source: Cargo.toml:154-160 | system] (confidence: high)
- O Cargo Book (repositório oficial `rust-lang/cargo`) define `members`/`exclude` e `workspace.dependencies` como mecanismos do manifesto raiz; a remoção de members e deps segue esse modelo. [source: https://github.com/rust-lang/cargo/blob/master/doc/book/src/reference/workspaces.md | web] (confidence: high)
- A pesquisa c041-01 mapeou as declarações e encontrou `grep` vazio em `*.rs` (sem `init_i18n`, `build.rs`, `cfg(feature)` ou reexport), classificando `flora-tasks`/`flora-files` como folhas órfãs; `flora-search` foi explicitamente **não** considerada dead. [source: docs/research/2026-11-17-041-rust-debloat.md:22-28; docs/research/2026-11-17-040-seed-ui-debloat.md:49-51 | system] (confidence: high)
- O projeto já usa `cargo-vet` com exemptions versionadas em `supply-chain/config.toml` (incluindo `argon2`, `dotenvy`, `jsonwebtoken`), e o ciclo removeu 6 dessas entradas preservando `dotenvy` e `jsonwebtoken/rust_crypto`. [source: supply-chain/config.toml:63-65,388-390,749-754; specs/041-rust-dead-crates/spec.md:16-17 | system] (confidence: high)
- O ciclo 040 estabeleceu o protocolo de verificação (baseline antes, gates idênticos depois, diff auditável, auditoria author-blind em contexto fresco) reutilizado pelo 041. [source: ROADMAP.md:58-59; .specify/validations/041-rust-dead-crates/validate.md:6-15 | system] (confidence: high)

## Market & Context

- O ROADMAP registra o ciclo 041 como entregue com −34 pacotes / −369 linhas do `Cargo.lock` e zero adições, gates idênticos ao baseline. [source: ROADMAP.md:59 | system] (confidence: high)
- O validate registra `Cargo.lock` 4898 → 4529 linhas, 34 pacotes removidos e nenhum novo; as 9 linhas `+ "bitflags"` são desambiguação após a saída do `bitflags 1.x` trazido por `bcrypt`, não adições de pacote. [source: .specify/validations/041-rust-dead-crates/validate.md:17-28; specs/041-rust-dead-crates/spec.md:18-20 | system] (confidence: high)
- A autonomia para merge vem de ADR-0003: push/PR/merge como decisão de máquina sob `frontend-build`, `rust-build`, `db-tests`, `dependency-review` verdes. [source: .specify/assessments/041-rust-dead-crates/decision.md:24-26; scripts/ci/check_gate_integrity.py:28-32 | system] (confidence: high)
- RbacService wiring (c041-02) e RLS/role least-privilege (c041-03) ficaram fora de escopo por exigirem decisão de autoridade do dono/SE; permanecem **[NV]**. [source: .specify/assessments/041-rust-dead-crates/decision.md:11-17; docs/research/2026-11-17-041-rust-debloat.md:29-37 | system] (confidence: high)
- Não foi consultado `crates.io` nem `docs.rs` nesta sessão (hosts não allowlisted); o delta de pacotes é proveniente apenas dos artefatos locais e do `Cargo.lock` da worktree. [source: política do brief; ausência de fetch externo | ASSUMPTION] (confidence: high)

## Data & Constraints

- Escopo registrado: remover 2 members (`flora-tasks`, `flora-files`), 5 workspace deps (`flora-tasks`, `flora-files`, `rust-i18n`, `argon2`, `bcrypt`), 4 deps de `flora-core` (`dotenv`, `argon2`, `bcrypt`, `rust-i18n`) e 6 exemptions do vet; preservar `dotenvy` e `jsonwebtoken/rust_crypto`. [source: specs/041-rust-dead-crates/spec.md:11-21; specs/041-rust-dead-crates/tasks.md:4-9 | system] (confidence: high)
- Gates registrados: `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo deny check bans licenses sources`, `cargo test --workspace` (46 passed / 43 ignored, idêntico ao baseline), `cargo vet check`; nenhum uso órfão após a remoção. [source: .specify/validations/041-rust-dead-crates/validate.md:6-15,30-36 | system] (confidence: high)
- Auditoria author-blind: F-1 (P1) foi classificado como falso positivo por concorrência (auditor rodou durante edição de `rbac.rs` do ciclo 042); reexecução com `CARGO_TARGET_DIR` isolado compilou 330 crates com exit 0, e a lição registrada foi auditar sobre snapshot congelado. [source: .specify/validations/041-rust-dead-crates/validate.md:38-53; .specify/validations/042-real-rbac-list/validate.md:29-34 | system] (confidence: high)
- No checkout atual (`e88dc8e4d`), os members órfãos não existem (`Cargo.toml:26-32` lista 5 members) e não há declaração de `bcrypt`/`dotenv`/`rust-i18n` em manifestos; porém `flora-core/Cargo.toml:35` **ainda declara `argon2 0.5.3`**, usado em `flora-core/src/services/password_service.rs:10-13`, com exemption ativa em `supply-chain/config.toml:63-65`. [source: Cargo.toml:26-32,118-124; flora-core/Cargo.toml:23-43; flora-core/src/services/password_service.rs:1-13; supply-chain/config.toml:63-65 | system] (confidence: high)
- O `Cargo.lock` desta worktree tem **4567 linhas**, divergente das 4529 registradas como resultado do ciclo 041; o histórico local registra que o ciclo 096 reintroduziu `argon2` para fallback de senha (FR-022), o que explica a divergência sem autorizar conclusão sobre o estado remoto. [source: Cargo.lock (wc -l); docs/governance/2026-11-21-096-auth-login-ptbr.md:20,26; docs/PROGRESSO.md:303 | system] (confidence: high)
- Conexão com DB usa superuser `postgres` e a migration de schema habilita RLS em 11 tabelas sem policies; portanto o isolamento continua dependente de `WHERE organization_id = ?` na aplicação, não do banco. [source: .env.example:24; docker-compose.yml:17; flora-core/src/migrations/sql/20240818000001_init_schema.sql:200-214; docs/research/2026-11-17-041-rust-debloat.md:34-37 | system] (confidence: high)
- Forgejo/proteções e CI remoto não observados nesta sessão: **[NV]**. [source: .specify/assessments/041-rust-dead-crates/decision.md:24-26; specs/041-rust-dead-crates/spec.md:36-38 | system] (confidence: high)

## Evidence Against the Idea

- A premissa "zero call sites" não é estável no tempo: `argon2` voltou a ter call site e exemption neste checkout (`password_service.rs:10-13`), documentado como reintrodução deliberada no ciclo 096 para o fallback de senha do FR-022. [source: flora-core/Cargo.toml:35; flora-core/src/services/password_service.rs:10-13; docs/governance/2026-11-21-096-auth-login-ptbr.md:20,26 | system] (confidence: high)
- O delta exato (4898 → 4529, 34 pacotes) é evidência do ciclo 041, mas o `Cargo.lock` atual tem 4567 linhas; tratar o número histórico como estado atual seria erro de leitura. [source: .specify/validations/041-rust-dead-crates/validate.md:17-20; Cargo.lock (wc -l) | system] (confidence: high)
- Nenhum delta de memória/CPU/runtime foi medido; o próprio artefato veta afirmações de performance além de tamanho de grafo/configuração. [source: .specify/assessments/041-rust-dead-crates/decision.md:19-22; specs/041-rust-dead-crates/spec.md:36-37 | system] (confidence: high)
- O ciclo adiou justamente as mudanças de maior impacto em segurança: `revoke_role` não implementado, `PermissionService` fora do `AppState`, sem DTO/semântica/último Owner; RLS inerte com superuser e `FORCE` às cegas zeraria linhas. [source: .specify/assessments/041-rust-dead-crates/decision.md:11-17; docs/research/2026-11-17-041-rust-debloat.md:29-37; flora-core/src/migrations/sql/20240818000001_init_schema.sql:200-214 | system] (confidence: high)
- O achado F-1 mostrou que a verificação author-blind pode produzir falso positivo P1 quando roda sobre árvore em edição; a confiança no "tudo verde" depende de snapshot congelado. [source: .specify/validations/041-rust-dead-crates/validate.md:45-53 | system] (confidence: high)
- O escopo não pode ser generalizado para todo "crate aparentemente morto": `flora-search` foi explicitamente preservada, e 'zero refs' era afirmação validada por grep em um recorte de tempo. [source: docs/research/2026-11-17-040-seed-ui-debloat.md:49-51; docs/research/2026-11-17-041-rust-debloat.md:24-28 | system] (confidence: high)

## Gaps & Open Questions

- [NEEDS CLARIFICATION: Como confirmar que as dependências e os members não possuem call sites indiretos via `build.rs`, feature-gate ou reexport, e como repetir essa checagem após reintroduções como a do ciclo 096?] [source: .specify/assessments/041-rust-dead-crates/intake.md:23; docs/governance/2026-11-21-096-auth-login-ptbr.md:26 | system] (confidence: high)
- [NEEDS CLARIFICATION: Qual efeito mensurável em memória/CPU se espera, além da redução do tamanho do grafo?] [source: .specify/assessments/041-rust-dead-crates/intake.md:24 | system] (confidence: high)
- [NEEDS CLARIFICATION: Quem autoriza o papel least-privilege e as políticas de RLS, hoje marcados como [NV] (c041-03)?] [source: .specify/assessments/041-rust-dead-crates/intake.md:25; .specify/memory/STATE.md:297 | system] (confidence: high)
- [NEEDS CLARIFICATION: O que falta para o wiring do RbacService entrar neste escopo, incluindo assign/revoke e proteção last-Owner (c041-02)?] [source: .specify/assessments/041-rust-dead-crates/intake.md:26; .specify/assessments/041-rust-dead-crates/decision.md:11-14 | system] (confidence: high)
- [NEEDS CLARIFICATION: A contagem de 46 passed / 43 ignored permanece idêntica após a regeneração do `Cargo.lock` no estado atual?] [source: .specify/assessments/041-rust-dead-crates/intake.md:27; .specify/validations/041-rust-dead-crates/validate.md:14 | system] (confidence: high)
- [NEEDS CLARIFICATION: As isenções removidas do cargo-vet afetam `dotenvy` ou `jsonwebtoken/rust_crypto`, que devem ser mantidos?] [source: .specify/assessments/041-rust-dead-crates/intake.md:28; supply-chain/config.toml:388-390,749-754 | system] (confidence: high)

## Sources

- `system` — `.specify/assessments/041-rust-dead-crates/intake.md:6-28` e `decision.md:5-26` — lidos diretamente 2026-11-25; policy: source local primária.
- `system` — `specs/041-rust-dead-crates/spec.md:6-38` e `tasks.md:3-12` — lidos diretamente 2026-11-25; policy: source local primária.
- `system` — `.specify/validations/041-rust-dead-crates/validate.md:6-63` — lido diretamente 2026-11-25; policy: source local primária.
- `system` — `docs/research/2026-11-17-041-rust-debloat.md:22-46`, `docs/research/2026-11-17-040-seed-ui-debloat.md:49-51`, `docs/research/index.md:159` — lidos diretamente 2026-11-25; policy: source local primária.
- `system` — `Cargo.toml:26-32,118-190`, `flora-core/Cargo.toml:23-69`, `Cargo.lock` (contagem de linhas), `supply-chain/config.toml:63-65,388-390,749-754` — lidos diretamente 2026-11-25; policy: source local primária.
- `system` — `flora-core/src/services/password_service.rs:1-13`, `.env.example:24`, `docker-compose.yml:17`, `flora-core/src/migrations/sql/20240818000001_init_schema.sql:200-214` — lidos diretamente 2026-11-25; policy: source local primária.
- `system` — `docs/governance/2026-11-21-096-auth-login-ptbr.md:20,26`, `docs/PROGRESSO.md:303`, `ROADMAP.md:58-59`, `.specify/memory/STATE.md:292-297`, `scripts/ci/check_gate_integrity.py:28-32` — lidos diretamente 2026-11-25; policy: source local primária.
- `web` — `https://github.com/rust-lang/cargo/blob/master/doc/book/src/reference/workspaces.md` — host `github.com`, policy `allowlisted`, acesso 2026-11-25 (o leitor serviu o espelho `raw.githubusercontent.com` do mesmo arquivo; nenhum link da página foi seguido).
- `auto-refused` — `https://crates.io/crates/argon2`, `https://crates.io/crates/bcrypt`, `https://docs.rs/rust-i18n` — hosts não allowlisted; **[UNVERIFIED — fetch skipped: host not allowlisted]**; nenhum dado de versão/vulnerabilidade tomado desses hosts.
- `auto-refused` — `https://doc.rust-lang.org/cargo/reference/workspaces.html` — host não allowlisted; **[UNVERIFIED — fetch skipped: host not allowlisted]**; referência equivalente obtida via repositório `github.com` allowlisted.
