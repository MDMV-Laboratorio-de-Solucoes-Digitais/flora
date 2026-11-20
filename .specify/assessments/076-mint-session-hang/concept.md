# Shape: hang do setup E2E do minter de sessão

- **Slug**: 076-mint-session-hang · **Recomendado**: globalSetup (pré-build) · **Data**: 2026-11-19T12:44Z

## Opção A — globalSetup com pré-build (recomendada)

- **Sketch**: mover a compilação de `flora-mint-session` (`cargo build -p flora-core --bin flora-mint-session --features dev-mint`, 30.18s com cache) para o globalSetup do Playwright, fora da janela de `test.setTimeout(180_000)`; o setup passa a consumir o binário pronto. Appetite: small.
- **Trade-offs**: espelha o que o CI já faz em `.github/workflows/rust.yml:192-195`; ataca a causa (compilação dentro da janela) sem mexer no binário. Risco: exige ajuste da config do Playwright e do setup.
- **Rabbit holes**: recompilar dentro do teste; alterar `flora-core/src/bin/flora-mint-session.rs:47-103` (não é a causa).

## Opção B — aumentar o timeout

- **Sketch**: elevar `test.setTimeout(180_000)` (`seed.setup.ts:14`). Mascara o sintoma, não a causa; o `execFileSync` sem timeout (`seed.setup.ts:36-53`) continua sem limite superior.

## Opção C — nada

- **Sketch**: manter como está. O CI segue mascarando o problema e `e2e-browser` é `continue-on-error: true` (`.github/workflows/rust.yml:150`); o pendura local fica sem sinal.

## Fora de escopo

Código de produção, workflows, ADRs, constituição, STATE, specs/076-*, leg RBAC, fechamento de issues.

## Premissas

O binário é síncrono e rápido (0.00s direto; 0.39s com target quente); a variável é a compilação (30.18s com cache); tempo com target totalmente frio não medido.
