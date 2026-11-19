# Quality Checklist: 081 — limpeza das chaves OIDC em `.env.example`

**Purpose**: revisão de qualidade dos requisitos FR-001, FR-002, FR-003, FR-004 e dos critérios SC-001, SC-002, SC-003, SC-004, além das portarias do ciclo 081, antes da implementação.
**Created**: 2026-11-19T22:10Z
**Feature**: [spec.md](spec.md) · [define.md](../../.specify/assessments/081-env-oidc-cleanup/define.md) · [decision.md](../../.specify/assessments/081-env-oidc-cleanup/decision.md)

> Itens gerados sem marcações; `[ ]` = aguardando revisão de qualidade de requisitos, **não** implementação.

**Review Ownership**: artefato de revisão de qualidade de requisitos, de responsabilidade do revisor. Marcar `[x]` somente quando o critério de qualidade estiver satisfeito.
**Marker Semantics**: `[ ]` = critério gerado e pendente de revisão; `[x]` = revisado e satisfeito quanto à qualidade dos requisitos. Nenhum marcador indica implementação concluída. A revisão de referência está em `checklists/requirements.md`.

## Requisitos

- [x] FR-001 — substituição, em `.env.example`, das linhas `:62-63` por 3 linhas de comentário que explicam o callback derivado de `FLORA_APP__BASE_URL` (`auth.rs:101`) e o uso de `FLORA_OIDC__ALLOWED_REDIRECT_URIS` (`oidc.rs:20-21`, `auth.rs:105-117`), com o exemplo comentado `# FLORA_OIDC__ALLOWED_REDIRECT_URIS=["http://127.0.0.1:4173/callback"]` (aspas duplas internas, sintaxe Figment 0.10.19), especificada?
- [x] FR-002 — remoção da linha `:65` (`# OIDC_ISSUER_URL=http://localhost:8080`), nunca lida em Rust (apenas contrato histórico e docs), especificada?
- [x] FR-003 — byte-identicidade de `:58-61` (`FLORA_OIDC__ISSUER_URL/CLIENT_ID/CLIENT_SECRET`) e `:66-68` (`ZITADEL_*`), `:64` salvo pela redação alinhada a "Legacy fallbacks read by `OidcConfig::default()` via `std::env::var:`", sem `deny_unknown_fields`, alias, variável nova ou mudança em Rust (chave legada `FLORA_OIDC__REDIRECT_URI` segue ignorada sem erro de startup), especificada?
- [x] FR-004 — diff rastreado restrito a `.env.example` (2 linhas substituídas, 1 removida, no máximo 1 redação em `:64`) + 5 artefatos 081 e documentos do ciclo, sem outro arquivo de código/config/doc/gate/CI/lockfile, com base `35b8d3142` preservada, especificada?

## Critérios de sucesso

- [x] SC-001 — `grep -n "FLORA_OIDC__\|OIDC_ISSUER_URL\|ZITADEL_" .env.example` mostra `__ISSUER_URL`, `__CLIENT_ID`, `__CLIENT_SECRET` e `__ALLOWED_REDIRECT_URIS` (comentado), com zero `FLORA_OIDC__REDIRECT_URI` e zero `OIDC_ISSUER_URL` isolado (só zera após T001 e T002; `[NV]` até a edição), verificável?
- [x] SC-002 — linha de exemplo comentada com aspas duplas internas idênticas a `.env.example:35`, produzindo array Figment válido aceito por `Vec<String>`, sem duplicar a allowlist exata de `auth.rs:105-117`, verificável?
- [x] SC-003 — `git diff --name-only`/`git diff --stat` com somente `.env.example` e artefatos 081, `python3 scripts/ci/check_gate_integrity.py` PASS e `cargo check -p flora-core` sem erro, nenhum comando lendo `.env` em runtime (`[NV]` até a execução), verificável?
- [x] SC-004 — `oidc.rs`, `config/mod.rs` e `auth.rs` byte-idênticos, zero mutação de docs de research, contrato OIDC, worktree `state-closure-072`, gate, CI, DB ou segredos, com consumidores externos e CI `[NV]`, verificável?

## Portarias

- [x] R1 — `go (scoped)` em `decision.md` (2026-11-19T22:06Z) antes de `specify`, com E=(2×5)/(1×1)=10,0 e opção A (editar só `.env.example`)?
- [x] R3 — zero perguntas carry-forward registradas em `clarify.md` (divergência `feature.json` 080→081 via `--dry-run`, sem bloqueio)?
- [x] R4 e R7 — rastreabilidade individual (FR-001..FR-004 e SC-001..SC-004 em `spec.md`, `plan.md` e `tasks.md`, sem intervalos; SC-001 em T001 e T002) registrada, sem execução indevida?
- [x] Nenhuma invocação de `/speckit.validate` ou `/speckit.release`?
- [x] Fronteira — `flora-core/src/config/{mod,oidc}.rs`, `flora-api/src/routes/auth.rs`, docs/specs de research, contrato OIDC, worktree `state-closure-072`, gate/scripts, `.github/workflows/`, ADRs, constituição, STATE, specs/081-* (exceto artefatos), DB/rede, segredos, `.tokens/`, leg RBAC e lockfiles fora de escopo?
- [x] `[NV]` — gates pós-edição e consumidores externos do template separados da evidência determinística de `.env.example`/leitores Figment/`std::env::var`/callback/research?

## Notes

- Marcar `[x]` somente após revisão confirmar o critério de qualidade de requisitos; itens em aberto permanecem com `[ ]`.
- Este `checklist.md` é gerenciado por `/speckit-checklist`; `checklists/requirements.md` tem ciclo de vida próprio, mantido por `/speckit-specify` e `/speckit-clarify`.
- Registrar achados inline; vincular à evidência em `.specify/assessments/081-env-oidc-cleanup/`.
