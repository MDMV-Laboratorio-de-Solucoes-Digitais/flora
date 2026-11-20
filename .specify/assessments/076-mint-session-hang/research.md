# Research: hang do setup E2E do minter de sessão

- **Coletado**: 2026-11-19T12:44Z · fontes: código local, config E2E e workflow de CI.

## Evidência

- `flora-core/src/bin/flora-mint-session.rs:47-103` — fluxo síncrono: sem DB, rede, stdin ou `await`. O binário em si não é a causa do hang.
- `flora-core/Cargo.toml:59-68` — feature `dev-mint` só habilita `clap`/`anyhow`; o custo é de compilação, não de runtime.
- Execução direta do binário: 0.00s; `cargo run ... --features dev-mint` com target quente: 0.39s.
- `cargo build -p flora-core --bin flora-mint-session --features dev-mint` com cache: 30.18s — a compilação é a parcela variável.
- `apps/frontend/tests/e2e/seed.setup.ts:14` — `test.setTimeout(180_000)` fixa a janela do teste.
- `apps/frontend/tests/e2e/seed.setup.ts:36-53` — `execFileSync` do `cargo run`, sem timeout e com stdio pipe; é aqui que a compilação pode travar a janela.
- `.github/workflows/rust.yml:192-195` — pré-compilação fora da janela de 180s (mascara o problema no CI).
- `.github/workflows/rust.yml:150` — job `e2e-browser` é `continue-on-error: true`.

## Riscos / lacunas

Tempo com target totalmente frio não medido (só 30.18s com cache é conhecido); `execFileSync` sem timeout não tem limite superior observado; `continue-on-error` pode esconder regressões; leg RBAC depende de maintainer — fora deste assessment.
