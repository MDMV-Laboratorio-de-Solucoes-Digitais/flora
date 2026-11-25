# Idea Intake: 041 — Rust dependency debloat

- **Slug**: 041-rust-dead-crates
- **Created**: 2026-11-25
- **Source**: repositório local — `.specify/assessments/041-rust-dead-crates/decision.md` (+ `specs/041-rust-dead-crates/spec.md`)
- **Type**: cost-saving

## Idea (as captured)

O ciclo 041 registra em `decision.md:5-9` e `specs/041-rust-dead-crates/spec.md:6-21` a remoção de quatro dependências Rust (dotenv, argon2, bcrypt, rust-i18n) e de dois members de workspace órfãos (flora-tasks, flora-files), descritos como tendo zero call sites verificados em `*.rs` e `Cargo.toml`, sem `build.rs`, feature-gate ou reexport. O artefato registra um delta de 34 pacotes / 369 linhas do `Cargo.lock`, apenas deleções, apresentado como contrapartida backend do debloat do ciclo 040 para o objetivo "VPS barata" (`spec.md:7-9`).

## Restated

Propõe-se enxugar o grafo de dependências Rust e o tamanho do checkout removendo crates e members de workspace sem uso. A mudança é registrada como remoção pura, com efeito comprovado apenas sobre o tamanho do grafo e da configuração.

## Origin & Context

- **Raised by**: Engenheiro de Software / maintainer da MDMV (`@lfgranja`), conforme registrado nos artefatos do ciclo.
- **Trigger**: Objetivo "VPS barata" registrado em `spec.md:7-9` como contrapartida backend do debloat frontend do ciclo 040; `decision.md:5-9` registra dependências e members sem call sites como alvo da remoção.

## First-Glance Unknowns

- [NEEDS CLARIFICATION: Como confirmar que as dependências e os members não possuem call sites indiretos via `build.rs`, feature-gate ou reexport?]
- [NEEDS CLARIFICATION: Qual efeito mensurável em memória/CPU se espera, além da redução do tamanho do grafo?]
- [NEEDS CLARIFICATION: Quem autoriza o papel least-privilege e as políticas de RLS, hoje marcados como [NV] (c041-03)?]
- [NEEDS CLARIFICATION: O que falta para o wiring do RbacService entrar neste escopo, incluindo assign/revoke e proteção last-Owner (c041-02)?]
- [NEEDS CLARIFICATION: A contagem de 46 passed / 43 ignored permanece idêntica após a regeneração do `Cargo.lock`?]
- [NEEDS CLARIFICATION: As isenções removidas do cargo-vet afetam `dotenvy` ou `jsonwebtoken/rust_crypto`, que devem ser mantidos?]
