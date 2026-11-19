# Validation: 071 — remover stub morto AuthService

**Data:** 2026-11-19T04:05Z · **Baseline:** `github/dev@cdf50eaff`
**Tipo:** remoção de código morto; nenhuma mudança de comportamento em produção.
**Veredito R4:** **APROVADO**.

## Gates e verificações (saídas coladas)

| Gate | Resultado |
|---|---|
| `grep -rn 'AuthService\|auth_service' --include='*.rs' --exclude-dir=target .` | **0 ocorrências** |
| `python3 scripts/ci/check_gate_integrity.py` | `gate-integrity: PASS`; 4 required checks; `all 8 db contract floors intact (51 total)`; exit 0 |
| `check_gate_integrity.py --remote` | `dev`/`main`: live required contexts match; exit 0; `[NV]` runner/Forgejo |
| `cargo fmt --all -- --check` | RED após a remoção dos imports; **exit 0** após `cargo fmt --all` + commit `style(rust)` |
| `cargo clippy --workspace --all-targets -- -D warnings` | `Finished dev profile`; exit 0 |
| `cargo deny check bans licenses sources` | `bans ok, licenses ok, sources ok`; exit 0 |
| `cargo build --workspace` | exit 0 |
| `cargo test --workspace` | `passed=105 failed=0 ignored=55`; exit 0 |
| `cargo check --workspace --all-targets` | exit 0 (342 crates) |
| Frontend check/lint/test/build | 0/0; Prettier/ESLint ok; 42/42; exit 0 |
| `git diff --check dev..HEAD` | exit 0 |
| `analyze.md` | 1 conflito (FR-004) corrigido na fonte → `go=True`, `errors=[]` |
| Verificador isolado | `PASS-WITH-NITS` — 5 pontos OK + 1 nit de contagem (10, não 11 arquivos/commits); corrigido por notas append-only |

## Fronteira verificada

- `git diff --name-only dev..HEAD` → **10 arquivos Rust** (1 excluído + 9 com referências removidas), 5 assessments, 7 specs, `STATE.md` (mais validate/release deste ciclo). Zero workflow, gate, migration, ADR ou constituição.
- `Error::ServiceUnavailable` preservado (`error.rs:177`, `oidc_transactions.rs:85,119`); trait `AuthProvider`/`UserInfo` intactos (`traits/auth_provider.rs:14,46`, reexport em `traits/mod.rs:7`).
- `routes/auth.rs`, `oidc_transactions.rs`, `session_service.rs` e a lógica do extractor não aparecem no diff; nenhum campo/comentário órfão em `AppState`.
- Nenhuma credencial, segredo ou padrão de injeção nos artefatos novos.

## Dívida / [NV]

Leg `rbac` no CI e renomeação de títulos sem prefixo de ciclo permanecem como dívidas anteriores; a emenda da constituição exige maintainer. CI futura, Forgejo, produção e aprovações: **[NV]** — CI ao vivo observada no BLOCO 7 antes do merge.

## SDD

R1 `go (scoped)`; R3 zero perguntas; R4 este documento aprovado antes de `release.md`; R5 sem bug reportado; R7 release com SemVer/changelog/rollback. Nenhuma invocação de `/speckit.validate` ou `/speckit.release`.
