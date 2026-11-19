# Quality Checklist: 080 — remoção da flag OIDC morta `local_fallback_enabled`

**Purpose**: revisão de qualidade dos requisitos FR-001, FR-002, FR-003, FR-004 e dos critérios SC-001, SC-002, SC-003, SC-004, além das portarias do ciclo 080, antes da implementação.
**Created**: 2026-11-19T21:35Z
**Feature**: [spec.md](spec.md) · [define.md](../../.specify/assessments/080-remove-dead-oidc-flag/define.md) · [decision.md](../../.specify/assessments/080-remove-dead-oidc-flag/decision.md)

> Itens gerados sem marcações; `[ ]` = aguardando revisão de qualidade de requisitos, **não** implementação.

**Review Ownership**: artefato de revisão de qualidade de requisitos, de responsabilidade do revisor. Marcar `[x]` somente quando o critério de qualidade estiver satisfeito.
**Marker Semantics**: `[ ]` = critério gerado e pendente de revisão; `[x]` = revisado e satisfeito quanto à qualidade dos requisitos. Nenhum marcador indica implementação concluída. A revisão de referência está em `checklists/requirements.md`.

## Requisitos

- [x] FR-001 — remoção da doc `/// Local fallback enabled when Zitadel is unavailable (default: true in dev).` e da linha `pub local_fallback_enabled: bool,` em `flora-core/src/config/oidc.rs:18-19`, deixando `code_challenge_method` seguido de `allowed_redirect_uris` (`:20-23`, com `#[serde(default)]` intacto), especificada?
- [x] FR-002 — remoção de `local_fallback_enabled: true,` do `impl Default for OidcConfig` (`:40`), mantendo as demais 5 inicializações (`:29-41`) e o `Ok(())` de `validate()` (`:73`) inalterados, especificada?
- [x] FR-003 — ausência de `deny_unknown_fields`, `#[serde(default)]`, alias de env ou substituto, com `Config` (`config/mod.rs:17-31`) e `Config::load` (`:43-49`) ignorando a chave legada `FLORA_OIDC__LOCAL_FALLBACK_ENABLED` sem erro de startup, especificada?
- [x] FR-004 — diff rastreado restrito a `flora-core/src/config/oidc.rs` (3 linhas removidas) + 5 artefatos 080, sem outro arquivo de código/config/doc/gate/CI/lockfile, especificada?

## Critérios de sucesso

- [x] SC-001 — `grep -rn "local_fallback" --include='*.rs' flora-core/src apps` com zero correspondências após as duas remoções (menções restantes apenas em 3 docs de research e worktree, fora de escopo), verificável?
- [x] SC-002 — `cargo check -p flora-core`, `cargo clippy -p flora-core --all-targets -D warnings` sem erro/warning novo e `cargo fmt --check` limpo (`[NV]` até a execução), verificável?
- [x] SC-003 — `python3 scripts/ci/check_gate_integrity.py` PASS, gate do alvo PASS e `Config::load` aceitando a chave legada (`[NV]` até a execução), verificável?
- [x] SC-004 — `git diff --stat` com somente `oidc.rs` (−3 linhas) + 5 artefatos 080, base `e3944e962` preservado e zero mutação de `.env.example`/auth/docs/worktree/gate/CI/DB/segredos, verificável?

## Portarias

- [x] R1 — `go (scoped)` em `decision.md` (2026-11-19T21:32Z) antes de `specify`, com E=(3×5)/(1×1)=15,0 e opção A?
- [x] R3 — zero perguntas carry-forward registradas em `clarify.md` (divergência `feature.json` 079→080 via `--dry-run`, sem bloqueio)?
- [x] R4 e R7 — rastreabilidade individual (FR-001..FR-004 e SC-001..SC-004 em `spec.md`, `plan.md` e `tasks.md`, sem intervalos; SC-001 em T001 e T002) registrada, sem execução indevida?
- [x] Nenhuma invocação de `/speckit.validate` ou `/speckit.release`?
- [x] Fronteira — auth, `.env.example`, docs de research, worktree `state-closure-072`, `config/mod.rs` e demais módulos, gate/scripts, `.github/workflows/`, ADRs, constituição, STATE, `specs/080-*` (exceto artefatos), DB/rede, segredos, `.tokens/`, issues, leg RBAC e lockfiles fora de escopo?
- [x] `[NV]` — gates pós-edição e consumidores externos de `OidcConfig` separados da evidência determinística de código/`grep`/config/research?

## Notes

- Marcar `[x]` somente após revisão confirmar o critério de qualidade de requisitos; itens em aberto permanecem com `[ ]`.
- Este `checklist.md` é gerenciado por `/speckit-checklist`; `checklists/requirements.md` tem ciclo de vida próprio, mantido por `/speckit-specify` e `/speckit-clarify`.
- Registrar achados inline; vincular à evidência em `.specify/assessments/080-remove-dead-oidc-flag/`.
