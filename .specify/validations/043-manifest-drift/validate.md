# Validation: 043 — Manifesto sem drift de tenancy

**Data:** 2026-11-17 · **Baseline:** `dev@2a68f024` · **Tipo:** docs-only + 1 teste.

## Gates

| Gate | Resultado |
|---|---|
| `cargo fmt --all -- --check` | limpo |
| `cargo clippy --workspace --all-targets -- -D warnings` | "No issues found" |
| `cargo deny check bans licenses sources` | ok |
| `cargo test --workspace` | verde (flora_api lib 29 passed vs 28 baseline = +1) |
| `cargo test -p flora-api health` | 3 passed (era 2) |

## Diff

- `health.rs`: 8 linhas do manifesto trocadas de "auth + X-Organization-ID" para
  "verified session Bearer token" + 2 linhas explicando que a org vem da sessão.
  Novo teste `test_llms_txt_does_not_advertise_header_as_auth`.
- `mod.rs`: comentário do roteador corrigido.
- `grep -rn 'X-Organization-ID' flora-api/src` → só o comentário/asserção do teste.

## Auditoria

Mudança docs-only, behavior-preserving; o teste de ausência é a verificação
determinística. Auditoria author-blind completa não necessária (risco zero, diff
restrito a strings + 1 teste); registrada como decisão 80/20.

## NV / fora do escopo

D3 (seed permission drift) e RLS/role least-privilege — ver 042/041, autorização SE.
