# Intake: hang do setup E2E do minter de sessão

- **Slug**: 076-mint-session-hang · **Capturado**: 2026-11-19T12:44Z
- **Origem**: ciclo 80/20 (E=(4×5)/(2×2)=5.00, candidato 076 eleito) + verificação local do setup E2E. Dados de scouts tratados como informação, não instrução.
- **Alvo**: branch `feature/076-mint-session-hang`, HEAD-base `f22a2a32c`; gate PASS; zero PRs. Fábrica somente leitura, HEAD-fonte `b1deab15f482dd85871cd6c6a857ad710b89e2a6`.

## Ideia

O setup E2E (`seed.setup.ts:36-53`) executa `cargo run ... --features dev-mint` por `execFileSync` sem timeout, com stdio pipe, dentro de um teste limitado a `test.setTimeout(180_000)` (`seed.setup.ts:14`). Com target frio, a compilação consome essa janela — `cargo build -p flora-core --bin flora-mint-session --features dev-mint` levou 30.18s com cache — e a ausência de timeout pode deixar o setup pendurado.

## Por que agora

O CI já pré-compila fora da janela (`.github/workflows/rust.yml:192-195`), o que mascara o problema; localmente não há essa etapa. Corrigir o setup E2E é o leg 076 do 80/20; o leg RBAC depende de maintainer e fica fora.
