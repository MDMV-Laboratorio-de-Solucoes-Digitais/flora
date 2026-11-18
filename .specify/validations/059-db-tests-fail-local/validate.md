# Validation: 059 — DB tests fail locally

**Data:** 2026-11-18T03:31Z · **Baseline:** `github/dev@2730fd2dd`
**Tipo:** correção de testes; sem CI/gate/autorização alterados.

## Gates e verificações (2026-11-18T03:28–03:30Z)

| Gate | Resultado |
|---|---|
| Sem env, leg `tasks` (`--ignored`) | exit **101** — 0 passed / 4 failed |
| Sem env, leg `auth` (`--ignored`) | exit **101** — 0 passed / 13 failed |
| `cargo test -- --ignored --list` | 50 testes |
| 8 legs com Postgres 18.6 isolado | exit 0 — 13/4/8/8/5/6/4/2 (50 reais) |
| Verificador isolado | PASS 5/5 |
| `python3 scripts/ci/check_gate_integrity.py` | PASS — 4 checks, 8 pisos / 50 total |
| `cargo fmt --all -- --check` | exit 0 (após formatação) |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit 0 |
| `cargo deny check bans licenses sources` | exit 0 |
| `cargo build --workspace` / `cargo test --workspace` | exit 0 / 0 |
| `bun run check` / `lint` | 0 erros, 0 warnings / exit 0 |
| `bun run test:unit -- --run --project server` | 42/42 |
| `bun run build` | exit 0 |

## Evidência

- Diff: 8 arquivos `.rs`, 9+/30−; `return Ok(None)` e `skipping DB-backed`
  → 0 ocorrências; `anyhow::bail!` → 9; 50 chamadores inalterados.
- `bail!` não viola lints: não usa `panic!`/`unwrap()`/`expect()` nem macros de
  print; nenhum `#[allow]` adicionado; `anyhow` já declarado.
- DB `flora` existente não tocada; `flora_059_verify` criada e removida.
- Divergência textual nos artefatos corrigida por notas append-only.

## SDD

Decisão 80/20 em `STATE.md` antes do BLOCO 6; `analyze` PASS; R1 `go (scoped)`;
FR 4/4, SC 4/4, T001–T005 concluídas; `converge.md` com entrada 1.
Scripts SDD da factory (HEAD `1083d321`, em `/tmp`): completeness/DAG/task-id
RED por dívida legada — informativo; converge append-only PASS.

## R4 — Veredito

**Approved (evidência local determinística; CI/PR ao vivo [NV]).** R4 aprovado
antes de R7. CI do PR observada ao vivo no Bloco 7 antes do merge (ADR-0003).

## NV

Runner/CI, proteção e aprovações ao vivo: **[NV]**. Sem merge/auto-merge/tag/deploy.
