# Bug Verification: Rust toolchain pinned to a compiler release with a documented vtable UB

- **Slug**: rust-toolchain-pin-bump-1-98-1
- **Tested**: 2026-11-16
- **Assessment**: ./assessment.md
- **Fix**: ./fix.md
- **Result**: verified

## Summary

A correção de configuração foi aplicada e **todos os gates locais equivalentes ao CI passaram sob a toolchain alvo 1.98.1**, com contagem de testes idêntica ao baseline (104 passed / 44 ignored / 0 failed). O YAML dos dois workflows é válido e os controles advisory da Dark Factory foram executados. O escopo do `verified` é estritamente local: execução em runner remoto (GitHub/Forgejo) e o repro upstream do segfault não foram rodados.

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
| CI remoto (GitHub/Forgejo) — `Rust CI` no PR com a mudança | — | not-run | Exige push/PR, que estão fora da autorização deste ciclo. |

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

- **CI remoto não executado:** os logs de `rustc --version` nos três blocos de setup e os checks de PR ainda não existem para este commit. Não reivindicar "CI verde" até o PR rodar.
- **Repro do segfault não executado:** o caso mínimo upstream (rust#161441) não foi compilado aqui; a remoção da UB é inferida da aplicação da point release oficial, não de um teste de runtime local.
- **Scripts de setup usam `--profile minimal`:** se um runner não executar `rustup component add rustfmt clippy`, os gates falham por componente ausente. Os workflows já fazem isso; risco residual apenas se esse passo for removido.
- **Ambiente local vs runner:** `1.98.1` foi instalada neste host; a disponibilidade de download no runner é a mesma assumida pelo padrão de CI já existente.

## Recommendation

Fechar o bug no escopo local e **submeter para verificação remota**: fazer push/abrir PR só com autorização explícita, confirmar `Rust CI` verde e `rustc 1.98.1` nos logs, então mesclar. Se o clippy remoto falhar com algum lint novo (improvável — patch release sem novos lints), reabrir com o log do PR. Próximo candidato 80/20: mínimos de contagem nos seis legs `db-tests` (`036-international-seed-verification.md:47,139`).
