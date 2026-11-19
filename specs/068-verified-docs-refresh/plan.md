# Implementation Plan: 068 — refresh verificado de documentação

**Entrada:** `spec.md`, `clarify.md`, `research.md`, `decision.md` (go).
**Ordem:** uma unidade por vez; commit de um arquivo por commit (Constituição §VI).

## Passos

1. **T001** — `docs/research/index.md`: cabeçalho `135 → 137`; tabela `135→137`, `202→58 diretórios (280 arquivos)`, `175→55 diretórios`, `43→54`, `42→54`, `12→4`; apêndice datado preservando a dívida 047+. *(FR-001, SC-001)*
2. **T002** — `docs/PROGRESSO.md`: apêndice `## Estado verificado — <ts> (append-only)` com contagens `find`, gate 8/51 + `--remote`, âncora `dev@cbad031b5` = `github/dev`, 0 PRs e runs verdes observados, **[NV]** futuro. *(FR-002, SC-001, SC-002)*
3. **T003/T004** — `OBJETIVO.md` e `ROADMAP.md`: nota datada de intervalo até 067 e ponteiro para PROGRESSO/index. *(FR-003)*
4. **T005** — `flora-api/src/routes/rbac.rs`: substituir as 2 linhas do comentário `//!` obsoleto; diff restrito ao cabeçalho. *(FR-004, SC-003)*
5. **T006** — `.specify/memory/STATE.md`: apêndice de execução (decisão já registrada antes do BLOCO 6). *(FR-005)*
6. **T007** — artefatos `analyze.md` (READ-ONLY) e `converge.md` (append-only).
7. **T008** — gates locais (Bloco 7.1), verificação isolada, PR, merge após CI verde, higiene.

## Gates

```
python3 scripts/ci/check_gate_integrity.py            # autoritativo
python3 scripts/ci/check_gate_integrity.py --remote   # autoridade de merge
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo deny check bans licenses sources
cargo build --workspace
cargo test --workspace
(cd apps/frontend && bun run check && bun run lint && bun run test:unit -- --run --project server && bun run build)
git diff --name-only   # revisão de fronteira
```

## Rollback

Reverter os commits do PR em `dev/` (novo commit de reversão, nunca force-push). Nenhuma mudança de dados, schema, workflow ou produção; portanto rollback é puramente documental e sem risco de estado.
