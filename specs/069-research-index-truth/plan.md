# Implementation Plan: 069 — corrigir nota falsa de dívida no índice de research

**Entrada:** `spec.md`, `clarify.md`, `research.md`, `decision.md` (go).
**Ordem:** um arquivo por commit, paths explícitos (Constituição §VI).

## Passos

1. **T001** `docs/research/index.md` — apêndice datado com o resultado da verificação de conjunto e o comando reprodutível; observação sobre atribuição de ciclo nos títulos. *(FR-001, FR-002, SC-001)*
2. **T002** `.specify/memory/STATE.md` — apêndice de execução. *(FR-003)*
3. **T003** artefatos `analyze.md` (READ-ONLY) e `converge.md` (append-only). *(FR-003)*
4. **T004** `validate.md` (R4) e `release.md` (R7) após gates. *(SC-002)*
5. **T005** gates locais, PR, merge após CI verde, higiene. *(SC-002, SC-003)*

## Gates

```
python3 scripts/ci/check_gate_integrity.py            # autoritativo
python3 scripts/ci/check_gate_integrity.py --remote
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo deny check bans licenses sources
cargo build --workspace
cargo test --workspace
(cd apps/frontend && bun run check && bun run lint && bun run test:unit -- --run --project server && bun run build)
comparação de conjuntos: 137 arquivos / 137 links / missing=[]
git diff --name-only dev..HEAD
```

## Rollback

Reverter os commits do PR em `dev/` com novo commit de reversão (sem force-push e sem reescrita). Mudança é puramente documental; sem dados, schema, workflow ou produção.
