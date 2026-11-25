# Decision: remover flag OIDC morta `local_fallback_enabled`

- **Slug**: 080-remove-dead-oidc-flag
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | define.md | concept.md | decision.md (anterior, decidido em 2026-11-19T21:32Z) | docs/research G9/NV-7 citados via intake/research

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | Flag write-only com default `true` que promete fallback inexistente (`oidc.rs:18-19,40`), registrada como risco P1 em G9 (`docs/research/2026-11-16-seed-oidc-identity-gap.md:120,147`) e NV-7 (`...016-auth-rbac-oidc-minimal-vps.md:56`). |
| Evidence strength | strong | Declaração/default colados, `grep` com zero leitores em Rust, `validate()` sem menção à flag, `Config` sem `deny_unknown_fields` e `.env.example:58-68` sem a chave (`research.md:7-13`). |
| Value vs. inaction | strong | Inação preserva uma garantia de segurança falsa e dívida de configuração que pode induzir operadores a confiar em comportamento inexistente (`define.md:7`). |
| Feasibility / appetite | strong | Opção A: remover 3 linhas de um único arquivo (`flora-core/src/config/oidc.rs:18-19,40`); sem DB, rede, segredos ou mudança de contrato de leitura (`concept.md:5-9`). |
| Strategic fit | strong | Remove o campo exatamente onde vive, sem criar segundo mecanismo nem shim e sem tocar auth, `config/mod.rs` ou `.env.example` (`concept.md:8-9,21-23`). |
| Risk posture | strong | Sem `deny_unknown_fields`, `FLORA_OIDC__LOCAL_FALLBACK_ENABLED` legado é ignorado sem erro de startup; sem consumidor de `Serialize` sobre config; diff de 1 arquivo (SC-004). `[NV]` gates pós-edição e deploy fora do repo. |

## Verdict & Rationale

Veredito histórico **`go (scoped)` preservado como `go`**; decisão original em 2026-11-19T21:32Z, branch `feature/080-remove-dead-oidc-flag`, HEAD-base `e3944e962`, gate PASS, zero PRs. Nada nos artefatos inverte a escolha: a flag nunca é lida (`validate()` em `oidc.rs:52-74`, único uso do struct em `config/mod.rs:24,59`), e G9/NV-7 registram explicitamente "declared but never consulted" e a recomendação "remover/implementar" (`:120,147` / `:56`). 80/20 original integral: **E=(3×5)/(1×1)=15,0** — valor 3 (remove garantia de segurança falsa + dívida de config), confiança 5 (evidência local integral), esforço 1 (3 linhas, 1 arquivo), risco 1 (zero leitores, sem consumidor de serialização). Alternativas do ciclo: `.env.example` 10,0 · `cssVariables` 5,0 · lucide 2,50 · OTLP 1,25 (sem receptor/dono G1, deps 0.33.1, risco alto). Opções B (implementar fallback) e C (manter flag morta) rejeitadas em `concept.md:11-19` — a primeira é o próprio risco G9, a segunda é o estado que 080 corrige. Os `[NV]` (execução de gates e comportamento de env legado em deploy fora do checkout) não bloqueiam o go.

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; nenhuma pergunta bloqueante nos artefatos).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: `flora-core/src/config/oidc.rs:18-19` expõe `local_fallback_enabled` com default `true` em `:40`, mas a flag nunca é consultada — garantia de segurança falsa e dívida de config (G9/NV-7).
- **Chosen approach**: Opção A — remoção direta de doc+campo (`:18-19`) e da inicialização em `:40`, sem substituto, sem `deny_unknown_fields`, sem alias de env.
- **In scope / out of scope**:
  - **In**: exatamente `flora-core/src/config/oidc.rs` (3 linhas) e os 5 artefatos 080. FR-001..FR-004 e SC-001..SC-004 de `define.md`.
  - **Out**: auth (`auth.rs`, `services/auth_service.rs`), `.env.example`, docs de research, worktree `state-closure-072`, `config/mod.rs` e demais módulos, gate/`scripts/ci/check_gate_integrity.py`, `.github/workflows/`, ADRs, constituição, STATE, specs/080-* (exceto artefatos), DB/rede, segredos, `.tokens/`, fechamento de issues, leg RBAC e lockfiles.
- **Success metrics**: `grep -rn "local_fallback" --include='*.rs' flora-core/src apps` → zero; `cargo check -p flora-core`, `cargo fmt --check`, `cargo clippy -p flora-core --all-targets -D warnings` e `python3 scripts/ci/check_gate_integrity.py` limpos; `git diff` só o arquivo-alvo + 5 artefatos, base `e3944e962` preservada.
- **Carried-forward open questions**: `[NV]` execução dos gates pós-edição (main agent); `[NV]` comportamento de deploy fora do repo com `FLORA_OIDC__LOCAL_FALLBACK_ENABLED` (compat preservada pela ausência de `deny_unknown_fields`); nenhuma bloqueia o go.
