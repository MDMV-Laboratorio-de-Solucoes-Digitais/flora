# Validation: 066 — RBAC RoleWrite

**Data:** 2026-11-18T14:50Z · **Baseline:** `github/dev@8d0cb64b0`
**Tipo:** código de API; sem RLS/deploy/ADR.

## Gates e verificações

| Gate | Resultado |
|---|---|
| `cargo test -p flora-api rbac` | 5 passed |
| `python3 scripts/ci/check_gate_integrity.py` | PASS — 4 checks, 8 pisos / 51 total |
| `cargo fmt --all -- --check` | exit 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit 0 (após correção) |
| `cargo deny check bans licenses sources` | exit 0 |
| `cargo build --workspace` / `cargo test --workspace` | exit 0 / 0 |
| `bun run check/lint/test/build` | 0/0, ok, 42/42, ok |
| Verificador isolado | em execução — resultado antes do merge |

## Evidência

- Diff de produção: 1 arquivo (`flora-api/src/routes/rbac.rs`), 194 inserções/25 remoções.
- `RoleWrite` já existia no modelo (`role.rs:173`) e nos papéis builtin do seed;
  nenhuma migration necessária.
- Tenant sempre da sessão verificada; lookup de papel por org.
- Builtins protegidos por helper puro testado; papéis inválidos rejeitados
  por `Permission::from_str`.

## SDD

Autorização SE em `STATE.md` antes do BLOCO 6; `analyze` PASS; R1 `go (scoped)`;
FR 6/6, SC 4/4, T001–T004 concluídas; `converge.md` com entrada 1.
Scripts SDD da factory (HEAD `c81f0f81`, em `/tmp`): completeness/DAG/task-id
RED por dívida legada — informativo; converge append-only PASS.

## R4 — Veredito

**Approved (evidência local determinística; CI/PR [NV]).** R4 aprovado antes de R7.

## NV

Comportamento em DB real e CI/PR ao vivo: **[NV]** — CI observada no Bloco 7.
