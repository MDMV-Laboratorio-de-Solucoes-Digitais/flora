# Idea Intake: Bump de segurança rustls

- **Slug**: 090-rustls-security-bump
- **Created**: 2026-11-20
- **Source**: repositório local — `.specify/assessments/090-rustls-security-bump/decision.md` (+ `specs/090-rustls-security-bump/spec.md`)
- **Type**: compliance

## Idea (as captured)

O plano de job advisory (`docs/research/2026-11-16-028-advisory-job-plan.md:40`)
registra quatro alertas canônicos de Rust no `Cargo.lock` raiz, todos em
`rustls`: `RUSTSEC-2026-0285` (medium, correção `0.23.45`),
`RUSTSEC-2026-0346` (medium, `0.23.46`), `RUSTSEC-2026-0348` (low, `0.23.46`) e
`RUSTSEC-2026-0349` (low, `0.23.46`) (`decision.md:12-23`). O lockfile
registrava `0.23.43` em `Cargo.lock:2748`, abaixo das quatro versões corretivas,
enquanto a isenção cargo-vet já apontava `0.23.46` em
`supply-chain/config.toml:1115-1117` — divergência entre o grafo resolvido e a
exceção auditada (`decision.md:23-26`). O artefato registra como direção um
bump patch restrito ao lockfile e à isenção, preservando `rustls = "0.23"` em
`Cargo.toml:189` (`decision.md:32-57`;
`specs/090-rustls-security-bump/spec.md:9-14,18-22`).

## Restated

Propõe-se resolver `rustls` para `0.23.46` no grafo travado e alinhar a isenção
cargo-vet à mesma versão, a fim de cobrir os quatro advisories sem alterar a
faixa declarada no manifest. O escopo registrado é de dependência, não de
mudança de API, de gate ou de processo de merge.

## Origin & Context

- **Raised by**: Engenheiro de Software / maintainer da MDMV (`@lfgranja`), conforme registrado nos artefatos do ciclo.
- **Trigger**: registro dos quatro advisories no plano `docs/research/2026-11-16-028-advisory-job-plan.md:40`, com `0.23.43` no lock abaixo das versões corretivas e isenção cargo-vet já em `0.23.46` (`decision.md:12-30`).

## First-Glance Unknowns

- [NEEDS CLARIFICATION: em qual fonte ou ferramenta foi verificado que `0.23.46` corrige efetivamente as quatro advisories, e quem valida essa equivalência?]
- [NEEDS CLARIFICATION: [NV] qual é o estado do CI do PR, das aprovações de maintainer, da proteção de branch e do ambiente de produção, não reconsultados neste ciclo?]
- [NEEDS CLARIFICATION: o bump altera dependências transitivas de `rustls` ou qualquer outro pacote do grafo, mesmo sem mudança de API?]
- [NEEDS CLARIFICATION: se `dependency-review` ou o gate acusarem achados após o bump, quem decide o descarte ou o seguimento?]
- [NEEDS CLARIFICATION: a isenção cargo-vet com `criteria = "safe-to-deploy"` continua adequada para a versão corrigida ou exige reavaliação do critério?]
