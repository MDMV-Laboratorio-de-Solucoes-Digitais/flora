# Idea Intake: Wildcard de recuperação com papel e one-shot

- **Slug**: `107-wildcard-one-shot`
- **Created**: 2026-11-22
- **Source**: INBOX autorizado pelo SE (`MDMV-Autonoma` revisão interagentes), bug `d11-wildcard-unbounded`
- **Type**: fix

## Idea (as captured)

> "MAJOR D11 — recuperação wildcard sem papel: restringir a papel administrativo, tornar o opt-in one-shot (auto-desligar após o primeiro login) e registrar ADR com data de expiração."

## Restated

Restringir o seletor temporário `*` a contas com papel administrativo e torná-lo utilizável uma única vez, com estado persistente e ADR de expiração.

## Origin & Context

- **Raised by**: SE, via revisão interagentes.
- **Trigger**: correção D15 concluída no ciclo 106; D11 é o próximo item autorizado do INBOX.

## First-Glance Unknowns

- [NEEDS CLARIFICATION: a expiração temporal do ADR deve ser 7 dias?]
- [NEEDS CLARIFICATION: o consumo deve ocorrer antes ou depois da emissão da sessão?]
- [NEEDS CLARIFICATION: restart/replica é coberto pelo marcador persistente?]
