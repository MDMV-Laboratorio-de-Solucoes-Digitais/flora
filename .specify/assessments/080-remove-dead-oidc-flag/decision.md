# Decision: remover flag OIDC morta `local_fallback_enabled`

- **Slug**: 080-remove-dead-oidc-flag · **Decidido**: 2026-11-19T21:32Z
- **Verdict**: **go (scoped)** · Artefatos: intake | research | define | concept
- **80/20**: E=(3×5)/(1×1)=15,0 — valor 3 (remove garantia de segurança falsa + dívida de config), confiança 5 (evidência local integral), esforço 1 (3 linhas, 1 arquivo), risco 1 (zero leitores, sem consumidor de serialização). Alternativas do ciclo: `.env.example` 10,0 · `cssVariables` 5,0 · lucide 2,50 · OTLP 1,25 (sem receptor/dono G1, deps 0.33.1, risco alto).

## Pontuação

| Critério | Nota | Justificativa |
|---|---|---|
| Validade | strong | G9 (`docs/research/2026-11-16-seed-oidc-identity-gap.md:120`) e NV-7 (`...016-auth-rbac-oidc-minimal-vps.md:56`) registrou a lacuna; recomendação `:147` diz explicitamente "remover/implementar" |
| Evidência | strong | `flora-core/src/config/oidc.rs:18-19,40` colado; `grep` → zero leitores em Rust; sem `deny_unknown_fields` (`config/mod.rs:17-31`); `.env.example:58-68` sem a chave |
| Valor vs. inação | strong | Inação mantém default `true` que sugere fallback de segurança inexistente, enganando operadores (risco P1 em G9) |
| Viabilidade | strong | Opção A: deletar 3 linhas de um arquivo; sem DB, rede, segredos ou mudança de contrato de leitura |
| Fit | strong | Remove campo write-only exatamente onde ele vive; não cria segundo mecanismo nem shim; `mod.rs` e auth intactos |
| Risco | strong | `Config` sem `deny_unknown_fields` ⇒ env legado `FLORA_OIDC__LOCAL_FALLBACK_ENABLED` ignorado sem erro; sem consumidor de `Serialize` sobre config; diff=1 arquivo (SC-004) |

## Handoff

Opção A: em `flora-core/src/config/oidc.rs`, remover doc+campo (`:18-19`) e a inicialização em `:40` — 3 linhas. Dentro = `flora-core/src/config/oidc.rs` + 5 artefatos 080. Fora = auth (`auth.rs`, `services/auth_service.rs`), `.env.example`, docs de research, worktree `state-closure-072`, `config/mod.rs` e demais módulos, gate/`scripts/ci/check_gate_integrity.py`, `.github/workflows/`, ADRs, constituição, STATE, specs/080-* (exceto artefatos), DB/rede, segredos, `.tokens/`, fechamento de issues, leg RBAC e lockfiles. FR-001..FR-004 e SC-001..SC-004 de `define.md`. Gates determinísticos pós-edição (main agent): `cargo check -p flora-core`, `cargo fmt --check`, `cargo clippy -p flora-core --all-targets -D warnings`, `python3 scripts/ci/check_gate_integrity.py`, `grep -rn "local_fallback" --include='*.rs' flora-core/src apps` → zero. Perguntas carry-forward: `[NV]` execução dos gates e comportamento de env legado em deploy fora do repo — nenhuma bloqueia o go.
