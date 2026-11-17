# Research: 048 — Governance drift

**Data:** 2026-11-17T14:33Z · **Evidência local**; remoto **[NV]**.

## Primária local (executada)

- `AGENTS.md:4` — "Members: flora-core, flora-api, flora-messaging, flora-tasks, flora-files, flora-search, flora-tests."
- `AGENTS.md:7` — "Toolchain pinned `1.98.0`."
- `Cargo.toml:19` — `package.rust-version = "1.98.1"`; `:26-32` — 5 membros; `:14` — membros herdam `rust-version.workspace`.
- `cargo metadata --no-deps` (sessão anterior) → 6 pacotes `flora*`: os 5 membros + raiz `flora`.
- `.specify/bugs/rust-toolchain-pin-bump-1-98-1/{assessment,fix,test}.md` — UB documentada em 1.98.0; fix `applied`, `verified`; pin 1.98.1 em 12 pontos.
- `.specify/bugs/ci-test-step-runs-zero-tests/test.md` — `cargo build/test --workspace` verificado em `rust.yml:39,41` neste checkout.
- `ls flora-api/src/routes/` — `tasks.rs` e `files.rs` existem como **módulos de rota**, não como crates `flora-tasks`/`flora-files`; `grep -rn 'flora-tasks\|flora-files' flora-api/src` → 0 ocorrências.
- `rustc --version` local → 1.99.1 (host); irrelevante para o pin do CI.

## Consequência

`AGENTS.md` é a instrução que orienta agentes; números errados induzem decisões erradas
(de backlog a verificação de gates). Correção é factual, sem alterar regras.

## NV

Execução de CI em GitHub/Forgejo e branch protection: **[NV]** — não verificada nesta sessão.
