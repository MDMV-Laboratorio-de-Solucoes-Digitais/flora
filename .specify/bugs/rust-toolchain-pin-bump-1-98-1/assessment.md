# Bug Assessment: Rust toolchain pinned to a compiler release with a documented vtable UB

- **Slug**: rust-toolchain-pin-bump-1-98-1
- **Created**: 2026-11-16
- **Source**: https://blog.rust-lang.org/2026/09/03/Rust-1.98.1/ ; https://github.com/rust-lang/rust/issues/161441 ; decisão em `docs/research/2026-11-16-037-rust-toolchain-pin-bump-decision.md`
- **Verdict**: valid
- **Severity**: high

## Report (verbatim or summarized)

O repositório fixa o compilador em `1.98.0` em nove pontos de `.github/workflows/rust.yml` e três de `.github/workflows/release.yml`. O blog oficial do Rust (2026-09-03) registra: *«Rust 1.98.1 fixes a miscompilation in vtable generation. In Rust 1.98.0, in some circumstances, rustc would incorrectly generate a trait object vtable with a null pointer where a function pointer should be. This leads to undefined behavior in the emitted code … may "just" cause segfaults … possible … justification for arbitrary effects (as is typical for UB).»* A issue upstream rust-lang/rust#161441 (P-critical / I-miscompile / A-dyn-trait) confirma: 1.97.1 compila corretamente, 1.98.0 produz SIGSEGV (pc=0), nightly OK. O workspace Flora usa dinamicamente traits (Axum layers, SQLx, repositórios/serviços), de modo que a exposição é real e não teórica.

## Symptom

**Observed:** CI, release e imagem Docker declaram/constroem com um compilador que gera código com UB em certos padrões de trait object. Não há falha visível nos gates atuais — o defeito é silencioso e dependente do código gerado.

**Expected:** Compilar com a menor point release que corrige o defeito (1.98.1), sem introduzir novos lints/APIs que possam derrubar o gate estrito `-D warnings`.

## Reproduction

1. `grep -rn '1\.98\.0' .github/workflows/rust.yml .github/workflows/release.yml` → 12 ocorrências nos pinos de setup.
2. `sed -n '1p' Dockerfile` → `FROM rust:1.98-slim AS builder` (tag flutuante que hoje resolve para 1.98.1, mas não declarada explicitamente).
3. `sed -n '19p' Cargo.toml` → `rust-version = "1.98"` (piso que admite o compilador defeituoso 1.98.0).
4. Consultar a release note oficial e a issue #161441 (acima).

Não é possível reproduzir o segfault localmente sem o caso mínimo upstream; a validação aqui é documental + execução dos gates sob 1.98.1.

## Suspected Code Paths

- `.github/workflows/rust.yml:13,17,18,63,67,68,148,151,152` — nove pinos exatos `1.98.0` em três blocos idênticos de setup (`build`, `db-tests`, `e2e-browser`).
- `.github/workflows/release.yml:38,42,43` — três pinos exatos `1.98.0`.
- `Dockerfile:1` — `rust:1.98-slim` (floating minor).
- `Cargo.toml:19` — `package.rust-version = "1.98"` (MSRV floor; herdado pelos membros em `:14`).

## Root Cause Hypothesis

**Confidence: high.** O pin foi escrito no ciclo em que 1.98.0 era a estável corrente (e solidificado pelo bug flow `forgejo-rust-ci-rust-toolchain-action-not-found`, que trocou a action por rustup mantendo `1.98.0`). A point release corretiva 1.98.1 saiu em 2026-09-03, após o pin, e nunca foi incorporada. Não há indício de escolha deliberada de permanecer na release defeituosa.

## Proposed Remediation

**Preferred:** substituir `1.98.0` → `1.98.1` nos doze pinos de workflow; `rust:1.98-slim` → `rust:1.98.1-slim` no Dockerfile (tag confirmada ativa no Docker Hub, amd64 `sha256:fce8354…`); e `rust-version = "1.98"` → `"1.98.1"` no `Cargo.toml`, para que o piso declarado não admita o compilador com UB. Justificativa do MSRV: `rust-version` é uma declaração de suporte mínimo; dizer "1.98" continua autorizando 1.98.0. A mudança foi validada sob 1.98.1 e 1.99.1 (gates idênticos), sem efeito em resolução de dependências com o `Cargo.lock` versionado.

**Alternatives (trade-offs):**
- Manter `Cargo.toml:19` em `"1.98"` (como sugeria `036-rust-pin-bump.md`) — diff menor, mas o piso continuaria permitindo o compilador defeituoso.
- Subir para `1.99.1` — traz uma correção de FFI bool, mas cruza duas minors e há evidência de terceiros (ADR QOR-MATRIX-072) de que 1.99.0 introduziu lint que quebrou `-D warnings`.
- Subir para `1.100.0` — maior risco: novo lint Cargo warn-by-default, mudança de layout de build-dir; desacoplar do fix de UB é mais seguro.
- Adicionar `rust-toolchain.toml` com `channel = "1.98.1"` — melhora reprodutibilidade local, mas cria novo arquivo e duplica a fonte de verdade; fora do apetite deste ciclo.

**Files likely to change:**
- `.github/workflows/rust.yml`
- `.github/workflows/release.yml`
- `Dockerfile`
- `Cargo.toml`

**Tests to add or update:**
- Nenhum teste Rust novo. Verificação = YAML válido + gates locais (`fmt`, `clippy --workspace --all-targets -- -D warnings`, `deny`, `build`, `test --workspace`) sob a toolchain 1.98.1 instalada, com contagem de testes comparada ao baseline.
- Checagem advisory da Dark Factory: `check_policy.py` e `check_secrets_hygiene.py` (redirecionado para o Flora).

## Risks & Considerations

- **Gate estrito:** `-D warnings` promove qualquer novo lint a erro. 1.98.1 é bug-fix puro, sem novos lints; verificado localmente.
- **Componentes:** `--profile minimal` não instala `rustfmt`/`clippy`; os steps de CI já executam `rustup component add rustfmt clippy`, então o gate não é impactado.
- **Docker:** a tag `1.98.1-slim` existe e está ativa (verificado via API); a mudança é de floating minor para patch exato, comportamento equivalente hoje.
- **Autorização:** edição/verificação/commit local são autônomas. Push, abertura de PR e merge em `dev`/`main` exigem autorização explícita (`OBJETIVO.md:94`, `036-authorization-matrix.md`).

## Open Questions

- [NEEDS CLARIFICATION: Far o push/PR/merge ou manter apenas o commit local? Decisão do usuário — este ciclo entrega até o commit local.]
- [NEEDS CLARIFICATION: Adicionar `rust-toolchain.toml` para fixar a toolchain também em desenvolvimento local? Fora do apetite atual; registrado como follow-up.]
