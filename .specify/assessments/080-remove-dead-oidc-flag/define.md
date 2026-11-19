# Define: remover flag OIDC morta `local_fallback_enabled`

- **Slug**: 080-remove-dead-oidc-flag · **Definido**: 2026-11-19T21:32Z

## Problema

`flora-core/src/config/oidc.rs:18-19` expõe `local_fallback_enabled` com default `true` (`:40`) e doc que promete fallback local quando o Zitadel está indisponível. A flag nunca é lida: `validate()` (`:52-74`) a ignora e não há nenhum outro consumidor (`OidcConfig` só em `flora-core/src/config/mod.rs:24,59`). O resultado é uma garantia de segurança **falsa** — exatamente o risco G9 registrado em `docs/research/2026-11-16-seed-oidc-identity-gap.md:120` ("declared but never consulted") — além de dívida de config que pode induzir operadores a confiarem em comportamento inexistente. A correção é de subcão de custo: remover a declaração e o default, sem tocar auth.

## Afetados / metas / não-metas

Afetados: maintainers e agentes que leem a config OIDC; nenhum runtime (a flag não era consultada). Metas: remover exatamente 3 linhas de `flora-core/src/config/oidc.rs` — a doc+campo em `:18-19` e a inicialização em `:40` — preservando byte-idêntico o restante do struct, `Default`, `validate()` e todos os demais arquivos. Não-metas: código de auth (`auth.rs`, `services/auth_service.rs`), `.env.example`, docs de research, worktree `state-closure-072`, gate/`scripts/ci/check_gate_integrity.py`, `.github/workflows/`, ADRs, constituição, STATE, specs/080-* (exceto os 5 artefatos), DB/rede, segredos, `.tokens/`, fechamento de issues, leg RBAC e qualquer realocação do campo.

## Requisitos funcionais

- **FR-001** — remover de `flora-core/src/config/oidc.rs` a doc `/// Local fallback enabled when Zitadel is unavailable (default: true in dev).` e a linha `pub local_fallback_enabled: bool,` (atual `:18-19`), deixando `code_challenge_method` seguido diretamente de `allowed_redirect_uris` (`:20-23`, com seu `#[serde(default)]` intacto).
- **FR-002** — remover a linha `local_fallback_enabled: true,` do `impl Default for OidcConfig` (atual `:40`), mantendo as demais 5 inicializações (`:29-41`) e o `Ok(())` final de `validate()` (`:73`) inalterados.
- **FR-003** — não introduzir `deny_unknown_fields`, `#[serde(default)]`, alias de env ou qualquer substituto para o campo: `Config` (`flora-core/src/config/mod.rs:17-31`) permanece sem `deny_unknown_fields` e `Config::load` (`:43-49`) continua ignorando a chave legada `FLORA_OIDC__LOCAL_FALLBACK_ENABLED` sem erro de startup.
- **FR-004** — limitar o diff rastreado a `flora-core/src/config/oidc.rs` (3 linhas removidas), além dos 5 artefatos `.specify/assessments/080-remove-dead-oidc-flag/`; nenhum outro arquivo de código, config, doc, gate, CI ou lockfile é modificado.

## Métricas

- **SC-001** — `grep -rn "local_fallback" --include='*.rs' flora-core/src apps` retorna **zero** correspondências (as únicas menções restantes no repo são os 3 docs de research históricos e a cópia em worktree, fora do escopo).
- **SC-002** — `cargo check -p flora-core` e `cargo clippy -p flora-core --all-targets -D warnings` terminam sem erro nem novo warning; `cargo fmt --check` limpo (a edição é só remoção de linhas, sem reindentação). `[NV]` não executado nesta avaliação.
- **SC-003** — `python3 scripts/ci/check_gate_integrity.py` passa e o gate do alvo segue PASS; `Config::load` continua aceitando um ambiente com a chave legada definida (compat por ausência de `deny_unknown_fields`). `[NV]` depende da execução pós-edição.
- **SC-004** — `git diff --stat` mostra somente `flora-core/src/config/oidc.rs` com 3 linhas a menos e os 5 artefatos 080; zero mutação de `.env.example`, auth, docs, worktree, gate, CI, DB ou segredos; base `e3944e962` preservada em todo o resto.

## Baseline

Branch `feature/080-remove-dead-oidc-flag`, HEAD `e3944e962`, gate PASS, zero PRs. `flora-core/src/config/oidc.rs:18-19,40` com a flag viva; `flora-core/src/config/mod.rs:17-31,43-49` sem `deny_unknown_fields`; `.env.example:58-68` sem a chave; zero leitores em Rust; research G9/NV-7 documenta a lacuna.
