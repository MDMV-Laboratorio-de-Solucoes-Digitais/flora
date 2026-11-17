# Bug Fix: Rust toolchain pinned to a compiler release with a documented vtable UB

- **Slug**: rust-toolchain-pin-bump-1-98-1
- **Fixed**: 2026-11-16
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

Elevou o compilador de `1.98.0` (release com UB documentada em geração de vtable de trait object) para `1.98.1` — a menor point release que corrige o defeito — em todos os pinos de CI/release, na imagem Docker e no MSRV declarado. Nenhuma mudança de código Rust; a correção é inteiramente de configuração de toolchain.

## Changes

| File | Change | Notes |
|------|--------|-------|
| `.github/workflows/rust.yml` | 9 × `1.98.0` → `1.98.1` | Blocos de setup dos jobs `build` (13,17,18), `db-tests` (63,67,68) e `e2e-browser` (148,151,152), incluindo o bootstrap condicional do rustup. `rustup component add rustfmt clippy` preservado — necessário porque `--profile minimal` não instala os componentes. |
| `.github/workflows/release.yml` | 3 × `1.98.0` → `1.98.1` | Linhas 38, 42, 43 (nome do step, `toolchain install`, `rustup default`). |
| `Dockerfile` | `rust:1.98-slim` → `rust:1.98.1-slim` | Tag exata confirmada `active` na API do Docker Hub (amd64 `sha256:fce8354…`). Antes já resolvia para 1.98.1; agora declarado explicitamente. |
| `Cargo.toml` | `package.rust-version = "1.98"` → `"1.98.1"` | `rust-version` é piso de suporte, não pin; manter `"1.98"` continuaria autorizando o compilador defeituoso. Membros herdam via `rust-version.workspace = true` (`:14`). |

Diff: 4 arquivos, 14 inserções, 14 remoções. Nenhuma dependência adicionada/atualizada; `Cargo.lock` intacto.

## Diff Highlights (optional)

```diff
-      - name: Set up Rust 1.98.0
+      - name: Set up Rust 1.98.1
-          rustup toolchain install 1.98.0 --profile minimal --no-self-update
-          rustup default 1.98.0
+          rustup toolchain install 1.98.1 --profile minimal --no-self-update
+          rustup default 1.98.1
-FROM rust:1.98-slim AS builder
+FROM rust:1.98.1-slim AS builder
-package.rust-version = "1.98"
+package.rust-version = "1.98.1"
```

## Tests Added or Updated

- Nenhum teste Rust novo (mudança de configuração de toolchain).
- Verificação ampliada: gates executados sob `rustc 1.98.1 (48a229cea 2026-09-01)` / `cargo 1.98.1 (797e8a9bc 2026-08-05)`, instalados localmente para este ciclo.
- Controles advisory da Dark Factory executados contra o Flora (`check_policy.py`, `check_secrets_hygiene.py` redirecionado).

## Local Verification

Todos os comandos exit 0, exceto o scan de segredos (advisory, findings conhecidos — ver `test.md`):

| Comando | Resultado |
|---|---|
| `python3 -c 'import yaml; yaml.safe_load(...rust.yml); yaml.safe_load(...release.yml)'` | `yaml-ok` |
| `cargo +1.98.1 fmt --all -- --check` | limpo |
| `cargo +1.98.1 clippy --workspace --all-targets -- -D warnings` | `Finished dev profile in 14.03s`, sem warnings |
| `cargo +1.98.1 deny check bans licenses sources` | `bans ok, licenses ok, sources ok` |
| `cargo +1.98.1 build --workspace` | `Finished dev profile in 1m 35s` |
| `cargo +1.98.1 test --workspace` | **104 passed, 0 failed, 44 ignored** |
| `rustup toolchain install 1.98.1 --profile minimal --no-self-update` + `rustup component add --toolchain 1.98.1 rustfmt clippy` | instalado com sucesso |

Log completo: `/tmp/opencode/flora-037-gates.log` (efêmero, fora do repo). Resultado idêntico ao baseline sob 1.99.1 (104/44/0), confirmando ausência de regressão de lint.

## Deviations from Assessment

- **MSRV alterado:** a avaliação listou `Cargo.toml` entre os arquivos a mudar e explicou a razão (piso que admitia 1.98.0). Isso difere da sugestão minimalista em `docs/research/2026-11-16-036-rust-pin-bump.md:99-100` ("no change required"); a justificativa é honestidade do piso de suporte, não necessidade de compilação. Nenhuma outra divergência.
- `Dockerfile` foi fixado para o patch exato em vez de manter a tag flutuante `1.98-slim`: a tag `1.98.1-slim` existe e está ativa, então o comportamento é equivalente hoje e a reprodutibilidade melhora.

## Follow-ups

- Push/PR/merge em `dev`/`main` exigem autorização explícita; este ciclo entrega até o commit local.
- Observar o `Rust CI` remoto no PR para confirmar `rustc 1.98.1` nos logs dos três jobs de setup e nos checks `build`/`db-tests`/`dependency-review`.
- Considerar `rust-toolchain.toml` (`channel = "1.98.1"`) para fixar também o desenvolvimento local — fora do apetite deste ciclo.
- Candidato para o ciclo 038: mínimos de contagem nos seis legs `db-tests` (hoje um filtro que casa 0 testes ainda sai com exit 0).
