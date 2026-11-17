# Bug Verification: Rust toolchain pinned to a compiler release with a documented vtable UB

- **Slug**: rust-toolchain-pin-bump-1-98-1
- **Tested**: 2026-11-16
- **Assessment**: ./assessment.md
- **Fix**: ./fix.md
- **Result**: verified

## Summary

A correção de configuração foi aplicada e **todos os gates locais equivalentes ao CI passaram sob a toolchain alvo 1.98.1**, com contagem de testes idêntica ao baseline (104 passed / 44 ignored / 0 failed). O YAML dos dois workflows é válido e os controles advisory da Dark Factory foram executados. A verificação remota foi posteriormente realizada no PR #252: todos os checks passaram, com 43 contratos DB e 104 testes unitários executados e `rustc 1.98.1` confirmado nos logs. Permanece NV apenas o repro upstream do segfault (a remoção da UB é inferida da aplicação da point release oficial); Forgejo não foi exercised neste ciclo.

## Checks Performed

| Check | Command / Action | Result | Notes |
|-------|------------------|--------|-------|
| Toolchain alvo disponível | `rustup toolchain install 1.98.1 --profile minimal --no-self-update` + `rustup component add --toolchain 1.98.1 rustfmt clippy` | pass | `rustc 1.98.1 (48a229cea 2026-09-01)`, `cargo 1.98.1 (797e8a9bc 2026-08-05)`. Componentes adicionados porque `minimal` não os inclui. |
| Reprodução documental do defeito | `grep -rn '1\.98\.0'` nos workflows + release note oficial + issue #161441 | pass | 12 pinos encontrados antes da edição; 0 depois. UB confirmada por fonte primária. |
| Sintaxe YAML | `python3 -c 'import yaml; yaml.safe_load(open(".github/workflows/rust.yml")); yaml.safe_load(open(".github/workflows/release.yml"))'` | pass | `yaml-ok`. |
| Formato | `cargo +1.98.1 fmt --all -- --check` | pass | exit 0, sem diff. |
| Clippy estrito | `cargo +1.98.1 clippy --workspace --all-targets -- -D warnings` | pass | exit 0; `Finished dev profile in 14.03s`; nenhum warning — a hipótese de risco de lint foi refutada. |
| Supply-chain | `cargo +1.98.1 deny check bans licenses sources` | pass | `bans ok, licenses ok, sources ok`. |
| Build | `cargo +1.98.1 build --workspace` | pass | exit 0; `Finished dev profile in 1m 35s`. |
| Suíte de testes | `cargo +1.98.1 test --workspace` | pass | **104 passed, 0 failed, 44 ignored** — idêntico ao baseline sob 1.99.1. |
| Tag Docker | API do Docker Hub `library/rust/tags/1.98.1-slim` | pass | `tag_status: active`, amd64 `sha256:fce8354…`. |
| Dark Factory — política | `python3 <DF>/scripts/policy/check_policy.py --command "cargo build --workspace" --policy <DF>/scripts/policy/policy.yaml` | pass | `{"decision":"allow","reason":"command allowed by policy"}` — advisory. |
| Dark Factory — segredos | `check_secrets_hygiene.py` redirecionado para escanear o Flora | advisory FAIL | 5 findings em fixtures/docs (`jwt.rs`, `028-tauri-smoke-closure.md`, `apps/web/...`); **nenhum** nos 4 arquivos alterados. Heurística conhecida; não é gate do Flora. |
| CI remoto (GitHub) — PR #252, run `43995602151` | `gh pr checks 252` + `gh run view 43995602151 --log` | pass | Todos os checks terminal: `frontend-build` pass, `rust-build` pass, `db-tests` pass, `dependency-review` pass; `supply-chain-vet`/`e2e-browser` pass (informativos). |
| Evidência de toolchain no CI | log do run `43995602151` | pass | `rustc 1.98.1 (48a229cea 2026-09-01)` nos 3 blocos de setup (`build`, `db-tests`, `e2e-browser`). |
| Evidência de testes DB no CI | log do run `43995602151`, step `db-tests` | pass | Pisos executados e atendidos: auth 12/12, messaging 4/4, websocket 8/8, search 8/8, notifications 5/5, files 6/6 = **43 contratos**. |
| Evidência da suíte unitária no CI | log do run `43995602151`, step `Test` | pass | 25 + 12 + 21 + 46 = **104 passed**, 43 ignored. `bans ok, licenses ok, sources ok`. |
| Merge | PR #252 → `dev`, merge commit `23c8febc3` | pass | Realizado após verificação determinística, sem aprovação humana (ADR-0003). |

## Output Excerpts

```
TOOLCHAIN: cargo 1.98.1 (797e8a9bc 2026-08-05)
rustc 1.98.1 (48a229cea 2026-09-01)
$ python3 -c '...yaml.safe_load...'   -> yaml-ok / EXIT=0
$ cargo +1.98.1 fmt --all -- --check  -> EXIT=0
$ cargo +1.98.1 clippy --workspace --all-targets -- -D warnings
    Finished `dev` profile in 14.03s   -> EXIT=0
$ cargo +1.98.1 deny check bans licenses sources
    bans ok, licenses ok, sources ok   -> EXIT=0
$ cargo +1.98.1 build --workspace
    Finished `dev` profile in 1m 35s   -> EXIT=0
$ cargo +1.98.1 test --workspace
    passed=104 failed=0 ignored=44     -> EXIT=0
```

## Residual Risks

- **CI remoto verificado no PR #252, não em Forgejo:** `rustc 1.98.1`, os 43 contratos DB e os 104 testes aparecem nos logs do run `43995602151`. A instância Forgejo (`origin`) não foi exercitada neste ciclo — NV.
- **Repro do segfault não executado:** o caso mínimo upstream (rust#161441) não foi compilado aqui; a remoção da UB é inferida da aplicação da point release oficial, não de um teste de runtime local.
- **Scripts de setup usam `--profile minimal`:** se um runner não executar `rustup component add rustfmt clippy`, os gates falham por componente ausente. Os workflows já fazem isso; risco residual apenas se esse passo for removido.
- **Ambiente local vs runner:** `1.98.1` foi instalada neste host; a disponibilidade de download no runner é a mesma assumida pelo padrão de CI já existente.

## Recommendation

Bug **fechado**: push, PR #252, verificação remota e merge (`23c8febc3`) foram executados por decisão de máquina sob os gates determinísticos, conforme ADR-0003. O próprio gap de contagem nos seis legs `db-tests` foi corrigido no mesmo PR (pisos 12/4/8/8/5/6). Próximos candidatos: promover `supply-chain-vet`/`e2e-browser` a gates ou fixar actions por SHA (`036-ci-baseline.md`, auditoria 037).
