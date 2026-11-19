# Decision: remoção da feature `sqlite` em flora-core

- **Slug**: 077-remove-sqlite-feature · **Decidido**: 2026-11-19T19:42Z
- **Verdict**: **go (scoped)** · Artefatos: intake | research | define | concept
- **80/20**: E=(2×5)/(1×1)=10,00 — candidato 077 eleito (Tauri 3,75; GIN-JSONB 2,00; OTLP 1,67); leg RBAC bloqueada por maintainer.

## Pontuação

| Critério | Nota | Justificativa |
|---|---|---|
| Validade | strong | Stack revoga SQLite (§4 linha 52); `default = []` (`flora-core/Cargo.toml:60`), sqlx postgres-only (`Cargo.toml:143`), zero consumidores |
| Evidência | strong | Linhas coladas em manifests/config; varredura determinística sem correspondências |
| Valor vs. inação | strong | Inação mantém feature enganosa que contradiz `.env.example:81` e o padrão normativo |
| Viabilidade | strong | Opção A: uma linha, sem DB/rede, sem segredos |
| Fit | strong | Alinhamento direto à stack normativa (§1 linha 13; §4 linhas 48, 51-52) |
| Risco | strong | Diff de uma linha; default, sqlx e gate intactos (SC-002/SC-004) |

## Handoff

Opção A (remover a linha 61 de `flora-core/Cargo.toml`); dentro = esse manifest + 5 artefatos 077; fora = `Cargo.toml` raiz, `.env.example`, código Rust, workflows, gate/scripts, `deny.toml`, ADRs, constituição, STATE, specs/077-*, DB/rede, segredos, fechamento de issues, leg RBAC. FR-001..FR-004 e SC-001..SC-004 de `define.md`. Perguntas carry-forward: nenhuma. `[NV]` extensões de arquivo não varridas; necessidade offline real decidiria por ADR de projeto (§4 linha 52).
