# Implementation Plan: 072 — refresh verificado de estado

**Entrada**: `spec.md`, `clarify.md`, `research.md`, `decision.md` (go scoped).
**Ordem**: um arquivo por commit, paths explícitos, nunca `git add -A`/`.`.

## Passos

1. **T001** Apêndice em `OBJETIVO.md` com estado verificado 2026-11-19, âncora `cdf50eaff`, contagens 61/58/57/57/4/137 e cobertura 001–070. *(FR-001, FR-002, FR-003, SC-001, SC-003)*
2. **T002** Apêndice equivalente em `ROADMAP.md`. *(FR-001, FR-002, FR-003, SC-001, SC-003)*
3. **T003** Apêndice em `docs/PROGRESSO.md`, preservando a nota 02:10Z e citando explicitamente 069/070. *(FR-001, FR-002, FR-003, SC-001)*
4. **T004** Apêndice em `docs/research/index.md`, preservando 137/137 e corrigindo apenas contagens de specs/assess/validate/release. *(FR-001, FR-002, FR-003, SC-001)*
5. **T005** Nota curta em `index.md` da raiz, se ainda apontar âncora antiga. *(FR-005, SC-001)*
6. **T006** Apêndices em `.specify/memory/STATE.md`: decisão 80/20, execução e fechamento, com `[NV]`. *(FR-004, FR-005)*
7. **T007** `analyze.md` READ-ONLY; corrigir fonte se houver conflito e reexecutar. *(FR-005, SC-002)*
8. **T008** `converge.md` append-only. *(FR-005)*
9. **T009** Gates locais do alvo + `validate.md` (R4) e `release.md` (R7) após evidência colada. *(FR-004, SC-004)*
10. **T010** PR via `gh` para `dev/`, sem merge; higiene de worktree/branch própria. *(FR-004, SC-002, SC-004)*

## Gates

```
python3 scripts/ci/check_gate_integrity.py
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo deny check bans licenses sources
cargo build --workspace
cargo test --workspace
(frontend, se tocado — não é o caso): bun run check / lint / test:unit -- --run --project server / build
git diff --name-only dev..HEAD
git diff --numstat -- OBJETIVO.md ROADMAP.md docs/PROGRESSO.md docs/research/index.md
```

## Verificação append-only

Antes de editar cada documento: `sha256sum <arquivo>` + `wc -c <arquivo>`; após a edição, confirmar que os primeiros N bytes são idênticos e que o tamanho só aumentou.

## Rollback

Reverter os commits deste PR em `dev/` com novo commit de reversão; nenhum dado, schema, required check, piso, deploy ou produção é alterado. Não há force-push nem reescrita de histórico.
