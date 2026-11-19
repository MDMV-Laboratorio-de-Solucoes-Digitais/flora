# Requirements Quality Checklist — 081

**Feature**: [spec.md](../spec.md) · **Slug**: 081-env-oidc-cleanup
**Created**: 2026-11-19T22:10Z

> `[x]` = requisito revisado quanto à qualidade, **não** implementação concluída. Revisão em 2026-11-19T22:10Z, contra `define.md`/`decision.md` (acessados 2026-11-19T22:10Z). A execução dos gates e a compatibilidade com consumidores externos do template permanecem `[NV]`.

## Completude

- [x] FR-001 especifica a substituição exata, em `.env.example`, das linhas `:62` (`# Redirect/callback URI (verify config key support in your flow):`) e `:63` (`FLORA_OIDC__REDIRECT_URI=http://localhost:3000/auth/callback`) por 3 linhas de comentário que explicam que o callback padrão é derivado de `FLORA_APP__BASE_URL` (`flora-api/src/routes/auth.rs:101`, `{base_url}/api/v1/auth/callback`) e que URIs extras exigem `FLORA_OIDC__ALLOWED_REDIRECT_URIS` (`oidc.rs:20-21`, allowlist exata em `auth.rs:105-117`), incluindo o exemplo **comentado** `# FLORA_OIDC__ALLOWED_REDIRECT_URIS=["http://127.0.0.1:4173/callback"]` (aspas duplas internas, sintaxe Figment 0.10.19 / `Cargo.lock:941-942`).
- [x] FR-002 especifica a remoção da linha atual `:65` (`# OIDC_ISSUER_URL=http://localhost:8080`), nunca lida em Rust — apenas em `specs/001-flora-seed-v0-1/contracts/OIDC.md:10,17` e docs históricos.
- [x] FR-003 exige byte-identicidade de `:58-61` (`FLORA_OIDC__ISSUER_URL/CLIENT_ID/CLIENT_SECRET`, com leitores Figment via `config/mod.rs:43-46` → `oidc.rs:9,11,13`) e `:66-68` (`ZITADEL_*`, lidos por `std::env::var` em `oidc.rs:27,29,31`), além de `:64` salvo pela alteração opcional de redação para "Legacy fallbacks read by `OidcConfig::default()` via `std::env::var:`"; proíbe `deny_unknown_fields`, alias, variável nova e qualquer mudança em arquivo Rust — a chave legada `FLORA_OIDC__REDIRECT_URI` exportada por algum deploy continua sendo ignorada sem erro de startup (`config/mod.rs:17-31,43-49`).
- [x] FR-004 limita o diff rastreado a `.env.example` (2 linhas substituídas, 1 removida, no máximo 1 redação ajustada em `:64`), além dos 5 artefatos `.specify/assessments/081-env-oidc-cleanup/` e dos documentos do próprio ciclo em `specs/081-env-oidc-cleanup/`; nenhum outro arquivo de código, config, doc, gate, CI ou lockfile é modificado; base `35b8d3142` preservada em todo o resto.

## Clareza e consistência

- [x] SC-001 é verificável: `grep -n "FLORA_OIDC__\|OIDC_ISSUER_URL\|ZITADEL_" .env.example` mostra `FLORA_OIDC__ISSUER_URL`, `__CLIENT_ID`, `__CLIENT_SECRET` e `FLORA_OIDC__ALLOWED_REDIRECT_URIS` (comentado), com **zero** ocorrências de `FLORA_OIDC__REDIRECT_URI` e de `OIDC_ISSUER_URL` fora da sequência `ZITADEL_ISSUER_URL`; o `grep` só zera após T001 e T002. `[NV]` depende da edição.
- [x] SC-002 é verificável: a linha de exemplo permanece **comentada** e usa aspas duplas internas idênticas a `.env.example:35`; descomentá-la produz um array Figment válido (`figment 0.10.19`) aceito por `Vec<String>`, sem duplicar a allowlist exata de `auth.rs:105-117` com o callback derivado de `FLORA_APP__BASE_URL`.
- [x] SC-003 é verificável: `git diff --name-only` e `git diff --stat` listam somente `.env.example` e os artefatos 081; `python3 scripts/ci/check_gate_integrity.py` passa e o gate segue PASS; `cargo check -p flora-core` termina sem erro — nenhum desses comandos lê `.env` em runtime (`.env.example:33-34`). `[NV]` não executado nesta avaliação.
- [x] SC-004 é verificável: `flora-core/src/config/oidc.rs`, `flora-core/src/config/mod.rs` e `flora-api/src/routes/auth.rs` permanecem byte-idênticos; zero mutação de docs de research, contrato OIDC, worktree, gate, CI, DB ou segredos; consumidores externos do template e CI `[NV]` — não verificáveis localmente, mas sem quebra de leitura de config.
- [x] IDs explícitos e individuais: FR-001, FR-002, FR-003, FR-004 e SC-001, SC-002, SC-003, SC-004 — sem intervalos (R4/R7); a rastreabilidade por T001..T004 em `plan.md` e `tasks.md` usa cada ID nominalmente, com SC-001 em T001 e T002 (o `grep` só zera após a substituição em `:62-63` e a remoção em `:65`).
- [x] Evidência determinística (template `.env.example:31-35,58-68` colado; leitores Figment em `config/mod.rs:17-31,43-49` → `oidc.rs:9,11,13,20-21,27,29,31`; callback e allowlist em `auth.rs:101,105-117`; contrato histórico `specs/001-flora-seed-v0-1/contracts/OIDC.md:10,17`; `figment 0.10.19` em `Cargo.lock:941-942`; drift de config em research) distinguida de `[NV]` para gates pós-edição e consumidores externos.

## Edge cases

- [x] E se as linhas `:62-63` ou `:65` já estiverem ausentes? FR-001/FR-002 são idempotentes: nada a substituir/remover e SC-001 já satisfeito.
- [x] E se um operador descomentar o exemplo com aspas simples ou sem aspas? SC-002 exige aspas duplas internas idênticas a `.env.example:35`; sintaxe inválida é risco de config do operador, registrado como `[NV]` em `decision.md`, sem adicionar validação em Rust (FR-003).
- [x] E se algum deploy exportar `FLORA_OIDC__REDIRECT_URI`? Sem `deny_unknown_fields`, a chave continua sendo ignorada com segurança e sem erro de startup (FR-003/SC-003); compatibilidade preservada.
- [x] E se `:58-61` ou `:66-68` forem alterados por acidente? FR-003 exige byte-idênticos; SC-003/SC-004 detectam qualquer diff além das linhas permitidas.
- [x] E se houver risco de DB/rede/segredos/`.tokens/`? Nenhum comando de DB é executado; a fronteira de paths permanece explícita em FR-004/SC-004.

## Portarias

- [x] R1 `go (scoped)` registrado em `decision.md` (2026-11-19T22:06Z) antes da escrita dos artefatos, com E=(2×5)/(1×1)=10,0 e opção A (editar somente `.env.example`) eleita.
- [x] R3 zero perguntas carry-forward registradas em `clarify.md` (divergência `feature.json` 080→081 documentada via `--dry-run`, sem bloqueio); os `[NV]` não são perguntas bloqueantes.
- [x] R4 e R7 respeitados na rastreabilidade: FR/SC individuais em `spec.md`, `plan.md` e `tasks.md`, sem intervalos; T001..T004 rastreáveis, sem invocar `/speckit.validate` nem `/speckit.release`.
- [x] Fora de escopo: `flora-core/src/config/{mod,oidc}.rs`, `flora-api/src/routes/auth.rs`, docs/specs de research, contrato `specs/001-flora-seed-v0-1/contracts/OIDC.md`, worktree espelho `state-closure-072`, gate/`scripts/ci/check_gate_integrity.py`, `.github/workflows/`, ADRs, constituição, STATE, specs/081-* (exceto artefatos), DB/rede, segredos, `.tokens/`, leg RBAC e lockfiles.
